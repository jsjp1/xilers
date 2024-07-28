use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DeviceSpec {
    pub ip: String,
    pub port: u16,
    pub os: String,
}
