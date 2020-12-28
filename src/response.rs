use std::io::prelude::*;
use std::net::TcpStream;
use std::fs;

use super::logger;

pub fn from_file(filename: &str) -> String {
    fs::read_to_string(filename).unwrap()
}

pub fn ok(stream: TcpStream, contents: String, path: String) {
    let response = format!("{}{}", String::from("HTTP/1.1 200 OK\r\n\r\n"), contents);
    logger::info(path);
    send_response(stream, response);
}

pub fn error(stream: TcpStream, contents: String, path: String) {
    let response = format!("{}{}", String::from("HTTP/1.1 404 NOT FOUND\r\n\r\n"), contents);
    logger::error(path);
    send_response(stream, response);
}

pub fn send_response(mut stream: TcpStream, response: String) {
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
