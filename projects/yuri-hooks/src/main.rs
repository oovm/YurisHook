use dll_syringe::{process::OwnedProcess, Syringe};

fn main() {
    let target_process = OwnedProcess::find_first_by_name("gamemd").unwrap();
    let syringe = Syringe::for_process(target_process);
    let here = std::env::current_exe().unwrap();
    let dll = here.parent().unwrap().join("gamemdx.dll");
    let injected_payload = syringe.inject(dll).unwrap();

    syringe.eject(injected_payload).unwrap();
}
