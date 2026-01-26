//! File System Watcher Service
//!
//! Monitors directories for changes and emits events for UI updates.

use notify::{
    Config as NotifyConfig,
    Event as NotifyEvent,
    RecommendedWatcher,
    RecursiveMode,
    Watcher,
    event::{CreateKind, ModifyKind, RemoveKind, RenameMode},
};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::{Duration, Instant};

/// Events emitted by the file watcher
#[derive(Debug, Clone)]
pub enum WatcherEvent {
    /// A file or directory was created
    Created(PathBuf),
    /// A file was modified
    Modified(PathBuf),
    /// A file or directory was deleted
    Deleted(PathBuf),
    /// A file or directory was renamed (old_path, new_path)
    Renamed(PathBuf, PathBuf),
    /// Generic change that should trigger refresh
    Changed(PathBuf),
    /// Watcher error occurred
    Error(String),
}

/// Configuration for the file watcher
#[derive(Debug, Clone)]
pub struct WatcherConfig {
    /// Debounce duration for coalescing rapid events
    pub debounce: Duration,
    /// Glob patterns to ignore
    pub ignore_patterns: Vec<String>,
    /// Maximum events to buffer before forcing flush
    pub max_buffer_size: usize,
}

impl Default for WatcherConfig {
    fn default() -> Self {
        Self {
            debounce: Duration::from_millis(200),
            ignore_patterns: vec![
                "node_modules".to_string(),
                "target".to_string(),
                ".git".to_string(),
            ],
            max_buffer_size: 100,
        }
    }
}

/// File watcher service with debouncing
pub struct FileWatcherService {
    /// The underlying notify watcher
    watcher: Option<RecommendedWatcher>,
    /// Channel receiver for raw notify events
    raw_rx: Receiver<Result<NotifyEvent, notify::Error>>,
    /// Currently watched paths
    watched_paths: HashSet<PathBuf>,
    /// Configuration
    config: WatcherConfig,
    /// Buffered events for debouncing
    event_buffer: Vec<(Instant, WatcherEvent)>,
    /// Last flush time
    last_flush: Instant,
}

impl FileWatcherService {
    /// Create a new file watcher service
    pub fn new(config: WatcherConfig) -> Result<Self, String> {
        let (tx, rx) = channel();

        let notify_config = NotifyConfig::default()
            .with_poll_interval(Duration::from_secs(1));

        let watcher = RecommendedWatcher::new(
            move |res| {
                let _ = tx.send(res);
            },
            notify_config,
        ).map_err(|e| format!("Failed to create watcher: {}", e))?;

        Ok(Self {
            watcher: Some(watcher),
            raw_rx: rx,
            watched_paths: HashSet::new(),
            config,
            event_buffer: Vec::new(),
            last_flush: Instant::now(),
        })
    }

    /// Start watching a directory
    pub fn watch(&mut self, path: &Path) -> Result<(), String> {
        if let Some(ref mut watcher) = self.watcher {
            let canonical = path.canonicalize()
                .map_err(|e| format!("Failed to canonicalize path: {}", e))?;

            if !self.watched_paths.contains(&canonical) {
                watcher.watch(&canonical, RecursiveMode::Recursive)
                    .map_err(|e| format!("Failed to watch {:?}: {}", canonical, e))?;
                self.watched_paths.insert(canonical);
                log::info!("Watching directory: {:?}", path);
            }
        }
        Ok(())
    }

    /// Stop watching a directory
    pub fn unwatch(&mut self, path: &Path) -> Result<(), String> {
        if let Some(ref mut watcher) = self.watcher {
            let canonical = path.canonicalize()
                .map_err(|e| format!("Failed to canonicalize path: {}", e))?;

            if self.watched_paths.remove(&canonical) {
                watcher.unwatch(&canonical)
                    .map_err(|e| format!("Failed to unwatch {:?}: {}", canonical, e))?;
                log::info!("Stopped watching directory: {:?}", path);
            }
        }
        Ok(())
    }

    /// Check if a path should be ignored based on patterns
    fn should_ignore(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();
        for pattern in &self.config.ignore_patterns {
            if path_str.contains(pattern) {
                return true;
            }
        }
        false
    }

    /// Convert notify event to watcher event
    fn convert_event(&self, event: NotifyEvent) -> Option<WatcherEvent> {
        let path = event.paths.first()?.clone();

        if self.should_ignore(&path) {
            return None;
        }

        match event.kind {
            notify::EventKind::Create(CreateKind::File) |
            notify::EventKind::Create(CreateKind::Folder) => {
                Some(WatcherEvent::Created(path))
            }
            notify::EventKind::Modify(ModifyKind::Data(_)) |
            notify::EventKind::Modify(ModifyKind::Metadata(_)) => {
                Some(WatcherEvent::Modified(path))
            }
            notify::EventKind::Remove(RemoveKind::File) |
            notify::EventKind::Remove(RemoveKind::Folder) => {
                Some(WatcherEvent::Deleted(path))
            }
            notify::EventKind::Modify(ModifyKind::Name(RenameMode::Both)) => {
                if event.paths.len() >= 2 {
                    Some(WatcherEvent::Renamed(
                        event.paths[0].clone(),
                        event.paths[1].clone(),
                    ))
                } else {
                    Some(WatcherEvent::Changed(path))
                }
            }
            _ => Some(WatcherEvent::Changed(path)),
        }
    }

    /// Poll for events (non-blocking)
    ///
    /// Returns debounced events - call this every frame
    pub fn poll(&mut self) -> Vec<WatcherEvent> {
        let now = Instant::now();

        // Collect raw events
        while let Ok(result) = self.raw_rx.try_recv() {
            match result {
                Ok(event) => {
                    if let Some(watcher_event) = self.convert_event(event) {
                        self.event_buffer.push((now, watcher_event));
                    }
                }
                Err(e) => {
                    self.event_buffer.push((now, WatcherEvent::Error(e.to_string())));
                }
            }
        }

        // Check if we should flush
        let should_flush = !self.event_buffer.is_empty() && (
            now.duration_since(self.last_flush) >= self.config.debounce ||
            self.event_buffer.len() >= self.config.max_buffer_size
        );

        if should_flush {
            self.last_flush = now;

            // Deduplicate by path (keep most recent)
            let mut seen_paths: HashSet<PathBuf> = HashSet::new();
            let mut result = Vec::new();

            for (_, event) in self.event_buffer.drain(..).rev() {
                let path = match &event {
                    WatcherEvent::Created(p) |
                    WatcherEvent::Modified(p) |
                    WatcherEvent::Deleted(p) |
                    WatcherEvent::Changed(p) => Some(p.clone()),
                    WatcherEvent::Renamed(_, p) => Some(p.clone()),
                    WatcherEvent::Error(_) => None,
                };

                if let Some(p) = path {
                    if !seen_paths.contains(&p) {
                        seen_paths.insert(p);
                        result.push(event);
                    }
                } else {
                    result.push(event);
                }
            }

            result.reverse();
            result
        } else {
            Vec::new()
        }
    }

    /// Get currently watched paths
    pub fn watched_paths(&self) -> &HashSet<PathBuf> {
        &self.watched_paths
    }

    /// Check if service is active
    pub fn is_active(&self) -> bool {
        self.watcher.is_some()
    }
}

impl Drop for FileWatcherService {
    fn drop(&mut self) {
        self.watcher = None;
        log::info!("File watcher service stopped");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_watcher_creation() {
        let config = WatcherConfig::default();
        let watcher = FileWatcherService::new(config);
        assert!(watcher.is_ok());
    }

    #[test]
    fn test_ignore_patterns() {
        let config = WatcherConfig {
            ignore_patterns: vec!["node_modules".to_string()],
            ..Default::default()
        };
        let watcher = FileWatcherService::new(config).unwrap();

        assert!(watcher.should_ignore(Path::new("/project/node_modules/foo.js")));
        assert!(!watcher.should_ignore(Path::new("/project/src/main.rs")));
    }

    #[test]
    fn test_watch_directory() {
        let temp = TempDir::new().unwrap();
        let config = WatcherConfig::default();
        let mut watcher = FileWatcherService::new(config).unwrap();

        let result = watcher.watch(temp.path());
        assert!(result.is_ok());
        assert!(watcher.watched_paths().len() == 1);
    }
}
