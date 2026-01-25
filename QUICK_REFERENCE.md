# VibeTerm - Quick Reference Card

## 🚀 Launch
```bash
./target/release/vibeterm
```

## ⌨️ Essential Shortcuts

### File Operations
| Shortcut | Action |
|----------|--------|
| `Cmd+S` | Save file |

### Tab Management
| Shortcut | Action |
|----------|--------|
| `Cmd+T` | New tab |
| `Cmd+W` | Close tab/pane |
| `Cmd+[` | Previous tab |
| `Cmd+]` | Next tab |
| `Cmd+1-9` | Go to tab N |

### Pane Management
| Shortcut | Action |
|----------|--------|
| `Cmd+D` | Split vertical (terminal) |
| `Cmd+Shift+D` | Split horizontal (terminal) |
| `Cmd+N` | New terminal pane |
| `Cmd+Shift+E` | New editor pane |
| `Ctrl+Tab` | Next pane |
| `Ctrl+Shift+Tab` | Previous pane |

### File Tree
| Key | Action |
|-----|--------|
| `↑/↓` | Navigate |
| `Enter` | Open file/toggle dir |
| `Space` | Toggle directory |

## 🎯 Quick Tips

### Multi-Pane Workflow
1. `Cmd+D` - Split terminal
2. Click file in tree
3. `Cmd+Shift+E` - Split editor
4. `Ctrl+Tab` - Switch between panes

### File Editing
1. Click file in sidebar
2. Edit content
3. `Cmd+S` - Save
4. Modified indicator (coming soon)

### Terminal Usage
- All terminal shortcuts work natively
- Mouse selection, copy/paste supported
- Multiple terminals side-by-side

## 🛡️ Safety Features
- Cannot close last pane in tab
- Cannot close last tab
- File paths preserved across reloads
- Save only works with existing files

## 📝 Current Limitations
- "Save As" not yet implemented (only Cmd+S for existing files)
- No visual modified indicator yet
- No status bar notifications yet
- No file close confirmation

## 🔜 Coming Soon
- Status bar with save notifications
- Modified file indicator (dot in tab)
- Save As dialog
- Close confirmation for modified files
- More keyboard shortcuts

## 📦 Build Info
- Version: 0.1.0
- Binary size: ~9.1MB
- Build: 0 warnings
- Clippy: Clean

## 🎨 Theme
- Dark brown color scheme
- Warm tones for comfortable coding
- Built-in syntax highlighting (coming soon)
