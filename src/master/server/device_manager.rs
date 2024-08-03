use device::device::file_sys::FileSystem;
use device::device::spec::DeviceSpec;

use std::collections::HashMap;

pub struct DeviceManager {
    id_spec_map: HashMap<String, DeviceSpec>,
    id_fs_map: HashMap<String, FileSystem>,
}

impl DeviceManager {
    fn new() -> Self {
        DeviceManager {
            id_spec_map: HashMap::new(),
            id_fs_map: HashMap::new(),
        }
    }
}
