use std::io::{self, Read};

pub fn read_key() -> Result<char, std::io::Error> {
    let mut buffer = [0,1];
    let _ = io::stdin().read(&mut buffer);
    Ok(buffer[0] as char)
}




