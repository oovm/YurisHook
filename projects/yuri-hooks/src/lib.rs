// #![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

mod errors;
// mod game_strings;

// use std::arch::asm;
// use std::ffi::{c_char, CString};
pub use crate::errors::{Result, YuriError, YuriErrorKind};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GroundType {
    // Terrain speed multipliers.
    cost: [f32; 8],
    // Can build on this terrain?
    buildable: i32,
}

impl GroundType {
    pub fn get_land_type_from_name() -> bool {
        unsafe {
            let f: extern "fastcall" fn() -> bool = unsafe { std::mem::transmute(0x4E12D0 as *const ()) };
            f()
        }
    }
}

//     use detour::RawDetour;
//     use std::mem;
//
//     fn add5(val: i32) -> i32 {
//         val + 5
//     }
//
//     fn add10(val: i32) -> i32 {
//         val + 10
//     }
//
//     let mut hook = unsafe { RawDetour::new(add5 as *const (), add10 as *const ())? };
//
//     assert_eq!(add5(5), 10);
//     assert_eq!(hook.is_enabled(), false);
//
//     unsafe { hook.enable()? };
//     assert!(hook.is_enabled());
//
//     let original: fn(i32) -> i32 = unsafe { mem::transmute(hook.trampoline()) };
//
//     assert_eq!(add5(5), 15);
//     assert_eq!(original(5), 10);
//
//     unsafe { hook.disable()? };
//     assert_eq!(add5(5), 10);

#[used]
#[doc(hidden)]
#[link_section = ".CRT$XCU"]
static RUST_CTOR: unsafe extern "C" fn() -> usize = rust_ctor;
#[used]
#[link_section = ".CRT$XCU"]
static RUST_DTOR: unsafe extern "C" fn() = rust_dtor;

unsafe extern "C" fn rust_ctor() -> usize {
    println!("我是一个dll啊");
    0
}

unsafe extern "C" fn rust_dtor() {
    println!("立即卸载吧")
}
