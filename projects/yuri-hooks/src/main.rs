use win_memory::WindowsProcess;
use yuri_hooks::{YuriError, YuriGameManager};

fn main() {
    let mut win = YuriGameManager::new().unwrap();
    println!("{:#?}", win.game_options().unwrap());
    println!("{:#?}", win.power_ups().unwrap());
    win.set_move_feedback(true);
}
