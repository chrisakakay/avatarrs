use chrono::Local;
use colored::*;

fn print_log(print_type: ColoredString, msg: String) {
    println!("[{}] {} {}", Local::now().format("%H:%M:%S%.3f"), print_type, msg);
}

pub fn info(msg: String) {
    print_log("I".green(), msg);
}

pub fn error(msg: String) {
    print_log("E".red(), msg);
}

pub fn warn(msg: String) {
    print_log("W".yellow(), msg);
}
