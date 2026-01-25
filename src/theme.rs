//! VibeTerm Theme System
//!
//! TUI-style aesthetics with user-customizable colors

use egui::{Color32, CornerRadius, FontFamily, FontId, Stroke, Style, Visuals};
use crate::config::{Config, RuntimeTheme};

// ========================================
// Box Drawing Characters (TUI Style)
// ========================================

/// Box drawing characters for TUI aesthetic
#[allow(dead_code)]
pub mod tui {
    // Single line
    pub const HORIZONTAL: char = '─';
    pub const VERTICAL: char = '│';
    pub const TOP_LEFT: char = '┌';
    pub const TOP_RIGHT: char = '┐';
    pub const BOTTOM_LEFT: char = '└';
    pub const BOTTOM_RIGHT: char = '┘';
    pub const T_DOWN: char = '┬';
    pub const T_UP: char = '┴';
    pub const T_RIGHT: char = '├';
    pub const T_LEFT: char = '┤';
    pub const CROSS: char = '┼';

    // Double line
    pub const DOUBLE_HORIZONTAL: char = '═';
    pub const DOUBLE_VERTICAL: char = '║';

    // File tree icons (ASCII style)
    pub const FOLDER_CLOSED: &str = "[+]";
    pub const FOLDER_OPEN: &str = "[-]";
    pub const FILE: &str = " - ";
    pub const TREE_BRANCH: &str = "├──";
    pub const TREE_LAST: &str = "└──";
    pub const TREE_PIPE: &str = "│  ";
    pub const TREE_SPACE: &str = "   ";

    // Tab indicators
    pub const TAB_ACTIVE: &str = "▶";
    pub const TAB_INACTIVE: &str = " ";
    pub const TAB_MODIFIED: &str = "*";
    pub const TAB_CLOSE: &str = "×";

    // Pane indicators
    pub const PANE_FOCUSED: &str = "●";
    pub const PANE_UNFOCUSED: &str = "○";

    // Separators
    pub const SEPARATOR: &str = " │ ";
}

// ========================================
// Layout Constants
// ========================================

pub const TAB_BAR_HEIGHT: f32 = 24.0;
#[allow(dead_code)]
pub const SIDEBAR_WIDTH: f32 = 200.0;
pub const STATUS_BAR_HEIGHT: f32 = 18.0;
pub const DIVIDER_WIDTH: f32 = 1.0;

// ========================================
// Theme Application
// ========================================

/// Apply VibeTerm theme to egui context
pub fn apply_theme(ctx: &egui::Context, theme: &RuntimeTheme) {
    let mut style = Style::default();

    // Visuals (colors)
    let mut visuals = Visuals::dark();

    // Window backgrounds
    visuals.window_fill = theme.surface;
    visuals.panel_fill = theme.background;
    visuals.faint_bg_color = theme.surface;
    visuals.extreme_bg_color = theme.background;
    visuals.code_bg_color = theme.surface;

    // Text colors
    visuals.override_text_color = Some(theme.text);

    // No rounded corners for TUI look
    let corner = CornerRadius::same(0);

    // Widget colors
    visuals.widgets.noninteractive.bg_fill = theme.surface;
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, theme.border);
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, theme.text_dim);
    visuals.widgets.noninteractive.corner_radius = corner;

    visuals.widgets.inactive.bg_fill = theme.surface;
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, theme.border);
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, theme.text);
    visuals.widgets.inactive.corner_radius = corner;

    visuals.widgets.hovered.bg_fill = theme.surface_light;
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, theme.primary);
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, theme.text);
    visuals.widgets.hovered.corner_radius = corner;

    visuals.widgets.active.bg_fill = theme.primary;
    visuals.widgets.active.bg_stroke = Stroke::new(1.0, theme.primary);
    visuals.widgets.active.fg_stroke = Stroke::new(1.0, theme.background);
    visuals.widgets.active.corner_radius = corner;

    visuals.widgets.open.bg_fill = theme.surface_light;
    visuals.widgets.open.bg_stroke = Stroke::new(1.0, theme.primary);
    visuals.widgets.open.fg_stroke = Stroke::new(1.0, theme.text);
    visuals.widgets.open.corner_radius = corner;

    // Selection
    visuals.selection.bg_fill = theme.selection;
    visuals.selection.stroke = Stroke::new(1.0, theme.primary);

    // Hyperlinks
    visuals.hyperlink_color = theme.secondary;

    // Window styling
    visuals.window_stroke = Stroke::new(1.0, theme.border);
    visuals.window_shadow = egui::epaint::Shadow::NONE;

    // Popup styling
    visuals.popup_shadow = egui::epaint::Shadow::NONE;

    // No resize handle (TUI style)
    visuals.resize_corner_size = 0.0;

    style.visuals = visuals;

    // Tight spacing for TUI look
    style.spacing.item_spacing = egui::vec2(4.0, 2.0);
    style.spacing.window_margin = egui::Margin::same(2);
    style.spacing.button_padding = egui::vec2(4.0, 2.0);
    style.spacing.indent = 12.0;
    style.spacing.scroll.bar_width = 6.0;

    // No animation (instant response like TUI)
    style.animation_time = 0.0;

    ctx.set_style(style);
}

/// Configure monospace fonts for terminal aesthetic with CJK support
pub fn configure_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // Try to load system CJK font for Korean/Japanese/Chinese support
    #[cfg(target_os = "macos")]
    {
        // macOS system Korean font paths
        let cjk_font_paths = [
            "/System/Library/Fonts/AppleSDGothicNeo.ttc",
            "/System/Library/Fonts/Supplemental/Arial Unicode.ttf",
            "/Library/Fonts/Arial Unicode.ttf",
        ];

        for path in &cjk_font_paths {
            if let Ok(font_data) = std::fs::read(path) {
                fonts.font_data.insert(
                    "CJK".to_owned(),
                    egui::FontData::from_owned(font_data).into(),
                );
                log::info!("Loaded CJK font from: {}", path);
                break;
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        // Linux CJK font paths
        let cjk_font_paths = [
            "/usr/share/fonts/truetype/noto/NotoSansCJK-Regular.ttc",
            "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
            "/usr/share/fonts/noto-cjk/NotoSansCJK-Regular.ttc",
        ];

        for path in &cjk_font_paths {
            if let Ok(font_data) = std::fs::read(path) {
                fonts.font_data.insert(
                    "CJK".to_owned(),
                    egui::FontData::from_owned(font_data).into(),
                );
                log::info!("Loaded CJK font from: {}", path);
                break;
            }
        }
    }

    // Add CJK font as fallback for both Proportional and Monospace
    if fonts.font_data.contains_key("CJK") {
        fonts
            .families
            .entry(FontFamily::Proportional)
            .or_default()
            .push("CJK".to_owned());

        fonts
            .families
            .entry(FontFamily::Monospace)
            .or_default()
            .push("CJK".to_owned());
    }

    ctx.set_fonts(fonts);
}

/// Get terminal theme for egui_term
pub fn get_terminal_theme(config: &Config) -> egui_term::TerminalTheme {
    use egui_term::{ColorPalette, TerminalTheme};

    let t = &config.theme;
    let palette = ColorPalette {
        foreground: t.text.clone(),
        background: t.background.clone(),
        black: t.black.clone(),
        red: t.red.clone(),
        green: t.green.clone(),
        yellow: t.yellow.clone(),
        blue: t.blue.clone(),
        magenta: t.magenta.clone(),
        cyan: t.cyan.clone(),
        white: t.white.clone(),
        bright_black: t.bright_black.clone(),
        bright_red: t.bright_red.clone(),
        bright_green: t.bright_green.clone(),
        bright_yellow: t.bright_yellow.clone(),
        bright_blue: t.bright_blue.clone(),
        bright_magenta: t.bright_magenta.clone(),
        bright_cyan: t.bright_cyan.clone(),
        bright_white: t.bright_white.clone(),
        bright_foreground: None,
        dim_foreground: t.text_dim.clone(),
        dim_black: "#1A0F0C".to_string(),
        dim_red: "#9A5442".to_string(),
        dim_green: "#5A7C6B".to_string(),
        dim_yellow: "#A88E64".to_string(),
        dim_blue: "#2A2C40".to_string(),
        dim_magenta: "#7E4652".to_string(),
        dim_cyan: "#4D7373".to_string(),
        dim_white: "#A8A59A".to_string(),
    };

    TerminalTheme::new(Box::new(palette))
}

/// Get monospace font ID
pub fn mono_font(size: f32) -> FontId {
    FontId::monospace(size)
}

/// Get default colors (for backwards compatibility)
#[allow(dead_code)]
pub mod colors {
    use super::*;

    pub const BACKGROUND: Color32 = Color32::from_rgb(0x2E, 0x1A, 0x16);
    pub const SURFACE: Color32 = Color32::from_rgb(0x3A, 0x24, 0x1E);
    pub const SURFACE_LIGHT: Color32 = Color32::from_rgb(0x46, 0x2E, 0x26);
    pub const TEXT: Color32 = Color32::from_rgb(0xF4, 0xF1, 0xDE);
    pub const TEXT_DIM: Color32 = Color32::from_rgb(0xA0, 0x96, 0x8A);
    pub const PRIMARY: Color32 = Color32::from_rgb(0xE0, 0x7A, 0x5F);
    pub const SECONDARY: Color32 = Color32::from_rgb(0x81, 0xB2, 0x9A);
    pub const BORDER: Color32 = Color32::from_rgb(0x4A, 0x2E, 0x28);
    pub const SELECTION: Color32 = Color32::from_rgb(0x46, 0x2E, 0x26);
}
