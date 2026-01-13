use crate::database::Record;
use crate::argument_parser::Args;

pub struct App {
    arguments: Args,
    database: String,
    current: i32,
    loaded_records: Option<Vec<Record>>,
}
