#![allow(dead_code)]
use http_server_from_scratch::OUServer;
use http_server_from_scratch::FileHandler;

use std::env;

fn main() {
    let server_addr = "127.0.0.1".to_string();
    let server_port = 8080;

    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));

    let server = OUServer::new(server_addr, server_port);
    let file_handler = FileHandler::new(
        env::var("PUBLIC_PATH").unwrap_or(default_path)
    );
    server.run(file_handler);
}
