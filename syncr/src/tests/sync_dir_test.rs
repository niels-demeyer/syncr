use std::fs::{self, File};
use std::io::Write;
use tempfile::TempDir;
use std::collections::HashMap;

use crate::metadata::FileMetadata;
use super::{sync_directories, collect_metadata_recursive};

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

#[test]
fn test_collect_metadata_empty_directory() {
    let dir = TempDir::new().unwrap();
    let mut metadata_map = HashMap::new();
    
    let result = collect_metadata_recursive(dir.path(), dir.path(), &mut metadata_map);
    
    assert!(result.is_ok());
    assert!(metadata_map.is_empty());
}

#[test]
fn test_collect_metadata_with_files() {
    let dir = TempDir::new().unwrap();
    let mut metadata_map = HashMap::new();
    
    // Create a file
    let file_path = dir.path().join("test.txt");
    let mut file = File::create(&file_path).unwrap();
    file.write_all(b"test content").unwrap();
    
    let result = collect_metadata_recursive(dir.path(), dir.path(), &mut metadata_map);
    
    assert!(result.is_ok());
    assert_eq!(metadata_map.len(), 1);
    assert!(metadata_map.contains_key("test.txt"));
    
    let metadata = &metadata_map["test.txt"];
    assert_eq!(metadata.path, "test.txt");
    assert_eq!(metadata.size, 12); // "test content" is 12 bytes
}

#[test]
fn test_collect_metadata_with_nested_directories() {
    let dir = TempDir::new().unwrap();
    let mut metadata_map = HashMap::new();
    
    // Create nested directory structure
    let subdir_path = dir.path().join("subdir");
    fs::create_dir(&subdir_path).unwrap();
    
    // Create files in root and subdirectory
    let file1_path = dir.path().join("root.txt");
    let mut file1 = File::create(&file1_path).unwrap();
    file1.write_all(b"root content").unwrap();
    
    let file2_path = subdir_path.join("nested.txt");
    let mut file2 = File::create(&file2_path).unwrap();
    file2.write_all(b"nested content").unwrap();
    
    let result = collect_metadata_recursive(dir.path(), dir.path(), &mut metadata_map);
    
    assert!(result.is_ok());
    assert_eq!(metadata_map.len(), 2);
    assert!(metadata_map.contains_key("root.txt"));
    assert!(metadata_map.contains_key("subdir/nested.txt"));
    
    let root_metadata = &metadata_map["root.txt"];
    assert_eq!(root_metadata.path, "root.txt");
    assert_eq!(root_metadata.size, 12); // "root content" is 12 bytes
    
    let nested_metadata = &metadata_map["subdir/nested.txt"];
    assert_eq!(nested_metadata.path, "subdir/nested.txt");
    assert_eq!(nested_metadata.size, 14); // "nested content" is 14 bytes
}

#[test]
fn test_collect_metadata_with_deeply_nested_structure() {
    let dir = TempDir::new().unwrap();
    let mut metadata_map = HashMap::new();
    
    // Create deep directory structure
    let subdir1 = dir.path().join("level1");
    fs::create_dir(&subdir1).unwrap();
    
    let subdir2 = subdir1.join("level2");
    fs::create_dir(&subdir2).unwrap();
    
    let subdir3 = subdir2.join("level3");
    fs::create_dir(&subdir3).unwrap();
    
    // Create a file in the deepest directory
    let deep_file = subdir3.join("deep.txt");
    let mut file = File::create(&deep_file).unwrap();
    file.write_all(b"deep content").unwrap();
    
    let result = collect_metadata_recursive(dir.path(), dir.path(), &mut metadata_map);
    
    assert!(result.is_ok());
    assert_eq!(metadata_map.len(), 1);
    assert!(metadata_map.contains_key("level1/level2/level3/deep.txt"));
    
    let metadata = &metadata_map["level1/level2/level3/deep.txt"];
    assert_eq!(metadata.path, "level1/level2/level3/deep.txt");
    assert_eq!(metadata.size, 12); // "deep content" is 12 bytes
}

#[test]
fn test_collect_metadata_invalid_base_dir() {
    let dir = TempDir::new().unwrap();
    let invalid_dir = dir.path().join("nonexistent");
    let mut metadata_map = HashMap::new();
    
    let result = collect_metadata_recursive(&invalid_dir, &invalid_dir, &mut metadata_map);
    
    assert!(result.is_err());
}