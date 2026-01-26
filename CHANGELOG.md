# VibeTerm Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.6.0] - 2025-01-26

### Added

#### P0: Multi-Pane Contextual Sidebar
- CWD tracking per terminal pane (libproc/procfs)
- Project root auto-detection (.git, Cargo.toml, package.json, etc.)
- Sidebar auto-switches to focused pane's directory
- Pane indicators (clickable mini-tabs)
- Async directory loading (max 1000 files, 10 levels deep)

#### P1: Scrollback & Text Selection
- Scrollback buffer support (egui_term built-in)
- Text selection: click-drag, double-click word, triple-click line
- Clipboard copy with Cmd+C

#### P2: Command Palette
- Quick command access with Cmd+P (Ctrl+P on Linux)
- Fuzzy search across 9 commands
- Keyboard navigation (arrows + Enter)

#### P2: Tab Drag-and-Drop
- Reorder workspace tabs by dragging
- 5px threshold to prevent accidental drags
- Visual ghost preview and drop indicators

### Technical
- Added dependencies: libproc, procfs, fuzzy-matcher
- Created modules: src/project.rs, src/pty_tracker.rs, src/ui/command_palette.rs
- Tokio runtime integration for async operations

### Fixed
- Critical: Tokio runtime initialization panic

## [0.4.0] - 2026-01-24 - Code Cleanup & Event System Completion

### Fixed
- ✅ Resolved all compiler warnings
- ✅ Connected all unused message variants to functionality
- ✅ Implemented file save functionality (Cmd+S)
- ✅ Added keyboard shortcuts for pane management

### Added

#### File Operations
- **Cmd+S**: Save current editor file
  - Saves to existing file path if available
  - Sets modified flag to false after save
  - Properly preserves file path in editor state

#### Pane Management Shortcuts
- **Cmd+N**: New terminal pane (vertical split)
- **Cmd+Shift+E**: New editor pane (vertical split)
- **Cmd+W**: Close current pane (protected - won't close last pane)
- **Cmd+D**: Split terminal vertically
- **Cmd+Shift+D**: Split terminal horizontally

#### Tab Management (existing)
- **Cmd+T**: New tab
- **Cmd+W**: Close tab (when multiple tabs exist)
- **Cmd+[**: Previous tab
- **Cmd+]**: Next tab
- **Cmd+1~9**: Select tab by number
- **Ctrl+Tab**: Focus next pane
- **Ctrl+Shift+Tab**: Focus previous pane

### Changed

#### Message Handlers
- `SaveFile`: Now fully implemented with file writing
- `NewEditorPane`: Creates new editor split
- `ConvertPaneToTerminal`: Converts current pane to terminal
- `ConvertPaneToEditor`: Converts current pane to editor
- `FileLoaded`: Now properly stores file path for save functionality

#### Code Quality
- Marked internal/future-use methods with `#[allow(dead_code)]` or `pub(crate)`
- Added tab ID display in tab bar for better identification
- Improved file path tracking throughout editor lifecycle
- Removed unused variables and cleaned up warnings

### Technical Details

#### File Save Implementation
```rust
// File save flow:
1. User presses Cmd+S
2. Check if focused pane is an editor
3. If file_path exists, write content to disk
4. Reset modified flag on success
5. Show status (future: status bar notification)
```

#### Pane Protection Logic
```rust
// Cmd+W behavior:
- If >1 pane: Close current pane
- If 1 pane: No action (prevent empty state)
- Focus moves to sibling pane after close
```

#### Modified Flag Tracking
- Set to `true` on any editor input action
- Reset to `false` after successful save
- Future: Display indicator in tab/editor UI

### Build Status
- ✅ `cargo build` - 0 warnings
- ✅ `cargo build --release` - 0 warnings
- ✅ All features compiled successfully

### Notes for Future Development
- File save dialog needed for "Save As" (when file_path is None)
- Status bar for save confirmation messages
- Error dialogs for file I/O failures
- Visual indicator for modified files (dot in tab name)
- Keyboard shortcuts for ConvertPaneToTerminal/Editor (currently implemented but not bound)
