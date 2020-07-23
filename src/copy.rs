use std::path::PathBuf;
use std::fs;

use checksums;

fn file_hasnt_changed(file_1: &PathBuf, file_2: &PathBuf) -> bool{
    let checksum_one = checksums::hash_file(file_1, checksums::Algorithm::MD5);
    let checksum_two = checksums::hash_file(file_2, checksums::Algorithm::MD5);
    if checksum_one.eq(&checksum_two) {
        return true
    }
    false
}

pub fn copy_dirs(origin: &PathBuf, destination: &PathBuf, overwrite: bool, ignore: &Vec<String>) -> u64 {
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
                    let file_exists = new_destination.exists();
                    if file_exists && !overwrite {
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
                    if file_exists && file_hasnt_changed(&path, &new_destination) {
                        continue;
                    }
                    let res = fs::copy(&path, new_destination);
                    match res {
                        Ok(copied) => bytes_copied += copied,
                        Err(e) => {
                            eprintln!("Failed to copy file with path {} and error: {:?}", &path.display(), e);
                        }
                    }
                }
            }
        }
    }
    bytes_copied
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