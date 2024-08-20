use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeviceSpec {
    pub ip: String,
    pub os: String,
    pub os_version: String,
}
