use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use tempfile::TempDir;

use crate::sync::sync_directories;

#[test]
fn test_sync_with_empty_directories() {
    let source_dir = TempDir::new().unwrap();
    let dest_dir = TempDir::new().unwrap();
    
    assert!(sync_directories(source_dir.path(), dest_dir.path()).is_ok());
}

#[test]
fn test_sync_with_files() {
    let source_dir = TempDir::new().unwrap();
    let dest_dir = TempDir::new().unwrap();
    
    // Create a file in source
    let file_path = source_dir.path().join("test.txt");
    let mut file = File::create(file_path).unwrap();
    file.write_all(b"test content").unwrap();
    
    assert!(sync_directories(source_dir.path(), dest_dir.path()).is_ok());
    
    // Check if file exists in destination
    assert!(dest_dir.path().join("test.txt").exists());
}

#[test]
fn test_sync_with_subdirectories() {
    let source_dir = TempDir::new().unwrap();
    let dest_dir = TempDir::new().unwrap();
    
    // Create a subdirectory with a file
    fs::create_dir(source_dir.path().join("subdir")).unwrap();
    let file_path = source_dir.path().join("subdir/test.txt");
    let mut file = File::create(file_path).unwrap();
    file.write_all(b"test content").unwrap();
    
    assert!(sync_directories(source_dir.path(), dest_dir.path()).is_ok());
    
    // Check if subdirectory and file exist in destination
    assert!(dest_dir.path().join("subdir").exists());
    assert!(dest_dir.path().join("subdir/test.txt").exists());
}

#[test]
fn test_sync_updates_existing_files() {
    let source_dir = TempDir::new().unwrap();
    let dest_dir = TempDir::new().unwrap();
    
    // Create files in both directories with different content
    let source_file_path = source_dir.path().join("test.txt");
    let mut source_file = File::create(&source_file_path).unwrap();
    source_file.write_all(b"updated content").unwrap();
    
    let dest_file_path = dest_dir.path().join("test.txt");
    let mut dest_file = File::create(dest_file_path).unwrap();
    dest_file.write_all(b"old content").unwrap();
    
    assert!(sync_directories(source_dir.path(), dest_dir.path()).is_ok());
    
    // Check if file in destination has been updated
    let content = fs::read_to_string(dest_dir.path().join("test.txt")).unwrap();
    assert_eq!(content, "updated content");
}