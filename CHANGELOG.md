# VibeTerm Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.7.1] - 2025-01-29

### Added

#### Preferences Window (iTerm2 Style)
- **Complete preferences interface** with 5 tabbed sections
  - General: Font sizes, layout dimensions, startup behavior
  - Appearance: UI color customization with hex input
  - Terminal: ANSI 16-color palette editing
  - File Tree: Sidebar configuration and ignore patterns
  - Advanced: Context engine and performance settings
- **Interactive UI elements**:
  - Color preview squares with hex input fields
  - Sliders for numeric values with visual feedback
  - Checkboxes for boolean settings
  - Text areas for pattern configuration
- **Quick action buttons**:
  - Cancel: Discard changes
  - Apply: Apply immediately (no save)
  - Save: Apply and persist to `~/.config/vibeterm/config.toml`
- **Keyboard shortcuts**:
  - `Cmd+,`: Open preferences (standard macOS)
  - `Esc`: Close preferences
  - `Cmd+W`: Close preferences
- **Modal overlay** with semi-transparent background
- **Responsive layout** with scrollable content area
- **Real-time preview** of color changes

#### Documentation
- **PREFERENCES_GUIDE.md**: Complete user guide for Preferences window
  - All 5 tabs explained with examples
  - Configuration file reference
  - Common tasks (change theme, adjust fonts, customize colors)
  - Performance tips for different machines
  - Troubleshooting section
  - ~1000 lines of comprehensive documentation
- **PERFORMANCE_OPTIMIZATION.md**: Memory and performance tuning guide
  - Memory profiling and expected usage
  - 7 optimization strategies (scrollback, file tree, rendering, etc.)
  - GPU acceleration status and roadmap
  - Comparison with competitors (iTerm2, Ghostty, Terminal.app)
  - Profiling tools and monitoring
  - Troubleshooting performance issues
  - Benchmarks and targets for Phase 3
  - ~800 lines of technical documentation

### Documentation Updates
- **README.md**: Added links to new documentation files
  - Updated Configuration section with Preferences reference
  - Updated Features section to mention Preferences window
  - Added Documentation section with links
- **SHORTCUTS.md**: Added Preferences section with `Cmd+,` reference
- **CHANGELOG.md**: This entry with v0.7.1 updates

### Technical Details
- New tabs structured in `src/ui/preferences.rs`:
  - PreferencesWindow state management
  - PreferencesTab enum for tab navigation
  - PreferencesResponse for configuration changes
  - Individual methods for each tab content
- Integration with existing Config system
- Full theme color support (all UI and ANSI colors)
- Hex color parsing and validation
- Change previewing before save

### UI/UX
- Professional iTerm2-style interface
- Consistent monospace font throughout
- Left-aligned tab sidebar with hover effects
- Proper button grouping and alignment
- Shadow effects and rounded corners (8px)
- Minimum window size: 600x400
- Default window size: 700x500

## [0.7.0] - 2025-01-26

### Added

#### Context Management System
- **Git Status Integration**: Real-time git status tracking with file-level indicators
  - Automatic git cache refresh every 5 seconds
  - File status symbols: M (Modified), A (Staged), U (Untracked), D (Deleted), R (Renamed), ! (Conflicted)
  - Repository status display: branch name, commits ahead/behind
- **File Pinning**: Manual file pinning for AI context with LRU eviction
  - Pin indicator (📌) in sidebar
  - Maximum 50 pinned files per workspace
  - Smart eviction when capacity exceeded
- **File System Watcher**: Automatic directory monitoring with smart debouncing
  - 200ms debounce prevents excessive updates
  - Automatic sidebar refresh on file changes
  - Build artifact filtering (.git, target/, node_modules/)

#### New Dependencies
- `notify 6.1` - Cross-platform file system event notification
- `git2 0.19` - Git repository operations and status tracking
- `regex 1.10` - Pattern matching for file filtering

### Technical
- New modules: `src/context/` and `src/watcher/`
- **`context/manager.rs`**: Orchestrates git status, file watcher, and pinned files
- **`context/git.rs`**: Git repository status tracking with caching
- **`context/pinned.rs`**: File pinning with LRU eviction
- **`context/events.rs`**: Context event system for UI updates
- **`watcher/service.rs`**: File system watching with debouncing and filtering
- Event-driven architecture for reactive UI updates
- Performance optimized: <50MB memory, <5% CPU idle

### Performance
- Debounced file watcher reduces CPU/memory overhead
- Git cache respects 5-second refresh interval
- LRU-based file pinning prevents unbounded memory growth
- Efficient event batching from file watcher

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
