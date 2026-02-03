use clap::{ArgGroup, Parser};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, group(
        ArgGroup::new("case").required(false).args(["content", "file"])
))]
pub struct Args {
    #[arg(short, long)]
    pub title: Option<String>,

    #[arg(short, long)]
    pub content: Option<String>,

    #[arg(short, long)]
    pub file: Option<String>,

    #[arg(short, long)]
    pub list: bool,

    #[arg(short, long)]
    pub show: Option<i32>,

    #[arg(short, long)]
    pub delete: Option<i32>,
    //#[arg(short, long, default_value_t = 1)]
    //count: u8,
}
