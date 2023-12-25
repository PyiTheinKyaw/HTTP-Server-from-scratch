mod properties;

use std::io::{Read};
use std::net::{SocketAddr, TcpListener};
use std::convert::TryFrom;

use super::request::Request;
use properties::ServerProperties;
use crate::http::http_handler::HTTPHandler;

#[derive(Debug)]
pub struct Server {
    pub socket_addr: SocketAddr
}

impl Server {

    pub fn new(addr: String, port: u16) -> Self {
        Server {
            socket_addr: SocketAddr::new(addr.parse().unwrap(), port)
        }
    }

    /*
    This function will listen on given address and port.
     */
    pub fn run(self, handler: impl HTTPHandler) {
        // let listen = TcpListener::bind(self.get_address()).unwrap();
        match TcpListener::bind(self.socket_addr) {
            Ok(listen) => {
                println!("Server is listening on http://{}", &self.socket_addr);
                println!("By default: / and /index will take you index.html");

                loop {
                    if let Ok((mut tcp_stream, _)) = listen.accept() {

                        // Crate a space to read data from stream
                        // For now, limit is 1KB.
                        // For production ready server we have to be smarter than this.
                        let mut raw_buffer = [0; Server::BUFFER_SIZE];

                        match tcp_stream.read(&mut raw_buffer) {

                            Ok(_) => {
                                // let result: Result<Request, String> = buffer.try_into();
                                let response = match Request::try_from(&raw_buffer[..]) {
                                    Ok(request) => handler.handle_request(request),
                                    Err(e) => handler.handle_bad_request(&e),
                                };

                                if let Err(e) = response.send(&mut tcp_stream) {
                                    println!("Error while parsing raw_bytes to Request. Reason: {}", e)
                                }
                            },
                            Err(e) => println!("Failed to read from incoming stream. Reason: {}", e)
                        }
                    }
                }
            },
            Err(e) => println!("Error occur: {}", e),
        }
    }
}