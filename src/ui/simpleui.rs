use crate::database::Database;
use crate::Record;
use rusqlite;
use std::io::{self, IsTerminal, Read, Write};

pub fn list_records(db: &Database) -> rusqlite::Result<()> {
    let notes = db.get_all_records_from_database()?;

    let amount = notes.len();

    for note in notes {
        println!("{}", note);
    }
    println!("Found {} records", amount);
    Ok(())
}

pub fn add_record(db: &Database, title: Option<String>) -> rusqlite::Result<()> {
    let mut can_pipeline = false;
    let title = match title {
        Some(title) => {
            can_pipeline = true;
            title
        }
        None => {
            let mut buffor = String::new();
            println!("Enter title: ");
            std::io::stdin().read_line(&mut buffor).unwrap();
            buffor
        }
    };

    let mut stdin = io::stdin();

    let content = if can_pipeline && !stdin.is_terminal() {
        let mut buffor = String::new();
        stdin
            .read_to_string(&mut buffor)
            .expect("Error with pipeline");
        buffor
    } else {
        let mut buffor = String::new();
        println!("Enter content: ");
        std::io::stdin().read_line(&mut buffor).unwrap();
        buffor
    };

    let note = Record::new(
        Some(title.trim().to_string()),
        Some(content.trim().to_string()),
    );
    db.insert_record_to_database(&note)?;

    println!("Succeed");
    Ok(())
}

pub fn remove_record(db: &Database, id: i32) -> rusqlite::Result<()> {
    if id != 0 {
        db.remove_record(id)?;
        return Ok(());
    }
    let mut buffor = String::new();
    print!("Enter ID of record to remove: ");
    std::io::stdout().flush().unwrap(); // No error should occure here
    std::io::stdin()
        .read_line(&mut buffor)
        .expect("Error occured with text input");

    let id = buffor.trim().parse::<i32>().unwrap_or_else(|error| {
        eprintln!("Error while parsing: {}", error);
        std::process::exit(0);
    });

    db.remove_record(id)?;

    Ok(())
}

pub fn show_record(db: &Database, id: i32) -> rusqlite::Result<()> {
    if id != 0 {
        let record = db.get_record_from_database(id)?;
        record.display();
        return Ok(());
    }
    let mut buffor = String::new();
    print!("Enter ID of record to show: ");
    std::io::stdout().flush().unwrap(); // No error should occure here
    std::io::stdin()
        .read_line(&mut buffor)
        .expect("Error occured with text input");

    // let id = buffor.trim().parse::<i32>().unwrap_or_else(|error| {
    //     eprintln!("Error while parsing: {}", error);
    //     std::process::exit(0);
    // });
    //
    let id: i32 = buffor.trim().parse().expect("Invalid ID");

    let record = db.get_record_from_database(id)?;
    record.display();

    Ok(())
}
