use device::device::file_sys::FileSystem;
use device::device::spec::DeviceSpec;

use std::collections::HashMap;

pub struct DeviceManager {
    // spec을 가리키는 id와 fs를 가리키는 id가 동일해야 됨 (client의 고유 id)
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
