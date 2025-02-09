use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use crate::logger::{Logger, LogLevel};
use crate::request::Request;
use crate::response::Response;
use crate::templates::Templates;

pub struct Server {
    host: String,
    port: u16,
    root_dir: PathBuf,
    templates: Templates,
}

impl Server {
    pub fn new(host: String, port: u16, root_dir: PathBuf) -> Self {
        let http_server = Self {
            host,
            port,
            root_dir,
            templates: Templates::load(),
        };

        return http_server;
    }

    pub fn serve(&self) {
        let listener = TcpListener::bind(self.addr().as_str()).unwrap();

        for stream in listener.incoming() {
            if let Ok(stream) = stream {
                self.handle_request(stream);
            }
        }
    }

    pub fn handle_request(&self, mut stream: TcpStream) {
        if let Some(request) = Request::from_stream(&stream) {
            self.handle_response(request, &mut stream);
        } else {
            Logger::log(LogLevel::WARN, "Failed to read request.")
        }
    }

    pub fn handle_response(&self, request: Request, stream: &mut TcpStream) {
        if let Some(mut response) = Response::new(request) {
            response.serve(&self.root_dir);
            let _ = stream.write_all(response.to_string().as_bytes());
            Self::log_response(&response);
        } else {
            Logger::log(LogLevel::WARN, "Failed to send response.")
        }
    }

    pub fn addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub fn addr_with_protocol(&self) -> String {
        format!("http://{}", self.addr())
    }

    pub fn log_response(response: &Response) {
        let status_line = response.request.to_string().lines().next().unwrap().to_string();
        let log_message = &format!(
            "\"{}\" {} {}",
            status_line,
            response.status_code.to_code(),
            response.body.len(),
        );
        Logger::log(LogLevel::INFO, log_message);
    }
}
