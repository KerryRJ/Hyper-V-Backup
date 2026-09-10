use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

use windows::Win32::Foundation;
use windows::Win32::Storage::Vhd;
use windows::Win32::Storage::Vhd::{OpenVirtualDisk, SetVirtualDiskInformation};
use windows::core;

/// A safe RAII wrapper that automatically manages the Virtual Disk lifecycle.
struct VirtualDisk {
    handle: Foundation::HANDLE,
}

impl VirtualDisk {
    /// Opens a VHDX file cleanly, handling wide string allocation safely.
    pub fn open<P: AsRef<OsStr>>(path: P) -> core::Result<Self> {
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
    pub fn set_change_tracking(&self, enabled: bool) -> core::Result<()> {
        let disk_info = Vhd::SET_VIRTUAL_DISK_INFO {
            Version: Vhd::SET_VIRTUAL_DISK_INFO_CHANGE_TRACKING_STATE,
            Anonymous: Vhd::SET_VIRTUAL_DISK_INFO_0 { ChangeTrackingEnabled: enabled.into() },
        };

        unsafe {
            SetVirtualDiskInformation(self.handle, &disk_info).ok()?;
        }

        Ok(())
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
