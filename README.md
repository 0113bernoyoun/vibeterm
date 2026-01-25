# VibeTerm

**Your flow, uninterrupted.**

![Version](https://img.shields.io/badge/version-0.4.0-blue)
![Platform](https://img.shields.io/badge/platform-macOS-lightgrey)
![Rust](https://img.shields.io/badge/rust-stable-orange)

[한국어](./README.ko.md)

## Why VibeTerm?

You're in the zone. Ideas are flowing. The last thing you need is to break your momentum switching between apps, windows, and contexts.

**VibeTerm is built for vibe coders** — developers who want to stay in flow and get things done without friction. One terminal. Everything you need. No distractions.

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

## Project Structure

```
src/
├── main.rs          # Entry point
├── app.rs           # Main application state
├── config.rs        # Configuration management
├── menu.rs          # Native menu bar
├── theme.rs         # Theme system, CJK fonts
└── ui/
    ├── mod.rs
    ├── tab_bar.rs   # Tab bar component
    ├── sidebar.rs   # File explorer
    └── status_bar.rs
```

## Known Limitations

- **Korean IME**: Due to winit/egui IME support limitations, Korean input may be incomplete in some environments.
  - [winit#1497](https://github.com/rust-windowing/winit/issues/1497)
  - [egui#248](https://github.com/emilk/egui/issues/248)

## Roadmap

- [ ] Vertical split (Cmd+Shift+D)
- [ ] Scrollback buffer
- [ ] Text selection and copy
- [ ] Tab drag and drop reordering
- [ ] New window (Cmd+Shift+N)
- [ ] AI integration for vibe coding

## License

MIT License

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.
