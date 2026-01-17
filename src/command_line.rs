use crate::database::{Database, Record};
use crate::utility::{search_record_by_tag, search_record_by_title};
use std::io::Write;

pub fn command_line(path: &str) -> rusqlite::Result<()> {
    let db = Database::new(path);
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
            "add" => add_to_database(&db, commands.get(1).cloned(), commands.get(2).cloned())?, 
            "rm" => remove_record(&db, commands.get(1).cloned())?, //println!("Removing..."), // todo!()
            "ls" => list_records(&db)?, //println!("Listing..."), // todo!()
            "show" => show_record(&db, commands.get(1).cloned())?, //println!("Showing..."), // todo!()
            "modify" => {
                if commands.len() == 1 {
                    println!("Avaible options for 'modify' are: 'title', 'content'");
                    continue;
                }
                match commands[1].as_str() {
                    "title" => modify_record_title(&db, commands.get(2).cloned(), commands.get(3).cloned())?,
                    "content" => modify_record_content(&db, commands.get(2).cloned(), commands.get(3).cloned())?,
                    _ => {}
                }

            }
            "search" => {
                if commands.len() == 1 {
                    println!("Avaible options for 'search' are: 'tag', 'title'");
                    continue;
                }
                match commands[1].as_str() {
                    "tag" => search_record_tag(&db, commands.get(2).cloned())?,
                    "title" => search_record_title(&db, commands.get(2).cloned())?,
                    _ => {}
                }
            },
            "tag" => {
                if commands.len() == 1 {
                    println!("Avaible options for 'tag' are: 'add', 'rm'");
                    continue;
                }

                match commands[1].as_str() {
                    "add" => insert_tag_into_record(&db, commands.get(2).cloned(), commands.get(3).cloned())?,
                    "rm" => remove_tag_from_record(&db, commands.get(2).cloned(), commands.get(3).cloned())?,
                    _ => {}
                }
            }
            _ => println!("Command not found") // println!("Unknown command: {}", commands[0]),
        }
    }
    Ok(())
}

fn add_to_database(db: &Database, title: Option<String>, content: Option<String>) -> rusqlite::Result<()> {
    let mut title = title;
    let mut content = content;
    if title.is_none() {
        title = Some(input("Input title: "));
    };
    if content.is_none() {
        content = Some(input("Input content: "));
    }
    let record_to_add = Record::new(title, content);
    db.insert_record_to_database(&record_to_add)?;
    println!("Succeed");
    Ok(())
}

fn remove_record(db: &Database, id: Option<String>) -> rusqlite::Result<()> { 
    let id: i32 = match id {
        Some(num) => num.parse::<i32>().unwrap(),
        None => input("Input ID: ").parse::<i32>().unwrap(),
    };


    db.remove_record(id)?;
    
    println!("Succeed");
    Ok(())
}

fn list_records(db: &Database) -> rusqlite::Result<()> {
    let records = db.get_all_records_from_database()?;

    for record in records {
        println!("{}", record);
    }

    Ok(())
}

fn show_record(db: &Database, id: Option<String>) -> rusqlite::Result<()> {
    let id: i32 = match id {
        Some(num) => num.parse::<i32>().unwrap(),
        None => input("Input ID: ").parse::<i32>().unwrap(),
    };

    let record = db.get_record_from_database(id)?;
    
    record.display();

    Ok(())
}

fn search_record_tag(db: &Database, tag: Option<String>) -> rusqlite::Result<()> {
    let records = db.get_all_records_from_database()?;
    let tag = match tag {
        Some(ta) => ta,
        None => input("Input TAG to search for: "),
    };
    
    let found_records = search_record_by_tag(records, &tag).unwrap ();

    if found_records.is_empty() {
        println!("No records found");
        return Ok(())
    }

    for record in found_records {
        println!("{}", record);
    };
    Ok(())
}

fn search_record_title(db: &Database, title: Option<String>) -> rusqlite::Result<()> {
    let records = db.get_all_records_from_database()?;
    let title = match title {
        Some(ta) => ta,
        None => input("Input title to search for: "),
    };
    
    let found_records = search_record_by_title(records, &title).unwrap ();

    if found_records.is_empty() {
        println!("No records found");
        return Ok(())
    }

    for record in found_records {
        println!("{}", record);
    };
    Ok(())
}

fn insert_tag_into_record(db: &Database, id: Option<String>, tag: Option<String>) -> rusqlite::Result<()> {
    let id = match id {
        Some(num) => num.parse::<i32>().unwrap(),
        None => input("Input ID: ").parse::<i32>().unwrap(),
    };
    let tag = match tag {
        Some(t) => t,
        None => input("Input TAG: "),
    };

     db.add_tag_to_record(id, &tag)?;
     println!("Succeed");
     Ok(())
}

fn remove_tag_from_record(db: &Database, id: Option<String>, tag: Option<String>) -> rusqlite::Result<()> {
    let id = match id {
        Some(num) => num.parse::<i32>().unwrap(),
        None => input("Input ID: ").parse::<i32>().unwrap(),
    };
    let tag = match tag {
        Some(t) => t,
        None => input("Input TAG: "),
    };

     db.remove_tag_from_record(id, &tag)?;
     println!("Succeed");
     Ok(())
}

fn modify_record_title(db: &Database, id: Option<String>, title: Option<String>) -> rusqlite::Result<()> {
    let id = match id {
        Some(num) => num.parse::<i32>().unwrap(),
        None => input("Input ID: ").parse::<i32>().unwrap(),
    };

    let title = match title {
        Some(title) => Some(title),
        None => Some(input("Input title to set: "))
    };

    db.modify_record(id, title, None)?;
    Ok(())
}

fn modify_record_content(db: &Database, id: Option<String>, content: Option<String>) -> rusqlite::Result<()> {
    let id = match id {
        Some(num) => num.parse::<i32>().unwrap(),
        None => input("Input ID: ").parse::<i32>().unwrap(),
    };

    let content = match content {
        Some(content) => Some(content),
        None => Some(input("Input content to set: "))
    };

    db.modify_record(id, None, content)?;
    Ok(())
}

fn input(querry: &str) -> String {
    let mut buffor = String::new();
    print!("{}", querry);
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut buffor).expect("Error with input");
    buffor.trim().to_string()
}
