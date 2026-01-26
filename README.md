# VibeTerm

**Your flow, uninterrupted.**

![Version](https://img.shields.io/badge/version-0.6.0-blue)
![Platform](https://img.shields.io/badge/platform-macOS-lightgrey)
![Rust](https://img.shields.io/badge/rust-stable-orange)

[한국어](./README.ko.md)

## Why VibeTerm?

You're in the zone. Ideas are flowing. The last thing you need is to break your momentum switching between apps, windows, and contexts.

**VibeTerm is built for vibe coders** — developers who use AI CLI tools like Claude Code and Codex, and want IDE-level productivity without leaving the terminal.

- **Split, don't switch.** Open multiple terminals side by side. No more ⌘+Tab hell.
- **Files at your fingertips.** Browse and open files without leaving the terminal.
- **Your workspace, your way.** Tabs, panes, themes — organize how you think.
- **Native & fast.** Built with Rust. Feels like part of macOS.

```
+------------------------------------------+
| [macOS Native Menu Bar]                   |
+------------------------------------------+
|  [Tab Bar]    ▶1 shell │ 2 file.rs   +   |
+--------+---------------------------------+
| [Side  |   [Terminal Area]               |
|  bar]  |   +-------------+-------------+ |
|        |   |  Pane 1    ║  Pane 2     | |
| Files  |   |  (focused) ║             | |
|        |   +============+-------------+ |
|  Tree  |   Divider (draggable)          |
+--------+---------------------------------+
|  [Status Bar]  VibeTerm │ Panes: 2       |
+------------------------------------------+
```

## Features

### Multi-Pane Workspace (P0)
- **Split horizontally and vertically** — organize terminals side by side
- **Auto-switching sidebar** — follows your focused pane's directory
- **Smart project detection** — auto-detects .git, Cargo.toml, package.json, and more
- **Pane indicators** — clickable mini-tabs in sidebar header to jump between panes
- **Async loading** — non-blocking file tree (up to 1000 files, 10 levels deep)
- **Drag to resize** — smooth pane dividers

### Terminal Text Interaction (P1)
- **Scrollback buffer** — scroll through terminal history
- **Text selection** — click-drag to select, double-click to select word, triple-click to select line
- **Clipboard copy** — `Cmd+C` to copy selected text

### Command Palette & Tab Organization (P2)
- **Command Palette** — `Cmd+P` / `Ctrl+P` with fuzzy search (9 built-in commands)
- **Tab drag-and-drop** — reorder tabs with mouse (5px drag threshold, ghost preview)
- **Quick navigation** — jump between tabs and panes instantly

### Everything in One Place
- Integrated file explorer in the sidebar
- Multiple tabs for different contexts
- Native macOS menu bar

### Built for Speed
- Alacritty-powered terminal backend
- Full ANSI escape sequence support
- Async PTY communication

### Make it Yours
- Dark brown theme (fully customizable)
- CJK font support (Korean/Japanese/Chinese)
- IME input support

## Roadmap

**VibeTerm** is evolving into the ultimate terminal for vibe coding — where the terminal becomes an intelligent canvas shared between you and AI.

### Completed

| Version | Features |
|---------|----------|
| **v0.5** | Vertical split, scrollback, text selection, Command Palette |
| **v0.6** | Multi-pane contextual sidebar, tab reordering, command palette ✓ |

### Coming Soon

| Phase | Features |
|-------|----------|
| **v0.7** | Smart Context (auto-pinning, PTY interception, semantic search) |
| **v0.8** | Ghost Text preview, one-tap apply, AI Inspector panel |
| **v0.9** | MCP integration, multi-session orchestration, smart handoff |
| **v1.0** | Aura effects, smooth animations, full AI integration |

See [vibeterm_specification.md](./vibeterm_specification.md) for the complete roadmap.

## Installation

### Requirements
- macOS (Apple Silicon / Intel)
- Rust (Stable)

### Build

```bash
git clone https://github.com/0113bernoyoun/vibeterm.git
cd vibeterm
cargo build --release
cargo run --release
```

## Keyboard Shortcuts

### Tab & Pane Navigation
| Shortcut | Action |
|----------|--------|
| `Cmd+T` | New tab |
| `Cmd+W` | Close current tab |
| `Cmd+D` | Split pane horizontally |
| `Cmd+Shift+D` | Split pane vertically |
| `Cmd+1-9` | Switch to tab (1-9) |
| `Ctrl+Tab` | Next tab |
| `Ctrl+Shift+Tab` | Previous tab |

### Sidebar & UI
| Shortcut | Action |
|----------|--------|
| `Cmd+B` | Toggle sidebar |
| `Cmd+,` | Open preferences |

### Command Palette
| Shortcut | Action |
|----------|--------|
| `Cmd+P` | Open Command Palette (macOS) |
| `Ctrl+P` | Open Command Palette (Linux) |

### Text Selection & Interaction
| Shortcut | Action |
|----------|--------|
| Click + Drag | Select text |
| Double-click | Select word |
| Triple-click | Select line |
| `Cmd+C` | Copy selected text |
| Scroll wheel | Scrollback buffer |

## Command Palette

The Command Palette provides quick access to all terminal operations with fuzzy search. Press `Cmd+P` (macOS) or `Ctrl+P` (Linux) to open.

### Available Commands

| Command | Shortcut | Description |
|---------|----------|-------------|
| **New Tab** | `Cmd+T` | Create a new terminal tab |
| **Close Tab** | `Cmd+W` | Close the current tab |
| **Split Horizontally** | `Cmd+D` | Split pane left-right |
| **Split Vertically** | `Cmd+Shift+D` | Split pane top-bottom |
| **Close Pane** | - | Close focused pane |
| **Toggle Sidebar** | `Cmd+B` | Show/hide file explorer |
| **Settings** | `Cmd+,` | Open preferences |
| **Next Tab** | `Ctrl+Tab` | Jump to next tab |
| **Previous Tab** | `Ctrl+Shift+Tab` | Jump to previous tab |

### Using the Command Palette

1. Open with `Cmd+P` / `Ctrl+P`
2. Type to search (fuzzy matching)
3. Press `Enter` to execute or `Esc` to cancel
4. Search is case-insensitive and matches partial words

## Sidebar Features

The sidebar provides contextual file browsing tied to your focused pane.

### Smart Project Detection

VibeTerm automatically detects project roots by looking for:
- `.git` (Git repositories)
- `Cargo.toml` (Rust projects)
- `package.json` (Node.js projects)
- `tsconfig.json` (TypeScript projects)
- `pyproject.toml` (Python projects)
- And more...

### Pane Indicators

- The sidebar header displays mini-tabs for each pane
- Click a mini-tab to instantly focus that pane
- The sidebar automatically switches to show the focused pane's directory
- Supports non-blocking async loading (up to 1000 files, 10 directory levels)

### Navigation

- Scroll through the file tree
- Click to open/focus files in the terminal
- Files are loaded asynchronously for smooth interaction

## Configuration

Config file: `~/.config/vibeterm/config.toml`

```toml
[theme]
background = "#2E1A16"
surface = "#3A241E"
primary = "#E07A5F"
text = "#F4F1DE"
text_dim = "#A0968A"
border = "#4A2E28"

[font]
family = "JetBrains Mono"
size = 13.0
```

## Tech Stack

| Component | Library | Version |
|-----------|---------|---------|
| Language | Rust | Stable |
| GUI | egui + eframe | 0.31 |
| Terminal | egui_term | 0.1 |
| Menu | muda | 0.15 |
| Config | serde + toml | 1.0 / 0.8 |
| Async | tokio | 1.0 |

## Known Limitations

- **Korean IME**: Due to winit/egui IME limitations, Korean input may be incomplete in some environments.

## License

MIT License

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.
