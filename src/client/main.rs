use sysinfo::{System, SystemExt};

fn main() {
    let mut system = System::new_all();
    system.refresh_all();

    let os_name = system
        .name()
        .unwrap_or_else(|| "os 정보 확인 불가".to_string());
    let os_version = system
        .long_os_version()
        .unwrap_or_else(|| "os version 정보 확인 불가".to_string());
}
