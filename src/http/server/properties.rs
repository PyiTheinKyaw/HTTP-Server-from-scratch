use super::super::server::Server;

pub trait ServerProperties {
    const BUFFER_SIZE: usize;
}

impl ServerProperties for Server {
    const BUFFER_SIZE: usize = 1024;

}
