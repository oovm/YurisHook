use dll_syringe::{Syringe, process::OwnedProcess};
use sysinfo::{PidExt, ProcessExt, SystemExt};

fn main() {
    // let mut system = sysinfo::System::new();
    // system.refresh_processes();
    // for (id, p) in system.processes() {
    //     if id.as_u32().eq(&27808) {
    //         println!("{}:{}", id, p.name());
    //     }
    // }
    let target_process = OwnedProcess::find_first_by_name("gamemd").unwrap();
    let syringe = Syringe::for_process(target_process);
    let here = std::env::current_exe().unwrap();
    let dll = here.parent().unwrap().join("gamemdx.dll");
    let injected_payload = syringe.inject(dll).unwrap();
    syringe.eject(injected_payload).unwrap();
}