use std::time::Instant;

mod config;
use config::Config;

mod copy;
use copy::CopyInfo;

fn main() {
    let config = Config::new("./config.json").unwrap();
    let now = Instant::now();
    let mut copy_info: CopyInfo = CopyInfo::new();

    copy_info.backup(config);

    let (bytes, byte_type) = match copy_info.get_bytes_copied() {
        b if b > 1_000_000_000 => {
            (b / 1_000_000_000, "GB")
        }
        b if b > 1_000_000 => {
            (b / 1_000_000, "MB")
        }
        b if b > 1_000 => {
            (b / 1_000, "KB")
        }
        b => {
            (b, "B")
        }
    };

    println!("Copied {} {} in {} seconds", bytes, byte_type, now.elapsed().as_secs());
    println!("Copied {} out of {} files: {}% successfully transferred", copy_info.get_files_copied(), copy_info.get_files_total(), copy_info.get_successful_transfers());
}
