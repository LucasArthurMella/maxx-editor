use std::process::Command;

pub fn enter_raw_mode(){
    Command::new("stty")
        .arg("-echo")
        .arg("raw")
        .status()
        .unwrap();
}

pub fn leave_raw_mode(){
    Command::new("stty")
        .arg("echo")
        .arg("-raw")
        .status()
        .unwrap();
}

pub fn clear_screen(){
    print!("{esc}c", esc = 27 as char);
}
