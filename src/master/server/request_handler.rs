use super::device_manager::DeviceManager;
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

    pub fn add_device_manager(&self, id: Uuid, device_manager: DeviceManager) -> bool {
        let _manager_map_mutex = self.client_group.lock();
        match _manager_map_mutex {
            Ok(mut manager_map) => {
                let insert_res = manager_map.insert(id, device_manager);
                match insert_res {
                    Some(_) => true,
                    None => false,
                }
            }
            Err(e) => {
                log::error!("{}", e);
                false
            }
        }
    }
}

pub struct Handler {
    pub client_group: ClientGroup,
    pub db: db::MongoDB,
}

impl Handler {
    pub fn new(db_ip: String, db_port: String) -> Self {
        Handler {
            db: db::MongoDB::new(db_ip, db_port),
            client_group: ClientGroup::new(),
        }
    }

    pub async fn init(&self) {
        let db_client = self.db.connect_mongodb().await.unwrap();

        db::MongoDB::create_collection(&db_client, "xilers", "device_spec").await;
        db::MongoDB::create_collection(&db_client, "xilers", "device_file_system").await;
    }

    pub fn handle_connection(&self, stream: &mut TcpStream) {
        let mut buffer = [0; 1024];

        let res = stream.read(&mut buffer);
        match res {
            Ok(_) => {
                let request = String::from_utf8_lossy(&buffer[..]);
                let api_request = self.parse_request(&request);
            }
            Err(e) => {
                log::error!("{}", e);
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
