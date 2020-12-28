
pub fn get(path: String) -> String {
    return format!("GET {} HTTP/1.1", path);
}

pub fn post(path: String) -> String {
    return format!("POST {} HTTP/1.1", path);
}
