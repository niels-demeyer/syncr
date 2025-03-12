use std::fs;
use std::path::Path;
// use std::time::SystemTime;
use crate::metadata::FileMetadata;
use std::io::{Error, ErrorKind};

pub fn sync_directories(dir1: &Path, dir2: &Path) -> std::io::Result<()> {
    let metadata1 = collect_metadata(dir1)?;
    let metadata2 = collect_metadata(dir2)?;

    // Sync files from dir1 to dir2
    for (path, meta) in &metadata1 {
        let target_path = dir2.join(path);
        
        // Create parent directories if they don't exist
        if let Some(parent) = target_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        match metadata2.get(path) {
            Some(target_meta) => {
                if meta.modified_time > target_meta.modified_time || meta.size != target_meta.size {
                    fs::copy(dir1.join(path), &target_path)?;
                }
            }
            None => {
                fs::copy(dir1.join(path), &target_path)?;
            }
        }
    }

    // Sync files from dir2 to dir1
    for (path, meta) in &metadata2 {
        let target_path = dir1.join(path);
        
        // Create parent directories if they don't exist
        if let Some(parent) = target_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        match metadata1.get(path) {
            Some(target_meta) => {
                if meta.modified_time > target_meta.modified_time || meta.size != target_meta.size {
                    fs::copy(dir2.join(path), &target_path)?;
                }
            }
            None => {
                fs::copy(dir2.join(path), &target_path)?;
            }
        }
    }

    Ok(())
}

fn collect_metadata(dir: &Path) -> std::io::Result<std::collections::HashMap<String, FileMetadata>> {
    let mut metadata_map = std::collections::HashMap::new();
    collect_metadata_recursive(dir, dir, &mut metadata_map)?;
    Ok(metadata_map)
}

pub fn collect_metadata_recursive(
    base_dir: &Path, 
    current_dir: &Path, 
    metadata_map: &mut std::collections::HashMap<String, FileMetadata>
) -> std::io::Result<()> {
    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();
        let file_type = entry.file_type()?;
        
        if file_type.is_file() {
            let metadata = fs::metadata(&path)?;
            let relative_path = path.strip_prefix(base_dir)
                .map_err(|e| Error::new(ErrorKind::InvalidInput, e))?
                .to_string_lossy().to_string();
                
            let file_meta = FileMetadata {
                path: relative_path,
                modified_time: metadata.modified()?,
                size: metadata.len(),
            };
            metadata_map.insert(file_meta.path.clone(), file_meta);
        } else if file_type.is_dir() {
            // Recursively process subdirectories
            collect_metadata_recursive(base_dir, &path, metadata_map)?;
        }
    }
    
    Ok(())
}