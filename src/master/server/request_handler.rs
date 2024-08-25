use super::device_manager::DeviceManager;
use super::error_handler::ErrorType;
use super::error_handler::NotAbortError;
use crate::server::api;
use crate::server::db;

use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex, MutexGuard};
use uuid::Uuid;

pub struct ClientGroup {
    pub client_group: HashMap<Uuid, DeviceManager>, // 기존에는 여기서 Arc Mutex로 감쌌는데, get_device와 같은 작업할 때만 new로 감싸기
}

impl ClientGroup {
    pub fn new() -> Self {
        ClientGroup {
            client_group: (HashMap::new()),
        }
    }

    pub fn get_device_manager(&mut self, id: Uuid) -> Result<Option<&mut DeviceManager>, String> {
        let device_manager_opt = self.client_group.get_mut(&id);
        match device_manager_opt {
            Some(manager) => Ok(Some(manager)),
            None => Ok(None),
        }
    }

    pub fn add_device_manager(
        &mut self,
        id: Uuid,
        device_manager: DeviceManager,
    ) -> Result<(), String> {
        self.client_group.insert(id, device_manager);
        Ok(())
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

    pub fn handle_connection(&mut self, stream: &mut TcpStream) -> Result<(), ErrorType> {
        // let mut buffer = [0; 1024];
        let mut buffer = vec![0; 1_000_000]; // test용으로 잠깐... 이후 수정 필요

        let res = stream.read(&mut buffer);
        match res {
            Ok(_) => {
                let request = String::from_utf8_lossy(&buffer[..]);

                let api_request = RequestHandler::parse_request(&request);
                let http_version = api_request.http_version.to_string();
                let response_data = api::method_router(api_request, self.client_group.borrow_mut());

                // TODO: http response status code 추가
                match response_data {
                    Ok(response) => {
                        let http_response = RequestHandler::http_response_wrapping(
                            response,
                            http_version,
                            "200 OK".to_string(),
                        );
                        stream.write(http_response.as_bytes()).unwrap();
                        Ok(())
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

    fn parse_request<'a>(request_str: &'a str) -> api::Request {
        let request_slice_vec: Vec<&'a str> = request_str.split("\r\n").collect();
        let header_info: Vec<&'a str> = request_slice_vec[0].split(" ").collect();
        let method = header_info[0];
        let path = header_info[1];
        let http_version = header_info[2];
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
            http_version,
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
