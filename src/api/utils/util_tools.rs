use chrono::Utc;
use std::io::{self, Write};

pub fn now() -> String {
    Utc::now().format("%H:%M %d-%m-%Y").to_string()
}


pub fn input(querry: &str) -> String {
    let mut input_text = String::new();
    print!("{}", querry);
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut input_text).expect("Error with input function");
    input_text.trim().to_string()
}
