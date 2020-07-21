use std::path::PathBuf;
use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub origin: Vec<PathBuf>,
    pub destination: PathBuf,
    pub ignore: Vec<String>,
    pub overwrite: bool
}

impl Config {
    pub fn new(config_file_name: PathBuf) -> Config {
        let file_string = fs::read_to_string(config_file_name).unwrap();
        let config: Config = serde_json::from_str(&file_string).unwrap();
        for entry in &config.origin {
            assert!(entry.exists() && entry.is_dir(), "One or more of the origins either does not exist or is not a directory.");
        }
        if !config.destination.exists() {
            fs::create_dir(&config.destination).expect("Failed to create destination folder");
        }
        config
    }

    //use std::str::FromStr;
    //Need to modify this so that it matches the new Config struct
    // pub fn new_command_line(args: &Vec<String>) -> Config {
    //     let origin: PathBuf = PathBuf::from_str(&args[1]).unwrap();
    //     let destination: PathBuf = PathBuf::from(&args[2].to_owned());
    //     assert!(origin.exists(), "Origin directory must already exist");
    //     if !destination.exists() {
    //         let res = fs::create_dir(&destination);
    //         res.expect("Failed to create a destination directory");
    //     }
    //     else {
    //         assert!(origin.canonicalize().unwrap() != destination.canonicalize().unwrap(), "Origin and destination cannot be the same");
    //     }
    //     let overwrite: bool;
    //     if args.len() < 4 {
    //         println!("Overwrite flag was not set, defaulting to false");
    //         overwrite = false;
    //     }
    //     else {
    //         if args[3].to_lowercase() == "true" {
    //             overwrite = true;
    //         }
    //         else {
    //             overwrite = false;
    //         }
    //     }
    //     Config {
    //         origin,
    //         destination,
    //         overwrite
    //     }
    // }
}