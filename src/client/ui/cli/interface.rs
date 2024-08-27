use std::{
    borrow::BorrowMut,
    io::{self, Write},
};
use sysinfo::{System, SystemExt};
use uuid::Uuid;

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
}

// TODO: gui와 공통된 부분 빼기
impl interface::Interface for Cli {
    async fn entry(&self) {
        match self.enter_group() {
            Some(_uuid) => {
                // 이미 존재하는 그룹에 참가
                let new_spec_uuid = self.register_device_spec(_uuid).await.unwrap();
                println!("Spec UUID: {}", new_spec_uuid);

                let new_fs_uuid = self.register_device_fs(_uuid).await.unwrap();
                println!("FS UUID: {}", new_fs_uuid);

                let device_manager = request::get_device_manager(&self.master_addr, _uuid)
                    .await
                    .unwrap();

                println!("Device Manager: {:?}", device_manager);
            }
            None => {
                // 새로 그룹을 생성
                let new_manager_uuid = self.create_group().await.unwrap();
                println!("Group UUID : {}", new_manager_uuid);

                let new_spec_uuid = self.register_device_spec(new_manager_uuid).await.unwrap();
                println!("Spec UUID: {}", new_spec_uuid);

                let new_fs_uuid = self.register_device_fs(new_manager_uuid).await.unwrap();
                println!("FileTree UUID : {}", new_fs_uuid);
            }
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
        println!("FileTree 구성 작업을 시작합니다.");
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
