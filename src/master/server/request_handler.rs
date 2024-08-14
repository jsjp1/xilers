use super::device_manager::DeviceManager;
use super::error_handler::NotAbortError;
use super::error_handler::{ErrorHandler, ErrorType};
use crate::server::api;
use crate::server::db;

use std::collections::HashMap;
use std::io::Read;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

pub struct ClientGroup {
    pub client_group: Arc<Mutex<HashMap<Uuid, DeviceManager>>>,
}

impl ClientGroup {
    pub fn new() -> Self {
        ClientGroup {
            client_group: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn add_device_manager(
        &self,
        id: Uuid,
        device_manager: DeviceManager,
    ) -> Result<(), String> {
        let _manager_map_mutex = self.client_group.lock();
        match _manager_map_mutex {
            Ok(mut manager_map) => {
                manager_map.insert(id, device_manager);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
}

pub struct RequestHandler {
    // request handling과 error 처리
    pub client_group: ClientGroup,
    pub db: db::MongoDB,
}

impl RequestHandler {
    pub fn new(db_ip: String, db_port: String) -> Self {
        RequestHandler {
            db: db::MongoDB::new(db_ip, db_port),
            client_group: ClientGroup::new(),
        }
    }

    pub async fn init(&self) {
        let db_client = self.db.connect_mongodb().await.unwrap();

        db::MongoDB::create_collection(&db_client, "xilers", "device_spec").await;
        db::MongoDB::create_collection(&db_client, "xilers", "device_file_system").await;
    }

    pub fn handle_connection(&self, stream: &mut TcpStream) -> Result<(), ErrorType> {
        let mut buffer = [0; 1024];

        let res = stream.read(&mut buffer);
        match res {
            Ok(_) => {
                let request = String::from_utf8_lossy(&buffer[..]);
                let api_request = self.parse_request(&request);

                // TODO: api 결과 받아서 처리 -> nonblocking io + callback?
                // TODO: throw된 error받아서 error handler로 보냄

                Ok(())
            }
            Err(e) => {
                let _abort_type = NotAbortError::Minor(e.to_string());
                let not_abort_error = ErrorType::NotAbortError(_abort_type);
                Err(not_abort_error)
            }
        }
    }

    fn parse_request<'a>(&'a self, request_str: &'a str) -> api::Request {
        let request_slice_vec: Vec<&'a str> = request_str.split("\r\n").collect();
        let header_info: Vec<&'a str> = request_slice_vec[0].split(" ").collect();
        let method = header_info[0];
        let path = header_info[1];
        let payload = request_slice_vec[request_slice_vec.len() - 1];

        log::debug!(
            "\nMethod: {}\nPath: {}\nPayload: {}\n",
            method,
            path,
            payload
        );
        let api_request_struct = api::Request {
            method,
            path,
            payload,
        };

        api_request_struct
    }
}
