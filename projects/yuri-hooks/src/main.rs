use win_memory::WindowsProcess;

fn main() {
    let win = WindowsProcess::with_name("gamemd.exe").unwrap();
    println!("{:#?}", win);
    let ShortGame = win.read_data::<i8>(0xA8B262);
    println!("ShortGame: {:#?}", ShortGame);
}
