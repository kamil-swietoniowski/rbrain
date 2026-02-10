use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
pub enum Commands {
    Get {
        #[arg(long)]
        id: Option<i32>,
    },
    Ls,
    Rm {
        #[arg(long)]
        id: Option<i32>,

        #[arg(long)]
        force: bool,
    },
    Lf {
        #[arg(long, short, conflicts_with = "content")]
        title: Option<String>,

        #[arg(long, short, conflicts_with = "title")]
        content: Option<String>,
    },
    Add {
        #[arg(short, long)]
        title: Option<String>,

        #[arg(short, long, conflicts_with = "file")]
        content: Option<String>,

        #[arg(short, long, conflicts_with = "content")]
        file: Option<String>,

        #[arg(long)]
        encrypt: bool,
    },
    Mod {
        #[arg(long)]
        id: Option<i32>,

        #[arg(short, long)]
        title: Option<String>,

        #[arg(short, long)]
        content: Option<String>,

        #[arg(long, conflicts_with = "appendf")]
        append: Option<String>,

        #[arg(long, conflicts_with = "append")]
        appendf: Option<String>,

        #[arg(long, conflicts_with = "decrypt")]
        encrypt: bool,

        #[arg(long, conflicts_with = "encrypt")]
        decrypt: bool,
    },
    Menu,
    Tag {
        #[arg(long)]
        id: Option<i32>,

        #[arg(short, long, conflicts_with = "remove", conflicts_with = "list")]
        add: bool,

        #[arg(short, long, conflicts_with = "add", conflicts_with = "list")]
        remove: bool,

        #[arg(short, long)]
        tag: Option<String>,

        #[arg(short, long)]
        force: bool,

        #[arg(short, long, conflicts_with = "remove", conflicts_with = "add")]
        list: bool,
    },
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
// , group(
//         ArgGroup::new("case").required(false).args(["content", "file"])
// ))]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

pub enum Action {
    Add(Option<String>, Source, bool),
    Get(Option<i32>),
    Ls,
    Rm(Option<i32>, bool),
    Lf(LookFor),
    Mod(Option<i32>, Option<String>, Option<String>, Append, Encrypt),
    Tag(Option<i32>, TagAct, bool),
    Menu,
}

#[derive(PartialEq)]
pub enum Append {
    Append(String),
    AppendFile(String),
    NotSpecified,
}

pub enum LookFor {
    Title(String),
    Content(String),
    Tag(String),
    NotSpecified,
}
pub enum Source {
    Content(Option<String>),
    File(Option<String>),
    NotSpecified,
}
pub enum Encrypt {
    Encrypt,
    Decrypt,
    Nothing,
}

pub enum TagAct {
    Add(Option<String>),
    Remove(Option<String>),
    List,
}

impl Args {
    pub fn collect() -> Action {
        let args = Args::parse();

        match args.command {
            Commands::Add {
                title,
                content,
                file,
                encrypt,
            } => {
                let source = if file.is_some() {
                    Source::File(file)
                } else if content.is_some() {
                    Source::Content(content)
                } else {
                    Source::NotSpecified
                };
                Action::Add(title, source, encrypt)
            }
            Commands::Ls => Action::Ls,
            Commands::Lf { title, content } => {
                let var = match (title, content) {
                    (Some(title), None) => LookFor::Title(title),
                    (None, Some(content)) => LookFor::Content(content),
                    (None, None) => LookFor::NotSpecified,
                    (_, _) => {
                        eprintln!("Something went wrong");
                        std::process::exit(1);
                    }
                };

                Action::Lf(var)
            }
            Commands::Rm { id, force } => Action::Rm(id, force),
            Commands::Mod {
                id,
                title,
                content,
                append,
                appendf,
                encrypt,
                decrypt,
            } => {
                let append = match (append, appendf) {
                    (Some(app), None) => Append::Append(app),
                    (None, Some(app)) => Append::AppendFile(app),
                    _ => Append::NotSpecified,
                };
                let enc = match (encrypt, decrypt) {
                    (true, false) => Encrypt::Encrypt,
                    (false, true) => Encrypt::Decrypt,
                    _ => Encrypt::Nothing,
                };
                Action::Mod(id, title, content, append, enc)
            }
            Commands::Get { id } => Action::Get(id),
            Commands::Tag {
                id,
                add,
                remove,
                tag,
                force,
                list,
            } => {
                let act = match (add, remove, list) {
                    (false, false, true) => TagAct::List,
                    (true, false, false) => TagAct::Add(tag),
                    (false, true, false) => TagAct::Remove(tag),
                    _ => panic!("Should not happen"),
                };

                Action::Tag(id, act, force)
            }

            Commands::Menu => Action::Menu,
        }
    }
}
