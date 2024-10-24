// #![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

mod errors;
// mod game_strings;

use get_last_error::Win32Error;
use proc_mem::{ProcMemError, Process};
use std::os::windows::ffi::OsStringExt;
use windows::{
    Win32::{
        Foundation::{CloseHandle, HANDLE},
        System::{
            Diagnostics::ToolHelp::{
                CreateToolhelp32Snapshot, PROCESSENTRY32W, Process32FirstW, Process32NextW, TH32CS_SNAPMODULE32,
                TH32CS_SNAPPROCESS,
            },
            ProcessStatus::{EnumProcesses, GetModuleFileNameExW},
            Threading::{
                OpenProcess, PROCESS_NAME_NATIVE, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ, QueryFullProcessImageNameW,
            },
        },
    },
    core::{Error, PWSTR},
};
// use std::arch::asm;
// use std::ffi::{c_char, CString};
pub use crate::errors::{Result, YuriError, YuriErrorKind};

mod map_hooks;

// #[used]
// #[link_section = ".CRT$XCU"]
// static RUST_CTOR: unsafe extern "C" fn() -> usize = rust_ctor;
//
// #[used]
// #[link_section = ".CRT$XCU"]
// static RUST_DTOR: unsafe extern "C" fn() = rust_dtor;
//
// unsafe extern "C" fn rust_ctor() -> usize {
//     println!("我是一个dll");
//     0
// }
//

// class Powerups {
// 	public:
// 	// all these actually point to arrays with 0x13 items, see ePowerup for their numbering
// 	/**
// 	 * e.g. Powerups::Weights[pow_Unit] is the weight of the free unit crate
// 	 */
//
// 	// the name of the effect, for INI reading purposes
// 	static constexpr reference<const char*, 0x7E523Cu, 19u> const Effects{};
//
// 	// the weight of the effect
// 	static constexpr reference<int, 0x81DA8Cu, 19u> const Weights{};
//
// 	// the effect-specific argument
// 	static constexpr reference<double, 0x89EC28u, 19u> const Arguments{};
//
// 	// can this crate appear on water?
// 	static constexpr reference<bool, 0x89ECC0u, 19u> const Naval{};
//
// 	// index into AnimTypeClass::Array
// 	static constexpr reference<int, 0x81DAD8u, 19u> const Anims{};
// };

// fn power_up_weights() -> &'static [i32; 19] {
//     unsafe {
//         &*(0x81DA8C_u32 as *const [i32; 19])
//     }
// }

// unsafe extern "C" fn rust_dtor() {
//     let a = power_up_weights();
//     println!("{:#?}", a);
//     println!("立即卸载吧")
// }

pub fn create_snapshot(pid: u32) -> windows::core::Result<HANDLE> {
    let h_snap = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, pid) };
    return h_snap;
}

fn get_all_process_ids() -> windows::core::Result<Vec<u32>> {
    let mut process_ids = Vec::with_capacity(1024);
    let mut bytes_returned = 0;

    unsafe {
        EnumProcesses(
            process_ids.as_mut_ptr() as *mut u32,
            (process_ids.capacity() * size_of::<u32>()) as u32,
            &mut bytes_returned,
        )?;
    }
    let process_count = bytes_returned as usize / size_of::<u32>();
    // release unused
    unsafe {
        process_ids.set_len(process_count);
    }
    Ok(process_ids)
}

unsafe fn get_process_name_by_pid(pid: u32) -> windows::core::Result<String> {
    let handle = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, pid)?;
    if handle.is_invalid() {
        return Err(Error::from_win32());
    }
    let mut buffer = PWSTR::from_raw([0u16; 512].as_mut_ptr());
    let mut size = 512_u32;

    QueryFullProcessImageNameW(handle, PROCESS_NAME_NATIVE, buffer, &mut size)?;
    CloseHandle(handle)?;

    let mut s = buffer.to_string().unwrap();
    Ok(s)
}

#[test]
fn test() {
    unsafe {
        let h_snap = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0).unwrap();
        if h_snap.is_invalid() {
            panic!("{}", Error::from_win32())
        }
        let mut pe32 = PROCESSENTRY32W::default();
        pe32.dwSize = size_of::<PROCESSENTRY32W>() as u32;
        // skip first, always `[System Process]`
        Process32FirstW(h_snap, &mut pe32).unwrap();
        if h_snap.is_invalid() {
            panic!("{}", Error::from_win32())
        }
        while let Ok(_) = Process32NextW(h_snap, &mut pe32) {
            if h_snap.is_invalid() {
                panic!("{}", Error::from_win32())
            }
            let process_name = String::from_utf16_lossy(&pe32.szExeFile);

            println!("{}", process_name);
            Process32NextW(h_snap, &mut pe32).unwrap();
        }
    }
}
