use super::super::http_handler::HTTPHandler;
use super::super::request::Request;
use super::super::response::Response;
use super::super::response::status_code::StatusCode;

pub struct FileHandler {}

impl<'fh> HTTPHandler<'fh> for FileHandler{
    const SERVER_NAME: &'fh str = "OuOu";
    const PROTOCOL_VERSION: &'fh str = "HTTP/1.1";
    fn handle_request(&self, req: Request) -> Response {
        Response::new(
            StatusCode::Ok,
            &Self::SERVER_NAME,
            &Self::PROTOCOL_VERSION,
            Some("<h1>TEST</h1>".to_string()),
        )
    }

    fn handle_bad_request(&self) -> Response {
        Response::new(
            StatusCode::BadRequest,
            &Self::SERVER_NAME,
            &Self::PROTOCOL_VERSION,
            None
        )
    }
}