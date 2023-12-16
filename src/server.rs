#[derive(Debug)]
pub struct Server {
    pub addr: String,
    pub port: String,
}

impl Server {
    pub fn new(addr: String, port: String) -> Self {
        Server { addr, port }
    }

    pub fn run(self) {
        println!("Server is running at : {}", self.get_address());
    }

    fn get_address(&self) -> String {
        format!("{}:{}", self.addr, self.port)
    }
}