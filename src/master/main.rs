use server::error_handler::ErrorHandler;

mod server;

#[tokio::main]
async fn main() {
    let _logger_init = server::log::init_logger().unwrap();

    let server_ip: String = String::from("0.0.0.0");
    let server_port: String = String::from("8080");
    let db_ip: String = String::from("127.0.0.1");
    let dp_port: String = String::from("27017");

    ErrorHandler::create_error_log_dir();
    let server = server::server::Server::new(server_ip, server_port, db_ip, dp_port);

    let listener = server.init().await;
    server.run(listener);
}
