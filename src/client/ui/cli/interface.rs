use actix_web::web::delete;
use std::process;
use std::sync::mpsc::Receiver;
use std::sync::{mpsc, Arc, Mutex};
use std::{
    borrow::BorrowMut,
    io::{self, Write},
};
use sysinfo::{System, SystemExt};
use uuid::Uuid;

use super::super::interface;
use super::super::request;
use crate::ui::request::DeviceManager;
use device::device::{file_sys::FileSystem, spec::DeviceSpec};

#[derive(Debug)]
enum ActionNum {
    DeviceList = 0,
    FileSystem,
    FileTransfer,
    Exit,
}

impl TryFrom<i32> for ActionNum {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ActionNum::DeviceList),
            1 => Ok(ActionNum::FileSystem),
            2 => Ok(ActionNum::FileTransfer),
            3 => Ok(ActionNum::Exit),
            _ => Err(()),
        }
    }
}

pub struct Cli {
    master_addr: String,
    device_manager_uuid: Uuid,
    device_uuid: Uuid,
}

impl Cli {
    fn print_indent(indent: usize, msg: &str) {
        print!("{}{}", " ".repeat(indent * 4), msg);
    }

    fn println_indent(indent: usize, msg: &str) {
        println!("{}{}", " ".repeat(indent * 4), msg);
    }

    async fn sync_device_manager(&self, tx: std::sync::mpsc::Sender<DeviceManager>) {
        let mut interval = tokio::time::interval(std::time::Duration::from_millis(10));

        let master_addr = self.master_addr.clone();
        let device_manager_uuid = self.device_manager_uuid.clone();

        let t = tokio::task::spawn(async move {
            loop {
                interval.tick().await;
                let device_manager = request::get_device_manager(&master_addr, device_manager_uuid)
                    .await
                    .unwrap();

                tx.send(device_manager).unwrap();
            }
        });
    }

    fn render_device_lst(&self, indent: usize, device_manager: &DeviceManager) {
        println!("");
        let device_spec_map = &device_manager.id_spec_map;
        let device_uuid_lst = device_spec_map.keys();

        Cli::println_indent(indent, "Device 목록: ");

        for (idx, uuid) in device_uuid_lst.clone().enumerate() {
            let _spec = device_spec_map.get(uuid).unwrap();
            Cli::println_indent(
                indent + 1,
                &format!("{}> {}({})_{}", idx, _spec.os, _spec.os_version, _spec.ip),
            );
        }
        println!("");
    }

    fn render_file_system(&self, indent: usize, device_manager: &DeviceManager) {
        println!("");
        self.render_device_lst(indent, device_manager);
        let device_fs_map = &device_manager.id_fs_map;

        Cli::print_indent(indent, "\nFileSystem을 확인할 Device를 선택해주세요: ");
        io::stdout().flush().unwrap();

        let mut selected_device_uuid = String::new();
        io::stdin().read_line(&mut selected_device_uuid).unwrap();

        let selected_num: usize = selected_device_uuid.trim().parse().unwrap();
        let selected_device_fs = device_fs_map
            .get(&device_fs_map.keys().nth(selected_num).unwrap())
            .unwrap();
        Cli::print_indent(indent, &format!("{:?}", selected_device_fs));
        println!("");
    }

    fn render_file_transfer(&self, indent: usize, device_manager: &DeviceManager) {} // network 모듈? interface 활용
}

// TODO: gui와 공통된 부분 빼기
impl interface::Interface for Cli {
    fn new(master_addr: String) -> Self {
        Cli {
            master_addr,
            device_manager_uuid: Uuid::nil(),
            device_uuid: Uuid::new_v4(),
        }
    }

    async fn entry(&mut self) {
        self.enter_group().await;

        println!("DeviceManager UUID: {}", self.device_manager_uuid);
        self.register_device_spec(self.device_manager_uuid).await;
        self.register_device_fs(self.device_manager_uuid).await;

        let mut device_manager =
            request::get_device_manager(&self.master_addr, self.device_manager_uuid)
                .await
                .unwrap();

        self.render(&mut device_manager).await;
    }

    async fn render(&self, device_manager: &mut DeviceManager) {
        let indent: usize = 0;

        let (tx, rx) = mpsc::channel();
        self.sync_device_manager(tx).await;

        loop {
            match rx.recv() {
                Ok(recv) => {
                    let _ = std::mem::replace(device_manager, recv);
                }
                Err(_) => {
                    println!("알 수 없는 오류가 발생했습니다. 프로그램을 종료합니다.");
                    process::exit(-1);
                }
            }

            Cli::println_indent(
                indent + 1,
                &format!("{}> {}", ActionNum::DeviceList as usize, "DeviceListCheck"),
            );
            Cli::println_indent(
                indent + 1,
                &format!("{}> {}", ActionNum::FileSystem as usize, "FileSystemCheck"),
            );
            Cli::println_indent(
                indent + 1,
                &format!("{}> {}", ActionNum::FileTransfer as usize, "FileTransfer"),
            );
            Cli::println_indent(
                indent + 1,
                &format!("{}> {}", ActionNum::Exit as usize, "Exit"),
            );

            Cli::print_indent(indent, "\n동작을 선택해주세요: ");
            io::stdout().flush().unwrap();

            let mut action_num: String = String::new();
            io::stdin()
                .read_line(&mut action_num)
                .expect("입력에 실패했습니다.");
            let action_num: i32 = action_num.trim().parse().unwrap();

            Cli::println_indent(
                indent,
                "------------------------------------------------------",
            );

            match ActionNum::try_from(action_num).unwrap() {
                ActionNum::DeviceList => self.render_device_lst(indent + 1, device_manager),
                ActionNum::FileSystem => self.render_file_system(indent + 1, device_manager),
                ActionNum::FileTransfer => self.render_file_transfer(indent + 1, device_manager),
                ActionNum::Exit => self.exit(None).await,
            };

            Cli::println_indent(
                indent,
                "------------------------------------------------------",
            );
        }
    }

    async fn exit(&self, error_opt: Option<String>) {
        println!("프로그램을 종료합니다. {:?}", error_opt);

        {
            let deleted_device_uuid_spec = request::delete_device_spec(
                &self.master_addr,
                self.device_manager_uuid,
                self.device_uuid,
            )
            .await;

            let deleted_device_uuid_fs = request::delete_device_fs(
                &self.master_addr,
                self.device_manager_uuid,
                self.device_uuid,
            )
            .await;

            match (deleted_device_uuid_spec, deleted_device_uuid_fs) {
                (Ok(uuid), Ok(_)) => {
                    println!("Group에서 device를 제거하는데 성공했습니다: {}", uuid)
                }
                _ => println!("Device를 제거하는 과정에서 문제가 발생했습니다."),
            }
        }

        process::exit(-1)
    }

    async fn register_device_spec(&self, manager_uuid: Uuid) {
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

        match request::post_device_spec(&self.master_addr, manager_uuid, self.device_uuid, spec)
            .await
        {
            Ok(uuid) => {
                println!("device spec 등록 완료: {}", uuid);
            }
            Err(e) => {
                println!("서버와의 연결상태를 다시 확인해주시기 바랍니다.");
                println!("{:?}", e);
                process::exit(-1);
            }
        }
    }

    async fn register_device_fs(&self, manager_uuid: Uuid) {
        println!("Group에 공유할 file system의 root를 지정해주세요.");
        println!("e.g. MacOSX : /Users/username/Desktop/public_dir");
        println!("     Linux  : /home/username/public_dir");

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

        match request::post_device_fs(&self.master_addr, manager_uuid, self.device_uuid, device_fs)
            .await
        {
            Ok(uuid) => {
                println!("device fs 등록 완료: {}", uuid);
            }
            Err(e) => {
                println!("서버와의 연결상태를 다시 확인해주시기 바랍니다.");
                println!("{:?}", e);
                process::exit(-1);
            }
        }
    }

    async fn enter_group(&mut self) {
        loop {
            print!("이미 존재하는 group에 참가합니다. [Y/n]: ");
            io::stdout().flush().unwrap();

            let mut group_enter: String = String::new();
            io::stdin()
                .read_line(&mut group_enter)
                .expect("입력에 실패했습니다.");

            let group_enter = group_enter.trim().to_lowercase();
            if group_enter.eq("n") {
                self.create_group().await;
                return;
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

        self.device_manager_uuid = manager_uuid;
    }

    async fn create_group(&mut self) {
        let new_manager_uuid = request::post_device_manager(&self.master_addr).await;

        match new_manager_uuid {
            Ok(uuid) => {
                self.device_manager_uuid = uuid;
            }
            Err(e) => {
                println!("서버와의 연결상태를 다시 확인해주시기 바랍니다.");
                println!("{:?}", e);
            }
        }
    }
}
