use actix_web::rt;
use std::{sync::mpsc, thread};

mod server;
use server::error_handler::{ErrorHandler, ErrorType, NotAbortError};
use server::server::Server;

fn main() {
    let _logger_init = server::log::init_logger().unwrap();
    ErrorHandler::create_error_log_dir();

    let server = Server::new(
        "127.0.0.1".to_string(),
        8080,
        "127.0.0.1".to_string(),
        27017,
    );

    loop {
        let (tx, rx) = mpsc::channel();
        let server_clone = server.clone();

        let t = thread::spawn(move || {
            let server_future = server_clone.init_and_run(tx);
            rt::System::new().block_on(server_future)
        });

        let server_handle = rx.recv().unwrap();

        match t.join().unwrap() {
            Ok(_) => {
                log::info!("서버가 정상적으로 종료되었습니다.");
                break;
            }
            Err(e) => {
                let error_type = ErrorType::NotAbortError(NotAbortError::Severe(format!(
                    "서버가 비정상적으로 종료되었습니다. {}",
                    e
                )));
                ErrorHandler::process_error(error_type);

                log::info!("서버를 재시작합니다.");
                continue;
            }
        }
    }
}
