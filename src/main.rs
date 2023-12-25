use http_server_from_scratch::OUServer;
use http_server_from_scratch::FileHandler;

fn main() {
    let server_addr = "127.0.0.1".to_string();
    let server_port = 8080;

    let server = OUServer::new(server_addr, server_port);
    let file_handler = FileHandler {};
    server.run(file_handler);
}
