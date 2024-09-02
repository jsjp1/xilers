use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub client: ClientConfig,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub master_ip: String,
    pub master_port: u16,
}
#[derive(Debug, Deserialize)]
pub struct ClientConfig {
    pub file_storage: String,
    pub listen_port: u16,
}
