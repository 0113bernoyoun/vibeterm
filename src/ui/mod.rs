//! UI Components for VibeTerm
//!
//! TUI-style components using box-drawing characters

mod tab_bar;
mod sidebar;
mod status_bar;

pub use tab_bar::{TabBar, TabInfo};
pub use sidebar::{Sidebar, FileEntry};
pub use status_bar::StatusBar;
