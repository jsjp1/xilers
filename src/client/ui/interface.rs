use std::sync::{Arc, Mutex};
use uuid::Uuid;

use super::request::DeviceManager;

pub trait Interface {
    fn new(master_addr: String, listen_port: u16, file_storage: String) -> Self;
    async fn entry(&mut self);
    async fn exit(&self, error_opt: Option<String>);
    async fn render(&self, device_manager: Arc<Mutex<DeviceManager>>);
    async fn register_device_fs(&self, manager_uuid: Uuid);
    async fn register_device_spec(&self, manager_uuid: Uuid);
    async fn enter_group(&mut self);
    async fn create_group(&mut self);
}
