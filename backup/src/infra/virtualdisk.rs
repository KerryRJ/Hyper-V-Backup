use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use futures::Stream;
use windows::Win32::Foundation::*;
use windows::Win32::Storage::FileSystem::{self, CopyFileW, CreateFileW, ReadFile, SetFilePointerEx, WriteFile};
use windows::Win32::Storage::Vhd::{self, OPEN_VIRTUAL_DISK_FLAG_NONE, OPEN_VIRTUAL_DISK_PARAMETERS, OPEN_VIRTUAL_DISK_PARAMETERS_0, QUERY_CHANGES_VIRTUAL_DISK_FLAG_NONE, VIRTUAL_STORAGE_TYPE};
use windows::Win32::Storage::Vhd::{GetVirtualDiskInformation, OpenVirtualDisk, QueryChangesVirtualDisk, SetVirtualDiskInformation};
use windows::Win32::System::Threading::GetCurrentProcess;
use windows::core::PCWSTR;
use crate::infra::VirtualHardDiskSettingData;

#[path = "changedrangestream.rs"]
mod changedrangestream;
#[path = "filehandle.rs"]
mod filehandle;
#[path = "virtualdiskrange.rs"]
mod virtualdiskrange;
use filehandle::FileHandle;
use changedrangestream::ChangedRangeStream;
pub(crate) use virtualdiskrange::VirtualDiskRange;

/// A safe RAII wrapper that automatically manages the Virtual Disk lifecycle.
pub(crate) struct VirtualDisk {
    handle: HANDLE,
}

impl VirtualDisk {
    pub(crate) async fn open(virtual_hard_disk_setting_data: &VirtualHardDiskSettingData) -> std::result::Result<Self, windows::core::Error> {
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
        let path = wide_path(OsStr::new(virtual_hard_disk_setting_data.path.as_str()));
        unsafe {
            OpenVirtualDisk(&virtual_storage_type, PCWSTR(path.as_ptr()), Vhd::VIRTUAL_DISK_ACCESS_NONE, OPEN_VIRTUAL_DISK_FLAG_NONE, Some(&open_virtual_disk_parameters), &mut handle).ok()?;
        }
        Ok(Self { handle })
    }

    pub(crate) fn virtual_disk_changes<'a>(&'a self, change_tracking_id: &OsStr, bytes: u64) -> std::result::Result<impl Stream<Item = std::result::Result<VirtualDiskRange, windows::core::Error>> + 'a, windows::core::Error> {
        Ok(ChangedRangeStream::new(self, change_tracking_id, bytes))
    }

    // pub(crate) fn copy_file<P: AsRef<OsStr>, Q: AsRef<OsStr>>(source: P, destination: Q) -> std::result::Result<(), windows::core::Error> {
    //     let source = wide_path(source);
    //     let destination = wide_path(destination);
    //     unsafe { CopyFileW(PCWSTR::from_raw(source.as_ptr()), PCWSTR::from_raw(destination.as_ptr()), false) }
    // }

    // pub(crate) fn copy_ranges<P: AsRef<OsStr>, Q: AsRef<OsStr>>(source: P, destination: Q, ranges: &[ChangedRange]) -> std::result::Result<u64, windows::core::Error> {
    //     let source = wide_path(source);
    //     let destination = wide_path(destination);
    //     let source = unsafe { CreateFileW(PCWSTR::from_raw(source.as_ptr()), FileSystem::FILE_GENERIC_READ.0, FileSystem::FILE_SHARE_READ, None, FileSystem::OPEN_EXISTING, FileSystem::FILE_ATTRIBUTE_NORMAL, None)? };
    //     let source = FileHandle(source);
    //     let destination = unsafe { CreateFileW(PCWSTR::from_raw(destination.as_ptr()), FileSystem::FILE_GENERIC_WRITE.0, FileSystem::FILE_SHARE_READ, None, FileSystem::CREATE_ALWAYS, FileSystem::FILE_ATTRIBUTE_NORMAL, None)? };
    //     let destination = FileHandle(destination);
    //     let mut copied = 0u64;
    //     let mut buffer = vec![0u8; 1024 * 1024];

    //     for range in ranges {
    //         unsafe {
    //             SetFilePointerEx(source.0, range.byte_offset as i64, None, FileSystem::FILE_BEGIN)?;
    //         }
    //         let mut remaining = range.byte_length;
    //         while remaining > 0 {
    //             let requested = remaining.min(buffer.len() as u64) as u32;
    //             let mut read = 0u32;
    //             unsafe {
    //                 ReadFile(source.0, Some(&mut buffer[..requested as usize]), Some(&mut read), None)?;
    //             }
    //             if read == 0 {
    //                 return Err(windows::core::Error::from_thread());
    //             }
    //             let mut written = 0u32;
    //             unsafe {
    //                 WriteFile(destination.0, Some(&buffer[..read as usize]), Some(&mut written), None)?;
    //             }
    //             if written != read {
    //                 return Err(windows::core::Error::from_thread());
    //             }
    //             remaining -= read as u64;
    //             copied += read as u64;
    //         }
    //     }

    //     Ok(copied)
    // }

    /// Sets the state of Resilient Change Tracking (RCT)
    // pub(crate) fn set_change_tracking(&self, enabled: bool) -> std::result::Result<(), windows::core::Error> {
    //     let disk_info = Vhd::SET_VIRTUAL_DISK_INFO {
    //         Version: Vhd::SET_VIRTUAL_DISK_INFO_CHANGE_TRACKING_STATE,
    //         Anonymous: Vhd::SET_VIRTUAL_DISK_INFO_0 { ChangeTrackingEnabled: enabled.into() },
    //     };

    //     unsafe {
    //         SetVirtualDiskInformation(self.handle, &disk_info).ok()?;
    //     }

    //     Ok(())
    // }

    // pub(crate) fn query_changes(&self, change_tracking_id: &OsStr, byte_length: u64) -> std::result::Result<Vec<ChangedRange>, windows::core::Error> {
    //     let mut encoded = wide_path(change_tracking_id);
    //     let mut changes = Vec::new();
    //     let mut byte_offset = 0u64;

    //     while byte_offset < byte_length {
    //         let (ranges, processed_length) = Self::query_change_batch(self.handle, &encoded, byte_offset, byte_length - byte_offset)?;
    //         changes.extend(ranges);
    //         if processed_length == 0 {
    //             break;
    //         }
    //         byte_offset = byte_offset.saturating_add(processed_length);
    //     }

    //     encoded.clear();
    //     Ok(changes)
    // }

    fn query_change_batch(handle: HANDLE, change_tracking_id: &[u16], byte_offset: u64, byte_length: u64) -> std::result::Result<(Vec<VirtualDiskRange>, u64), windows::core::Error> {
        let mut ranges = vec![Vhd::QUERY_CHANGES_VIRTUAL_DISK_RANGE::default(); 256];
        let mut range_count = ranges.len() as u32;
        let mut processed_length = 0u64;
        unsafe {
            QueryChangesVirtualDisk(
                handle,
                PCWSTR::from_raw(change_tracking_id.as_ptr()),
                byte_offset,
                byte_length,
                QUERY_CHANGES_VIRTUAL_DISK_FLAG_NONE,
                ranges.as_mut_ptr(),
                &mut range_count,
                &mut processed_length,
            )
            .ok()?;
        }
        ranges.truncate(range_count as usize);
        Ok((
            ranges
                .into_iter()
                .map(|range| VirtualDiskRange {
                    byte_offset: range.ByteOffset,
                    byte_length: range.ByteLength,
                })
                .collect(),
            processed_length,
        ))
    }

    pub(super) fn duplicate_handle(&self) -> std::result::Result<usize, windows::core::Error> {
        let mut handle = HANDLE::default();
        unsafe {
            DuplicateHandle(
                GetCurrentProcess(),
                self.handle,
                GetCurrentProcess(),
                &mut handle,
                0,
                false,
                DUPLICATE_SAME_ACCESS,
            )
            .ok()
            .ok_or_else(windows::core::Error::from_thread)?;
        }
        Ok(handle.0 as usize)
    }
}

fn wide_path<P: AsRef<OsStr>>(path: P) -> Vec<u16> {
    let mut encoded: Vec<u16> = path.as_ref().encode_wide().collect();
    encoded.push(0);
    encoded
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
