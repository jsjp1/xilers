use super::request_handler::ClientGroup;

pub struct Request<'a> {
    pub method: &'a str,
    pub path: &'a str,
    pub payload: &'a str,
}

pub fn method_router(request: Request, client_group: &ClientGroup) {
    // TODO: return값 혹은 error 반환하도록
    match request.method {
        "GET" => {
            get::path_router(request, client_group);
        }
        "POST" => {
            post::path_router(request, client_group);
        }
        _ => {
            log::warn!("유효하지 않은 요청입니다.");
        }
    }
}

pub mod api_path {
    pub static ROOT: &'static str = "/api";

    pub mod get {
        pub const DEVICE_GROUP: &'static str = "/device-group";
        pub const DEVICE_SPEC: &'static str = "/device-spec";
        pub const DEVICE_FS: &'static str = "/device-fs";
    }

    pub mod post {}
}

mod get {
    use super::super::request_handler::ClientGroup;
    use super::api_path;
    use super::Request;

    pub fn path_router(request: Request, client_group: &ClientGroup) {
        // 이런식으로 하면 추후 확장 어려울 것 같음
        let is_group_request = request.path.contains(api_path::get::DEVICE_GROUP);
        let is_spec_request = request.path.contains(api_path::get::DEVICE_SPEC);
        let is_fs_request = request.path.contains(api_path::get::DEVICE_FS);

        if is_group_request & !is_spec_request & !is_fs_request {
        } else if is_group_request & is_spec_request & !is_fs_request {
        } else if is_group_request & is_spec_request & is_fs_request {
        } else {
            // 잘못된 api 요청
        }
    }

    fn get_client_group(path: &str, client_group: &ClientGroup) {}

    fn get_device_spec(path: &str, client_group: &ClientGroup) {}
}

mod post {
    use super::super::request_handler::ClientGroup;
    use super::Request;

    pub fn path_router(request: Request, client_group: &ClientGroup) {}
}
