use std::{
    borrow::BorrowMut,
    io::{self, Write},
};
use sysinfo::{System, SystemExt};
use uuid::Uuid;

use crate::ui::request::DeviceManager;

use super::super::request;
use device::device::{file_sys::FileSystem, spec::DeviceSpec};

use super::super::interface;

pub struct Cli {
    master_addr: String,
}

impl Cli {
    pub fn new(master_addr: String) -> Self {
        Cli { master_addr }
    }

    fn print_indent(indent: usize, msg: &str) {
        print!("{}{}", " ".repeat(indent * 4), msg);
    }

    fn println_indent(indent: usize, msg: &str) {
        println!("{}{}", " ".repeat(indent * 4), msg);
    }
}

// TODO: gui와 공통된 부분 빼기
impl interface::Interface for Cli {
    async fn entry(&self) {
        let manager_uuid = match self.enter_group() {
            Some(_uuid) => _uuid,
            None => self.create_group().await.unwrap(),
        };
        println!("DeviceManager UUID: {}", manager_uuid);
        let new_spec_uuid = self.register_device_spec(manager_uuid).await.unwrap();
        println!("Spec UUID: {}", new_spec_uuid);
        let new_fs_uuid = self.register_device_fs(manager_uuid).await.unwrap();
        println!("FileSystem UUID : {}", new_fs_uuid);

        let device_manager = request::get_device_manager(&self.master_addr, manager_uuid)
            .await
            .unwrap();

        self.render(&device_manager);
    }

    fn render(&self, device_manager: &DeviceManager) {
        let device_spec_map = &device_manager.id_spec_map;
        let device_fs_map = &device_manager.id_fs_map;
        let device_uuid_lst = device_spec_map.keys();
        let indent: usize = 0;
        loop {
            Cli::println_indent(indent, "Device 목록: ");

            let mut selected_device_uuid = String::new();
            for (idx, uuid) in device_uuid_lst.clone().enumerate() {
                Cli::println_indent(
                    indent + 1,
                    &format!(
                        "{}> {}({})_{}",
                        idx,
                        device_spec_map.get(uuid).unwrap().os,
                        device_spec_map.get(uuid).unwrap().os_version,
                        device_spec_map.get(uuid).unwrap().ip
                    ),
                );
            }
            Cli::print_indent(indent, "\nFileSystem을 확인할 Device를 선택해주세요: ");
            io::stdout().flush().unwrap();

            io::stdin()
                .read_line(&mut selected_device_uuid)
                .expect("입력에 실패했습니다.");
            let selected_num: usize = selected_device_uuid
                .trim()
                .parse()
                .expect("usize로 변환하는 과정에서 문제가 발생했습니다.");
            let selected_device_fs = device_fs_map
                .get(&device_fs_map.keys().nth(selected_num).unwrap())
                .unwrap();
            Cli::print_indent(indent, &format!("{:?}", selected_device_fs)); // TOOD:
        }
    }

    async fn register_device_fs(&self, manager_uuid: Uuid) -> Result<Uuid, ()> {
        println!("Group에 공유할 file system의 root를 지정해주세요.");
        println!("e.g. MacOSX : /Users/username/Desktop/public_dir");
        println!("     Linux  : /home/username/Desktop/public_dir");

        print!("입력: ");
        io::stdout().flush().unwrap();

        let mut device_fs_root: String = String::new();
        io::stdin()
            .read_line(&mut device_fs_root)
            .expect("입력에 실패했습니다.");

        let mut device_fs_root_trimmed = device_fs_root.trim().to_owned();
        let mut device_fs = FileSystem::new(device_fs_root_trimmed.borrow_mut());
        println!("FileSystem 구성 작업을 시작합니다.");
        device_fs.init_file_node();

        match request::post_device_fs(&self.master_addr, manager_uuid, device_fs).await {
            Ok(uuid) => Ok(uuid),
            Err(e) => {
                println!("서버와의 연결상태를 다시 확인해주시기 바랍니다.");
                println!("{:?}", e);
                Err(())
            }
        }
    }

    async fn register_device_spec(&self, manager_uuid: Uuid) -> Result<Uuid, ()> {
        println!("device의 정보를 master에 저장합니다.");

        let mut system = System::new_all();
        system.refresh_all();

        let os = system
            .name()
            .expect("system의 name을 구하는데 문제가 발생했습니다.");
        let os_version = system
            .os_version()
            .expect("system의 os version 구하는데 문제가 발생했습니다.");
        let spec = DeviceSpec {
            ip: "".to_string(),
            os,
            os_version,
            listen_port: "8081".to_string(), // 임시 값
        };

        match request::post_device_spec(&self.master_addr, manager_uuid, spec).await {
            Ok(uuid) => Ok(uuid),
            Err(e) => {
                println!("서버와의 연결상태를 다시 확인해주시기 바랍니다.");
                println!("{:?}", e);
                Err(())
            }
        }
    }

    fn enter_group(&self) -> Option<Uuid> {
        loop {
            print!("이미 존재하는 group에 참가합니다. [Y/n]: ");
            io::stdout().flush().unwrap();

            let mut group_enter: String = String::new();
            io::stdin()
                .read_line(&mut group_enter)
                .expect("입력에 실패했습니다.");

            let group_enter = group_enter.trim().to_lowercase();
            if group_enter.eq("n") {
                return None;
            } else if group_enter.ne("y") {
                println!("잘못된 입력값입니다.");
                continue;
            } else {
                break;
            }
        }

        print!("참여하실 group의 uuid값을 입력해주세요: ");
        io::stdout().flush().unwrap();

        let mut manager_uuid_str: String = String::new();
        io::stdin()
            .read_line(&mut manager_uuid_str)
            .expect("입력에 실패했습니다.");
        let manager_uuid_str = manager_uuid_str.trim();
        let manager_uuid = Uuid::parse_str(&manager_uuid_str).unwrap();

        Some(manager_uuid)
    }

    async fn create_group(&self) -> Result<Uuid, ()> {
        let new_manager_uuid = request::post_device_manager(&self.master_addr).await;

        match new_manager_uuid {
            Ok(uuid) => Ok(uuid),
            Err(e) => {
                println!("서버와의 연결상태를 다시 확인해주시기 바랍니다.");
                println!("{:?}", e);
                Err(())
            }
        }
    }
}
