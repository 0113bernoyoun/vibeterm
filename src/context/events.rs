//! Context events - file system and git change events

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum ContextEvent {
    FileSystemChanged {
        path: PathBuf,
        affected_dir: PathBuf,
    },
    GitStatusUpdated,
    FilePinned(PathBuf),
    FileUnpinned(PathBuf),
    Error(String),
}