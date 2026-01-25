# VibeTerm

**Your flow, uninterrupted.**

![Version](https://img.shields.io/badge/version-0.4.0-blue)
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

### Stay in Flow
- Split your workspace horizontally (Cmd+D)
- Switch focus with a click
- Resize panes by dragging

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

### Coming Soon

| Phase | Features |
|-------|----------|
| **v0.5** | Vertical split, scrollback, text selection, Command Palette |
| **v0.6** | Smart Context (auto-pinning, PTY interception, semantic search) |
| **v0.7** | Ghost Text preview, one-tap apply, AI Inspector panel |
| **v0.8** | MCP integration, multi-session orchestration, smart handoff |
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

| Shortcut | Action |
|----------|--------|
| `Cmd+T` | New tab |
| `Cmd+W` | Close current pane/tab |
| `Cmd+D` | Split horizontally |
| `Cmd+B` | Toggle sidebar |
| `Cmd+,` | Preferences |
| `Cmd+1-9` | Switch to tab |
| `Ctrl+Tab` | Next pane |
| `Ctrl+Shift+Tab` | Previous pane |

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
