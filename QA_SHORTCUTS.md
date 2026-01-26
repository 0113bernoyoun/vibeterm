# VibeTerm QA Testing - Keyboard Shortcuts Quick Reference

## Essential Shortcuts for Testing

### Application
- `Cmd+Q` - Quit VibeTerm
- `Cmd+,` - Open Settings (if implemented)

### Tabs
- `Cmd+T` - New Tab
- `Cmd+W` - Close Current Tab
- `Cmd+Shift+[` - Previous Tab
- `Cmd+Shift+]` - Next Tab
- `Cmd+1` to `Cmd+9` - Switch to Tab 1-9

### Panes
- `Cmd+D` - Split Pane Horizontally
- `Cmd+Shift+D` - Split Pane Vertically
- `Cmd+[` - Focus Previous Pane
- `Cmd+]` - Focus Next Pane
- `Cmd+Option+←/→/↑/↓` - Navigate Between Panes (directional)

### Command Palette
- `Cmd+P` or `Cmd+Shift+P` - Open Command Palette
- `ESC` - Close Command Palette
- `↑/↓` - Navigate Results
- `Enter` - Execute Selected Command

### Sidebar
- `Cmd+B` - Toggle Sidebar
- Click file in sidebar - Open file viewer
- Click pane indicator - Switch to that pane

### Terminal
- `Cmd+C` - Copy (when text selected)
- `Cmd+V` - Paste
- `Cmd+K` - Clear Terminal (if implemented)
- Mouse wheel - Scroll terminal history

### Text Selection
- Click-drag - Select text
- Double-click - Select word
- Triple-click - Select line
- `Cmd+A` - Select all (if implemented)

---

## Testing Checklist

Before starting tests:
- [ ] VibeTerm binary built in release mode
- [ ] No other VibeTerm instances running
- [ ] Clean terminal history (fresh launch)
- [ ] Activity Monitor open to track performance

During testing:
- [ ] Take screenshots of issues
- [ ] Note exact keyboard shortcuts used
- [ ] Record error messages verbatim
- [ ] Check Console.app for crash logs

After testing:
- [ ] Review `vibeterm.log` if present
- [ ] Check for memory leaks (Activity Monitor)
- [ ] Verify all test cases documented

---

## Known Platform Behaviors (macOS)

- **Cmd+H**: Hides application (standard macOS behavior)
- **Cmd+M**: Minimizes window (standard macOS behavior)
- **Cmd+Tab**: Switches applications (not VibeTerm specific)
- **Fn+←/→**: Home/End keys on MacBook keyboards

---

## Troubleshooting

### If VibeTerm won't launch:
```bash
# Check for existing processes
ps aux | grep vibeterm

# Kill any stuck processes
killall vibeterm

# Check logs
tail -f vibeterm.log
```

### If graphics glitches occur:
- Try force-quit and relaunch
- Check GPU driver (About This Mac → System Report → Graphics)
- Test with `RUST_LOG=debug` for diagnostics

### If tests are unclear:
- Refer to feature specifications in COMPLETION_SUMMARY.md
- Check source code in src/ directory
- Review UI implementation in src/ui/ modules

