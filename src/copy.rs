use std::path::PathBuf;
use std::fs;
use std::ops::AddAssign;
use crate::config::Config;

use checksums;

pub struct CopyInfo {
    bytes_copied: u64,
    files_copied: u32,
    files_total: u32
}

impl CopyInfo {
    pub fn new() -> Self {
        CopyInfo {
            bytes_copied: 0,
            files_copied: 0,
            files_total: 0
        }
    }

    pub fn get_bytes_copied(&self) -> u64 {
        self.bytes_copied
    }

    pub fn get_files_copied(&self) -> u32 {
        self.files_copied
    }

    pub fn get_files_total(&self) -> u32 {
        self.files_total
    }

    pub fn get_successful_transfers(&self) -> f64 {
        if self.files_total > 0 {
            let percent = (self.files_copied as f64 / self.files_total as f64) * 100 as f64;
            percent.floor()
        }
        else {
            0 as f64
        }
    }

    pub fn backup(&mut self, config: Config) {
        for entry in &config.origin {
            let mut intermediate_destination = config.destination.clone();
            intermediate_destination.push(entry.file_name().unwrap());
            if !intermediate_destination.exists() {
                fs::create_dir(&intermediate_destination).expect("Could not create folder");
            }
            match self.copy_dirs(entry, &intermediate_destination, &config) {
                Ok(()) => (),
                Err(e) => eprintln!("Failed to copy input folder {} with error\n{}", entry.display(), e)
            }
        }
    }

    ///Recursively walks a filesystem. Copies folders and files unless they are included in the configs ignores.
    ///Also ignores files if they exist in the destination and config is not set to overwrite or if the two files
    ///have the same hash (the file has not changed since the last backup)
    ///
    ///Returns a `Result` with an empty `Ok` on success so an error copying a single folder/file does not cause a `panic`. 
    pub fn copy_dirs(&mut self, origin: &PathBuf, destination: &PathBuf, config: &Config) -> Result<(), &'static str> {
        if origin.is_dir() {
            for entry in origin.read_dir().expect("Failed to read directory contents") {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    let mut new_destination = destination.clone();
                    new_destination.push(path.file_name().unwrap());
                    if path.is_dir() {
                        if !config.ignore_folders.contains(&path.file_name().unwrap().to_str().unwrap().to_owned()) {
                            if !new_destination.exists() {
                                fs::create_dir(&new_destination).expect("Failed to create new directory during copy");
                            }
                            match self.copy_dirs(&path, &new_destination, &config) {
                                Ok(()) => (),
                                Err(_e) => eprintln!("Failed to copy directory: {}", path.display())
                            }
                        }
                    }
                    else if path.is_file() {
                        match path.extension() {
                            Some(extension) => {
                                let extension = extension.to_str().unwrap().to_owned();
                                if config.ignore_extensions.contains(&extension) {
                                    continue;
                                }
                            },
                            None => {}
                        }
                        if new_destination.exists() {
                            if !config.overwrite {
                                continue;
                            }
                            if file_hasnt_changed(&path, &new_destination) {
                                continue;
                            }
                            else {
                                println!("Overwriting file with path: {}", path.display());
                            }
                        }
                        self.files_total += 1;
                        let res = fs::copy(&path, new_destination);
                        match res {
                            Ok(copied) => {
                                self.bytes_copied += copied;
                                self.files_copied += 1;
                                //println!("Copied file: {}", path.display());
                            },
                            Err(e) => {
                                eprintln!("Failed to copy file with path {} and error: {:?}", &path.display(), e);
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

impl AddAssign for CopyInfo {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            bytes_copied: self.bytes_copied + other.bytes_copied,
            files_copied: self.files_copied + other.files_copied,
            files_total: self.files_total + other.files_total
        }
    }
}

fn file_hasnt_changed(file_1: &PathBuf, file_2: &PathBuf) -> bool{
    let checksum_one = checksums::hash_file(file_1, checksums::Algorithm::MD5);
    let checksum_two = checksums::hash_file(file_2, checksums::Algorithm::MD5);
    if checksum_one.eq(&checksum_two) {
        return true
    }
    false
}

/* This is something I found on stackoverflow after writing my own solution. Does it have useful info?
use std::fs;
use std::path::{Path, PathBuf};

pub fn copy<U: AsRef<Path>, V: AsRef<Path>>(from: U, to: V) -> Result<(), std::io::Error> {
    let mut stack = Vec::new();
    stack.push(PathBuf::from(from.as_ref()));

    let output_root = PathBuf::from(to.as_ref());
    let input_root = PathBuf::from(from.as_ref()).components().count();

    while let Some(working_path) = stack.pop() {
        println!("process: {:?}", &working_path);

        // Generate a relative path
        let src: PathBuf = working_path.components().skip(input_root).collect();

        // Create a destination if missing
        let dest = if src.components().count() == 0 {
            output_root.clone()
        } else {
            output_root.join(&src)
        };
        if fs::metadata(&dest).is_err() {
            println!(" mkdir: {:?}", dest);
            fs::create_dir_all(&dest)?;
        }

        for entry in fs::read_dir(working_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else {
                match path.file_name() {
                    Some(filename) => {
                        let dest_path = dest.join(filename);
                        println!("  copy: {:?} -> {:?}", &path, &dest_path);
                        fs::copy(&path, &dest_path)?;
                    }
                    None => {
                        println!("failed: {:?}", path);
                    }
                }
            }
        }
    }

    Ok(())
}
*/