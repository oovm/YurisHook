use crate::YuriError;
use win_memory::WindowsProcess;

#[derive(Debug)]
pub struct GameOptions {
    pub bases: bool,
    pub bridge_destruction: bool,
    pub crates: bool,
    pub short_game: bool,
    pub sw_allowed: bool,
    pub multi_engineer: bool,
    pub allies_allowed: bool,
    pub harvester_truce: bool,
    pub ctf: bool,
    pub fow: bool,
    pub mcv_redeploy: bool,
}

impl GameOptions {
    pub fn current(pid: &WindowsProcess) -> std::result::Result<GameOptions, YuriError> {
        Ok(GameOptions {
            bases: pid.read_data(0xA8B258)?,
            bridge_destruction: pid.read_data(0xA8B260)?,
            crates: pid.read_data(0xA8B261)?,
            short_game: pid.read_data(0xA8B262)?,
            sw_allowed: pid.read_data(0xA8B263)?,
            multi_engineer: pid.read_data(0xA8B26C)?,
            allies_allowed: pid.read_data(0xA8B31C)?,
            harvester_truce: pid.read_data(0xA8B31D)?,
            ctf: pid.read_data(0xA8B31E)?,
            fow: pid.read_data(0xA8B31F)?,
            mcv_redeploy: pid.read_data(0xA8B320)?,
        })
    }
}
