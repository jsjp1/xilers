use std::borrow::BorrowMut;
use std::sync::Mutex;

use actix_web::{web, HttpRequest, HttpResponse, Responder, Result};
use device::device::file_sys::FileSystem;
use device::device::spec::DeviceSpec;
use uuid::Uuid;

use crate::server;
use crate::server::device_manager::DeviceManager;

pub async fn add_device_manager(
    data: web::Data<Mutex<server::server::AppState>>,
) -> Result<impl Responder> {
    let mut data_lock = data.lock().unwrap();
    let client_group = data_lock.client_group.borrow_mut();

    let new_manager_uuid = Uuid::new_v4();
    let new_manager = DeviceManager::new();

    client_group
        .add_device_manager(new_manager_uuid, new_manager)
        .unwrap();

    log::debug!(
        "새로운 manager가 추가되었습니다. uuid: {}",
        new_manager_uuid
    );

    Ok(HttpResponse::Ok().body(new_manager_uuid.to_string()))
}

pub async fn add_device_spec(
    req: HttpRequest,
    data: web::Data<Mutex<server::server::AppState>>,
    path: web::Path<(String, String)>,
    spec: web::Bytes, // serialize된 spec
) -> Result<impl Responder> {
    let mut data_lock = data.lock().unwrap();
    let client_group = data_lock.client_group.borrow_mut();

    let manager_uuid = Uuid::parse_str(&path.0).unwrap();
    let manager = match client_group.get_device_manager(manager_uuid) {
        Some(manager) => manager,
        None => {
            return Ok(HttpResponse::NotFound().body("해당하는 manager가 없습니다."));
        }
    };

    let new_spec_uuid = Uuid::parse_str(&path.1).unwrap();
    let mut spec: DeviceSpec = serde_json::from_slice(spec.as_ref()).unwrap();
    spec.ip = req.peer_addr().unwrap().ip().to_string();

    manager.add_device_spec(new_spec_uuid, spec.clone());
    log::debug!("새로운 spec이 추가되었습니다. uuid: {}", new_spec_uuid);

    Ok(HttpResponse::Ok().body(new_spec_uuid.to_string()))
}

pub async fn add_device_fs(
    data: web::Data<Mutex<server::server::AppState>>,
    path: web::Path<(String, String)>,
    fs: web::Bytes, // serialize된 fs
) -> Result<impl Responder> {
    let mut data_lock = data.lock().unwrap();
    let client_group = data_lock.client_group.borrow_mut();

    let manager_uuid = Uuid::parse_str(&path.0).unwrap();
    let manager = match client_group.get_device_manager(manager_uuid) {
        Some(manager) => manager,
        None => {
            return Ok(HttpResponse::NotFound().body("해당하는 manager가 없습니다."));
        }
    };

    let new_fs_uuid = Uuid::parse_str(&path.1).unwrap();
    let fs: FileSystem = serde_json::from_slice(fs.as_ref()).unwrap();

    manager.add_device_fs(new_fs_uuid, fs.clone());
    log::debug!("새로운 fs가 추가되었습니다. uuid: {}", new_fs_uuid);

    Ok(HttpResponse::Ok().body(new_fs_uuid.to_string()))
}
