use dll_syringe::{
    Syringe,
    process::{BorrowedProcessModule, OwnedProcess},
};
use yuri_hooks::{YuriError, YuriErrorKind};

fn main() -> Result<(), YuriError> {
    let game_dir = match std::env::current_exe() {
        Ok(o) => match o.parent() {
            Some(o) => Ok(o.to_path_buf()),
            None => Err(YuriErrorKind::GameNotFound),
        },
        Err(_) => Err(YuriErrorKind::GameNotFound),
    }?;
    // let game_exe = game_dir.join("gamemd.exe");
    let yuri_dll = game_dir.join("yuri_hooks.dll");

    let game_pid = match OwnedProcess::find_first_by_name("gamemd") {
        Some(s) => Ok(s),
        None => Err(YuriErrorKind::GameNotFound),
    }?;
    let syringe = Syringe::for_process(game_pid);
    let injected_payload = match syringe.inject(yuri_dll) {
        Ok(o) => Ok(o),
        Err(_) => Err(YuriErrorKind::GameNotFound),
    }?;

    syringe.eject(injected_payload).unwrap();
    Ok(())
    // let my_function: extern "fastcall" fn() -> i32 =
    //     unsafe { std::mem::transmute(0x4E12D0_i32) };
    //
    // unsafe {
    //     let result = my_function();
    //     println!("{:#?}", result);
    //     // 使用结果
    // }
}
