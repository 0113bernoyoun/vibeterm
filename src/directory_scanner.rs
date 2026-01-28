//! Directory scanning utilities for sidebar file tree
//!
//! Provides recursive directory scanning with configurable limits for
//! depth and file count to prevent excessive resource usage.

use std::path::PathBuf;
use crate::ui::FileEntry;

/// Scan directory recursively with limits (for async loading)
///
/// # Arguments
/// * `root` - The root directory to scan
/// * `max_depth` - Maximum recursion depth (0 = root only)
/// * `max_files` - Maximum total files to include
///
/// # Returns
/// A vector of `FileEntry` items representing the directory tree
pub fn scan_directory(root: &PathBuf, max_depth: usize, max_files: usize) -> Vec<FileEntry> {
    use std::fs;

    let mut entries = Vec::new();
    let mut file_count = 0;

    fn scan_recursive(
        path: &PathBuf,
        depth: usize,
        max_depth: usize,
        entries: &mut Vec<FileEntry>,
        file_count: &mut usize,
        max_files: usize,
    ) -> bool {
        if depth >= max_depth || *file_count >= max_files {
            return false;
        }

        let Ok(dir_entries) = fs::read_dir(path) else {
            return true;
        };

        let mut items: Vec<_> = dir_entries.filter_map(|e| e.ok()).collect();
        items.sort_by_key(|e| e.path());

        for (idx, entry) in items.iter().enumerate() {
            if *file_count >= max_files {
                return false;
            }

            let path = entry.path();
            let is_dir = path.is_dir();
            let name = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("?")
                .to_string();

            // Skip hidden files (starting with .)
            if name.starts_with('.') {
                continue;
            }

            let is_last = idx == items.len() - 1;

            entries.push(FileEntry {
                name,
                path,
                is_dir,
                is_expanded: false,
                depth,
                is_last,
                git_status: None,
                is_pinned: false,
            });

            *file_count += 1;

            if is_dir {
                if !scan_recursive(&entry.path(), depth + 1, max_depth, entries, file_count, max_files) {
                    return false;
                }
            }
        }

        true
    }

    scan_recursive(root, 0, max_depth, &mut entries, &mut file_count, max_files);
    entries
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    /// Create a test directory structure for testing
    fn create_test_tree() -> TempDir {
        let temp = TempDir::new().unwrap();
        let root = temp.path();

        // Create structure:
        // root/
        //   file1.txt
        //   dir1/
        //     file2.txt
        //     subdir/
        //       file3.txt
        //   dir2/
        //     file4.txt
        //   .hidden_dir/
        //     hidden_file.txt

        fs::write(root.join("file1.txt"), "content1").unwrap();

        fs::create_dir(root.join("dir1")).unwrap();
        fs::write(root.join("dir1/file2.txt"), "content2").unwrap();
        fs::create_dir(root.join("dir1/subdir")).unwrap();
        fs::write(root.join("dir1/subdir/file3.txt"), "content3").unwrap();

        fs::create_dir(root.join("dir2")).unwrap();
        fs::write(root.join("dir2/file4.txt"), "content4").unwrap();

        fs::create_dir(root.join(".hidden_dir")).unwrap();
        fs::write(root.join(".hidden_dir/hidden_file.txt"), "hidden").unwrap();

        temp
    }

    #[test]
    fn test_scan_empty_directory() {
        let temp = TempDir::new().unwrap();
        let entries = scan_directory(&temp.path().to_path_buf(), 10, 1000);
        assert!(entries.is_empty());
    }

    #[test]
    fn test_max_depth_limit() {
        let temp = create_test_tree();
        let root = temp.path().to_path_buf();

        // Depth 1: should only get root level items
        let entries = scan_directory(&root, 1, 1000);

        // Should have root-level items only (dir1, dir2, file1.txt)
        // Hidden dir should be excluded
        let depths: Vec<usize> = entries.iter().map(|e| e.depth).collect();
        assert!(depths.iter().all(|&d| d == 0), "All entries should be at depth 0");

        // Depth 2: should include one level of children
        let entries = scan_directory(&root, 2, 1000);
        let max_depth = entries.iter().map(|e| e.depth).max().unwrap_or(0);
        assert!(max_depth <= 1, "Max depth should be 1 with max_depth=2");
    }

    #[test]
    fn test_max_files_limit() {
        let temp = create_test_tree();
        let root = temp.path().to_path_buf();

        // Limit to 2 files
        let entries = scan_directory(&root, 10, 2);
        assert!(entries.len() <= 2, "Should have at most 2 entries");

        // Limit to 100 files (should get everything except hidden)
        let entries = scan_directory(&root, 10, 100);
        assert!(entries.len() >= 4, "Should have at least 4 visible entries");
    }

    #[test]
    fn test_hidden_files_excluded() {
        let temp = create_test_tree();
        let root = temp.path().to_path_buf();

        let entries = scan_directory(&root, 10, 1000);

        // No hidden directories or files should be present
        let has_hidden = entries.iter().any(|e| e.name.starts_with('.'));
        assert!(!has_hidden, "Hidden files/dirs should be excluded");
    }

    #[test]
    fn test_directory_structure() {
        let temp = create_test_tree();
        let root = temp.path().to_path_buf();

        let entries = scan_directory(&root, 10, 1000);

        // Check that directories are correctly marked
        let dir_names: Vec<&str> = entries
            .iter()
            .filter(|e| e.is_dir)
            .map(|e| e.name.as_str())
            .collect();

        assert!(dir_names.contains(&"dir1"), "dir1 should be marked as directory");
        assert!(dir_names.contains(&"dir2"), "dir2 should be marked as directory");
    }

    #[test]
    fn test_nonexistent_directory() {
        let path = PathBuf::from("/nonexistent/path/that/does/not/exist");
        let entries = scan_directory(&path, 10, 1000);
        assert!(entries.is_empty(), "Nonexistent path should return empty");
    }
}
