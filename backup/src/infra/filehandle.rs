use windows::Win32::Foundation::{CloseHandle, HANDLE};

pub(super) struct FileHandle(pub(super) HANDLE);

// A file handle is an owned process-wide Windows resource and can be moved between threads.
unsafe impl Send for FileHandle {}

impl Drop for FileHandle {
    fn drop(&mut self) {
        if !self.0.is_invalid() {
            unsafe {
                let _ = CloseHandle(self.0);
            }
        }
    }
}
