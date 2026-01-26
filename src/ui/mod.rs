//! UI Components for VibeTerm
//!
//! TUI-style components using box-drawing characters

mod tab_bar;
mod sidebar;
mod status_bar;
mod command_palette;

pub use tab_bar::{TabBar, TabInfo};
pub use sidebar::{Sidebar, FileEntry, SidebarResponse};
pub use status_bar::StatusBar;
pub use command_palette::CommandPalette;
