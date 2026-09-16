use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

use windows::Win32::Foundation;
use windows::Win32::Storage::Vhd;
use windows::Win32::Storage::Vhd::{GetVirtualDiskInformation, OpenVirtualDisk, QueryChangesVirtualDisk, SetVirtualDiskInformation};
use windows::core;

#[derive(Clone, Debug)]
pub(crate) struct ChangedRange {
    pub(crate) byte_offset: u64,
    pub(crate) byte_length: u64,
}

/// A safe RAII wrapper that automatically manages the Virtual Disk lifecycle.
pub(crate) struct VirtualDisk {
    handle: Foundation::HANDLE,
}

impl VirtualDisk {
    /// Opens a VHDX file cleanly, handling wide string allocation safely.
    pub(crate) fn open<P: AsRef<OsStr>>(path: P) -> core::Result<Self> {
        // Safe, native wide-string conversion with stack-allocated buffer
        let mut encoded: Vec<u16> = path.as_ref().encode_wide().collect();
        encoded.push(0);

        let storage_type = Vhd::VIRTUAL_STORAGE_TYPE {
            DeviceId: Vhd::VIRTUAL_STORAGE_TYPE_DEVICE_VHDX,
            VendorId: Vhd::VIRTUAL_STORAGE_TYPE_VENDOR_MICROSOFT,
        };

        let open_params = Vhd::OPEN_VIRTUAL_DISK_PARAMETERS {
            Version: Vhd::OPEN_VIRTUAL_DISK_VERSION_1,
            Anonymous: Vhd::OPEN_VIRTUAL_DISK_PARAMETERS_0 {
                Version1: Vhd::OPEN_VIRTUAL_DISK_PARAMETERS_0_0 { RWDepth: 0 },
            },
        };

        let mut handle = Foundation::HANDLE::default();

        // Encapsulate unsafe block strictly to the FFI boundary
        unsafe {
            OpenVirtualDisk(
                &storage_type,
                core::PCWSTR::from_raw(encoded.as_ptr()),
                Vhd::VIRTUAL_DISK_ACCESS_ALL,
                Vhd::OPEN_VIRTUAL_DISK_FLAG_NONE,
                Some(&open_params),
                &mut handle,
            )
            .ok()?;
        }

        Ok(Self { handle })
    }

    /// Sets the state of Resilient Change Tracking (RCT)
    pub(crate) fn set_change_tracking(&self, enabled: bool) -> core::Result<()> {
        let disk_info = Vhd::SET_VIRTUAL_DISK_INFO {
            Version: Vhd::SET_VIRTUAL_DISK_INFO_CHANGE_TRACKING_STATE,
            Anonymous: Vhd::SET_VIRTUAL_DISK_INFO_0 { ChangeTrackingEnabled: enabled.into() },
        };

        unsafe {
            SetVirtualDiskInformation(self.handle, &disk_info).ok()?;
        }

        Ok(())
    }

    pub(crate) fn query_changes(&self, change_tracking_id: &OsStr, byte_length: u64) -> core::Result<Vec<ChangedRange>> {
        let mut encoded: Vec<u16> = change_tracking_id.encode_wide().collect();
        encoded.push(0);

        let mut ranges = vec![Vhd::QUERY_CHANGES_VIRTUAL_DISK_RANGE::default(); 256];
        let mut range_count = ranges.len() as u32;
        let mut processed_length = 0;

        unsafe {
            QueryChangesVirtualDisk(
                self.handle,
                core::PCWSTR::from_raw(encoded.as_ptr()),
                0,
                byte_length,
                Vhd::QUERY_CHANGES_VIRTUAL_DISK_FLAG_NONE,
                ranges.as_mut_ptr(),
                &mut range_count,
                &mut processed_length,
            )
            .ok()?;
        }

        ranges.truncate(range_count as usize);
        Ok(ranges
            .into_iter()
            .map(|range| ChangedRange {
                byte_offset: range.ByteOffset,
                byte_length: range.ByteLength,
            })
            .collect())
    }

    pub(crate) fn change_tracking_state(&self) -> core::Result<(bool, String)> {
        let mut info = Vhd::GET_VIRTUAL_DISK_INFO {
            Version: Vhd::GET_VIRTUAL_DISK_INFO_CHANGE_TRACKING_STATE,
            Anonymous: Vhd::GET_VIRTUAL_DISK_INFO_0 { ChangeTrackingState: Default::default() },
        };
        let mut info_size = std::mem::size_of::<Vhd::GET_VIRTUAL_DISK_INFO>() as u32;

        unsafe {
            GetVirtualDiskInformation(self.handle, &mut info_size, &mut info, None).ok()?;
        }

        let state = unsafe { info.Anonymous.ChangeTrackingState };
        let id = String::from_utf16_lossy(&state.MostRecentId)
            .trim_end_matches('\0')
            .to_owned();
        Ok((state.Enabled.as_bool(), id))
    }
}

/// The Drop trait guarantees the handle closes even if a panic or error happens later.
impl Drop for VirtualDisk {
    fn drop(&mut self) {
        if !self.handle.is_invalid() {
            unsafe {
                let _ = Foundation::CloseHandle(self.handle);
            }
        }
    }
}
