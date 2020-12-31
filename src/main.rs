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

    println!("Copied {} bytes in {} seconds", copy_info.get_bytes_copied(), now.elapsed().as_secs());
    println!("Copied {} out of {} files: {}% successfully transferred", copy_info.get_files_copied(), copy_info.get_files_total(), copy_info.get_successful_transfers());
}
