use log::SetLoggerError;

mod server;

static LOGGER: server::log::Logger = server::log::Logger;

fn init_logger() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(log::LevelFilter::Debug))
}

fn main() {
    let _logger_init = init_logger().unwrap();

    let server_ip: String = String::from("0.0.0.0");
    let server_port: String = String::from("8080");
    let server = server::server::Server::new(server_ip, server_port);

    let listener = server.init();
    server.run(listener);
}
