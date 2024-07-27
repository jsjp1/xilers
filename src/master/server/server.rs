use device::device::spec::DeviceSpec;

use std::collections::HashMap;

struct DeviceManager {
    id_spec_map: HashMap<String, DeviceSpec>,
}

impl DeviceManager {
    fn new() -> Self {
        DeviceManagee {
            id_spec_map: HashMap::new(),
        }
    }
}

pub struct Server {
    pub ip: &'static str,
    pub port: u16,
}

impl Server {
    pub fn new(ip: &'static str, port: &'static str) -> Self {
        Server { ip, port }
    }
}
