pub struct Request<'a> {
    pub method: &'a str,
    pub path: &'a str,
    pub payload: &'a str,
}

pub fn method_router(request: Request) {
    match request.method {
        "GET" => {
            get::path_router(request);
        }
        "POST" => {
            post::path_router(request);
        }
        _ => {
            log::warn!("유효하지 않은 요청입니다.");
        }
    }
}

mod get {
    use super::Request;

    pub fn path_router(request: Request) {}
}

mod post {
    use super::Request;

    pub fn path_router(request: Request) {}
}
