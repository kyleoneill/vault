use std::path::PathBuf;
use std::fs;
use std::time::Instant;
use std::str::FromStr;

mod config;
use config::Config;

mod copy;
use copy::copy_dirs;
use copy::CopyInfo;

mod disk;
use disk::disk_has_enough_space;

fn main() {
    let config_file_name: PathBuf = PathBuf::from_str("./config.json").unwrap();
    let now = Instant::now();
    let mut copy_info: CopyInfo = CopyInfo::new(0, 0, 0);
    let config = Config::new(config_file_name);
    if disk_has_enough_space(&config) {
        for entry in &config.origin {
            let mut intermediate_destination = config.destination.clone();
            intermediate_destination.push(entry.file_name().unwrap());
            if !intermediate_destination.exists() {
                fs::create_dir(&intermediate_destination).expect("Could not create folder");
            }
            copy_info += copy_dirs(entry, &intermediate_destination, config.overwrite, &config.ignore);
        }
        println!("Copied {} bytes in {} seconds", copy_info.get_bytes_copied(), now.elapsed().as_secs());
        println!("Copied {} out of {} files: {}% successfully transferred", copy_info.get_files_copied(), copy_info.get_files_total(), copy_info.get_successful_transfers());
    }
    else {
        eprintln!("The disk for the destination folder does not have enough space.");
    }
}
