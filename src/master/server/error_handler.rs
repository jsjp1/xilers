use chrono::Local;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process;

static ERROR_LOG_DIR: &'static str = "/tmp/xilers/error_log";

pub enum ErrorType {
    AbortError(String),
    NotAbortError(NotAbortError),
}

pub enum NotAbortError {
    Minor(String),  // 로그를 남기지 않음
    Severe(String), // 로그를 남겨야되는 오류
}

pub struct ErrorHandler;

impl ErrorHandler {
    // request handler만으로부터 에러를 받아 처리하는 class
    pub fn process_error(e_type: ErrorType) {
        match e_type {
            ErrorType::AbortError(e) => {
                log::error!("AbortError: {}, 프로그램을 종료합니다.", e);
                ErrorHandler::save_error_log(e);
                process::exit(0);
            }
            ErrorType::NotAbortError(e) => match e {
                NotAbortError::Minor(l) => {
                    log::error!("NotAbortError: {}, 프로그램을 종료하지 않습니다.", l);
                }
                NotAbortError::Severe(l) => {
                    log::error!(
                        "NotAbortError: {}, 프로그램을 종료하지 않습니다. 로그를 납깁니다.",
                        l
                    );
                    ErrorHandler::save_error_log(l);
                }
            },
        }
    }

    pub fn create_error_log_dir() {
        let is_created = std::fs::create_dir_all(ERROR_LOG_DIR);
        match is_created {
            Ok(()) => (),
            Err(e) => {
                log::warn!("이미 존재하는 디렉토리입니다. {}", e);
                ()
            }
        }
    }

    fn create_error_log_file() -> File {
        let now = Local::now();
        let formatted_date = now.format("%Y-%m-%d").to_string();

        let file_name = format!("{}_error.txt", formatted_date);
        let path_file_concat = format!("{}/{}", ERROR_LOG_DIR, file_name);

        let _path = Path::new(&path_file_concat);

        if _path.exists() {
            let file = OpenOptions::new()
                .write(true)
                .append(true) // 이어쓰기
                .open(_path)
                .unwrap();
            file
        } else {
            let file = File::create(_path).unwrap();
            file
        }
    }

    fn save_error_log(_e: String) {
        let mut error_log_file = ErrorHandler::create_error_log_file(); // TODO: file io -> overhead 큼 -> 항상 체크하는 것보다 cache 활용하는 방식으로 변경
        match writeln!(error_log_file, "{}", _e) {
            Ok(_) => {
                log::info!("error log를 저장했습니다.");
            }
            Err(e) => {
                log::error!("{}", e);
            }
        }
    }
}
