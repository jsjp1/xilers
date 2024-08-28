use signal_hook::consts::SIGINT;
use std::env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

mod ui;
use ui::cli::interface::Cli;
use ui::interface::Interface;

#[tokio::main]
async fn main() {
    let master_ip = "http://xilers.jieeen.kr";
    let master_port = 8080;

    let master_addr = format!("{}:{}", master_ip, master_port);

    if env::args().len() == 1 {
        println!("usage: ./client [cli | gui]");
        return;
    }

    // TODO: ui 옵션 파싱 방식 변경
    if env::args().nth(1).unwrap() == "cli" {
        println!("CLI mode로 시작합니다.");
        let mut cli: Cli = Interface::new(master_addr);
        cli.entry().await;
    } else if env::args().nth(1).unwrap() == "gui" {
        println!("GUI mode로 시작합니다.");
        // let gui = Gui::new(master_addr);
        // gui.entry();
    } else {
        println!("usage: ./client [cli | gui]");
        return;
    }
}
