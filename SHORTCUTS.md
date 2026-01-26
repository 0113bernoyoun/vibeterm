# VibeTerm Keyboard Shortcuts

## Tab Management

| Shortcut | Action |
|----------|--------|
| `Cmd+T` | New tab (opens in home directory) |
| `Cmd+W` | Close current tab (requires >1 tab) |
| `Cmd+[` | Previous tab |
| `Cmd+]` | Next tab |
| `Cmd+1` to `Cmd+9` | Select tab by number |

## Pane Management

### Creation & Splitting
| Shortcut | Action |
|----------|--------|
| `Cmd+D` | Split current pane vertically (new terminal) |
| `Cmd+Shift+D` | Split current pane horizontally (new terminal) |
| `Cmd+N` | New terminal pane (vertical split) |
| `Cmd+Shift+E` | New editor pane (vertical split) |

### Navigation
| Shortcut | Action |
|----------|--------|
| `Ctrl+Tab` | Focus next pane |
| `Ctrl+Shift+Tab` | Focus previous pane |
| Mouse click | Focus clicked pane |

### Closing
| Shortcut | Action |
|----------|--------|
| `Cmd+W` | Close current pane (protected - won't close last pane) |

## File Operations

| Shortcut | Action |
|----------|--------|
| `Cmd+S` | Save current editor file |
| Click file in sidebar | Open file in current editor pane |
| `Enter` (sidebar) | Open selected file/toggle directory |
| `Space` (sidebar) | Toggle selected directory |

## File Tree Navigation

| Shortcut | Action |
|----------|--------|
| `↑` Arrow Up | Move selection up |
| `↓` Arrow Down | Move selection down |
| `Enter` | Open file or toggle directory |
| `Space` | Toggle directory expansion |
| `Cmd+Shift+C` | Collapse all directories in sidebar |
| `Cmd+Shift+E` | Expand all directories in sidebar |

## Terminal

- **Mouse events**: Fully handled by iced_term
- **Copy/Paste**: Standard terminal shortcuts (depends on terminal emulator)
- **Focus**: Click terminal or use Ctrl+Tab to navigate

## Editor

| Shortcut | Action |
|----------|--------|
| `Cmd+S` | Save file (if file_path exists) |
| Standard text shortcuts | Cut, copy, paste, undo, redo |
| Arrow keys | Navigate cursor |
| Click | Position cursor |

## Notes

### Pane Protection
- Cannot close the last pane in a tab (keeps at least 1 pane)
- Cannot close the last tab (keeps at least 1 tab)

### File Saving
- `Cmd+S` only works when:
  - Current focused pane is an editor
  - File has a path (was opened from file tree)
- Future: "Save As" dialog for new files

### Context Awareness
- Some shortcuts behave differently based on context:
  - `Cmd+W`: Closes pane if >1 pane exists
  - Sidebar shortcuts only work when sidebar is focused
  - Editor shortcuts only work in editor panes

### Future Shortcuts (not yet implemented)
- `Cmd+P`: Quick file picker
- `Cmd+Shift+F`: Global search
- `Cmd+,`: Settings/preferences
- `F11`: Toggle fullscreen
- `Cmd+K`: Clear terminal
