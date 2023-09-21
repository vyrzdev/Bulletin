use std::env;
use std::fmt::Error;
use std::fs::File;
use std::path::PathBuf;
use std::process::exit;
use serde::{Deserialize, Serialize};
// Expect config file @ ~/.config/bulletin/config

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    subscriptions_path: String,
}

impl Config {
    fn load(config_file: File) -> Config {
        return match serde_json::from_reader(config_file) {
            Ok(config) => config,
            Err(e) => {
                eprintln!("Error parsing config file: {}", e);
                exit(1);
            }
        }
    }
}

fn bulletin_help() {
    println!("Usage: bulletin <command> <arguments, >");
    println!("Commands:");
    println!("help : Show this message");
    println!("subscribe <address>: Subscribe to bulletins");
}

fn main() {
    // Get user
    let home_dir = match env::home_dir() {
        None => {
            println!("Could not find home directory!");
            exit(2);
        }
        Some(home_dir) => home_dir
    };

    //
    let config = Config::load(match File::open(home_dir.join(".config/bulletin/config")) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening config file @ ~/.config/bulletin/config: {}", e);
            exit(1);
        }
    });
    println!("Config: {}", config.subscriptions_path);
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            println!("Missing command, please try bulletin help");
            exit(1);
        }
        _ => {
            let command = &args[1];
            match command.as_str() {
                "help" => {
                    bulletin_help();
                },
                "list" => {
                    list_subscriptions(&config);
                    exit(0);
                }
                _ => {
                    println!("Invalid command, please try bulletin help");
                    exit(1);
                }
            }
        }
    }
}