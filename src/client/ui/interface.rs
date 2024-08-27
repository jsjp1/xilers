use uuid::Uuid;

use super::request::DeviceManager;

pub trait Interface {
    async fn entry(&self);
    async fn register_device_fs(&self, manager_uuid: Uuid) -> Result<Uuid, ()>;
    async fn register_device_spec(&self, manager_uuid: Uuid) -> Result<Uuid, ()>;
    fn enter_group(&self) -> Option<Uuid>;
    async fn create_group(&self) -> Result<Uuid, ()>;
    fn render(&self, device_manager: &DeviceManager);
}
