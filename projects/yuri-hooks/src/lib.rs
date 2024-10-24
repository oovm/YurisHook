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

mod map_hooks;

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
