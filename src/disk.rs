use sysinfo::SystemExt;
use sysinfo::DiskExt;
use std::path::Prefix::*;
use std::path::{PathBuf, Component, Prefix};

use super::Config;

fn get_path_prefix(path: &PathBuf) -> Prefix {
    match path.components().next().unwrap() {
        Component::Prefix(prefix_component) => prefix_component.kind(),
        _ => panic!(),
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
    let panic_msg = format!("Could not find a disk that corresponds with drive prefix {}", drive_prefix);
    panic!(panic_msg);
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

pub fn disk_has_enough_space(config: &Config) -> bool {
    let output_drive_prefix = match get_path_prefix(&config.destination) {
        Disk(disk) => disk,
        _ => panic!("Destination path does not have a drive prefix.")
    };
    let space_remaining: u64 = get_remaining_disk_space(output_drive_prefix);
    let mut total_size: u64 = 0;
    for dir in &config.origin {
        total_size += dir_size(&dir);
    }
    if total_size > space_remaining {
        return false
    }
    true
}