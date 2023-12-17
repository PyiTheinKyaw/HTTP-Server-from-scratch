use std::io::Read;
use std::net::{SocketAddr, TcpListener, TcpStream};

#[derive(Debug)]
pub struct Server {
    pub socket_addr: SocketAddr,
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
                    if let Ok((mut tcp_stream, socket_addr)) = listen.accept() {
                        // Crate a space to read data from stream
                        // For now, limit is 1kb.
                        // For production ready server we have to be smarter than this.
                        let mut raw_buffer = [0; 1024];

                        if let Ok(_) = tcp_stream.read(&mut raw_buffer) {
                            println!("{}", String::from_utf8_lossy(&raw_buffer));
                        }
                        else {
                            println!("Failed to read from incoming connection");
                        }
                    }
                }
            },
            Err(e) => println!("Error occur: {}", e),
        }
    }
}