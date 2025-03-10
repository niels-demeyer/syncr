// src/sync.rs

use std::fs;
use std::path::Path;
use std::time::SystemTime;
use crate::metadata::FileMetadata;

pub fn sync_directories(dir1: &Path, dir2: &Path) -> std::io::Result<()> {
    let metadata1 = collect_metadata(dir1)?;
    let metadata2 = collect_metadata(dir2)?;

    // Sync files from dir1 to dir2
    for (path, meta) in &metadata1 {
        let target_path = dir2.join(path);
        match metadata2.get(path) {
            Some(target_meta) => {
                if meta.modified > target_meta.modified || meta.size != target_meta.size {
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
        match metadata1.get(path) {
            Some(target_meta) => {
                if meta.modified > target_meta.modified || meta.size != target_meta.size {
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

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let metadata = fs::metadata(&path)?;
            let file_meta = FileMetadata {
                path: path.strip_prefix(dir)?.to_string_lossy().to_string(),
                modified: metadata.modified()?,
                size: metadata.len(),
            };
            metadata_map.insert(file_meta.path.clone(), file_meta);
        }
    }

    Ok(metadata_map)
}