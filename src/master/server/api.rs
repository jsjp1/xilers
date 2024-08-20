use super::request_handler::ClientGroup;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json;
use uuid::Uuid;

#[derive(Copy, Clone)]
pub struct Request<'a> {
    pub method: &'a str,
    pub path: &'a str,
    pub payload: &'a str,
    pub http_version: &'a str,
}

pub fn method_router(request: Request, client_group: &mut ClientGroup) -> Result<String, String> {
    // TODO: return값 혹은 error 반환하도록
    // GET의 Ok -> 요구하는 정보 반환
    // POST의 Ok -> uuid값을 전달 (현재는 기기 등록만 구현)
    match request.method {
        "GET" => get::path_router(request, client_group),
        "POST" => post::path_router(request, client_group),
        _ => Err("지원하지 않는 method입니다.".to_string()),
    }
}

pub fn extract_uuid_from_request(request_path: &str, regex_path: &str) -> Uuid {
    let re = Regex::new(regex_path).unwrap();
    let uuid_str = re.find(request_path).unwrap().as_str();
    Uuid::parse_str(uuid_str).unwrap()
}

pub fn serialize_object<T: Serialize>(obj: &T) -> Result<String, serde_json::Error> {
    serde_json::to_string(obj)
}

pub fn deserialize_object<'a, T: Deserialize<'a>>(
    json_str: &'a str,
) -> Result<T, serde_json::Error> {
    serde_json::from_str(json_str)
}

mod get {
    use super::super::request_handler::ClientGroup;
    use super::Request;

    use regex::Regex;
    use uuid::Uuid;

    pub fn path_router(request: Request, client_group: &mut ClientGroup) -> Result<String, String> {
        let path_device_manager = Regex::new(r"^/device-manager").unwrap();
        let path_device_spec = Regex::new(r"/spec").unwrap();
        let path_device_fs = Regex::new(r"/fs").unwrap();

        let uuid_regex =
            r"\b[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}\b";
        let request_path = request.path;

        if path_device_fs.is_match(request_path) {
            log::debug!("device fs 요청");
            let manager_uuid = super::extract_uuid_from_request(request_path, uuid_regex);
            let _request_path = &request_path[request_path.find("/fs").unwrap()..];
            let fs_uuid = super::extract_uuid_from_request(_request_path, uuid_regex);

            get_device_fs(client_group, manager_uuid, fs_uuid)
        } else if path_device_spec.is_match(request_path) {
            log::debug!("device spec 요청");
            let manager_uuid = super::extract_uuid_from_request(request_path, uuid_regex);
            let _request_path = &request_path[request_path.find("/spec").unwrap()..];
            let spec_uuid = super::extract_uuid_from_request(_request_path, uuid_regex);

            get_device_spec(client_group, manager_uuid, spec_uuid)
        } else if path_device_manager.is_match(request_path) {
            log::debug!("device manager 요청");
            let uuid = super::extract_uuid_from_request(request_path, uuid_regex);

            get_device_manager(client_group, uuid)
        } else {
            Err("지원하지 않는 path입니다.".to_string())
        }
    }

    fn get_device_fs(
        client_group: &mut ClientGroup,
        manager_uuid: Uuid,
        fs_uuid: Uuid,
    ) -> Result<String, String> {
        let device_manager_opt = client_group.get_device_manager(manager_uuid);
        match device_manager_opt {
            Ok(device_manager) => match device_manager {
                Some(manager) => {
                    let device_fs_opt = manager.get_device_fs(fs_uuid);
                    match device_fs_opt {
                        Some(fs) => {
                            let serialized_fs = super::serialize_object(&fs);
                            match serialized_fs {
                                Ok(serialized) => Ok(serialized),
                                Err(e) => Err(e.to_string()),
                            }
                        }
                        None => Err("해당하는 device fs가 없습니다.".to_string()),
                    }
                }
                None => Err("해당하는 device manager가 없습니다.".to_string()),
            },
            Err(e) => Err(e),
        }
    }

    fn get_device_spec(
        client_group: &mut ClientGroup,
        manager_uuid: Uuid,
        spec_uuid: Uuid,
    ) -> Result<String, String> {
        let device_manager_opt = client_group.get_device_manager(manager_uuid);
        match device_manager_opt {
            Ok(device_manager) => match device_manager {
                Some(manager) => {
                    let device_spec_opt = manager.get_device_spec(spec_uuid);
                    match device_spec_opt {
                        Some(spec) => {
                            let serialized_spec = super::serialize_object(&spec);
                            match serialized_spec {
                                Ok(serialized) => Ok(serialized),
                                Err(e) => Err(e.to_string()),
                            }
                        }
                        None => Err("해당하는 device spec이 없습니다.".to_string()),
                    }
                }
                None => Err("해당하는 device manager가 없습니다.".to_string()),
            },
            Err(e) => Err(e),
        }
    }

    fn get_device_manager(client_group: &mut ClientGroup, uuid: Uuid) -> Result<String, String> {
        let device_manager_opt = client_group.get_device_manager(uuid);
        match device_manager_opt {
            Ok(device_manager) => match device_manager {
                Some(manager) => {
                    let serialized_manager = super::serialize_object(&manager);
                    match serialized_manager {
                        Ok(serialized) => Ok(serialized),
                        Err(e) => Err(e.to_string()),
                    }
                }
                None => Err("해당하는 device manager가 없습니다.".to_string()),
            },
            Err(e) => Err(e),
        }
    }
}

mod post {
    use crate::server::device_manager::DeviceManager;

    use super::super::request_handler::ClientGroup;
    use super::Request;
    use regex::Regex;
    use uuid::Uuid;

    pub fn path_router(request: Request, client_group: &mut ClientGroup) -> Result<String, String> {
        let path_device_manager = Regex::new(r"^/device-manager").unwrap();
        let path_device_spec = Regex::new(r"/spec").unwrap();
        let path_device_fs = Regex::new(r"/fs").unwrap();

        let uuid_regex =
            r"\b[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}\b";
        let request_path = request.path;

        if path_device_fs.is_match(request_path) {
            log::debug!("device fs 등록");
            let manager_uuid = super::extract_uuid_from_request(request_path, uuid_regex);

            post_device_fs(client_group, request, manager_uuid)
        } else if path_device_spec.is_match(request_path) {
            log::debug!("device spec 등록");
            let manager_uuid = super::extract_uuid_from_request(request_path, uuid_regex);

            post_device_spec(client_group, request, manager_uuid)
        } else if path_device_manager.is_match(request_path) {
            log::debug!("device manager 등록");

            post_device_manager(client_group)
        } else {
            Err("지원하지 않는 path입니다.".to_string())
        }
    }

    fn post_device_fs(
        client_group: &mut ClientGroup,
        request: Request,
        manager_uuid: Uuid,
    ) -> Result<String, String> {
        let device_manager = client_group.get_device_manager(manager_uuid);
        match device_manager {
            Ok(manager) => match manager {
                Some(m) => {
                    let uuid = Uuid::new_v4();
                    let mut device_fs_str = request.payload;
                    device_fs_str = device_fs_str.trim_matches(|c| c == '\0');
                    let device_fs = super::deserialize_object(device_fs_str);
                    match device_fs {
                        Ok(fs) => {
                            m.add_device_fs(uuid, fs);
                            Ok(uuid.to_string())
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                }
                None => Err("해당하는 device manager가 없습니다.".to_string()),
            },
            Err(e) => Err(e),
        }
    }

    fn post_device_spec(
        client_group: &mut ClientGroup,
        request: Request,
        manager_uuid: Uuid,
    ) -> Result<String, String> {
        let device_manager = client_group.get_device_manager(manager_uuid);
        match device_manager {
            Ok(manager) => match manager {
                Some(m) => {
                    let uuid = Uuid::new_v4();
                    let mut device_spec_str = request.payload;
                    device_spec_str = device_spec_str.trim_matches(|c| c == '\0');
                    let device_spec = super::deserialize_object(device_spec_str);
                    match device_spec {
                        Ok(spec) => {
                            m.add_device_spec(uuid, spec);
                            Ok(uuid.to_string())
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                }
                None => Err("해당하는 device manager가 없습니다.".to_string()),
            },
            Err(e) => Err(e),
        }
    }

    fn post_device_manager(client_group: &mut ClientGroup) -> Result<String, String> {
        let uuid = Uuid::new_v4();
        let device_manager = DeviceManager::new();

        let res = client_group.add_device_manager(uuid, device_manager);
        match res {
            Ok(_) => {
                log::debug!("{:?}", uuid);
                Ok(uuid.to_string())
            }
            Err(e) => Err(e),
        }
    }
}
