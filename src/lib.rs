mod http;
pub use http::server::Server;

#[cfg(test)]
mod tests {
    use crate::Server;

    #[test]
    fn test_server_info() {

        let server = Server::new("127.0.0.1".to_string(), 8080);

        assert_eq!("127.0.0.1:8080".parse(), Ok(server.socket_addr));
        assert_eq!(server.socket_addr.port(), 8080);
        assert_eq!(server.socket_addr.is_ipv4(), true);
    }
}