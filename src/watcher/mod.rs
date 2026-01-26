//! File System Watcher Service
//!
//! Monitors file system changes and emits events for sidebar refresh and context updates.
//!
//! ## Overview
//!
//! The file watcher provides non-blocking, debounced file system event monitoring.
//! It automatically ignores build artifacts and uses platform-specific optimizations:
//!
//! - **macOS**: FSEvents for efficient directory monitoring
//! - **Linux**: inotify for fine-grained file tracking
//! - **Debouncing**: 200ms default debounce prevents excessive updates
//! - **Smart Filtering**: Ignores .git, target/, node_modules/, and other artifacts
//!
//! ## Architecture
//!
//! The watcher is event-driven and integrates with the [`ContextManager`](crate::context::ContextManager):
//!
//! 1. File system changes trigger OS-level notifications
//! 2. Events are buffered with configurable debouncing
//! 3. [`FileWatcherService::poll()`](service::FileWatcherService::poll) returns accumulated events
//! 4. Events are processed by ContextManager and emitted as UI updates
//!
//! ## Event Types
//!
//! ```ignore
//! pub enum WatcherEvent {
//!     Created(PathBuf),      // New file or directory created
//!     Modified(PathBuf),     // Existing file modified
//!     Deleted(PathBuf),      // File or directory deleted
//!     Renamed(PathBuf, PathBuf), // File renamed or moved
//!     Changed(PathBuf),      // Generic file change
//! }
//! ```
//!
//! ## Usage Example
//!
//! ```ignore
//! use vibeterm::watcher::{FileWatcherService, WatcherConfig};
//! use std::time::Duration;
//! use std::path::Path;
//!
//! // Create with custom config
//! let config = WatcherConfig {
//!     debounce: Duration::from_millis(200),
//!     ignore_patterns: vec![],
//!     max_buffer_size: 100,
//! };
//!
//! let mut watcher = FileWatcherService::new(config)?;
//!
//! // Watch a directory
//! watcher.watch(Path::new("/path/to/project"))?;
//!
//! // Poll for events in your event loop
//! for event in watcher.poll() {
//!     println!("File system event: {:?}", event);
//! }
//! ```
//!
//! ## Performance Notes
//!
//! - **Memory**: O(n) where n = number of watched files (typically <10MB)
//! - **CPU**: <1% during idle, <5% with moderate file changes
//! - **Latency**: 200-250ms typical (debounce + processing)
//! - **Throughput**: Handles 100+ events/second with smart coalescing

pub mod service;

pub use service::{FileWatcherService, WatcherConfig, WatcherEvent};
