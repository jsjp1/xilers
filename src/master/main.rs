mod server;

fn main() {
    let server = server::server::Server::new("192.168.0.0.1", "11111");
    server.print();
}
