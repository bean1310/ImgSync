use std::path::Path;
use thiserror::Error;

pub mod slack;

pub trait Storage {
    fn upload(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>>;
}

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Failed to access")] 
    AccessError,
}