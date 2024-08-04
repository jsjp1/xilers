use super::handler;

use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Server {
    pub ip: String,
    pub port: String,
    pub handler: Arc<Mutex<handler::Handler>>,
}

impl Server {
    /**
     * 받은 요청을 기반으로 DeviceManager의 정보를 이용해 응답
     */
    pub fn new(ip: String, port: String, db_ip: String, db_port: String) -> Self {
        Server {
            ip,
            port,
            handler: Arc::new(Mutex::new(handler::Handler::new(db_ip, db_port))),
        }
    }

    pub async fn init(&self) -> TcpListener {
        log::info!("Handler초기화를 진행합니다.");
        self.handler.lock().unwrap().init().await;

        let _ip_and_port = format!("{}:{}", self.ip, self.port);

        let _listener = TcpListener::bind(&_ip_and_port);
        let listener = match _listener {
            Ok(listener) => listener,
            Err(e) => {
                log::error!("{}", e);
                panic!();
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
                    let handler = Arc::clone(&self.handler);
                    let _thread = thread::spawn(move || {
                        let handler = handler.lock().unwrap();
                        handler.handle_connection(stream);
                    });

                    thread_handler.push(_thread);
                }
                Err(e) => {
                    log::error!("{}", e);
                }
            }
        }

        for handle in thread_handler {
            handle.join().unwrap(); // 데드락 -> panic
        }
    }
}
