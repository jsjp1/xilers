use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DeviceSpec {
    pub ip: &'static str,
    pub port: &'static str,
    pub os: String,
}
