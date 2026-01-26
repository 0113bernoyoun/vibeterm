//! Manual Context Pinning
//!
//! User-controlled file pinning for AI context management.

use std::collections::{HashMap, VecDeque};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

/// Reason why a file was pinned
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PinReason {
    Manual,
    RecentlyEdited,
    TerminalMentioned,
}

impl PinReason {
    pub fn display(&self) -> &'static str {
        match self {
            PinReason::Manual => "pinned",
            PinReason::RecentlyEdited => "edited",
            PinReason::TerminalMentioned => "mentioned",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            PinReason::Manual => "📌",
            PinReason::RecentlyEdited => "✏️",
            PinReason::TerminalMentioned => "💬",
        }
    }
}

/// A pinned file entry
#[derive(Debug, Clone)]
pub struct PinnedFile {
    pub path: PathBuf,
    pub reason: PinReason,
    pub pinned_at: u64,
    last_accessed: u64,
}

impl PinnedFile {
    pub fn new(path: PathBuf, reason: PinReason) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        Self {
            path,
            reason,
            pinned_at: now,
            last_accessed: now,
        }
    }

    pub fn touch(&mut self) {
        self.last_accessed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
    }

    pub fn file_name(&self) -> &str {
        self.path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("?")
    }
}

/// Collection of pinned files with LRU eviction
pub struct PinnedFiles {
    files: HashMap<PathBuf, PinnedFile>,
    lru_order: VecDeque<PathBuf>,
    max_files: usize,
}

impl PinnedFiles {
    pub fn new(max_files: usize) -> Self {
        Self {
            files: HashMap::new(),
            lru_order: VecDeque::new(),
            max_files: max_files.max(1),
        }
    }

    pub fn pin(&mut self, path: PathBuf, reason: PinReason) -> bool {
        let canonical = match path.canonicalize() {
            Ok(p) => p,
            Err(_) => path.clone(),
        };

        if self.files.contains_key(&canonical) {
            self.touch(&canonical);
            return false;
        }

        while self.files.len() >= self.max_files {
            self.evict_oldest();
        }

        let pinned = PinnedFile::new(canonical.clone(), reason);
        self.files.insert(canonical.clone(), pinned);
        self.lru_order.push_back(canonical);

        log::debug!("Pinned file: {:?} ({:?})", path, reason);
        true
    }

    pub fn unpin(&mut self, path: &Path) -> bool {
        let canonical = match path.canonicalize() {
            Ok(p) => p,
            Err(_) => path.to_path_buf(),
        };

        if self.files.remove(&canonical).is_some() {
            self.lru_order.retain(|p| p != &canonical);
            log::debug!("Unpinned file: {:?}", path);
            true
        } else {
            false
        }
    }

    pub fn toggle(&mut self, path: PathBuf) -> bool {
        let canonical = match path.canonicalize() {
            Ok(p) => p,
            Err(_) => path.clone(),
        };

        if self.files.contains_key(&canonical) {
            self.unpin(&canonical);
            false
        } else {
            self.pin(path, PinReason::Manual);
            true
        }
    }

    pub fn is_pinned(&self, path: &Path) -> bool {
        let canonical = match path.canonicalize() {
            Ok(p) => p,
            Err(_) => path.to_path_buf(),
        };
        self.files.contains_key(&canonical)
    }

    pub fn get(&self, path: &Path) -> Option<&PinnedFile> {
        let canonical = match path.canonicalize() {
            Ok(p) => p,
            Err(_) => path.to_path_buf(),
        };
        self.files.get(&canonical)
    }

    pub fn touch(&mut self, path: &Path) {
        let canonical = match path.canonicalize() {
            Ok(p) => p,
            Err(_) => path.to_path_buf(),
        };

        if let Some(pinned) = self.files.get_mut(&canonical) {
            pinned.touch();
            self.lru_order.retain(|p| p != &canonical);
            self.lru_order.push_back(canonical);
        }
    }

    fn evict_oldest(&mut self) {
        if let Some(oldest) = self.lru_order.pop_front() {
            self.files.remove(&oldest);
            log::debug!("Evicted pinned file (LRU): {:?}", oldest);
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &PinnedFile> {
        self.lru_order.iter()
            .filter_map(|p| self.files.get(p))
    }

    pub fn len(&self) -> usize {
        self.files.len()
    }

    pub fn is_empty(&self) -> bool {
        self.files.is_empty()
    }

    pub fn paths(&self) -> Vec<&PathBuf> {
        self.files.keys().collect()
    }

    pub fn clear(&mut self) {
        self.files.clear();
        self.lru_order.clear();
    }
}

impl Default for PinnedFiles {
    fn default() -> Self {
        Self::new(50)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    #[test]
    fn test_pin_unpin() {
        let temp = TempDir::new().unwrap();
        let file1 = temp.path().join("test1.txt");
        fs::write(&file1, "test").unwrap();

        let mut pinned = PinnedFiles::new(10);
        assert!(pinned.pin(file1.clone(), PinReason::Manual));
        assert!(pinned.is_pinned(&file1));
        assert!(pinned.unpin(&file1));
        assert!(!pinned.is_pinned(&file1));
    }

    #[test]
    fn test_lru_eviction() {
        let temp = TempDir::new().unwrap();
        let file1 = temp.path().join("f1.txt");
        let file2 = temp.path().join("f2.txt");
        let file3 = temp.path().join("f3.txt");

        for f in [&file1, &file2, &file3] {
            fs::write(f, "test").unwrap();
        }

        let mut pinned = PinnedFiles::new(2);
        pinned.pin(file1.clone(), PinReason::Manual);
        pinned.pin(file2.clone(), PinReason::Manual);
        pinned.pin(file3.clone(), PinReason::Manual);

        assert_eq!(pinned.len(), 2);
        assert!(!pinned.is_pinned(&file1)); // Evicted
        assert!(pinned.is_pinned(&file3));
    }
}
