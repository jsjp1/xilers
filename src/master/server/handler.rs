use super::device_manager::DeviceManager;
use crate::server::db;

use std::collections::HashMap;
use std::net::TcpStream;

pub struct Handler {
    pub client_groups: HashMap<String, DeviceManager>,
    pub db: db::MongoDB,
}

impl Handler {
    pub fn new(db_ip: String, db_port: String) -> Self {
        Handler {
            db: db::MongoDB::new(db_ip, db_port),
            client_groups: HashMap::new(),
        }
    }

    pub async fn init(&self) {
        let db_client = self.db.connect_mongodb().await.unwrap();

        // TODO: 이미 존재하는 경우에 대한 처리
        db::MongoDB::create_collection(&db_client, "xilers", "device_spec").await;
        db::MongoDB::create_collection(&db_client, "xilers", "device_file_system").await;
    }

    pub fn handle_connection(&self, stream: TcpStream) {}
}
