use win_memory::WindowsProcess;

fn main() {
    let win = WindowsProcess::with_name("gamemd.exe").unwrap();
    println!("{:#?}", win);
}
