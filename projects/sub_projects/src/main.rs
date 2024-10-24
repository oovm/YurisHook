use dll_syringe::{Syringe, process::OwnedProcess};
use sysinfo::{PidExt, ProcessExt, SystemExt};

fn main() {
    let mut system = sysinfo::System::new();
    system.refresh_processes();
    for (id, p) in system.processes() {
        if id.as_u32().eq(&27808) {
            println!("{}:{}", id, p.name());
        }
    }

    // 通过进程名找到目标进程
    let target_process = OwnedProcess::find_first_by_name("gamemd").unwrap();

    // 新建一个注入器
    let syringe = Syringe::for_process(target_process);

    let here = std::env::current_exe().unwrap();
    let dll = here.parent().unwrap().join("yuri_hooks.dll");
    println!("{}", dll.display());

    // 将我们刚刚编写的dll加载进去
    let injected_payload = syringe.inject(dll).unwrap();

    // do something else

    // 将我们刚刚注入的dll从目标程序内移除
    syringe.eject(injected_payload).unwrap();
}