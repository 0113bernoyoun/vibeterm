//! Configuration Management
//!
//! User-customizable colors and settings

use egui::Color32;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Main configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub theme: ThemeConfig,
    pub font: FontConfig,
    pub ui: UiConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: ThemeConfig::default(),
            font: FontConfig::default(),
            ui: UiConfig::default(),
        }
    }
}

/// Theme/color configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ThemeConfig {
    /// Main background color (hex)
    pub background: String,
    /// Panel/surface color
    pub surface: String,
    /// Lighter surface (hover)
    pub surface_light: String,
    /// Primary text color
    pub text: String,
    /// Dimmed text color
    pub text_dim: String,
    /// Primary accent color
    pub primary: String,
    /// Secondary accent color
    pub secondary: String,
    /// Border color
    pub border: String,
    /// Selection color
    pub selection: String,

    // Terminal ANSI colors
    pub black: String,
    pub red: String,
    pub green: String,
    pub yellow: String,
    pub blue: String,
    pub magenta: String,
    pub cyan: String,
    pub white: String,
    pub bright_black: String,
    pub bright_red: String,
    pub bright_green: String,
    pub bright_yellow: String,
    pub bright_blue: String,
    pub bright_magenta: String,
    pub bright_cyan: String,
    pub bright_white: String,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            // Dark brown theme
            background: "#2E1A16".to_string(),
            surface: "#3A241E".to_string(),
            surface_light: "#462E26".to_string(),
            text: "#F4F1DE".to_string(),
            text_dim: "#A0968A".to_string(),
            primary: "#E07A5F".to_string(),
            secondary: "#81B29A".to_string(),
            border: "#4A2E28".to_string(),
            selection: "#462E26".to_string(),

            // ANSI colors
            black: "#2E1A16".to_string(),
            red: "#E07A5F".to_string(),
            green: "#81B29A".to_string(),
            yellow: "#F2CC8F".to_string(),
            blue: "#3D405C".to_string(),
            magenta: "#B56576".to_string(),
            cyan: "#6EA4A4".to_string(),
            white: "#F4F1DE".to_string(),
            bright_black: "#4A2E28".to_string(),
            bright_red: "#E88C74".to_string(),
            bright_green: "#9AC4AD".to_string(),
            bright_yellow: "#F5D9A6".to_string(),
            bright_blue: "#5A5D7A".to_string(),
            bright_magenta: "#C87E8E".to_string(),
            bright_cyan: "#8ABABA".to_string(),
            bright_white: "#FFFFF0".to_string(),
        }
    }
}

/// Font configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct FontConfig {
    /// Font size for terminal
    pub terminal_size: f32,
    /// Font size for UI
    pub ui_size: f32,
}

impl Default for FontConfig {
    fn default() -> Self {
        Self {
            terminal_size: 14.0,
            ui_size: 12.0,
        }
    }
}

/// UI layout configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct UiConfig {
    /// Sidebar width
    pub sidebar_width: f32,
    /// Tab bar height
    pub tab_bar_height: f32,
    /// Status bar height
    pub status_bar_height: f32,
    /// Show sidebar by default
    pub show_sidebar: bool,
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            sidebar_width: 220.0,
            tab_bar_height: 28.0,
            status_bar_height: 20.0,
            show_sidebar: true,
        }
    }
}

impl Config {
    /// Load config from file or return default
    pub fn load() -> Self {
        if let Some(path) = Self::config_path() {
            if path.exists() {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    if let Ok(config) = toml::from_str(&content) {
                        log::info!("Loaded config from {:?}", path);
                        return config;
                    }
                }
            }
        }
        Self::default()
    }

    /// Save config to file
    #[allow(dead_code)]
    pub fn save(&self) -> anyhow::Result<()> {
        if let Some(path) = Self::config_path() {
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let content = toml::to_string_pretty(self)?;
            std::fs::write(&path, content)?;
            log::info!("Saved config to {:?}", path);
        }
        Ok(())
    }

    /// Get config file path
    fn config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|p| p.join("vibeterm").join("config.toml"))
    }
}

/// Parse hex color string to Color32
pub fn parse_hex_color(hex: &str) -> Color32 {
    let hex = hex.trim_start_matches('#');
    if hex.len() >= 6 {
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
        Color32::from_rgb(r, g, b)
    } else {
        Color32::GRAY
    }
}

/// Runtime theme colors (parsed from config)
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct RuntimeTheme {
    pub background: Color32,
    pub surface: Color32,
    pub surface_light: Color32,
    pub text: Color32,
    pub text_dim: Color32,
    pub primary: Color32,
    pub secondary: Color32,
    pub border: Color32,
    pub selection: Color32,

    pub black: Color32,
    pub red: Color32,
    pub green: Color32,
    pub yellow: Color32,
    pub blue: Color32,
    pub magenta: Color32,
    pub cyan: Color32,
    pub white: Color32,
    pub bright_black: Color32,
    pub bright_red: Color32,
    pub bright_green: Color32,
    pub bright_yellow: Color32,
    pub bright_blue: Color32,
    pub bright_magenta: Color32,
    pub bright_cyan: Color32,
    pub bright_white: Color32,
}

impl From<&ThemeConfig> for RuntimeTheme {
    fn from(config: &ThemeConfig) -> Self {
        Self {
            background: parse_hex_color(&config.background),
            surface: parse_hex_color(&config.surface),
            surface_light: parse_hex_color(&config.surface_light),
            text: parse_hex_color(&config.text),
            text_dim: parse_hex_color(&config.text_dim),
            primary: parse_hex_color(&config.primary),
            secondary: parse_hex_color(&config.secondary),
            border: parse_hex_color(&config.border),
            selection: parse_hex_color(&config.selection),

            black: parse_hex_color(&config.black),
            red: parse_hex_color(&config.red),
            green: parse_hex_color(&config.green),
            yellow: parse_hex_color(&config.yellow),
            blue: parse_hex_color(&config.blue),
            magenta: parse_hex_color(&config.magenta),
            cyan: parse_hex_color(&config.cyan),
            white: parse_hex_color(&config.white),
            bright_black: parse_hex_color(&config.bright_black),
            bright_red: parse_hex_color(&config.bright_red),
            bright_green: parse_hex_color(&config.bright_green),
            bright_yellow: parse_hex_color(&config.bright_yellow),
            bright_blue: parse_hex_color(&config.bright_blue),
            bright_magenta: parse_hex_color(&config.bright_magenta),
            bright_cyan: parse_hex_color(&config.bright_cyan),
            bright_white: parse_hex_color(&config.bright_white),
        }
    }
}
