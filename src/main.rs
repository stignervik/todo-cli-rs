use clap::{Parser, Subcommand};
use serde::ser::SerializeMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut todos = ToDos::new();

    let args = Args::parse();
    println!("Commands {:?}", args);

    // Check first if the file exist, if so read the todos and store it.
    let file_path = "todos.yaml";
    let file_exist = Path::new(file_path).exists();
    let values: Vec<String> = if file_exist {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        serde_yaml::from_str(&contents)?
    } else {
        Vec::new()
    };

    todos.store.extend(values);

    match &args.cmd {
        Commands::Create { name } => {
            println!("Created a new ToDo: {:?}", name);
            if let Some(name) = name {
                todos.push(name.to_string());

                let serialized = serde_yaml::to_string(&todos.store);
                let mut file = File::create(file_path)?;
                file.write_all(serialized.as_bytes())?;
            }
        }d
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

    Ok(())
}
