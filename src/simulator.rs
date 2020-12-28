use enigo::*;

use super::logger;

fn get_key_sequence(temp: String) -> String {
    let v: Vec<&str> = temp.lines().collect();
    let c = v[15].to_string(); // BODY OF THE REQUEST
    let c2: Vec<&str> = c.split("\"").collect(); // SPLIT HACK

    return c2[3].to_string();
}

pub fn run(temp: String) {
    let key_sequence: String = get_key_sequence(temp.to_string());

    if !key_sequence.is_empty() {
        let mut enigo = Enigo::new();
        enigo.key_sequence_parse(&key_sequence);
        logger::warn(key_sequence.to_string());
    } else {
        logger::warn( "--- EMPTY COMMAND ---".to_string());
    }
}

