use sysinfo::{System, SystemExt};

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
        .post("http://xilers.jieeen.co.kr:8080/device-manager/")
        .body("")
        .send()
        .await
        .unwrap();
    // 응답 본문을 텍스트로 변환
    let body = uuid.text().await.unwrap();
    let spec = serde_json::to_string(&a).unwrap();
    let s = format!(r"{}", spec);

    let c = format!(
        "http://xilers.jieeen.co.kr:8080/device-manager/{}/spec",
        body
    );
    let d = client.post(c).body(s).send().await.unwrap();
    let aaa = d.text().await.unwrap();

    // 응답 출력
    println!("Response: {:?}", aaa);
}
