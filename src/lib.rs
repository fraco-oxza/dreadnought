use std::collections::HashMap;
use std::fmt;
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::process::exit;

use ansi_term::{Colour, Style};
use lazy_static::lazy_static;
use log::{debug, error, info};

pub mod cli;
pub mod pool;
pub mod watch;

lazy_static! {
    pub static ref ERROR_STYLE: Style = Style::new().fg(Colour::Red).bold();
    pub static ref REMARK_STYLE: Style = Style::new().fg(Colour::Cyan).bold();
}

pub fn close_with_error<T>(e: T, code: i32)
where
    T: std::error::Error,
{
    eprintln!("{} {}", ERROR_STYLE.paint("Error:"), e);
    exit(code);
}

#[derive(Debug)]
pub enum Method {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
    UNKNOWN(String),
}

#[derive(Debug)]
pub struct HttpPetition {
    method: Method,
    endpoint: String,
    http_version: String, //TODO: Implement more http versions
    header: HashMap<String, String>,
    body: String,
    stream: TcpStream,
}

impl HttpPetition {
    pub fn from_conn(mut stream: TcpStream) -> Result<Self, std::io::Error> {
        let mut buffer = [0_u8; 1];
        let mut header: Vec<u8> = Vec::new();
        loop {
            stream.read(&mut buffer)?;
            header.append(&mut buffer.to_vec());
            if header.ends_with(b"\r\n\r\n") {
                break;
            }
        }
        let header = String::from_utf8_lossy(&header);
        let mut header = header.trim_end().split("\r\n");

        let first_line: Vec<&str> = header.next().unwrap().split(" ").collect();
        if first_line.len() != 3 {
            unimplemented!();
        }

        let method = match first_line[0] {
            "GET" => Method::GET,
            "POST" => Method::POST,
            "HEAD" => Method::HEAD,
            "PUT" => Method::PUT,
            "DELETE" => Method::DELETE,
            "CONNECT" => Method::CONNECT,
            "OPTIONS" => Method::OPTIONS,
            "TRACE" => Method::TRACE,
            "PATCH" => Method::PATCH,
            unknown => Method::UNKNOWN(unknown.to_string()),
        };

        let endpoint = first_line[1].to_string();
        let http_version = first_line[2].to_string();

        let mut header_map = HashMap::new();
        for line in header {
            let line_splited = line.split(": ").collect::<Vec<&str>>();
            header_map.insert(line_splited[0].to_string(), line_splited[1].to_string());
        }

        let mut body = String::new();
        if header_map.contains_key("Content-Length") {
            let mut content: Vec<u8> = Vec::new();
            let length: u64 = header_map.get("Content-Length").unwrap().parse().unwrap();
            const BUFFER_SIZE: usize = 512;
            let mut buffer = [0_u8; BUFFER_SIZE];
            let mut i = 0;
            while length > (BUFFER_SIZE as u64 * i) {
                stream.read(&mut buffer).unwrap();
                content.append(&mut buffer.to_vec());
                i += 1;
            }
            body = String::from_utf8_lossy(&content)
                .replace("\u{0}", "")
                .to_string();
        }

        Ok(Self {
            method,
            endpoint,
            http_version,
            header: header_map,
            body,
            stream,
        })
    }
    pub fn response(mut self, content: &[u8]) -> Result<(), std::io::Error> {
        self.stream.write(content)?;
        Ok(())
    }
    pub fn get_peer_addr(&self) -> io::Result<std::net::SocketAddr> {
        self.stream.peer_addr()
    }
}

impl fmt::Display for HttpPetition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "  {:#?} {} {}",
            self.method, self.endpoint, self.http_version
        )?;
        for key in self.header.keys() {
            write!(
                f,
                "\n    {} {}",
                REMARK_STYLE.paint(format!("{}:", key)),
                self.header.get(key).unwrap()
            )?;
        }
        Ok(())
    }
}
