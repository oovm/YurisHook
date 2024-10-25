#![feature(coroutines)]
#![feature(gen_blocks)]
#![feature(coroutine_trait)]
#![feature(iter_from_coroutine)]
// #![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

pub mod ai_hooks;
mod errors;
mod game_mode_hooks;
mod game_option_hooks;
pub mod helpers;
// mod game_strings;
pub use crate::errors::{YuriError, YuriErrorKind, YuriResult};
use crate::{crate_hooks::PowerUps, game_mode_hooks::GameModeOptions, game_option_hooks::GameOptions};
use win_memory::{MemoryError, ProcessData, WindowsProcess};

mod crate_hooks;
mod map_hooks;

pub struct YuriGameManager {
    pid: WindowsProcess,
}

impl YuriGameManager {
    pub fn new() -> std::result::Result<YuriGameManager, YuriError> {
        Ok(Self { pid: WindowsProcess::with_name("gamemd.exe")? })
    }
    pub fn game_mode_options(&self) -> Result<GameModeOptions, MemoryError> {
        Ok(GameModeOptions::read(&self.pid)?)
    }
    pub fn game_options(&self) -> std::result::Result<GameOptions, YuriError> {
        GameOptions::current(&self.pid)
    }

    pub fn power_ups(&self) -> std::result::Result<PowerUps, YuriError> {
        Ok(PowerUps::read(&self.pid)?)
    }

    pub fn set_move_feedback(&mut self, value: bool) -> bool {
        self.pid.write_data::<bool>(0x822CF2, value).is_ok()
    }
}

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

// unsafe extern "C" fn rust_dtor() {
//     let a = power_up_weights();
//     println!("{:#?}", a);
//     println!("立即卸载吧")
// }
