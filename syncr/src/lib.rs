pub mod sync;
pub mod metadata;

#[cfg(test)]
pub mod tests {
    pub use crate::metadata::FileMetadata;
    pub use crate::sync::{sync_directories, collect_metadata_recursive};
}