// src/main.rs

use std::env;
use std::process;
use directory_sync::sync::sync_directories;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <source_directory> <destination_directory>", args[0]);
        process::exit(1);
    }

    let source = &args[1];
    let destination = &args[2];

    if let Err(e) = sync_directories(source, destination) {
        eprintln!("Error syncing directories: {}", e);
        process::exit(1);
    }

    println!("Directories synced successfully.");
}