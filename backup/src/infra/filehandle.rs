use windows::Win32::Foundation::{CloseHandle, HANDLE};

pub(super) struct FileHandle(pub(super) HANDLE);

impl Drop for FileHandle {
    fn drop(&mut self) {
        if !self.0.is_invalid() {
            unsafe {
                let _ = CloseHandle(self.0);
            }
        }
    }
}
