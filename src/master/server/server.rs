use super::device_manager::DeviceManager;

use std::collections::HashMap;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

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

        log::info!("Master now running on {}:{}", self.ip, self.port);

        listener
    }

    pub fn run(self, listener: TcpListener) {
        let mut thread_handlers = vec![];
        let server = Arc::new(Mutex::new(self));

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let server = Arc::clone(&server);

                    let handle = thread::spawn(move || {
                        let server = server.lock().unwrap();
                        server.handle_connection(stream);
                    });

                    thread_handlers.push(handle);
                }
                Err(e) => {
                    log::warn!("{}", e);
                }
            }
        }

        for handle in thread_handlers {
            handle.join().unwrap(); // panic
        }
    }

    fn handle_connection(&self, mut stream: TcpStream) {
        let _current_thread_id = thread::current().id();
        log::debug!("Handled by thread: {:?}", _current_thread_id);

        let mut buffer = [0; 1024];
        let bytes_read = stream.read(&mut buffer);
    }
}
