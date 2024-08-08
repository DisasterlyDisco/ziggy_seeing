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

#[derive(Debug)]
struct Location {
    latitude: f32,
    longitude: f32,
}

impl Location {
    pub fn from_str(location_string: &str) -> Result<Location, &str> {
        Location::from_string(String::from(location_string))
    }

    pub fn from_string(location_string: String) -> Result<Location, &'static str> {
        location_string
            .split(&[' ', ',', ':', ';'])
            .map(|coord_string| coord_string.parse::<f32>())
            .collect::<Result<Vec<f32>, std::num::ParseFloatError>>()
            .map_err(|_| "One of the coordinates was not a float")
            .and_then(|coord_vector| {
                if coord_vector.len() == 2 {
                    Ok(coord_vector)
                } else {
                    Err("Number of coordinates should be two")
                }
            })
            .and_then(|coord_vector| {
                let location = Location {
                    latitude: coord_vector[0],
                    longitude: coord_vector[1],
                };
                match (location.latitude_valid(), location.longitude_valid()) {
                    (true, false) => Err("The longitude is not within reasonable bounds"),
                    (false, true) => Err("The latitude is not within reasonable bounds"),
                    (false, false) => Err("The coodinates are not within reasonable bounds"),
                    (true, true) => Ok(location),
                }
            })
    }

    pub fn latitude_valid(&self) -> bool {
        self.latitude >= -90.0 && self.latitude <= 90.0
    }

    pub fn longitude_valid(&self) -> bool {
        self.longitude >= -180.0 && self.longitude <= 180.0
    }
}

struct CallingEntity {
    name: String,
    location: Location,
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
        location: Location::from_str(
            cli.location
                .or(entity_file.location)
                .unwrap_or_else(request_location)
                .as_str()
            )
            .or_else(request_proper_location_loop)
            .unwrap(),
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

fn request_proper_location_loop(error_message: &str) -> Result<Location, &str> {
    println!("The given location could not be parsed: {:?}", error_message);
    println!("Please input your location as longitude latitude");

    let location_result = Location::from_string(
        request_string()
    );

    return location_result.or_else(request_proper_location_loop);
}
