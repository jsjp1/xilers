use super::request_handler;
use crate::server::error_handler::NotAbortError;

use super::error_handler::{ErrorHandler, ErrorType};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Server {
    pub ip: String,
    pub port: u16,
    pub handler: Arc<Mutex<request_handler::RequestHandler>>, // 멀티 스레드 -> Arc Mutex 스마트 포인터 이용
}

impl Server {
    // 받은 요청을 기반으로 DeviceManager의 정보를 이용해 응답
    pub fn new(ip: String, port: u16, db_ip: String, db_port: String) -> Self {
        Server {
            ip,
            port,
            handler: Arc::new(Mutex::new(request_handler::RequestHandler::new(
                db_ip, db_port,
            ))),
        }
    }

    pub async fn init(&self) -> TcpListener {
        log::info!("Handler초기화를 진행합니다.");
        let handler_init_res = self.handler.lock().unwrap().init().await;
        match handler_init_res {
            Ok(_) => {}
            Err(error_type) => ErrorHandler::process_error(error_type),
        }

        let _ip_and_port = format!("{}:{}", self.ip, self.port);

        let _listener = TcpListener::bind(&_ip_and_port);
        let listener = match _listener {
            Ok(listener) => listener,
            Err(e) => {
                let abort_error = ErrorType::AbortError(e.to_string());
                ErrorHandler::process_error(abort_error);
                panic!() // 문맥상 없어도됨, 문법상 필요
            }
        };

        log::info!("TCP connection 대기중 {}:{}", self.ip, self.port);

        listener
    }

    pub fn run(&self, listener: TcpListener) {
        let mut thread_handler = vec![];
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    log::info!("connection 생성 {:?}", stream);
                    // TODO: thread per request에서 pool 혹은 nonblocking io로 변경
                    let handler = Arc::clone(&self.handler); // Arc clone 이용해 참조 카운터 증가
                    let _thread = thread::spawn(move || {
                        let mut handler = handler.lock().unwrap();

                        let mut stream = stream;
                        match handler.handle_connection(&mut stream) {
                            Ok(_) => {}
                            Err(error_type) => ErrorHandler::process_error(error_type),
                        }
                    });

                    thread_handler.push(_thread);
                }
                Err(e) => {
                    let abort_error = ErrorType::AbortError(e.to_string());
                    ErrorHandler::process_error(abort_error)
                }
            }
        }

        for handle in thread_handler {
            match handle.join() {
                Ok(_) => {}
                Err(e) => {
                    let _abort_type = NotAbortError::Severe(format!("{:?}", e));
                    let not_abort_error = ErrorType::NotAbortError(_abort_type);
                    ErrorHandler::process_error(not_abort_error)
                }
            }
        }
    }
}
