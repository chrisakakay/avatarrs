use std::io::prelude::*;
use std::net::TcpStream;
use std::fs;

pub enum CODE {
    C200,
    C404
}

pub fn to_text(code: CODE) -> String {
    match code {
        CODE::C200 => String::from("HTTP/1.1 200 OK\r\n\r\n"),
        CODE::C404 => String::from("HTTP/1.1 404 NOT FOUND\r\n\r\n"),
    }
}

pub fn send(mut stream: TcpStream, status: CODE, contents: String) {
    let response = format!("{}{}", to_text(status), contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

pub fn from_file(filename: &str) -> String {
    fs::read_to_string(filename).unwrap()
}
