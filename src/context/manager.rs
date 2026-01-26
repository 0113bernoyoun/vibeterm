//! Context manager - orchestrates context providers and events

use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::time::Duration;

use super::events::ContextEvent;
use super::git::{FileGitStatus, GitStatusCache, RepoStatus};
use super::pinned::{PinReason, PinnedFile, PinnedFiles};
use super::ContextConfig;
use crate::watcher::{FileWatcherService, WatcherConfig, WatcherEvent};

pub struct ContextManager {
    watcher: Option<FileWatcherService>,
    git_cache: GitStatusCache,
    pinned: PinnedFiles,
    events: VecDeque<ContextEvent>,
    config: ContextConfig,
    active_dir: Option<PathBuf>,
}

impl ContextManager {
    pub fn new(config: ContextConfig) -> Self {
        let watcher = if config.enable_file_watcher {
            let watcher_config = WatcherConfig {
                debounce: Duration::from_millis(config.watcher_debounce_ms),
                ignore_patterns: vec![],
                max_buffer_size: 100,
            };
            match FileWatcherService::new(watcher_config) {
                Ok(w) => {
                    log::info!("File watcher service initialized");
                    Some(w)
                }
                Err(e) => {
                    log::warn!("Failed to create file watcher: {}", e);
                    None
                }
            }
        } else {
            None
        };

        let git_cache = GitStatusCache::new(Duration::from_secs(config.git_refresh_interval_secs));
        let pinned = PinnedFiles::new(config.max_pinned_files);

        Self {
            watcher,
            git_cache,
            pinned,
            events: VecDeque::new(),
            config,
            active_dir: None,
        }
    }

    pub fn set_active_directory(&mut self, path: &Path) {
        let canonical = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());

        if let (Some(watcher), Some(prev_dir)) = (&mut self.watcher, &self.active_dir) {
            let _ = watcher.unwatch(prev_dir);
        }

        if let Some(watcher) = &mut self.watcher {
            if let Err(e) = watcher.watch(&canonical) {
                log::warn!("Failed to watch directory {:?}: {}", canonical, e);
            }
        }

        if self.config.enable_git_status {
            self.git_cache.set_root(&canonical);
        }

        self.active_dir = Some(canonical);
    }

    pub fn poll(&mut self) -> Vec<ContextEvent> {
        let mut result = Vec::new();

        if let Some(watcher) = &mut self.watcher {
            for event in watcher.poll() {
                match event {
                    WatcherEvent::Created(path)
                    | WatcherEvent::Modified(path)
                    | WatcherEvent::Deleted(path)
                    | WatcherEvent::Changed(path) => {
                        let affected_dir = path
                            .parent()
                            .map(|p| p.to_path_buf())
                            .unwrap_or_else(|| path.clone());

                        result.push(ContextEvent::FileSystemChanged {
                            path: path.clone(),
                            affected_dir,
                        });

                        self.git_cache.mark_dirty();
                    }
                    WatcherEvent::Renamed(_, new_path) => {
                        let affected_dir = new_path
                            .parent()
                            .map(|p| p.to_path_buf())
                            .unwrap_or_else(|| new_path.clone());

                        result.push(ContextEvent::FileSystemChanged {
                            path: new_path,
                            affected_dir,
                        });
                        self.git_cache.mark_dirty();
                    }
                    WatcherEvent::Error(e) => {
                        result.push(ContextEvent::Error(e));
                    }
                }
            }
        }

        if self.config.enable_git_status && self.git_cache.refresh_if_needed() {
            result.push(ContextEvent::GitStatusUpdated);
        }

        result.extend(self.events.drain(..));

        result
    }

    pub fn pin_file(&mut self, path: PathBuf) {
        if self.pinned.pin(path.clone(), PinReason::Manual) {
            self.events.push_back(ContextEvent::FilePinned(path));
        }
    }

    pub fn unpin_file(&mut self, path: &Path) {
        if self.pinned.unpin(path) {
            self.events
                .push_back(ContextEvent::FileUnpinned(path.to_path_buf()));
        }
    }

    pub fn toggle_pin(&mut self, path: PathBuf) {
        if self.pinned.toggle(path.clone()) {
            self.events.push_back(ContextEvent::FilePinned(path));
        } else {
            self.events.push_back(ContextEvent::FileUnpinned(path));
        }
    }

    pub fn is_pinned(&self, path: &Path) -> bool {
        self.pinned.is_pinned(path)
    }

    pub fn pinned_files(&self) -> impl Iterator<Item = &PinnedFile> {
        self.pinned.iter()
    }

    pub fn get_git_status(&self, path: &Path) -> FileGitStatus {
        self.git_cache.get_status_for_absolute(path)
    }

    pub fn repo_status(&self) -> Option<&RepoStatus> {
        self.git_cache.repo_status()
    }

    pub fn refresh_git_status(&mut self) {
        self.git_cache.refresh();
    }

    pub fn is_git_available(&self) -> bool {
        self.config.enable_git_status && self.git_cache.is_in_repo()
    }

    pub fn active_directory(&self) -> Option<&Path> {
        self.active_dir.as_deref()
    }
}

impl Default for ContextManager {
    fn default() -> Self {
        Self::new(ContextConfig::default())
    }
}