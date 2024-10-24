use std::sync::Arc;

macro_rules! gamemd_string {
    ($name:ident, $addr:expr) => {
        pub static $name: Arc<&'static str> = Arc::new(&concat!("", stringify!($name)));
    };
}

pub mod game_strings {
    use super::*;

    // unsorted names
    gamemd_string!(YURI_S_REVENGE, 0x849F48);
    gamemd_string!(BLOWFISH_DLL, 0x840A78);
    gamemd_string!(XXICON_SHP, 0x8204FC);
    gamemd_string!(LSSOBS_SHP, 0x8297F4);
    gamemd_string!(_800, 0x8297DC);
    gamemd_string!(_640, 0x8297E0);
    gamemd_string!(_none_, 0x817474);
    gamemd_string!(none, 0x817694);
    gamemd_string!(Neutral, 0x82BA08);
    gamemd_string!(Civilian, 0x818164);
    gamemd_string!(Special, 0x817318);

    // ini file names
    gamemd_string!(UIMD_INI, 0x827DC8);
    gamemd_string!(THEMEMD_INI, 0x825D94);
    gamemd_string!(EVAMD_INI, 0x825DF0);
    gamemd_string!(SOUNDMD_INI, 0x825E50);
    gamemd_string!(BATTLEMD_INI, 0x826198);
    gamemd_string!(AIMD_INI, 0x82621C);
    gamemd_string!(ARTMD_INI, 0x826254);
    gamemd_string!(RULESMD_INI, 0x826260);
    gamemd_string!(RA2MD_INI, 0x826444);
    gamemd_string!(MAPSELMD_INI, 0x830370);
    gamemd_string!(MISSIONMD_INI, 0x839724);

    // ini section names
    gamemd_string!(General, 0x826278);
    gamemd_string!(Basic, 0x82BF9C);
    gamemd_string!(AudioVisual, 0x839EA8);
    gamemd_string!(AI, 0x839DA4);
    gamemd_string!(CombatDamage, 0x839E8C);
    gamemd_string!(Radiation, 0x839E80);
    gamemd_string!(ToolTips, 0x833188);
    gamemd_string!(CrateRules, 0x839E9C);
    gamemd_string!(JumpjetControls, 0x839D58);
    gamemd_string!(Waypoints, 0x82DB0C);
    gamemd_string!(VariableNames, 0x83D824);

    gamemd_string!(MCVRedeploys, 0x83CF68);

    // EVA entry names
    gamemd_string!(EVA_StructureSold, 0x819030);
    gamemd_string!(EVA_InsufficientFunds, 0x819044);
    gamemd_string!(EVA_UnitSold, 0x822630);
    gamemd_string!(EVA_OreMinerUnderAttack, 0x824784);
    gamemd_string!(EVA_CannotDeployHere, 0x82012C);
    gamemd_string!(EVA_UnitReady, 0x8249A0);

    // CSF Labels
    gamemd_string!(TXT_TO_REPLAY, 0x83DB24);
    gamemd_string!(TXT_OK, 0x825FB0);
    gamemd_string!(TXT_CANCEL, 0x825FD0);
    gamemd_string!(TXT_CONTROL, 0x82729C);
    gamemd_string!(TXT_INTERFACE, 0x826FEC);
    gamemd_string!(TXT_SELECTION, 0x827250);
    gamemd_string!(TXT_TAUNT, 0x827218);
    gamemd_string!(TXT_TEAM, 0x826FA4);
    gamemd_string!(TXT_SAVING_GAME, 0x820DD4);
    gamemd_string!(TXT_GAME_WAS_SAVED, 0x829FE0);
    gamemd_string!(TXT_ERROR_SAVING_GAME, 0x829EBC);
    gamemd_string!(TXT_PRI, 0x843150);
    gamemd_string!(TXT_PRIMARY, 0x843158);
    gamemd_string!(TXT_POWER_DRAIN2, 0x843164);
    gamemd_string!(TXT_MONEY_FORMAT_1, 0x83FAB0);
    gamemd_string!(TXT_MONEY_FORMAT_2, 0x83FA9C);
    gamemd_string!(GUI_DEBUG, 0x827AF8);
}
