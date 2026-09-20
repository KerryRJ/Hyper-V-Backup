use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use windows::Win32::Foundation::*;
use windows::Win32::Storage::FileSystem::{self, CopyFileW, CreateFileW, ReadFile, SetFilePointerEx, WriteFile};
use windows::Win32::Storage::Vhd::{self, OPEN_VIRTUAL_DISK_FLAG_NONE, OPEN_VIRTUAL_DISK_PARAMETERS, OPEN_VIRTUAL_DISK_PARAMETERS_0, VIRTUAL_STORAGE_TYPE};
use windows::Win32::Storage::Vhd::{GetVirtualDiskInformation, OpenVirtualDisk, QueryChangesVirtualDisk, SetVirtualDiskInformation};
use windows::core::PCWSTR;
use crate::infra::VirtualHardDiskSettingData;

#[derive(Clone, Debug, serde::Serialize)]
pub(crate) struct ChangedRange {
    pub(crate) byte_offset: u64,
    pub(crate) byte_length: u64,
}

/// A safe RAII wrapper that automatically manages the Virtual Disk lifecycle.
pub(crate) struct VirtualDisk {
    handle: HANDLE,
}

impl VirtualDisk {
    pub(crate) fn open(virtual_hard_disk_setting_data: &VirtualHardDiskSettingData) -> std::result::Result<Self, windows::core::Error> {
        Self::open_path(OsStr::new(virtual_hard_disk_setting_data.path.as_str()))
    }

    pub(crate) fn open_path<P: AsRef<OsStr>>(path: P) -> std::result::Result<Self, windows::core::Error> {
        let virtual_storage_type = VIRTUAL_STORAGE_TYPE {
            DeviceId: Vhd::VIRTUAL_STORAGE_TYPE_DEVICE_VHDX,
            VendorId: Vhd::VIRTUAL_STORAGE_TYPE_VENDOR_MICROSOFT,
        };
        let open_virtual_disk_parameters = OPEN_VIRTUAL_DISK_PARAMETERS {
            Version: Vhd::OPEN_VIRTUAL_DISK_VERSION_2,
            Anonymous: OPEN_VIRTUAL_DISK_PARAMETERS_0 {
                Version2: Vhd::OPEN_VIRTUAL_DISK_PARAMETERS_0_1 {
                    GetInfoOnly: false.into(),
                    ReadOnly: true.into(),
                    ResiliencyGuid: Default::default(),
                },
            },
        };
        let mut handle = HANDLE::default();
        let path = wide_path(path);
        unsafe {
            OpenVirtualDisk(&virtual_storage_type, PCWSTR(path.as_ptr()), Vhd::VIRTUAL_DISK_ACCESS_NONE, OPEN_VIRTUAL_DISK_FLAG_NONE, Some(&open_virtual_disk_parameters), &mut handle).ok()?;
        }
        Ok(Self { handle })
    }

    pub(crate) async fn virtual_disk_changes(&self, change_tracking_id: &OsStr, bytes: u64) -> std::result::Result<Vec<ChangedRange>, windows::core::Error> {
        unimplemented!("QueryChangesVirtualDisk is not yet implemented")
    }

    pub(crate) fn copy_file<P: AsRef<OsStr>, Q: AsRef<OsStr>>(source: P, destination: Q) -> std::result::Result<(), windows::core::Error> {
        let source = wide_path(source);
        let destination = wide_path(destination);
        unsafe { CopyFileW(PCWSTR::from_raw(source.as_ptr()), PCWSTR::from_raw(destination.as_ptr()), false) }
    }

    pub(crate) fn copy_ranges<P: AsRef<OsStr>, Q: AsRef<OsStr>>(source: P, destination: Q, ranges: &[ChangedRange]) -> std::result::Result<u64, windows::core::Error> {
        let source = wide_path(source);
        let destination = wide_path(destination);
        let source = unsafe { CreateFileW(PCWSTR::from_raw(source.as_ptr()), FileSystem::FILE_GENERIC_READ.0, FileSystem::FILE_SHARE_READ, None, FileSystem::OPEN_EXISTING, FileSystem::FILE_ATTRIBUTE_NORMAL, None)? };
        let source = FileHandle(source);
        let destination = unsafe { CreateFileW(PCWSTR::from_raw(destination.as_ptr()), FileSystem::FILE_GENERIC_WRITE.0, FileSystem::FILE_SHARE_READ, None, FileSystem::CREATE_ALWAYS, FileSystem::FILE_ATTRIBUTE_NORMAL, None)? };
        let destination = FileHandle(destination);
        let mut copied = 0u64;
        let mut buffer = vec![0u8; 1024 * 1024];

        for range in ranges {
            unsafe {
                SetFilePointerEx(source.0, range.byte_offset as i64, None, FileSystem::FILE_BEGIN)?;
            }
            let mut remaining = range.byte_length;
            while remaining > 0 {
                let requested = remaining.min(buffer.len() as u64) as u32;
                let mut read = 0u32;
                unsafe {
                    ReadFile(source.0, Some(&mut buffer[..requested as usize]), Some(&mut read), None)?;
                }
                if read == 0 {
                    return Err(windows::core::Error::from_thread());
                }
                let mut written = 0u32;
                unsafe {
                    WriteFile(destination.0, Some(&buffer[..read as usize]), Some(&mut written), None)?;
                }
                if written != read {
                    return Err(windows::core::Error::from_thread());
                }
                remaining -= read as u64;
                copied += read as u64;
            }
        }

        Ok(copied)
    }

    /// Sets the state of Resilient Change Tracking (RCT)
    pub(crate) fn set_change_tracking(&self, enabled: bool) -> std::result::Result<(), windows::core::Error> {
        let disk_info = Vhd::SET_VIRTUAL_DISK_INFO {
            Version: Vhd::SET_VIRTUAL_DISK_INFO_CHANGE_TRACKING_STATE,
            Anonymous: Vhd::SET_VIRTUAL_DISK_INFO_0 { ChangeTrackingEnabled: enabled.into() },
        };

        unsafe {
            SetVirtualDiskInformation(self.handle, &disk_info).ok()?;
        }

        Ok(())
    }

    pub(crate) fn query_changes(&self, change_tracking_id: &OsStr, byte_length: u64) -> std::result::Result<Vec<ChangedRange>, windows::core::Error> {
        let mut encoded: Vec<u16> = change_tracking_id.encode_wide().collect();
        encoded.push(0);

        let mut changes = Vec::new();
        let mut byte_offset = 0u64;
        while byte_offset < byte_length {
            let mut ranges = vec![Vhd::QUERY_CHANGES_VIRTUAL_DISK_RANGE::default(); 256];
            let mut range_count = ranges.len() as u32;
            let mut processed_length = 0u64;

            unsafe {
                QueryChangesVirtualDisk(
                    self.handle,
                    PCWSTR::from_raw(encoded.as_ptr()),
                    byte_offset,
                    byte_length - byte_offset,
                    Vhd::QUERY_CHANGES_VIRTUAL_DISK_FLAG_NONE,
                    ranges.as_mut_ptr(),
                    &mut range_count,
                    &mut processed_length,
                )
                .ok()?;
            }

            ranges.truncate(range_count as usize);
            changes.extend(ranges.into_iter().map(|range| ChangedRange {
                byte_offset: range.ByteOffset,
                byte_length: range.ByteLength,
            }));

            if processed_length == 0 {
                break;
            }
            byte_offset = byte_offset.saturating_add(processed_length);
        }

        Ok(changes)
    }

    pub(crate) fn change_tracking_state(&self) -> std::result::Result<(bool, String), windows::core::Error> {
        let mut info = Vhd::GET_VIRTUAL_DISK_INFO {
            Version: Vhd::GET_VIRTUAL_DISK_INFO_CHANGE_TRACKING_STATE,
            Anonymous: Vhd::GET_VIRTUAL_DISK_INFO_0 { ChangeTrackingState: Default::default() },
        };
        let mut info_size = std::mem::size_of::<Vhd::GET_VIRTUAL_DISK_INFO>() as u32;

        unsafe {
            GetVirtualDiskInformation(self.handle, &mut info_size, &mut info, None).ok()?;
        }

        let state = unsafe { info.Anonymous.ChangeTrackingState };
        let id = String::from_utf16_lossy(&state.MostRecentId).trim_end_matches('\0').to_owned();
        Ok((state.Enabled.as_bool(), id))
    }
}

fn wide_path<P: AsRef<OsStr>>(path: P) -> Vec<u16> {
    let mut encoded: Vec<u16> = path.as_ref().encode_wide().collect();
    encoded.push(0);
    encoded
}

struct FileHandle(HANDLE);

impl Drop for FileHandle {
    fn drop(&mut self) {
        if !self.0.is_invalid() {
            unsafe {
                let _ = CloseHandle(self.0);
            }
        }
    }
}

impl Drop for VirtualDisk {
    fn drop(&mut self) {
        if !self.handle.is_invalid() {
            unsafe {
                let _ = CloseHandle(self.handle);
            }
        }
    }
}
