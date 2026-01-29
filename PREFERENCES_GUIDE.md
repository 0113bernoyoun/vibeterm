# VibeTerm Preferences Guide

Complete guide to customizing VibeTerm through the Preferences window.

## Opening Preferences

VibeTerm offers three ways to access preferences:

### Keyboard Shortcut (Recommended)
- **macOS**: `Cmd+,` (standard macOS shortcut)

### Menu
- **VibeTerm → Preferences...** (from menu bar)

### Command Palette
- Press `Cmd+P` to open Command Palette
- Search for "Settings" or "Preferences"
- Press `Enter` to open

## Window Overview

The Preferences window is organized into 5 tabs on the left sidebar:

```
┌─────────────────────────────────────────────┐
│  Preferences                          [X]   │
├──────────┬──────────────────────────────────┤
│ General  │  [Tab Content Area]              │
│ Appearance│                                  │
│ Terminal │  Use Apply/Save buttons below    │
│ File Tree│  to confirm changes              │
│ Advanced │                                  │
└──────────┴──────────────────────────────────┘
```

### Action Buttons

Located at the bottom of the window:

- **Cancel** - Close without saving any changes
- **Apply** - Apply changes immediately (without saving to disk)
- **Save** - Apply changes AND persist to config file

## Tab Guide

### 1. General Tab

Customize font sizes and basic layout settings.

#### Font Settings

- **Terminal Size** (10-24 pt)
  - Font size for terminal text
  - Affects readability of command output
  - Default: 14 pt
  - Changes apply immediately with Apply/Save

- **UI Size** (8-20 pt)
  - Font size for menus, buttons, and labels
  - Affects overall interface density
  - Default: 12 pt

#### Layout Settings

- **Sidebar Width** (150-400 px)
  - Width of the file tree sidebar on the left
  - Narrower: more space for terminal
  - Wider: easier file tree navigation
  - Default: 220 px

- **Tab Bar Height** (24-40 px)
  - Height of tab bar at the top
  - Default: 28 px

- **Status Bar Height** (16-32 px)
  - Height of status bar at the bottom
  - Default: 20 px

#### Startup Behavior

- **Show sidebar on startup** (Checkbox)
  - Display the file tree when VibeTerm opens
  - Default: enabled

- **Enable directory tracking** (Checkbox)
  - Automatically update file tree when terminal changes directory
  - Uses CWD polling to track pane location
  - Default: enabled

### 2. Appearance Tab

Customize UI colors and theme.

#### Theme Presets

Quick access to predefined themes:

- **Dark Brown** - Default warm, earthy brown theme (recommended)
- **Reset to Default** - Restore all colors to factory defaults

#### UI Colors

Customize individual UI element colors. Each color has:

- **Color preview square** - Click to see the current color
- **Hex input field** - Enter hex color codes (e.g., `#E07A5F`)

Colors you can customize:

| Color | Purpose | Default |
|-------|---------|---------|
| **Background** | Main window background | #2E1A16 |
| **Surface** | Panel and card backgrounds | #3A241E |
| **Surface Light** | Hover and elevated surfaces | #462E26 |
| **Text** | Primary text color | #F4F1DE |
| **Text Dim** | Secondary/dimmed text | #A0968A |
| **Primary** | Buttons, highlights | #E07A5F |
| **Secondary** | Accent color | #81B29A |
| **Border** | Borders, dividers | #4A2E28 |
| **Selection** | Text selection background | #462E26 |

#### Hex Color Format

Colors are specified in hexadecimal format:

- **Format**: `#RRGGBB` where RR, GG, BB are hex values (00-FF)
- **Example**: `#E07A5F` = Red channel: E0, Green: 7A, Blue: 5F
- **Shortcuts**:
  - Gray scale: `#808080`
  - Pure colors: `#FF0000` (red), `#00FF00` (green), `#0000FF` (blue)

### 3. Terminal Tab

Customize the terminal's ANSI color palette (16-color).

#### ANSI Color Palette

The terminal uses a standard 16-color palette:

- **Normal Colors** (left column) - Colors 0-7
  - Black, Red, Green, Yellow, Blue, Magenta, Cyan, White

- **Bright Colors** (right column) - Colors 8-15
  - Bright versions of the above colors

#### Resetting Colors

Click **Reset ANSI Colors to Default** to restore the factory palette.

#### Color Preview

At the bottom of the tab, a preview shows all 16 colors in action:

```
[■ ■ ■ ■ ■ ■ ■ ■]  <- Normal colors
[■ ■ ■ ■ ■ ■ ■ ■]  <- Bright colors
```

#### Examples

Common color customization scenarios:

**Make git diff more visible:**
- Green (color 2): `#00FF00`
- Red (color 1): `#FF5555`

**Dark theme optimization:**
- White (color 7): `#FFFFFF`
- Black (color 0): `#000000`

### 4. File Tree Tab

Customize the sidebar file browser behavior.

#### Display Settings

- **Show hidden files** (Checkbox)
  - Display files/directories starting with `.`
  - Examples: `.git`, `.env`, `.bashrc`
  - Default: disabled (hidden files hidden)

#### Performance Limits

- **Max Files** (100-5000)
  - Maximum number of files to display in file tree
  - Prevents UI slowdown in large directories
  - Default: 1000
  - Tip: Lower for performance, higher for completeness

- **Max Depth** (1-20)
  - How many directory levels to traverse
  - Higher = deeper directory trees shown
  - Default: 10 levels
  - Tip: Lower for faster loading, higher for full visibility

#### Ignore Patterns

Patterns to exclude from the file tree:

- **Input**: Text area with one pattern per line
- **Examples**:
  ```
  .git
  target
  node_modules
  .DS_Store
  *.log
  build/
  ```

#### Quick Add Buttons

Common patterns available as quick buttons:

- **.DS_Store** - Hides macOS metadata files
- **\*.log** - Hides log files
- **build/** - Hides build directories
- **Reset** - Restore default ignore patterns

#### Default Ignore Patterns

The default configuration already ignores:

```
.git
target
node_modules
```

### 5. Advanced Tab

Fine-tune advanced settings for power users.

#### Context Engine

Settings for AI context optimization (for future AI integration):

- **Max Tokens** (1000-128000)
  - Maximum context size in tokens
  - Higher = more context retained
  - Default: 32000
  - Affects memory usage

- **Target Ratio** (0.5-0.95)
  - Target token usage ratio before compression
  - Lower = compress more aggressively
  - Default: 0.8

- **Smart context management** (Checkbox)
  - Automatically optimize context based on usage patterns
  - Default: enabled

#### Performance Settings

Settings for rendering and performance:

- **GPU acceleration** (Checkbox)
  - Enable hardware-accelerated GPU rendering
  - Status: Not yet implemented (requires restart)
  - Note: Currently all rendering uses CPU with egui

#### Logging

- **Log Level**
  - Controls verbosity of debug output
  - Options: Off, Error, Warn, Info, Debug, Trace
  - Default: Info
  - Used for troubleshooting

#### Experimental Features

Beta/experimental features in development:

- **Enable split panes**
  - (Note: Split panes already fully implemented)

- **Enable tabs sync**
  - Synchronize tabs across sessions

- **Enable AI completions**
  - AI-powered command suggestions

## Configuration File

All settings are stored in a TOML configuration file:

**Location**: `~/.config/vibeterm/config.toml`

### Viewing the Config

```bash
cat ~/.config/vibeterm/config.toml
```

### Manual Editing

You can edit the config file directly:

```bash
nano ~/.config/vibeterm/config.toml
```

Example config file:

```toml
[theme]
background = "#2E1A16"
surface = "#3A241E"
surface_light = "#462E26"
text = "#F4F1DE"
text_dim = "#A0968A"
primary = "#E07A5F"
secondary = "#81B29A"
border = "#4A2E28"
selection = "#462E26"

[font]
terminal_size = 14.0
ui_size = 12.0

[ui]
sidebar_width = 220.0
tab_bar_height = 28.0
status_bar_height = 20.0
show_sidebar = true
enable_cwd_polling = true
show_hidden_files = false
max_files = 1000
max_depth = 10
file_tree_ignore_patterns = [".git", "target", "node_modules"]

[context]
max_tokens = 32000
target_ratio = 0.8
smart_context = true
```

## Common Tasks

### Change Theme

1. Open Preferences (`Cmd+,`)
2. Go to **Appearance** tab
3. Click **Dark Brown** preset or edit colors manually
4. Click **Apply** or **Save**

### Increase Terminal Font Size

1. Open Preferences (`Cmd+,`)
2. Go to **General** tab
3. Drag **Terminal Size** slider to desired size (e.g., 16 pt)
4. Click **Apply** to see changes immediately

### Hide Build Directories

1. Open Preferences (`Cmd+,`)
2. Go to **File Tree** tab
3. In the "Ignore Patterns" text area, add patterns:
   ```
   build
   dist
   .pytest_cache
   __pycache__
   ```
4. Click **Save**

### Customize Color Scheme

1. Open Preferences (`Cmd+,`)
2. Go to **Appearance** or **Terminal** tab
3. Click on color preview squares to see current hex values
4. Edit hex codes directly in the input fields
5. Click **Apply** to preview
6. Click **Save** to keep changes

### Reset Everything to Defaults

1. Open Preferences (`Cmd+,`)
2. Go to **Appearance** tab
3. Click **Reset to Default**
4. Go to **Terminal** tab
5. Click **Reset ANSI Colors to Default**
6. Click **Save**

## Performance Tips

### For Slow Machines

If VibeTerm feels sluggish:

1. Reduce **Max Files** to 500
2. Reduce **Max Depth** to 5
3. Disable **Show hidden files**
4. Disable **Enable directory tracking**
5. Reduce **Terminal Size** font slightly

These changes reduce UI updates and memory usage.

### For Development Work

If working with large projects:

1. Increase **Max Files** to 2000-5000
2. Increase **Max Depth** to 15-20
3. Enable **Show hidden files** (to see .env, .git configs)
4. Adjust ignore patterns to match your project structure

## Keyboard Shortcuts in Preferences

- `Esc` - Close preferences window
- `Cmd+W` - Close preferences window
- `Tab` - Focus next control
- `Shift+Tab` - Focus previous control

## Troubleshooting

### Settings Not Saving

**Problem**: Changes disappear when closing VibeTerm

**Solution**: Make sure to click **Save** button, not just **Apply**

### Changes Don't Apply

**Problem**: Changed a setting but nothing changed in the UI

**Solution**:
1. Click **Apply** or **Save**
2. Some settings require restarting VibeTerm (like CWD polling)
3. Check that values are in valid ranges

### Color Picker Not Working

**Problem**: Can't click on color preview squares

**Current Status**: Color picker dialog not yet implemented
**Workaround**: Edit hex values directly in text fields

### File Tree Missing Files

**Problem**: Expected files not showing in sidebar

**Possible Causes**:
- Pattern matches filename → check ignore patterns
- Max Files limit reached → increase it
- Max Depth too shallow → increase it
- Hidden files hidden → enable "Show hidden files"

## Future Features

Planned enhancements to Preferences:

- Full HSV color picker dialog
- Import/export configurations
- Theme sharing community
- Per-tab preferences
- Keyboard shortcut customization
- Live theme preview

## Getting Help

For issues or questions:

1. Check this guide first
2. Review the main [README.md](/Users/bernocrest/Desktop/dev/projects/vibeterm/README.md)
3. Open an issue on GitHub with:
   - Your config file (sanitized)
   - Steps to reproduce
   - Expected vs actual behavior

## See Also

- [README.md](/Users/bernocrest/Desktop/dev/projects/vibeterm/README.md) - General features
- [SHORTCUTS.md](/Users/bernocrest/Desktop/dev/projects/vibeterm/SHORTCUTS.md) - Keyboard shortcuts
- [PREFERENCES_IMPLEMENTATION.md](/Users/bernocrest/Desktop/dev/projects/vibeterm/PREFERENCES_IMPLEMENTATION.md) - Technical implementation details
