use super::super::server::Server;

pub trait ServerProperties {
    const BUFFER_SIZE: usize;
    const SERVER: &'static str;
}

impl ServerProperties for Server {
    const BUFFER_SIZE: usize = 1024;
    const SERVER: &'static str = "OuOu";

}
