use std::path::PathBuf;
use std::fs;
use std::time::Instant;
use std::str::FromStr;

mod config;
use config::Config;

fn copy_dirs(origin: &PathBuf, destination: &PathBuf, overwrite: bool, ignore: &Vec<String>) -> u64 {
    let mut bytes_copied: u64 = 0;
    if origin.is_dir() {
        for entry in origin.read_dir().expect("reading dir failed") {
            if let Ok(entry) = entry {
                let path = entry.path();
                let mut new_destination = destination.clone();
                new_destination.push(path.file_name().expect("Failed to get final entry when making new directory dir"));
                if ignore.contains(&path.file_name().unwrap().to_str().unwrap().to_owned()) {
                    continue
                }
                if path.is_dir() {
                    if !new_destination.exists() {
                        fs::create_dir(&new_destination).expect("Failed to create new directory during copy");
                    }
                    bytes_copied += copy_dirs(&path, &new_destination, overwrite, ignore);
                }
                else if path.is_file() {
                    if new_destination.exists() && !overwrite {
                        continue;
                    }
                    match path.extension() {
                        Some(extension) => {
                            let extension = format!(".{}", extension.to_str().unwrap().to_owned());
                            if ignore.contains(&extension) {
                                continue;
                            }
                        },
                        None => {}
                    }
                    let res = fs::copy(path, new_destination);
                    match res {
                        Ok(copied) => bytes_copied += copied,
                        Err(e) => eprintln!("Failed to copy file with error: {:?}", e)
                    }
                }
            }
        }
    }
    bytes_copied
}

fn main() {
    let config_file_name: PathBuf = PathBuf::from_str("./config.json").unwrap();
    let now = Instant::now();
    let mut bytes_copied: u64 = 0;
    let config = Config::new(config_file_name);
    for entry in &config.origin {
        let mut intermediate_destination = config.destination.clone();
        intermediate_destination.push(entry.file_name().unwrap());
        if !intermediate_destination.exists() {
            fs::create_dir(&intermediate_destination).expect("Could not create folder");
        }
        bytes_copied += copy_dirs(entry, &intermediate_destination, config.overwrite, &config.ignore);
    }
    println!("Copied {} bytes in {} seconds", bytes_copied, now.elapsed().as_secs());
}
