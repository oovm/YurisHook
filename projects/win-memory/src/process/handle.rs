use crate::MemoryError;
use std::{
    fmt::{Debug, Formatter, Pointer},
    ops::Deref,
};
use windows::{
    Win32::{
        Foundation::{CloseHandle, HANDLE, INVALID_HANDLE_VALUE},
        System::Threading::{OpenProcess, PROCESS_ALL_ACCESS, PROCESS_VM_READ, PROCESS_VM_WRITE},
    },
    core::Error,
};

/// Wrapper around winapi HANDLE for automatic closing of the handle upon destruction

#[derive(Clone)]
pub struct Handle(pub HANDLE);

impl Drop for Handle {
    fn drop(&mut self) {
        if self.is_valid() {
            unsafe {
                match CloseHandle(self.0) {
                    Ok(_) => {}
                    Err(_) => {}
                }
            };
        }
    }
}

impl Debug for Handle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Pointer::fmt(&self.0.0, f)
    }
}

impl Handle {
    /// if the wrapper contains a valid HANDLE this function returns true
    /// otherwise returns false
    pub fn is_valid(&self) -> bool {
        !self.0.is_invalid()
    }

    pub unsafe fn full_access(pid: u32) -> Result<Self, MemoryError> {
        match OpenProcess(PROCESS_ALL_ACCESS, false, pid) {
            Ok(h) => {
                if h.is_invalid() {
                    Err(MemoryError::GetHandleError { win32: Error::from_win32() })
                }
                else {
                    Ok(Handle(h))
                }
            }
            Err(e) => Err(MemoryError::GetHandleError { win32: e }),
        }
    }

    pub unsafe fn read_write(pid: u32) -> Result<Self, MemoryError> {
        match OpenProcess(PROCESS_VM_READ | PROCESS_VM_WRITE, false, pid) {
            Ok(h) => {
                if h.is_invalid() {
                    Err(MemoryError::GetHandleError { win32: Error::from_win32() })
                }
                else {
                    Ok(Handle(h))
                }
            }
            Err(e) => Err(MemoryError::GetHandleError { win32: e }),
        }
    }
}
