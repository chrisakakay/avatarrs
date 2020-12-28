use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

mod logger;
mod response;
mod simulator;
mod router;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:80").unwrap();
    logger::info(format!("{} {}", "Server started at:", "0.0.0.0:80"));
    for stream in listener.incoming() {
        handle_connection(stream.unwrap());
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let temp = String::from_utf8_lossy(&buffer[..]);
    let v: Vec<&str> = temp.lines().collect();
    let http_call = v[0];

    if http_call.starts_with(&router::get("/".to_string())) {
        response::ok(stream, response::from_file("webcontent/index.html"), http_call.to_string());
    } else if http_call.starts_with(&router::get("/index.js".to_string())) {
        response::ok(stream, response::from_file("webcontent/index.js"), http_call.to_string());
    } else if http_call.starts_with(&router::post("/keys".to_string())) {
        simulator::run(String::from_utf8_lossy(&buffer[..]).to_string());
        response::ok(stream, "{}".to_string(), http_call.to_string());
    } else {
        response::error(stream, response::from_file("webcontent/404.html"), http_call.to_string());
    };
}
