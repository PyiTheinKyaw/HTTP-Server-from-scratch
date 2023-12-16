use http_server_from_scratch::server::{Server};

fn main() {
    let server_addr = "127.0.0.1".to_string();
    let server_port = "8080".to_string();

    let server = Server::new(server_addr, server_port);
    server.run();
}