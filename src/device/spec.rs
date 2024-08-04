use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DeviceSpec {
    pub ip: String,
    pub port: u16, // 어떤 port로 listen하고 있는지
    pub os: String,
    pub os_version: String,
}
