use crate::http::error::ParseError;
use crate::http::request::Request;
use crate::http::response::Response;

pub mod file_handler;

pub trait HTTPHandler {
    fn handle_request(&self, req: Request) -> Response;

    fn handle_bad_request(&self, e: &ParseError) -> Response;
}
