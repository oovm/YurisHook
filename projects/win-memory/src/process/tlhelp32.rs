use crate::{MemoryError, process::Handle};
use std::mem::size_of;
use windows::Win32::{
    Foundation::{HMODULE, MAX_PATH},
    System::Diagnostics::ToolHelp::{
        CREATE_TOOLHELP_SNAPSHOT_FLAGS, CreateToolhelp32Snapshot, MAX_MODULE_NAME32, MODULEENTRY32W, Module32FirstW,
        Module32NextW, PROCESSENTRY32W, Process32FirstW, Process32NextW,
    },
};

/// Wrappers around tl32help functions to work a little cleaner

pub fn create_pe32() -> PROCESSENTRY32W {
    PROCESSENTRY32W {
        dwSize: size_of::<PROCESSENTRY32W>() as u32,
        cntUsage: 0,
        th32ProcessID: 0,
        th32DefaultHeapID: 0,
        th32ModuleID: 0,
        cntThreads: 0,
        th32ParentProcessID: 0,
        pcPriClassBase: 0,
        dwFlags: 0,
        szExeFile: [0u16; 260],
    }
}
pub fn create_me32() -> MODULEENTRY32W {
    MODULEENTRY32W {
        dwSize: size_of::<MODULEENTRY32W>() as u32,
        th32ModuleID: 0,
        th32ProcessID: 0,
        GlblcntUsage: 0,
        ProccntUsage: 0,
        modBaseAddr: std::ptr::null_mut(),
        modBaseSize: 0,
        hModule: HMODULE::default(),
        szModule: [0; MAX_MODULE_NAME32 as usize + 1],
        szExePath: [0; MAX_PATH as usize],
    }
}

pub fn create_snapshot(flags: CREATE_TOOLHELP_SNAPSHOT_FLAGS, pid: u32) -> Result<Handle, MemoryError> {
    let h_snap = Handle::new(unsafe { CreateToolhelp32Snapshot(flags, pid) }?);
    if h_snap.is_valid() { Ok(h_snap) } else { Err(MemoryError::CreateSnapshotFailure) }
}

pub fn process_first(h_snap: &Handle, pe32: &mut PROCESSENTRY32W) -> Result<(), MemoryError> {
    unsafe { Process32FirstW(h_snap.wrap, pe32).map_err(|e| MemoryError::IterateSnapshotFailure { win32: e }) }
}
pub fn process32next(h_snap: &Handle, pe32: &mut PROCESSENTRY32W) -> bool {
    unsafe { Process32NextW(h_snap.wrap, pe32).is_ok() }
}

pub fn module32first(h_snap: &Handle, me32: &mut MODULEENTRY32W) -> Result<(), MemoryError> {
    unsafe { Module32FirstW(h_snap.wrap, me32).map_err(|e| MemoryError::IterateSnapshotFailure { win32: e }) }
}
pub fn module32next(h_snap: &Handle, me32: &mut MODULEENTRY32W) -> bool {
    unsafe { Module32NextW(h_snap.wrap, me32).is_ok() }
}
