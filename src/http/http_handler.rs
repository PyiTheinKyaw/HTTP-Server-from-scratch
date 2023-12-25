use crate::http::request::Request;
use crate::http::response::Response;

pub mod file_handler;

pub trait HTTPHandler<'a> {
    const SERVER_NAME: &'a str;
    const PROTOCOL_VERSION: &'a str;
    fn handle_request(&self, req: Request) -> Response;

    fn handle_bad_request(&self) -> Response;
}
