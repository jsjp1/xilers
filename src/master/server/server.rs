use super::request_handler;

use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Server {
    pub ip: String,
    pub port: String,
    pub handler: Arc<Mutex<request_handler::Handler>>, // 멀티 스레드 -> Arc Mutex 스마트 포인터 이용
}

impl Server {
    /**
     * 받은 요청을 기반으로 DeviceManager의 정보를 이용해 응답
     */
    pub fn new(ip: String, port: String, db_ip: String, db_port: String) -> Self {
        Server {
            ip,
            port,
            handler: Arc::new(Mutex::new(request_handler::Handler::new(db_ip, db_port))),
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
                    let handler = Arc::clone(&self.handler); // Arc clone 이용해 참조 카운터 증가
                    let _thread = thread::spawn(move || {
                        let handler = handler.lock().unwrap();

                        let mut stream = stream;
                        handler.handle_connection(&mut stream);
                    });

                    thread_handler.push(_thread);
                }
                Err(e) => {
                    log::error!("{}", e);
                }
            }
        }

        for handle in thread_handler {
            handle.join().unwrap(); // 데드락 -> panic, 완료성 보장
        }
    }
}
