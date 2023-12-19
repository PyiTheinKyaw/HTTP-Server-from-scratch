mod properties;

use std::io::Read;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::convert::TryFrom;

use super::request::Request;
use properties::ServerProperties;

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
    pub fn run(self) {
        // let listen = TcpListener::bind(self.get_address()).unwrap();
        match TcpListener::bind(self.socket_addr) {
            Ok(listen) => {
                println!("Server is listening on {}", &self.socket_addr);
                loop {
                    if let Ok((mut tcp_stream, _)) = listen.accept() {

                        // Crate a space to read data from stream
                        // For now, limit is 1KB.
                        // For production ready server we have to be smarter than this.
                        let mut raw_buffer = [0; Server::BUFFER_SIZE];

                        self.handle_request(&mut raw_buffer, &mut tcp_stream);
                    }
                }
            },
            Err(e) => println!("Error occur: {}", e),
        }
    }

    fn handle_request(&self, buffer: &mut [u8], tcp_stream: &mut TcpStream) {
        if let Ok(_) = tcp_stream.read(buffer) {
            println!("{}", String::from_utf8_lossy(buffer));

            // let result: Result<Request, String> = buffer.try_into();
            match Request::try_from(buffer as &[u8]) {
                Ok(request) => {
                    println!("{:?}", request);
                },
                Err(e) => println!("Error while parsing raw_bytes to Request. Reason {}", e),
            }
        }
        else {
            println!("Failed to read from incoming connection");
        }
    }
}