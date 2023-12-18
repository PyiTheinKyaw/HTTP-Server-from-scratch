use http_server_from_scratch::OUServer;
#[test]
fn test_server_info() {

    let server = OUServer::new("127.0.0.1".to_string(), 8080);

    assert_eq!("127.0.0.1:8080".parse(), Ok(server.socket_addr));
    assert_eq!(server.socket_addr.port(), 8080);
    assert_eq!(server.socket_addr.is_ipv4(), true);
}