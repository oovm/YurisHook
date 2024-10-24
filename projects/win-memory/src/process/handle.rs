use crate::MemoryError;
use std::ops::Deref;
use windows::Win32::{
    Foundation::{CloseHandle, HANDLE, INVALID_HANDLE_VALUE},
    System::Threading::{OpenProcess, PROCESS_ALL_ACCESS, PROCESS_VM_READ, PROCESS_VM_WRITE},
};

/// Wrapper around winapi HANDLE for automatic closing of the handle upon destruction

#[derive(Debug, Clone)]
pub struct Handle(pub HANDLE);

impl Deref for Handle {
    type Target = HANDLE;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Drop for Handle {
    fn drop(&mut self) {
        if self.is_valid() {
            unsafe { CloseHandle(**self) };
        }
    }
}

impl Handle {
    /// if the wrapper contains a valid HANDLE this function returns true
    /// otherwise returns false
    pub fn is_valid(&self) -> bool {
        self.0 != INVALID_HANDLE_VALUE
    }
    /// when OpenProcess with PROCESS_ALL_ACCESS fails
    /// the function tries to OpenProcess with PROCESS_VM_READ & PROCESS_VM_WRITE
    /// if that fails too the function returns an ProcMemError
    pub fn read_write(pid: u32) -> Result<Self, MemoryError> {
        let mut h = unsafe { OpenProcess(PROCESS_ALL_ACCESS, false, pid)? };
        if h == INVALID_HANDLE_VALUE {
            h = unsafe { OpenProcess(PROCESS_VM_READ | PROCESS_VM_WRITE, false, pid)? };
            if h == INVALID_HANDLE_VALUE { Err(MemoryError::GetHandleError) } else { Ok(Handle(h)) }
        }
        else {
            Ok(Handle(h))
        }
    }
}
