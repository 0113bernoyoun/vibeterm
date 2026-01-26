//! Context Management System
//!
//! Provides intelligent context awareness for AI-assisted development workflows.
//!
//! ## Overview
//!
//! The context management system coordinates three subsystems to provide rich context
//! information about the active project:
//!
//! 1. **Git Status Integration** - Real-time tracking of file and repository status
//! 2. **File Pinning** - Manual file selection for AI context with LRU eviction
//! 3. **File System Watching** - Automatic monitoring of project changes
//!
//! ## Key Components
//!
//! - [`ContextManager`](manager::ContextManager) - Orchestrates all context providers
//! - [`GitStatusCache`](git::GitStatusCache) - Cached git status with 5-second refresh
//! - [`PinnedFiles`](pinned::PinnedFiles) - File pinning with LRU eviction (max 50 files)
//! - [`FileWatcherService`](crate::watcher::FileWatcherService) - File system event monitoring
//! - [`ContextEvent`](events::ContextEvent) - Event type for UI updates
//!
//! ## Usage Example
//!
//! ```ignore
//! use vibeterm::context::{ContextManager, ContextConfig};
//! use std::path::Path;
//!
//! // Initialize with defaults
//! let mut manager = ContextManager::new(ContextConfig::default());
//!
//! // Set active directory to watch
//! manager.set_active_directory(Path::new("/path/to/project"));
//!
//! // Poll for events in your render loop
//! let events = manager.poll();
//! for event in events {
//!     // Handle ContextEvent variants...
//! }
//! ```
//!
//! ## Configuration
//!
//! ```ignore
//! use vibeterm::context::ContextConfig;
//! use std::time::Duration;
//!
//! let config = ContextConfig {
//!     watcher_debounce_ms: 200,      // Debounce file watcher events
//!     git_refresh_interval_secs: 5,  // Refresh git cache every 5 seconds
//!     max_pinned_files: 50,           // Maximum pinned files
//!     enable_file_watcher: true,      // Enable file system watching
//!     enable_git_status: true,        // Enable git integration
//! };
//! ```

use std::time::Duration;

pub mod events;
pub mod git;
pub mod manager;
pub mod pinned;

pub use events::ContextEvent;
pub use git::{FileGitStatus, GitStatusCache, RepoStatus};
pub use manager::ContextManager;
pub use pinned::{PinReason, PinnedFile, PinnedFiles};

/// Configuration for context system behavior
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ContextConfig {
    pub watcher_debounce_ms: u64,
    pub git_refresh_interval_secs: u64,
    pub max_pinned_files: usize,
    pub enable_file_watcher: bool,
    pub enable_git_status: bool,
}

impl Default for ContextConfig {
    fn default() -> Self {
        Self {
            watcher_debounce_ms: 200,
            git_refresh_interval_secs: 5,
            max_pinned_files: 50,
            enable_file_watcher: true,
            enable_git_status: true,
        }
    }
}
