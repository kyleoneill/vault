use std::str::FromStr;
use std::fs;

use sysinfo::SystemExt;
use sysinfo::DiskExt;
use std::path::Prefix::*;
use std::path::{PathBuf, Component};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub origin: Vec<PathBuf>,
    pub destination: PathBuf,
    pub ignore_extensions: Vec<String>,
    pub ignore_folders: Vec<String>,
    pub overwrite: bool
}

impl Config {
    pub fn new(file_name: &str) -> Result<Self, &'static str> {
        let file_name: PathBuf = PathBuf::from_str(file_name).expect("Failed to convert config file name to PathBuf");
        match fs::read_to_string(file_name) {
            Ok(file_string) => {
                let config: Config = serde_json::from_str(&file_string).expect("Failed to deserialize config json");
                for entry in &config.origin {
                    assert!(entry.exists() && entry.is_dir(), "One or more of the origins either does not exist or is not a directory.");
                }
                if !config.destination.exists() {
                    fs::create_dir(&config.destination).expect("Failed to create destination folder");
                }
                if disk_has_enough_space(&config.destination, &config.origin) {
                    Ok(config)
                }
                else {
                    panic!("The output disk does not have enough remaining space")
                }
            },
            Err(e) => {
                match e.kind() {
                    std::io::ErrorKind::NotFound => {
                        Err("Config file does not exist")
                    },
                    _ => {
                        Err("Unknown error when reading config file to string")
                    }
                }
            }
        }

    }
}

pub fn disk_has_enough_space(destination: &PathBuf, origin: &Vec<PathBuf>) -> bool {
    let output_drive_prefix = get_disk_prefix(destination);
    let space_remaining: u64 = get_remaining_disk_space(output_drive_prefix);
    let mut total_size: u64 = 0;
    for dir in origin {
        total_size += dir_size(&dir);
    }
    if total_size > space_remaining {
        return false
    }
    true
}

fn get_disk_prefix(path: &PathBuf) -> u8 {
    match path.components().next().unwrap() {
        Component::Prefix(prefix_component) => match prefix_component.kind() {
            Disk(disk) => disk,
            _ => panic!("Output disk path is not a valid Windows path")
        },
        _ => panic!("Failed to get path prefix")
    }
}

fn get_remaining_disk_space(drive_prefix: u8) -> u64 {
    let mut sys = sysinfo::System::new_all();
    sys.refresh_all();
    let disks = sys.get_disks();
    for disk in disks {
        let mount_point = disk.get_mount_point().to_str().unwrap().as_bytes();
        if mount_point[0] == drive_prefix {
            return disk.get_available_space()
        }
    }
    panic!("Could not find a disk that corresponds with drive prefix {}", drive_prefix);
}

fn dir_size(dir: &PathBuf) -> u64 {
    let mut total_size: u64 = 0;
    if dir.is_dir() {
        for entry in dir.read_dir().unwrap() {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    total_size += dir_size(&path);
                }
                else if path.is_file() {
                    total_size += path.metadata().unwrap().len();
                }
            }
        }
    }
    total_size
}