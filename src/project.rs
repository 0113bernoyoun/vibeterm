//! Project root detection utilities

use std::path::{Path, PathBuf};

/// Project root markers (in priority order)
const PROJECT_MARKERS: &[&str] = &[
    ".git",
    "Cargo.toml",
    "package.json",
    "pyproject.toml",
    "go.mod",
    ".svn",
];

/// Detect project root by searching upward for marker files
///
/// Starting from `from` path, traverse upward until finding a directory
/// containing one of the PROJECT_MARKERS files/directories.
///
/// Returns the project root directory, or None if no markers found.
pub fn detect_project_root(from: &Path) -> Option<PathBuf> {
    let mut current = from.to_path_buf();

    loop {
        // Check if current directory contains any project marker
        for marker in PROJECT_MARKERS {
            if current.join(marker).exists() {
                return Some(current);
            }
        }

        // Move to parent directory
        if !current.pop() {
            // Reached filesystem root
            break;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_git_root() {
        // This test assumes we're in a git repo
        let current_dir = std::env::current_dir().unwrap();
        let root = detect_project_root(&current_dir);
        assert!(root.is_some());
        let root = root.unwrap();
        assert!(root.join(".git").exists());
    }

    #[test]
    fn test_no_project_root() {
        let root = detect_project_root(Path::new("/tmp"));
        // /tmp typically has no project markers
        assert!(root.is_none() || root.unwrap() != PathBuf::from("/tmp"));
    }
}
