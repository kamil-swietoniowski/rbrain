use crate::database::Database;
use crate::Record;
use rusqlite;
use std::io::Write;


pub fn list_records(db: &Database) -> rusqlite::Result<()>{
    let notes = db.get_all_records_from_database()?;

    let amount = notes.len();

    for note in notes {
        println!("{}", note);
    }
    println!("Found {} records", amount);    
    Ok(())
}

pub fn add_record(db: &Database) -> rusqlite::Result<()> {
    let mut title = String::new();
    println!("Enter title: ");
    std::io::stdin().read_line(&mut title).unwrap();

    let mut content = String::new();
    println!("Enter content: ");
    std::io::stdin().read_line(&mut content).unwrap();

    let note = Record::new(Some(title.trim().to_string()), Some(content.trim().to_string()));
    db.insert_record_to_database(&note)?;

    println!("Succeed");
    Ok(())

}

pub fn remove_record(db: &Database) -> rusqlite::Result<()> {
    let mut buffor = String::new();     
    print!("Enter ID of record to remove: ");
    std::io::stdout().flush().unwrap(); // No error should occure here
    std::io::stdin().read_line(&mut buffor).expect("Error occured with text input");

    let id = buffor.trim().parse::<i32>().unwrap_or_else(|error| {
        eprintln!("Error while parsing: {}", error);
        std::process::exit(0);
    });

    db.remove_record(id)?;

    Ok(())
}

pub fn show_record(db: &Database) -> rusqlite::Result<()> {
    let mut buffor = String::new();     
    print!("Enter ID of record to show: ");
    std::io::stdout().flush().unwrap(); // No error should occure here
    std::io::stdin().read_line(&mut buffor).expect("Error occured with text input");

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


