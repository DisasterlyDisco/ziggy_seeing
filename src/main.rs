use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    name: Option<String>,
    #[arg(short, long)]
    location: Option<String>,
}

struct CallingEntity {
    name: String,
    location: String,
}

fn main() {
    let cli = Cli::parse();

    let calling_entity = CallingEntity {
        name: cli.name.unwrap_or_else(request_name),
        location: cli.location.unwrap_or_else(request_location),
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
    println!("We don't know where you are. Where is your location?");
    request_string()
}

fn request_string() -> String {
    let mut user_input = String::new();

    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    user_input.trim().to_string()
}
