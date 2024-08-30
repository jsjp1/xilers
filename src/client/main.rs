use signal_hook::consts::SIGINT;
use std::env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

mod network;
mod ui;
use network::tcp::network::TcpNetwork;
use ui::cli::interface::Cli;
use ui::interface::Interface;

#[tokio::main]
async fn main() {
    // let master_ip = "http://xilers.jieeen.kr";
    let master_ip = "http://127.0.0.1";
    let master_port = 8080;
    let listen_port = 8081;
    let file_storage = "/Users/jin/Desktop/test";

    let master_addr = format!("{}:{}", master_ip, master_port);

    // let network: TCPNetwork = NetworkInterface::new(8082);
    // let t = network.listener_init(); // 다른 스레드에서 처리
    // let a = network.clone();
    // let thread = std::thread::spawn(move || {
    //     a.listen(t);
    // });
    // let mut stream = network.connect("127.0.0.1:8082".to_string());
    // network.send_request(
    //     &mut stream,
    //     "/Users/jin/Desktop/prj/app/analysis_options.yaml".to_string(),
    // );

    if env::args().len() == 1 {
        println!("usage: ./client [cli | gui]");
        return;
    }

    // TODO: ui 옵션 파싱 방식 변경
    if env::args().nth(1).unwrap() == "cli" {
        println!("CLI mode로 시작합니다.");
        let mut cli: Cli = Interface::new(master_addr, listen_port, file_storage.to_string());
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
