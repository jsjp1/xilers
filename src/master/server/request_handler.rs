use super::device_manager::DeviceManager;
use super::error_handler::ErrorType;
use super::error_handler::NotAbortError;
use crate::server::api;
use crate::server::db;

use std::collections::HashMap;
use std::io::{Read, Write};
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

    pub fn get_device_manager(&self, id: Uuid) -> Result<Option<DeviceManager>, String> {
        let client_group_lock = self.client_group.lock();
        match client_group_lock {
            Ok(m_guard) => {
                let device_manager_opt = m_guard.get(&id).cloned();
                Ok(device_manager_opt)
            }
            Err(e) => Err(e.to_string()),
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

    pub async fn init(&self) -> Result<(), ErrorType> {
        // TODO: db_client 추후 사용할 수 있으므로 반환하도록 변경
        match self.db.connect_mongodb().await {
            Ok(db_client) => {
                db::MongoDB::create_collection(&db_client, "xilers", "device_spec").await;
                db::MongoDB::create_collection(&db_client, "xilers", "device_file_system").await;

                Ok(())
            }
            Err(e) => {
                let _abort_type = NotAbortError::Severe(e);
                let not_abort_error = ErrorType::NotAbortError(_abort_type);
                Err(not_abort_error)
            }
        }
    }

    pub fn handle_connection(&self, stream: &mut TcpStream) -> Result<bool, ErrorType> {
        let mut buffer = [0; 1024];

        let res = stream.read(&mut buffer);
        match res {
            Ok(_) => {
                let request = String::from_utf8_lossy(&buffer[..]);
                let api_request = self.parse_request(&request);
                let http_version = api_request.http_version.to_string();

                let response_data = api::method_router(api_request, &self.client_group);

                match response_data {
                    Ok(response) => {
                        let http_response = RequestHandler::http_response_wrapping(
                            response,
                            http_version,
                            "200 OK".to_string(),
                        );
                        stream.write(http_response.as_bytes()).unwrap();
                        Ok(true)
                    }
                    Err(e) => {
                        let _abort_type = NotAbortError::Minor(e.to_string());
                        let not_abort_error = ErrorType::NotAbortError(_abort_type);
                        Err(not_abort_error)
                    }
                }
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

    fn http_response_wrapping(
        response: String,
        http_version: String,
        http_status: String,
    ) -> String {
        let response = format!(
            "{} {}\r\nContent-Length: {}\r\n\r\n{}",
            http_version,
            http_status,
            response.len(),
            response
        );
        response
    }
}
