use std::borrow::{Borrow, BorrowMut};

use sysinfo::{System, SystemExt};

use device::device::file_sys::{FileNode, FileSystem};
use device::device::spec::DeviceSpec;

#[tokio::main]
async fn main() {
    let mut system = System::new_all();
    system.refresh_all();

    let os_name = system
        .name()
        .unwrap_or_else(|| "os 정보 확인 불가".to_string());
    let os_version = system
        .long_os_version()
        .unwrap_or_else(|| "os version 정보 확인 불가".to_string());

    let a: DeviceSpec = DeviceSpec {
        ip: "11".to_string(),
        os: os_name,
        os_version: os_version,
    };

    let client = reqwest::Client::new();
    let uuid = client
        .post("http://127.0.0.1:8080/api/device-manager")
        .body("")
        .send()
        .await
        .unwrap();
    // 응답 본문을 텍스트로 변환
    let body = uuid.text().await.unwrap();
    println!("BODY: {:?}", body);
    let spec = serde_json::to_string(&a).unwrap();
    let s = format!(r"{}", spec);

    let c = format!("http://127.0.0.1:8080/api/device-manager/{}/spec", body);
    let d = client.post(c).body(s).send().await.unwrap();
    let aaa = d.text().await.unwrap();

    // 응답 출력
    println!("Response: {:?}", aaa);

    let mut root_path = "/Users/jin/Desktop/prj/tet/tt".to_string();
    let mut file_system = FileSystem::new(root_path.borrow_mut());
    file_system.init_file_node();

    // Structure 출력
    println!("{:#?}", file_system);

    let x = format!("http://127.0.0.1:8080/api/device-manager/{}/fs", body);
    let y = client
        .post(x)
        .body(serde_json::to_string(&file_system).unwrap())
        .send()
        .await
        .unwrap();
    let z = y.text().await.unwrap();

    println!("Response: {:?}", z);
}
