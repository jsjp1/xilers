use std::borrow::BorrowMut;
use std::sync::Mutex;

use actix_web::{web, HttpResponse, Responder, Result};
use uuid::Uuid;

use crate::server;

pub async fn get_device_manager(
    data: web::Data<Mutex<server::server::AppState>>,
    path: web::Path<String>,
) -> Result<impl Responder> {
    log::debug!("device manager 정보를 가져옵니다.");
    let mut data_lock = data.lock().unwrap();
    let client_group = data_lock.client_group.borrow_mut();

    let manager_uuid = Uuid::parse_str(&path).unwrap();

    let manager = match client_group.get_device_manager(manager_uuid) {
        Some(manager) => manager,
        None => {
            log::warn!("해당하는 manager가 없습니다.");
            return Ok(HttpResponse::NotFound().body("헤당하는 manager가 없습니다."));
        }
    };

    let serialized_manager = serde_json::to_string(&manager)
        .map_err(|e| e.to_string())
        .unwrap();

    Ok(HttpResponse::Ok().body(serialized_manager))
}

pub async fn get_device_spec(
    data: web::Data<Mutex<server::server::AppState>>,
    path: web::Path<(String, String)>,
) -> Result<impl Responder> {
    log::debug!("device spec 정보를 가져옵니다.");
    let mut data_lock = data.lock().unwrap();
    let client_group = data_lock.client_group.borrow_mut();

    let manager_uuid = Uuid::parse_str(&path.0).unwrap();
    let spec_uuid = Uuid::parse_str(&path.1).unwrap();

    let manager = match client_group.get_device_manager(manager_uuid) {
        Some(manager) => manager,
        None => {
            log::warn!("해당하는 manager가 없습니다.");
            return Ok(HttpResponse::NotFound().body("해당하는 manager가 없습니다."));
        }
    };

    let spec = match manager.get_device_spec(spec_uuid) {
        Some(spec) => spec,
        None => {
            log::warn!("해당하는 spec이 없습니다.");
            return Ok(HttpResponse::NotFound().body("헤당하는 spec이 없습니다."));
        }
    };

    let serialized_spec = serde_json::to_string(&spec)
        .map_err(|e| e.to_string())
        .unwrap();

    Ok(HttpResponse::Ok().body(serialized_spec))
}

pub async fn get_device_fs(
    data: web::Data<Mutex<server::server::AppState>>,
    path: web::Path<(String, String)>,
) -> Result<impl Responder> {
    log::debug!("device fs 정보를 가져옵니다.");
    let mut data_lock = data.lock().unwrap();
    let client_group = data_lock.client_group.borrow_mut();

    let manager_uuid = Uuid::parse_str(&path.0).unwrap();
    let fs_uuid = Uuid::parse_str(&path.1).unwrap();

    let manager = match client_group.get_device_manager(manager_uuid) {
        Some(manager) => manager,
        None => {
            log::warn!("해당하는 manager가 없습니다.");
            return Ok(HttpResponse::NotFound().body("해당하는 manager가 없습니다."));
        }
    };

    let fs = match manager.get_device_fs(fs_uuid) {
        Some(fs) => fs,
        None => {
            log::warn!("해당하는 fs가 없습니다.");
            return Ok(HttpResponse::NotFound().body("해당하는 fs가 없습니다."));
        }
    };

    let serialized_fs = serde_json::to_string(&fs)
        .map_err(|e| e.to_string())
        .unwrap();

    Ok(HttpResponse::Ok().body(serialized_fs))
}
