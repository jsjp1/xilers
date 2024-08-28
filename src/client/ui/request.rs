use std::collections::HashMap;

use device::device::{file_sys::FileSystem, spec::DeviceSpec};

use reqwest;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DeviceManager {
    pub id_spec_map: HashMap<Uuid, DeviceSpec>,
    pub id_fs_map: HashMap<Uuid, FileSystem>,
}

pub async fn get_device_manager(
    master_addr: &str,
    manager_uuid: Uuid,
) -> Result<DeviceManager, reqwest::Error> {
    let request_addr = format!("{}/api/device-manager/{}", master_addr, manager_uuid);
    let request = match reqwest::get(request_addr).await {
        Ok(response) => response,
        Err(e) => return Err(e),
    };

    let device_manager_str = request.text().await.unwrap();
    let device_manager = serde_json::from_str(&device_manager_str).unwrap();

    Ok(device_manager)
}

pub async fn post_device_manager(master_addr: &str) -> Result<Uuid, reqwest::Error> {
    let request_addr = format!("{}/api/device-manager", master_addr);
    let client = reqwest::Client::new();
    let request = match client.post(request_addr).body("").send().await {
        Ok(response) => response,
        Err(e) => return Err(e),
    };

    let new_manager_uuid_str = request.text().await.unwrap();
    let new_manager_uuid = Uuid::parse_str(&new_manager_uuid_str).unwrap();

    Ok(new_manager_uuid)
}

pub async fn post_device_spec(
    master_addr: &str,
    manager_uuid: Uuid,
    new_spec_uuid: Uuid,
    spec: DeviceSpec,
) -> Result<Uuid, reqwest::Error> {
    let request_addr = format!(
        "{}/api/device-manager/{}/spec/{}",
        master_addr, manager_uuid, new_spec_uuid
    );
    let client = reqwest::Client::new();

    let serialized_spec = serde_json::to_string(&spec).unwrap();
    let request = match client.post(request_addr).body(serialized_spec).send().await {
        Ok(response) => response,
        Err(e) => return Err(e),
    };

    let spec_uuid_str = request.text().await.unwrap();
    let spec_uuid = Uuid::parse_str(&spec_uuid_str).unwrap();

    Ok(spec_uuid)
}

pub async fn post_device_fs(
    master_addr: &str,
    manager_uuid: Uuid,
    new_fs_uuid: Uuid,
    fs: FileSystem,
) -> Result<Uuid, reqwest::Error> {
    let request_addr = format!(
        "{}/api/device-manager/{}/fs/{}",
        master_addr, manager_uuid, new_fs_uuid
    );
    let client = reqwest::Client::new();

    let serialized_fs = serde_json::to_string(&fs).unwrap();
    let request = match client.post(request_addr).body(serialized_fs).send().await {
        Ok(response) => response,
        Err(e) => return Err(e),
    };

    let fs_uuid_str = request.text().await.unwrap();
    let fs_uuid = Uuid::parse_str(&fs_uuid_str).unwrap();

    Ok(fs_uuid)
}
