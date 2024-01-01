use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::Write;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "TheStig")]
    name: String,

    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// Create a ToDo item.
    Create { name: Option<String> },
    /// Read a ToDo item.
    Read { name: Option<String> },
    /// Update a ToDo item.
    Update { name: Option<String> },
    /// Delete a ToDo item.
    Delete { name: Option<String> },
}

struct ToDos {
    store: Vec<String>,
}

impl ToDos {
    pub fn new() -> Self {
        let store = Vec::new();
        Self { store }
    }

    pub fn push(&mut self, todo: String) {
        self.store.push(todo);
    }

    pub fn print(&self) {
        println!("ToDo list:");
        for todo in &self.store {
            println! {"{}", todo};
        }
    }
}

fn main() {
    let mut todos = ToDos::new();

    let args = Args::parse();
    println!("Commands {:?}", args);

    match &args.cmd {
        Commands::Create { name } => {
            println!("Created a new ToDo: {:?}", name);
            if let Some(name) = name {
                todos.push(name.to_string());
            }
        }
        Commands::Read { name } => {
            println!("Read ToDo: {:?}", name);
            todos.print();
        }
        Commands::Update { name } => {
            println!("Update ToDo: {:?}", name)
        }
        Commands::Delete { name } => {
            println!("Delete ToDo with name {:?}", name);
        }
    }
}
