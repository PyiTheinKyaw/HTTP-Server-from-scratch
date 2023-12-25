use std::fmt::{Display, Formatter, Result as FmtResult};
use super::response::status_code::StatusCode;

mod status_code;

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Self { status_code, body}
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        todo!()
    }
}