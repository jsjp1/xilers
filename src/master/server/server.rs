use crate::server::error_handler::NotAbortError;

use super::api;
use super::error_handler::{ErrorHandler, ErrorType};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::{sync::mpsc, time};

use super::device_manager::DeviceManager;
use actix_web::Responder;
use actix_web::{dev::ServerHandle, middleware, rt, web, App, HttpRequest, HttpServer};
use uuid::Uuid;

pub struct AppState {
    pub client_group: ClientGroup,
}

#[derive(Clone, Debug)]
pub struct ClientGroup {
    pub client_group: HashMap<Uuid, DeviceManager>, // 기존에는 여기서 Arc Mutex로 감쌌는데, get_device와 같은 작업할 때만 new로 감싸기
}

impl ClientGroup {
    pub fn new() -> Self {
        ClientGroup {
            client_group: (HashMap::new()),
        }
    }

    pub fn get_device_manager(&mut self, id: Uuid) -> Option<&mut DeviceManager> {
        let device_manager_opt = self.client_group.get_mut(&id);
        device_manager_opt
    }

    pub fn add_device_manager(
        &mut self,
        id: Uuid,
        device_manager: DeviceManager,
    ) -> Result<(), String> {
        self.client_group.insert(id, device_manager);
        Ok(())
    }

    pub fn delete_device_manager(&mut self, id: Uuid) -> bool {
        match self.client_group.remove(&id) {
            Some(_) => true,
            None => false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Server {
    pub ip: String,
    pub port: u16,
    pub db_ip: String,
    pub db_port: u16,
    pub client_group: ClientGroup,
}

impl Server {
    // 받은 요청을 기반으로 DeviceManager의 정보를 이용해 응답
    pub fn new(ip: String, port: u16, db_ip: String, db_port: u16) -> Self {
        Server {
            ip,
            port,
            db_ip,
            db_port,
            client_group: ClientGroup::new(),
        }
    }

    pub async fn init_and_run(&self, tx: mpsc::Sender<ServerHandle>) -> std::io::Result<()> {
        let app_state = web::Data::new(Mutex::new(AppState {
            client_group: self.client_group.clone(),
        }));

        let server = HttpServer::new(move || {
            App::new().app_data(app_state.clone()).service(
                web::scope("/api")
                    .route(
                        "/device-manager",
                        web::post().to(api::post::add_device_manager),
                    )
                    .route(
                        "/device-manager/{manager_uuid}/spec/{device_uuid}",
                        web::post().to(api::post::add_device_spec),
                    )
                    .route(
                        "/device-manager/{manager_uuid}/fs/{device_uuid}",
                        web::post().to(api::post::add_device_fs),
                    )
                    .route(
                        "/device-manager/{manager_uuid}",
                        web::get().to(api::get::get_device_manager),
                    )
                    .route(
                        "/device-manager/{manager_uuid}/spec/{spec_uuid}",
                        web::get().to(api::get::get_device_spec),
                    )
                    .route(
                        "/device-manager/{manager_uuid}/fs/{fs_uuid}",
                        web::get().to(api::get::get_device_fs),
                    )
                    .route(
                        "/device-manager/{manager_uuid}",
                        web::delete().to(api::delete::delete_device_manager),
                    )
                    .route(
                        "/device-manager/{manager_uuid}/spec/{spec_uuid}",
                        web::delete().to(api::delete::delete_device_spec),
                    )
                    .route(
                        "/device-manager/{manager_uuid}/fs/{fs_uuid}",
                        web::delete().to(api::delete::delete_device_fs),
                    ),
            )
        })
        .bind((self.ip.clone(), self.port))?
        .workers(10)
        .run();

        let _ = tx.send(server.handle());

        server.await
    }
}
