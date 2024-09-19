use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use toml;
use reqwest;
use viuer::Config as ViuerConfig;
use image::io::Reader;
use std::io::Cursor;

const API_URL: &str = "https://api.wolframalpha.com/v1/result";
const API_SIMPLE_URL: &str = "https://api.wolframalpha.com/v1/simple";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {

    #[arg(long, short = 's', action = clap::ArgAction::SetTrue)]
    simple: bool,

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
    background: String,
    foreground: String,
    font_size: u32,
    units: String,
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

    let config = Config {
        app_id: new_key.to_string(),
        background: "FFFFFF".to_string(),
        foreground: "000000".to_string(),
        font_size: 14,
        units: "metric".to_string(),
    };
    let toml_string = toml::to_string(&config).expect("Failed to serialize config");

    fs::create_dir_all(config_directory).expect("Failed to create config directory");
    fs::write(config_file_path, toml_string).expect("Failed to write config file");

    println!("API Key has been updated and configuration has been reset");
    std::process::exit(0);
}

fn make_request(query: &str, simple_answer: bool) {
    if query.is_empty() {
        println!("Usage: wa-cli query [QUERY]");
        std::process::exit(1);
    }
    let config = read_config().expect("Config file not found or invalid, please run `wa-cli configure`");
    let client = reqwest::blocking::Client::new();

    let url = if simple_answer {
        API_SIMPLE_URL
    } else {
        API_URL
    };

    let response = client
        .get(url)
        .query(
            &[("appid", config.app_id),
            ("i", query.to_string()),
            ("units", config.units),
            ("background", config.background),
            ("foreground", config.foreground),
            ("fontsize", config.font_size.to_string())]
        )
        .send()
        .expect("Failed to send request");

    if simple_answer {
        let bytes = response.bytes().expect("Failed to read response bytes");

        let img = Reader::new(Cursor::new(bytes))
            .with_guessed_format()
            .expect("Failed to read image")
            .decode()
            .expect("Failed to decode image");
        
        let conf = ViuerConfig {
            absolute_offset: false,
            ..Default::default()
        };
        viuer::print(&img, &conf).expect("Image printing failed.");
        
    } else {
        let text = response.text().expect("Failed to read response text");
        println!("{}", text);
    }
}

fn read_config() -> Option<Config> {
    let config_directory = get_config_directory();
    let config_file_path = config_directory.join("config.toml");

    if let Ok(config_contents) = fs::read_to_string(config_file_path) {
        let config: Config = toml::de::from_str(&config_contents).ok()?;
        return Some(config);
    }

    None
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Configure) => configure(),
        Some(Commands::Query { query }) => {
            make_request(&query, cli.simple);
        }
        None => {
            println!("Usage: wa-cli [COMMAND]");
            println!();
            println!("For more information about a command, run `wa-cli <COMMAND> --help`.");
        }
    }
}

