use std::env;

mod config;
mod network;
mod ui;

use config::{ClientConfig, Config, ServerConfig};
use ui::cli::interface::Cli;
use ui::gui::interface::Gui;
use ui::interface::Interface;

#[tokio::main]
async fn main() {
    let config_content: Config = match std::fs::read_to_string("config.toml") {
        Ok(config_str) => toml::from_str(&config_str).expect(
            "config.toml파일을 파싱하는데 문제가 발생했습니다. 파일 내용을 확인하시기 바랍니다.",
        ),
        Err(e) => {
            println!(
                "파일이 존재하지 않습니다. 기본 설정을 적용합니다: {}",
                e.to_string()
            );
            Config {
                server: ServerConfig {
                    master_ip: String::from("http://127.0.0.1"),
                    master_port: 8080,
                },
                client: ClientConfig {
                    // TODO: os별 다른 기본 file_storage
                    file_storage: String::from("/tmp"),
                    listen_port: 8081,
                },
            }
        }
    };
    let master_addr = format!(
        "{}:{}",
        config_content.server.master_ip, config_content.server.master_port
    );

    if env::args().len() == 1 {
        println!("usage: ./client [cli | gui]");
        return;
    }

    // TODO: ui 옵션 파싱 방식 변경
    if env::args().nth(1).unwrap() == "cli" {
        println!("CLI mode로 시작합니다.");
        let mut cli: Cli = Interface::new(
            master_addr,
            config_content.client.listen_port,
            config_content.client.file_storage.to_string(),
        );
        cli.entry().await;
    } else if env::args().nth(1).unwrap() == "gui" {
        println!("GUI mode로 시작합니다.");
        let mut gui = Gui::new(
            master_addr,
            config_content.client.listen_port,
            config_content.client.file_storage.to_string(),
        );
        gui.entry().await;
    } else {
        println!("usage: ./client [cli | gui]");
        return;
    }
}
