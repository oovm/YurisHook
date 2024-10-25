use win_memory::{MemoryError, ProcessData, WindowsProcess};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SessionOptionsClass {
    pub mp_game_mode: i32,
    pub scen_index: i32,
    pub game_speed: i32,
    pub credits: i32,
    pub unit_count: i32,
    pub short_game: bool,
    pub super_weapons_allowed: bool,
    pub build_off_ally: bool,
    pub mcv_repacks: bool,
    pub crates_appear: bool,
    pub slot_data: [Vector3D<i32>; 8],
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vector3D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[derive(Copy, Clone, Debug)]
pub struct GameModeOptions {
    raw: RAW_GameModeOptions,
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone, Debug)]
struct RAW_GameModeOptions {
    pub mp_mode_index: i32,
    pub scenario_index: i32,
    pub bases: bool,
    pub money: i32,
    pub bridge_destruction: bool,
    /// 是否允许生成补给箱
    pub crates: bool,
    pub short_game: bool,
    /// 是否允许使用超级武器
    pub sw_allowed: bool,
    pub build_off_ally: bool,
    pub game_speed: i32,
    pub multi_engineer: bool,
    pub unit_count: i32,
    pub ai_players: i32,
    pub ai_difficulty: i32,
    pub ai_slots: RAW_AISlots,
    pub allies_allowed: bool,
    pub harvester_truce: bool,
    pub ctf: bool,
    pub fog_of_war: bool,
    pub mcv_redeploy: bool,
    map_description: [u16; 45],
}

impl Default for RAW_GameModeOptions {
    fn default() -> Self {
        Self {
            mp_mode_index: 0,
            scenario_index: 0,
            bases: false,
            money: 0,
            bridge_destruction: false,
            crates: false,
            short_game: false,
            sw_allowed: false,
            build_off_ally: false,
            game_speed: 0,
            multi_engineer: false,
            unit_count: 0,
            ai_players: 0,
            ai_difficulty: 0,
            ai_slots: Default::default(),
            allies_allowed: false,
            harvester_truce: false,
            ctf: false,
            fog_of_war: false,
            mcv_redeploy: false,
            map_description: [0; 45],
        }
    }
}

impl ProcessData for GameModeOptions {
    fn read(pid: &WindowsProcess) -> Result<Self, MemoryError>
    where
        Self: Sized,
    {
        Ok(Self { raw: pid.read_data(0xA8B250)? })
    }

    fn write(&self, pid: &mut WindowsProcess) -> Result<(), MemoryError> {
        Ok(pid.write_data(0xA8B250, self.raw)?)
    }
}
