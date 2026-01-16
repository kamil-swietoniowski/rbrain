use std::io::Write;

use crate::database::{Record, Database};
use crate::argument_parser::Args;
use clap::Parser;

pub struct App {
    arguments: Args,
    database: Database,
    current_record: Option<Record>,
    loaded_records: Option<Vec<Record>>
}

impl App {
    pub fn new() -> Self {
        let path = "rbrain.db";
        let arguments = Args::parse();
        let database = Database::new(path);
        let current_record = None;
        let loaded_records = None;
        Self {
            arguments, database, current_record, loaded_records
        }
    }
    pub fn command_line() -> rusqlite::Result<()> {
        loop {
            let mut buffer = String::new();
            print!("> ");
            std::io::stdout().flush().unwrap();
            
            std::io::stdin().read_line(&mut buffer).expect("Error reading line");

            // 1. Dzielimy na słowa i tworzymy wektor Stringów (własnych danych, nie referencji)
            let commands: Vec<String> = buffer
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();

            // 2. Sprawdzamy, czy użytkownik cokolwiek wpisał, aby uniknąć błędu indeksowania
            if commands.is_empty() {
                continue;
            }

            // 3. Dopasowujemy pierwszy element (jako &str)
            match commands[0].as_str() {
                "exit" => break,
                "add" => println!("Adding..."), // todo!()
                "rm" => println!("Removing..."), // todo!()
                "ls" => println!("Listing..."), // todo!()
                "show" => println!("Showing..."), // todo!()
                _ => println!("Unknown command: {}", commands[0]),
            }
        }
        Ok(())
    }
}
