use super::device_manager::DeviceManager;

use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};

pub struct Server {
    pub ip: String,
    pub port: String,
    pub devices_spec: HashMap<String, DeviceManager>,
}

impl Server {
    /**
     * 받은 요청을 기반으로 DeviceManager의 정보를 이용해 응답
     */
    pub fn new(ip: String, port: String) -> Self {
        Server {
            ip,
            port,
            devices_spec: HashMap::new(),
        }
    }

    pub fn init(&self) -> TcpListener {
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
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    log::info!("connection 생성 {:?}", stream);
                }
                Err(e) => {
                    log::error!("{}", e);
                }
            }
        }
    }
}
