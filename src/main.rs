use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use toml;
use reqwest;
use sixel_image::SixelImage;

const API_URL: &str = "https://api.wolframalpha.com/v1/result";
const API_FULL_URL: &str = "https://api.wolframalpha.com/v1/simple";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {

    #[arg(long, short = 'f', action = clap::ArgAction::SetTrue)]
    full: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Configures API Key for Wolfram API
    Configure,

    /// Query for the Wolfram API
    Query {
        #[arg(value_parser = clap::value_parser!(String))]
        query: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    app_id: String,
}

fn get_config_directory() -> PathBuf {
    if let Ok(config_home) = std::env::var("XDG_CONFIG_HOME") {
        return PathBuf::from(config_home).join("wa-cli");
    }
    PathBuf::from(std::env::var("HOME").unwrap()).join(".config").join("wa-cli")
}

fn configure() {
    let config_directory = get_config_directory();
    let config_file_path = config_directory.join("config.toml");

    print!("Enter your new Wolfram Alpha App ID: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut new_key = String::new();
    io::stdin().read_line(&mut new_key).expect("Failed to read input");
    let new_key = new_key.trim();

    let config = Config { app_id: new_key.to_string() };
    let toml_string = toml::to_string(&config).expect("Failed to serialize config");

    fs::create_dir_all(config_directory).expect("Failed to create config directory");
    fs::write(config_file_path, toml_string).expect("Failed to write config file");

    println!("API Key has been updated.");
    std::process::exit(0);
}

fn make_request(query: &str, full_answer: bool) {
    let app_id = read_config().expect("API Key is not configured. Please run 'wa-cli configure' to set it up.");
    let client = reqwest::blocking::Client::new();

    let url = if full_answer {
        API_FULL_URL
    } else {
        API_URL
    };

    let response = client
        .get(url)
        .query(&[("appid", app_id), ("i", query.to_string())])
        .send()
        .expect("Failed to send request");

    if full_answer {
        // Read response bytes directly
        let bytes = response.bytes().expect("Failed to read response bytes");
        let sixel_image = SixelImage::new(&bytes).unwrap();
        let serialized = sixel_image.serialize();
        println!("{}", serialized);
    } else {
        let text = response.text().expect("Failed to read response text");
        println!("{}", text);
    }
}

fn read_config() -> Option<String> {
    let config_directory = get_config_directory();
    let config_file_path = config_directory.join("config.toml");

    if let Ok(config_contents) = fs::read_to_string(config_file_path) {
        let config: Config = toml::de::from_str(&config_contents).ok()?;
        return Some(config.app_id);
    }

    None
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Configure) => configure(),
        Some(Commands::Query { query }) => {
            make_request(&query, cli.full);
        }
        None => {
            println!("Usage: wa-cli [COMMAND]");
            println!();
            println!("For more information about a command, run `wa-cli <COMMAND> --help`.");
        }
    }
}

