pub struct FileMetadata {
    pub path: String,
    pub modified_time: std::time::SystemTime,
    pub size: u64,
}

impl FileMetadata {
    pub fn new(path: String) -> std::io::Result<Self> {
        let metadata = std::fs::metadata(&path)?;
        let modified_time = metadata.modified()?;
        let size = metadata.len();

        Ok(FileMetadata {
            path,
            modified_time,
            size,
        })
    }

    pub fn from_directory(dir: &str) -> std::io::Result<Vec<FileMetadata>> {
        let mut metadata_list = Vec::new();
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            if entry.file_type()?.is_file() {
                let file_metadata = FileMetadata::new(entry.path().to_string_lossy().to_string())?;
                metadata_list.push(file_metadata);
            }
        }
        Ok(metadata_list)
    }
}