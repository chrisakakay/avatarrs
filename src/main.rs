use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

use chrono::Local;
use colored::*;
use enigo::*;

mod response;

fn log(msg: String) {
    let now = Local::now();
    println!("[{}] {}", now.format("%H:%M:%S%.3f"), msg);
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    log(format!("{} {} {}", "I".green(), "Server started at:", "127.0.0.1:7878".green()));
    for stream in listener.incoming() {
        handle_connection(stream.unwrap());
    }
}

fn get_key_sequence(temp: String) -> String {
    let v: Vec<&str> = temp.lines().collect();
    let c = v[16].to_string(); // BODY OF THE REQUEST
    let c2: Vec<&str> = c.split("\"").collect(); // SPLIT HACK

    return c2[3].to_string();
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let temp = String::from_utf8_lossy(&buffer[..]);
    let v: Vec<&str> = temp.lines().collect();

    let route_index = b"GET / HTTP/1.1\r\n";
    let js_index = b"GET /index.js HTTP/1.1\r\n";
    let keys_index = b"POST /keys HTTP/1.1\r\n";

    if buffer.starts_with(route_index) {
        log(format!("{} {}", "I".green(), v[0]));
        response::send(stream, response::CODE::C200, response::from_file("webcontent/index.html"));
    } else if buffer.starts_with(js_index) {
        log(format!("{} {}", "I".green(), v[0]));
        response::send(stream, response::CODE::C200, response::from_file("webcontent/index.js"));
    } else if buffer.starts_with(keys_index) {
        let key_sequence: String = get_key_sequence(String::from_utf8_lossy(&buffer[..]).to_string());
        if !key_sequence.is_empty() {
            let mut enigo = Enigo::new();
            log(format!("{} {}", "I".green(), key_sequence.yellow()));
            enigo.key_sequence_parse(&key_sequence);
        }
        response::send(stream, response::CODE::C200, "{}".to_string());
    } else {
        log(format!("{} {}", "E".red(), v[0]));
        response::send(stream, response::CODE::C404, response::from_file("webcontent/404.html"));
    };
}
