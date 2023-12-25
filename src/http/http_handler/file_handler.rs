use crate::http::error::ParseError;
use crate::http::methods::Methods;
use super::super::http_handler::HTTPHandler;
use super::super::request::Request;
use super::super::response::Response;
use super::super::response::status_code::StatusCode;

use std::fs;

pub struct FileHandler {
    file_path: String
}

impl FileHandler {
    pub fn new(file_path: String) -> Self {
        println!("Your HTML file have to place under {}", file_path);
        Self {file_path}
    }

    fn read_file(&self, file_name: &str) -> Option<String>
    {
        let path = format!("{}/{}", self.file_path, file_name);

        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.file_path) {
                    fs::read_to_string(path).ok()
                }
                else {
                    println!("DTA(Directory Traversal Attack): {}", file_name);
                    None
                }
            },
            Err(_) => None
        }
    }
}

impl HTTPHandler for FileHandler{
    fn handle_request(&self, req: Request) -> Response {
        match req.method() {
            Methods::GET => match req.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/index" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                path => match self.read_file(path) {
                    Some(c) => Response::new(StatusCode::Ok, Some(c)),
                    None => Response::new(StatusCode::NotFound, None),
                }
            },
            _ => Response::new(StatusCode::NotFound, None)
        }
    }

    fn handle_bad_request(&self, e: &ParseError) -> Response {
        println!("Failed to server request. Reason: {}", e);
        Response::new(
            StatusCode::BadRequest,
            None
        )
    }
}
