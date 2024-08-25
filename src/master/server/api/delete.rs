use std::borrow::BorrowMut;
use std::sync::Mutex;

use actix_web::{web, HttpResponse, Responder, Result};
use uuid::Uuid;

use crate::server;

pub async fn delete_device_manager(
    data: web::Data<Mutex<server::server::AppState>>,
    path: web::Path<String>,
) -> Result<impl Responder> {
    let mut data_lock = data.lock().unwrap();
    let client_group = data_lock.client_group.borrow_mut();

    let manager_uuid = Uuid::parse_str(&path).unwrap();
    log::debug!("device manager 정보를 삭제합니다. uuid: {}", manager_uuid);
    match client_group.delete_device_manager(manager_uuid) {
        true => Ok(HttpResponse::Ok().body(manager_uuid.to_string())),
        false => {
            log::warn!("해당하는 manager가 없습니다.");
            return Ok(HttpResponse::NotFound().body("해당하는 manager가 없습니다."));
        }
    }
}

pub async fn delete_device_spec(
    data: web::Data<Mutex<server::server::AppState>>,
    path: web::Path<(String, String)>,
) -> Result<impl Responder> {
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

    log::debug!("device spec 정보를 삭제합니다. uuid: {}", spec_uuid);
    match manager.delete_device_spec(spec_uuid) {
        true => Ok(HttpResponse::Ok().body(spec_uuid.to_string())),
        false => {
            log::warn!("해당하는 spec이 없습니다.");
            return Ok(HttpResponse::NotFound().body("해당하는 spec이 없습니다."));
        }
    }
}

pub async fn delete_device_fs(
    data: web::Data<Mutex<server::server::AppState>>,
    path: web::Path<(String, String)>,
) -> Result<impl Responder> {
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

    log::debug!("device fs 정보를 삭제합니다. uuid: {}", fs_uuid);
    match manager.delete_device_fs(fs_uuid) {
        true => Ok(HttpResponse::Ok().body(fs_uuid.to_string())),
        false => {
            log::warn!("해당하는 fs가 없습니다.");
            return Ok(HttpResponse::NotFound().body("해당하는 fs가 없습니다."));
        }
    }
}
