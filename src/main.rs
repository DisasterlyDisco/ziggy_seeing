use clap::Parser;
use serde::Deserialize;
use std::fs;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    name: Option<String>,
    #[arg(short, long)]
    location: Option<String>,
    #[arg(short, long)]
    entity_file_path: Option<String>,
}

#[derive(Deserialize, Default)]
struct EntityFile {
    name: Option<String>,
    location: Option<String>,
}

struct CallingEntity {
    name: String,
    location: String,
}

fn main() {
    let cli = Cli::parse();

    let entity_file: EntityFile = cli
        .entity_file_path
        .and_then(|entity_file_path| fs::read_to_string(entity_file_path).ok())
        .and_then(|file_contents| toml::from_str(&file_contents).ok())
        .unwrap_or_default();

    let calling_entity = CallingEntity {
        name: cli.name.or(entity_file.name).unwrap_or_else(request_name),
        location: cli
            .location
            .or(entity_file.location)
            .unwrap_or_else(request_location),
    };

    println!(
        "You are{:?}, and your location is {:?}?",
        calling_entity.name, calling_entity.location
    );
}

fn request_name() -> String {
    println!("We don't know you. What is your name?");
    request_string()
}

fn request_location() -> String {
    println!("We don't know where you are. What is your location?");
    request_string()
}

fn request_string() -> String {
    let mut user_input = String::new();

    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    user_input.trim().to_string()
}
