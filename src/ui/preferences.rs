//! Preferences Window
//!
//! iTerm2-style settings interface with tab-based navigation
//! Implemented as a separate native window using egui deferred viewports

use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use std::sync::mpsc::{channel, Sender, Receiver};
use egui::{
    Align, Button, Frame, Layout, Margin, RichText, ScrollArea, Stroke, Vec2,
    ViewportBuilder, ViewportCommand, ViewportId,
};
use crate::config::{Config, RuntimeTheme, ThemeConfig, UiConfig};
use crate::theme::mono_font;

/// Viewport ID for the preferences window
const PREFERENCES_VIEWPORT_ID: &str = "preferences_viewport";

/// Shared state between main app and preferences viewport
pub struct PreferencesSharedState {
    pub temp_config: Mutex<Config>,
    pub current_config: Mutex<Config>,
    pub active_tab: Mutex<PreferencesTab>,
    pub theme: Mutex<RuntimeTheme>,
}

impl PreferencesSharedState {
    fn new(config: Config, theme: RuntimeTheme) -> Self {
        Self {
            temp_config: Mutex::new(config.clone()),
            current_config: Mutex::new(config),
            active_tab: Mutex::new(PreferencesTab::General),
            theme: Mutex::new(theme),
        }
    }
}

/// Commands sent from preferences viewport to main app
pub enum PreferencesCommand {
    /// Apply config without saving (preview)
    ApplyConfig(Config),
    /// Save config and close window
    SaveAndClose(Config),
    /// Cancel and close window
    Cancel,
}

/// Preferences window state
pub struct PreferencesWindow {
    visible: Arc<AtomicBool>,
    shared_state: Arc<PreferencesSharedState>,
    command_tx: Sender<PreferencesCommand>,
    command_rx: Receiver<PreferencesCommand>,
}

/// Tab categories for preferences
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreferencesTab {
    General,
    Appearance,
    Terminal,
    FileTree,
    Advanced,
}

impl PreferencesTab {
    fn label(&self) -> &'static str {
        match self {
            Self::General => "General",
            Self::Appearance => "Appearance",
            Self::Terminal => "Terminal",
            Self::FileTree => "File Tree",
            Self::Advanced => "Advanced",
        }
    }

    fn all() -> &'static [Self] {
        &[
            Self::General,
            Self::Appearance,
            Self::Terminal,
            Self::FileTree,
            Self::Advanced,
        ]
    }
}

/// Response from preferences window
#[derive(Default)]
pub struct PreferencesResponse {
    /// If Some, apply this config immediately
    pub apply_config: Option<Config>,
    /// If true, save config to disk
    pub save_config: bool,
}

impl PreferencesWindow {
    pub fn new(config: Config) -> Self {
        let (command_tx, command_rx) = channel();
        let theme = RuntimeTheme::from(&config.theme);

        Self {
            visible: Arc::new(AtomicBool::new(false)),
            shared_state: Arc::new(PreferencesSharedState::new(config, theme)),
            command_tx,
            command_rx,
        }
    }

    /// Open the preferences window
    pub fn open(&mut self, config: Config) {
        // Update shared state
        {
            let mut temp = self.shared_state.temp_config.lock().unwrap();
            *temp = config.clone();
        }
        {
            let mut current = self.shared_state.current_config.lock().unwrap();
            *current = config.clone();
        }
        {
            let mut tab = self.shared_state.active_tab.lock().unwrap();
            *tab = PreferencesTab::General;
        }
        {
            let mut theme = self.shared_state.theme.lock().unwrap();
            *theme = RuntimeTheme::from(&config.theme);
        }

        self.visible.store(true, Ordering::SeqCst);
    }

    /// Close the preferences window
    pub fn close(&mut self) {
        self.visible.store(false, Ordering::SeqCst);
    }

    /// Is the window currently visible?
    pub fn is_visible(&self) -> bool {
        self.visible.load(Ordering::SeqCst)
    }

    /// Update the theme for live preview
    pub fn update_theme(&self, theme: RuntimeTheme) {
        let mut t = self.shared_state.theme.lock().unwrap();
        *t = theme;
    }

    /// Poll for commands from the preferences window (non-blocking)
    pub fn poll_commands(&self) -> Option<PreferencesCommand> {
        self.command_rx.try_recv().ok()
    }

    /// Show the preferences window using deferred viewport
    /// Returns PreferencesResponse with any actions to take
    pub fn show(&mut self, ctx: &egui::Context, current_config: &Config, theme: &RuntimeTheme) -> PreferencesResponse {
        let mut response = PreferencesResponse::default();

        // Process any pending commands from the viewport
        while let Some(cmd) = self.poll_commands() {
            match cmd {
                PreferencesCommand::ApplyConfig(config) => {
                    response.apply_config = Some(config);
                }
                PreferencesCommand::SaveAndClose(config) => {
                    response.apply_config = Some(config);
                    response.save_config = true;
                    self.visible.store(false, Ordering::SeqCst);
                }
                PreferencesCommand::Cancel => {
                    self.visible.store(false, Ordering::SeqCst);
                }
            }
        }

        if !self.visible.load(Ordering::SeqCst) {
            return response;
        }

        // Update current config and theme in shared state
        {
            let mut current = self.shared_state.current_config.lock().unwrap();
            *current = current_config.clone();
        }
        {
            let mut t = self.shared_state.theme.lock().unwrap();
            *t = theme.clone();
        }

        // Spawn the deferred viewport
        let visible = Arc::clone(&self.visible);
        let shared_state = Arc::clone(&self.shared_state);
        let command_tx = self.command_tx.clone();

        ctx.show_viewport_deferred(
            ViewportId::from_hash_of(PREFERENCES_VIEWPORT_ID),
            ViewportBuilder::default()
                .with_title("VibeTerm Preferences")
                .with_inner_size([700.0, 500.0])
                .with_min_inner_size([600.0, 400.0])
                .with_resizable(true)
                .with_close_button(true),
            move |ctx, class| {
                // Handle viewport close (window X button)
                if class == egui::ViewportClass::Deferred {
                    if ctx.input(|i| i.viewport().close_requested()) {
                        visible.store(false, Ordering::SeqCst);
                        let _ = command_tx.send(PreferencesCommand::Cancel);
                    }
                }

                Self::render_preferences_ui(ctx, &shared_state, &command_tx, &visible);
            },
        );

        response
    }

    /// Main rendering function for the preferences viewport
    fn render_preferences_ui(
        ctx: &egui::Context,
        shared_state: &Arc<PreferencesSharedState>,
        command_tx: &Sender<PreferencesCommand>,
        visible: &Arc<AtomicBool>,
    ) {
        // Clone theme for use throughout (minimize lock time)
        let theme = {
            let t = shared_state.theme.lock().unwrap();
            t.clone()
        };

        // Apply theme to viewport
        crate::theme::apply_theme(ctx, &theme);

        // Handle keyboard shortcuts
        let should_close = ctx.input(|i| {
            i.key_pressed(egui::Key::Escape)
                || (i.modifiers.command && i.key_pressed(egui::Key::W))
        });

        if should_close {
            visible.store(false, Ordering::SeqCst);
            let _ = command_tx.send(PreferencesCommand::Cancel);
            ctx.send_viewport_cmd(ViewportCommand::Close);
            return;
        }

        egui::CentralPanel::default()
            .frame(
                Frame::NONE
                    .fill(theme.background)
                    .inner_margin(Margin::same(0)),
            )
            .show(ctx, |ui| {
                let available = ui.available_size();

                ui.horizontal(|ui| {
                    // Left sidebar for tabs - responsive width (150px min, 20% of width max, 200px cap)
                    let sidebar_width = (available.x * 0.2).max(150.0).min(200.0);

                    ui.allocate_ui(egui::vec2(sidebar_width, available.y), |ui| {
                        Self::render_sidebar(ui, shared_state, &theme);
                    });

                    // Vertical divider
                    ui.separator();

                    // Content area - fills remaining space
                    ui.vertical(|ui| {
                        let content_height = ui.available_height() - 52.0; // Reserve space for bottom bar (48px + padding)

                        // Scrollable content area
                        ScrollArea::vertical()
                            .id_salt("prefs_content")
                            .max_height(content_height)
                            .show(ui, |ui| {
                                ui.add_space(16.0);
                                ui.horizontal(|ui| {
                                    ui.add_space(16.0);
                                    ui.vertical(|ui| {
                                        Self::render_content(ui, shared_state, &theme);
                                    });
                                });
                                ui.add_space(16.0);
                            });

                        // Bottom button bar
                        ui.separator();
                        Self::render_bottom_bar(ui, shared_state, command_tx, visible, &theme, ctx);
                    });
                });
            });
    }

    fn render_sidebar(ui: &mut egui::Ui, shared_state: &Arc<PreferencesSharedState>, theme: &RuntimeTheme) {
        // Render sidebar directly in the allocated space (no SidePanel)
        Frame::NONE
            .fill(theme.surface)
            .inner_margin(Margin::symmetric(8, 12))
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 4.0;

                // Get current active tab (read-only, quick drop)
                let active_tab = *shared_state.active_tab.lock().unwrap();

                for &tab in PreferencesTab::all() {
                    let is_active = tab == active_tab;

                    // Use Button widget for proper layout
                    let button = Button::new(
                        RichText::new(tab.label())
                            .font(mono_font(13.0))
                            .color(if is_active { theme.background } else { theme.text })
                    )
                    .fill(if is_active { theme.primary } else { theme.surface })
                    .min_size(egui::vec2(ui.available_width(), 32.0))
                    .corner_radius(4.0);

                    let response = ui.add(button);

                    // Hover effect for inactive tabs
                    if !is_active && response.hovered() {
                        ui.painter().rect_filled(
                            response.rect,
                            4.0,
                            theme.surface_light,
                        );
                    }

                    // Handle click
                    if response.clicked() {
                        *shared_state.active_tab.lock().unwrap() = tab;
                    }
                }
            });
    }

    fn render_content(ui: &mut egui::Ui, shared_state: &Arc<PreferencesSharedState>, theme: &RuntimeTheme) {
        ui.style_mut().spacing.item_spacing.y = 12.0;

        let active_tab = {
            let tab = shared_state.active_tab.lock().unwrap();
            *tab
        };

        match active_tab {
            PreferencesTab::General => Self::render_general_tab(ui, shared_state, theme),
            PreferencesTab::Appearance => Self::render_appearance_tab(ui, shared_state, theme),
            PreferencesTab::Terminal => Self::render_terminal_tab(ui, shared_state, theme),
            PreferencesTab::FileTree => Self::render_filetree_tab(ui, shared_state, theme),
            PreferencesTab::Advanced => Self::render_advanced_tab(ui, shared_state, theme),
        }
    }

    fn render_general_tab(ui: &mut egui::Ui, shared_state: &Arc<PreferencesSharedState>, theme: &RuntimeTheme) {
        ui.heading(RichText::new("General Settings").font(mono_font(16.0)).color(theme.text));
        ui.add_space(8.0);

        // Font Settings Section
        ui.label(RichText::new("Font").font(mono_font(13.0)).color(theme.text));
        ui.add_space(4.0);

        // Get and update config
        let mut temp_config = shared_state.temp_config.lock().unwrap();

        egui::Grid::new("font_grid")
            .num_columns(2)
            .spacing([40.0, 8.0])
            .show(ui, |ui| {
                ui.label(RichText::new("Terminal Size").font(mono_font(12.0)).color(theme.text_dim))
                    .on_hover_text("Font size for terminal text (10-24)");
                ui.add(egui::Slider::new(&mut temp_config.font.terminal_size, 10.0..=24.0)
                    .suffix(" pt"));
                ui.end_row();

                ui.label(RichText::new("UI Size").font(mono_font(12.0)).color(theme.text_dim))
                    .on_hover_text("Font size for UI elements (8-20)");
                ui.add(egui::Slider::new(&mut temp_config.font.ui_size, 8.0..=20.0)
                    .suffix(" pt"));
                ui.end_row();
            });

        ui.add_space(16.0);
        ui.separator();
        ui.add_space(8.0);

        // Layout Settings Section
        ui.label(RichText::new("Layout").font(mono_font(13.0)).color(theme.text));
        ui.add_space(4.0);

        egui::Grid::new("layout_grid")
            .num_columns(2)
            .spacing([40.0, 8.0])
            .show(ui, |ui| {
                ui.label(RichText::new("Sidebar Width").font(mono_font(12.0)).color(theme.text_dim))
                    .on_hover_text("Width of the file tree sidebar");
                ui.add(egui::Slider::new(&mut temp_config.ui.sidebar_width, 150.0..=400.0)
                    .suffix(" px"));
                ui.end_row();

                ui.label(RichText::new("Tab Bar Height").font(mono_font(12.0)).color(theme.text_dim))
                    .on_hover_text("Height of the top tab bar");
                ui.add(egui::Slider::new(&mut temp_config.ui.tab_bar_height, 24.0..=40.0)
                    .suffix(" px"));
                ui.end_row();

                ui.label(RichText::new("Status Bar Height").font(mono_font(12.0)).color(theme.text_dim))
                    .on_hover_text("Height of the bottom status bar");
                ui.add(egui::Slider::new(&mut temp_config.ui.status_bar_height, 16.0..=32.0)
                    .suffix(" px"));
                ui.end_row();
            });

        ui.add_space(16.0);
        ui.separator();
        ui.add_space(8.0);

        // Startup Behavior Section
        ui.label(RichText::new("Startup").font(mono_font(13.0)).color(theme.text));
        ui.add_space(4.0);

        ui.checkbox(&mut temp_config.ui.show_sidebar,
            RichText::new("Show sidebar on startup").font(mono_font(12.0)).color(theme.text))
            .on_hover_text("Display the file tree sidebar when the app opens");

        ui.checkbox(&mut temp_config.ui.enable_cwd_polling,
            RichText::new("Enable directory tracking").font(mono_font(12.0)).color(theme.text))
            .on_hover_text("Automatically update file tree when terminal changes directory");
    }

    fn render_appearance_tab(ui: &mut egui::Ui, shared_state: &Arc<PreferencesSharedState>, theme: &RuntimeTheme) {
        ui.heading(RichText::new("Appearance").font(mono_font(16.0)).color(theme.text));
        ui.add_space(8.0);

        // Theme Presets Section
        ui.label(RichText::new("Theme Presets").font(mono_font(13.0)).color(theme.text));
        ui.add_space(4.0);

        let mut temp_config = shared_state.temp_config.lock().unwrap();

        ui.horizontal(|ui| {
            if ui.button(RichText::new("Dark Brown").font(mono_font(12.0)))
                .on_hover_text("Warm, earthy brown theme (default)")
                .clicked()
            {
                temp_config.theme = ThemeConfig::default();
            }

            if ui.button(RichText::new("Reset to Default").font(mono_font(12.0)))
                .on_hover_text("Reset all colors to default values")
                .clicked()
            {
                temp_config.theme = ThemeConfig::default();
            }
        });

        ui.add_space(16.0);
        ui.separator();
        ui.add_space(8.0);

        // UI Colors Section
        ui.label(RichText::new("UI Colors").font(mono_font(13.0)).color(theme.text));
        ui.add_space(4.0);

        egui::Grid::new("ui_colors_grid")
            .num_columns(2)
            .spacing([40.0, 8.0])
            .show(ui, |ui| {
                Self::color_picker_row(ui, theme, "Background", &mut temp_config.theme.background,
                    "Main window background color");
                Self::color_picker_row(ui, theme, "Surface", &mut temp_config.theme.surface,
                    "Panel and card background color");
                Self::color_picker_row(ui, theme, "Surface Light", &mut temp_config.theme.surface_light,
                    "Hover and elevated surface color");
                Self::color_picker_row(ui, theme, "Text", &mut temp_config.theme.text,
                    "Primary text color");
                Self::color_picker_row(ui, theme, "Text Dim", &mut temp_config.theme.text_dim,
                    "Secondary and dimmed text color");
                Self::color_picker_row(ui, theme, "Primary", &mut temp_config.theme.primary,
                    "Primary accent color (buttons, highlights)");
                Self::color_picker_row(ui, theme, "Secondary", &mut temp_config.theme.secondary,
                    "Secondary accent color");
                Self::color_picker_row(ui, theme, "Border", &mut temp_config.theme.border,
                    "Border and separator color");
                Self::color_picker_row(ui, theme, "Selection", &mut temp_config.theme.selection,
                    "Text selection background color");
            });
    }

    fn render_terminal_tab(ui: &mut egui::Ui, shared_state: &Arc<PreferencesSharedState>, theme: &RuntimeTheme) {
        ui.heading(RichText::new("Terminal").font(mono_font(16.0)).color(theme.text));
        ui.add_space(8.0);

        let mut temp_config = shared_state.temp_config.lock().unwrap();

        // Reset button at the top
        if ui.button(RichText::new("Reset ANSI Colors to Default").font(mono_font(12.0)))
            .on_hover_text("Restore default ANSI color palette")
            .clicked()
        {
            let default = ThemeConfig::default();
            temp_config.theme.black = default.black;
            temp_config.theme.red = default.red;
            temp_config.theme.green = default.green;
            temp_config.theme.yellow = default.yellow;
            temp_config.theme.blue = default.blue;
            temp_config.theme.magenta = default.magenta;
            temp_config.theme.cyan = default.cyan;
            temp_config.theme.white = default.white;
            temp_config.theme.bright_black = default.bright_black;
            temp_config.theme.bright_red = default.bright_red;
            temp_config.theme.bright_green = default.bright_green;
            temp_config.theme.bright_yellow = default.bright_yellow;
            temp_config.theme.bright_blue = default.bright_blue;
            temp_config.theme.bright_magenta = default.bright_magenta;
            temp_config.theme.bright_cyan = default.bright_cyan;
            temp_config.theme.bright_white = default.bright_white;
        }

        ui.add_space(8.0);
        ui.label(RichText::new("ANSI Colors (16-color palette)").font(mono_font(13.0)).color(theme.text));
        ui.add_space(4.0);

        ui.columns(2, |columns| {
            // Normal colors (left column)
            columns[0].label(RichText::new("Normal Colors").font(mono_font(12.0)).strong().color(theme.text));
            columns[0].add_space(4.0);

            egui::Grid::new("normal_colors_grid")
                .num_columns(2)
                .spacing([20.0, 6.0])
                .show(&mut columns[0], |ui| {
                    Self::color_picker_row(ui, theme, "Black", &mut temp_config.theme.black, "ANSI color 0");
                    Self::color_picker_row(ui, theme, "Red", &mut temp_config.theme.red, "ANSI color 1");
                    Self::color_picker_row(ui, theme, "Green", &mut temp_config.theme.green, "ANSI color 2");
                    Self::color_picker_row(ui, theme, "Yellow", &mut temp_config.theme.yellow, "ANSI color 3");
                    Self::color_picker_row(ui, theme, "Blue", &mut temp_config.theme.blue, "ANSI color 4");
                    Self::color_picker_row(ui, theme, "Magenta", &mut temp_config.theme.magenta, "ANSI color 5");
                    Self::color_picker_row(ui, theme, "Cyan", &mut temp_config.theme.cyan, "ANSI color 6");
                    Self::color_picker_row(ui, theme, "White", &mut temp_config.theme.white, "ANSI color 7");
                });

            // Bright colors (right column)
            columns[1].label(RichText::new("Bright Colors").font(mono_font(12.0)).strong().color(theme.text));
            columns[1].add_space(4.0);

            egui::Grid::new("bright_colors_grid")
                .num_columns(2)
                .spacing([20.0, 6.0])
                .show(&mut columns[1], |ui| {
                    Self::color_picker_row(ui, theme, "Bright Black", &mut temp_config.theme.bright_black, "ANSI color 8");
                    Self::color_picker_row(ui, theme, "Bright Red", &mut temp_config.theme.bright_red, "ANSI color 9");
                    Self::color_picker_row(ui, theme, "Bright Green", &mut temp_config.theme.bright_green, "ANSI color 10");
                    Self::color_picker_row(ui, theme, "Bright Yellow", &mut temp_config.theme.bright_yellow, "ANSI color 11");
                    Self::color_picker_row(ui, theme, "Bright Blue", &mut temp_config.theme.bright_blue, "ANSI color 12");
                    Self::color_picker_row(ui, theme, "Bright Magenta", &mut temp_config.theme.bright_magenta, "ANSI color 13");
                    Self::color_picker_row(ui, theme, "Bright Cyan", &mut temp_config.theme.bright_cyan, "ANSI color 14");
                    Self::color_picker_row(ui, theme, "Bright White", &mut temp_config.theme.bright_white, "ANSI color 15");
                });
        });

        ui.add_space(16.0);
        ui.separator();
        ui.add_space(8.0);

        // Preview section
        ui.label(RichText::new("Preview").font(mono_font(13.0)).color(theme.text));
        ui.add_space(4.0);

        Frame::NONE
            .fill(crate::config::parse_hex_color(&temp_config.theme.black))
            .stroke(Stroke::new(1.0, theme.border))
            .corner_radius(4.0)
            .inner_margin(Margin::same(8))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    let colors = [
                        &temp_config.theme.black,
                        &temp_config.theme.red,
                        &temp_config.theme.green,
                        &temp_config.theme.yellow,
                        &temp_config.theme.blue,
                        &temp_config.theme.magenta,
                        &temp_config.theme.cyan,
                        &temp_config.theme.white,
                    ];
                    for color in colors {
                        let c = crate::config::parse_hex_color(color);
                        ui.label(RichText::new("X").font(mono_font(20.0)).color(c));
                    }
                });
                ui.horizontal(|ui| {
                    let bright_colors = [
                        &temp_config.theme.bright_black,
                        &temp_config.theme.bright_red,
                        &temp_config.theme.bright_green,
                        &temp_config.theme.bright_yellow,
                        &temp_config.theme.bright_blue,
                        &temp_config.theme.bright_magenta,
                        &temp_config.theme.bright_cyan,
                        &temp_config.theme.bright_white,
                    ];
                    for color in bright_colors {
                        let c = crate::config::parse_hex_color(color);
                        ui.label(RichText::new("X").font(mono_font(20.0)).color(c));
                    }
                });
            });
    }

    fn render_filetree_tab(ui: &mut egui::Ui, shared_state: &Arc<PreferencesSharedState>, theme: &RuntimeTheme) {
        ui.heading(RichText::new("File Tree").font(mono_font(16.0)).color(theme.text));
        ui.add_space(8.0);

        let mut temp_config = shared_state.temp_config.lock().unwrap();

        // Display Settings Section
        ui.label(RichText::new("Display").font(mono_font(13.0)).color(theme.text));
        ui.add_space(4.0);

        ui.checkbox(&mut temp_config.ui.show_hidden_files,
            RichText::new("Show hidden files").font(mono_font(12.0)).color(theme.text))
            .on_hover_text("Display files and folders starting with '.'");

        ui.add_space(8.0);

        egui::Grid::new("filetree_limits_grid")
            .num_columns(2)
            .spacing([40.0, 8.0])
            .show(ui, |ui| {
                ui.label(RichText::new("Max Files").font(mono_font(12.0)).color(theme.text_dim))
                    .on_hover_text("Maximum number of files to display (100-5000)");
                ui.add(egui::Slider::new(&mut temp_config.ui.max_files, 100..=5000)
                    .logarithmic(true));
                ui.end_row();

                ui.label(RichText::new("Max Depth").font(mono_font(12.0)).color(theme.text_dim))
                    .on_hover_text("Maximum directory depth to traverse (1-20)");
                ui.add(egui::Slider::new(&mut temp_config.ui.max_depth, 1..=20));
                ui.end_row();
            });

        ui.add_space(16.0);
        ui.separator();
        ui.add_space(8.0);

        // Ignore Patterns Section
        ui.label(RichText::new("Ignore Patterns").font(mono_font(13.0)).color(theme.text));
        ui.add_space(4.0);

        ui.label(RichText::new("Files and directories to exclude (one per line)")
            .font(mono_font(11.0))
            .color(theme.text_dim));

        // Convert Vec<String> to multiline text
        let mut ignore_text = temp_config.ui.file_tree_ignore_patterns.join("\n");

        let text_edit = egui::TextEdit::multiline(&mut ignore_text)
            .font(mono_font(11.0))
            .desired_width(f32::INFINITY)
            .desired_rows(6);

        if ui.add(text_edit).changed() {
            // Convert back to Vec<String>
            temp_config.ui.file_tree_ignore_patterns = ignore_text
                .lines()
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
        }

        ui.add_space(8.0);

        // Common patterns helper buttons
        ui.horizontal(|ui| {
            ui.label(RichText::new("Quick add:").font(mono_font(11.0)).color(theme.text_dim));

            if ui.button(RichText::new(".DS_Store").font(mono_font(11.0)))
                .on_hover_text("Add macOS metadata files")
                .clicked()
            {
                if !temp_config.ui.file_tree_ignore_patterns.contains(&".DS_Store".to_string()) {
                    temp_config.ui.file_tree_ignore_patterns.push(".DS_Store".to_string());
                }
            }

            if ui.button(RichText::new("*.log").font(mono_font(11.0)))
                .on_hover_text("Add log files")
                .clicked()
            {
                if !temp_config.ui.file_tree_ignore_patterns.contains(&"*.log".to_string()) {
                    temp_config.ui.file_tree_ignore_patterns.push("*.log".to_string());
                }
            }

            if ui.button(RichText::new("build/").font(mono_font(11.0)))
                .on_hover_text("Add build directories")
                .clicked()
            {
                if !temp_config.ui.file_tree_ignore_patterns.contains(&"build".to_string()) {
                    temp_config.ui.file_tree_ignore_patterns.push("build".to_string());
                }
            }

            if ui.button(RichText::new("Reset").font(mono_font(11.0)))
                .on_hover_text("Reset to default ignore patterns")
                .clicked()
            {
                temp_config.ui.file_tree_ignore_patterns = UiConfig::default().file_tree_ignore_patterns;
            }
        });
    }

    fn render_advanced_tab(ui: &mut egui::Ui, shared_state: &Arc<PreferencesSharedState>, theme: &RuntimeTheme) {
        // We don't actually need the config here yet, but keep lock pattern consistent
        let _temp_config = shared_state.temp_config.lock().unwrap();

        ui.heading(RichText::new("Advanced").font(mono_font(16.0)).color(theme.text));
        ui.add_space(8.0);

        // Context Engine Section
        ui.label(RichText::new("Context Engine").font(mono_font(13.0)).color(theme.text));
        ui.add_space(4.0);

        // Note: ContextConfig fields (max_tokens, target_ratio, smart_context) are not currently implemented
        ui.label(RichText::new("Context management settings")
            .font(mono_font(11.0))
            .color(theme.text_dim));
        ui.add_space(4.0);
        ui.label(RichText::new("(Settings will be available in future updates)")
            .font(mono_font(10.0))
            .color(theme.text_dim));

        ui.add_space(16.0);
        ui.separator();
        ui.add_space(8.0);

        // Performance Section
        ui.label(RichText::new("Performance").font(mono_font(13.0)).color(theme.text));
        ui.add_space(4.0);

        ui.label(RichText::new("Rendering backend and acceleration settings")
            .font(mono_font(11.0))
            .color(theme.text_dim));

        ui.add_space(4.0);

        // Note: These are UI-only toggles - actual implementation would require app restart
        ui.horizontal(|ui| {
            ui.checkbox(&mut false, RichText::new("GPU acceleration").font(mono_font(12.0)).color(theme.text_dim))
                .on_hover_text("Enable GPU rendering (requires restart)");

            ui.label(RichText::new("(Not yet implemented)")
                .font(mono_font(10.0))
                .color(theme.text_dim));
        });

        ui.add_space(16.0);
        ui.separator();
        ui.add_space(8.0);

        // Logging Section
        ui.label(RichText::new("Logging").font(mono_font(13.0)).color(theme.text));
        ui.add_space(4.0);

        ui.horizontal(|ui| {
            ui.label(RichText::new("Log Level:").font(mono_font(12.0)).color(theme.text_dim))
                .on_hover_text("Minimum severity level for log messages");

            // Note: This would need to be added to Config struct
            let mut current_level = 2; // Info
            egui::ComboBox::from_id_salt("log_level")
                .selected_text(match current_level {
                    0 => "Off",
                    1 => "Error",
                    2 => "Warn",
                    3 => "Info",
                    4 => "Debug",
                    5 => "Trace",
                    _ => "Info",
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut current_level, 0, "Off");
                    ui.selectable_value(&mut current_level, 1, "Error");
                    ui.selectable_value(&mut current_level, 2, "Warn");
                    ui.selectable_value(&mut current_level, 3, "Info");
                    ui.selectable_value(&mut current_level, 4, "Debug");
                    ui.selectable_value(&mut current_level, 5, "Trace");
                });
        });

        ui.add_space(16.0);
        ui.separator();
        ui.add_space(8.0);

        // Experimental Features Section
        ui.label(RichText::new("Experimental Features").font(mono_font(13.0)).color(theme.text));
        ui.add_space(4.0);

        ui.label(RichText::new("These features are in development and may be unstable")
            .font(mono_font(11.0))
            .color(theme.text_dim));

        ui.add_space(4.0);

        ui.checkbox(&mut false, RichText::new("Enable split panes").font(mono_font(12.0)).color(theme.text_dim))
            .on_hover_text("Allow splitting terminal into multiple panes");

        ui.checkbox(&mut false, RichText::new("Enable tabs sync").font(mono_font(12.0)).color(theme.text_dim))
            .on_hover_text("Synchronize tabs across sessions");

        ui.checkbox(&mut false, RichText::new("Enable AI completions").font(mono_font(12.0)).color(theme.text_dim))
            .on_hover_text("Show AI-powered command suggestions");
    }

    /// Parse a hex color string to Color32
    fn parse_hex_to_color32(hex: &str) -> Option<egui::Color32> {
        let hex = hex.strip_prefix('#').unwrap_or(hex);
        if hex.len() != 6 {
            return None;
        }
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        Some(egui::Color32::from_rgb(r, g, b))
    }

    /// Convert Color32 to hex string
    fn color32_to_hex(color: egui::Color32) -> String {
        format!("#{:02X}{:02X}{:02X}", color.r(), color.g(), color.b())
    }

    #[allow(dead_code)]
    fn color_picker(ui: &mut egui::Ui, theme: &RuntimeTheme, label: &str, hex: &mut String) {
        ui.horizontal(|ui| {
            ui.label(RichText::new(format!("{}:", label)).font(mono_font(12.0)).color(theme.text_dim));

            // Parse current hex to Color32
            let mut color = Self::parse_hex_to_color32(hex)
                .unwrap_or(egui::Color32::from_rgb(46, 26, 22));

            // Show color picker button
            if ui.color_edit_button_srgba(&mut color).changed() {
                // Convert back to hex string
                *hex = Self::color32_to_hex(color);
            }

            // Hex input field (still editable)
            ui.add(
                egui::TextEdit::singleline(hex)
                    .desired_width(80.0)
                    .font(mono_font(11.0)),
            );
        });
    }

    fn color_picker_row(ui: &mut egui::Ui, theme: &RuntimeTheme, label: &str, hex: &mut String, tooltip: &str) {
        ui.label(RichText::new(label).font(mono_font(12.0)).color(theme.text_dim))
            .on_hover_text(tooltip);

        ui.horizontal(|ui| {
            // Parse current hex to Color32
            let mut color = Self::parse_hex_to_color32(hex)
                .unwrap_or(egui::Color32::from_rgb(46, 26, 22));

            // Show color picker button
            if ui.color_edit_button_srgba(&mut color).changed() {
                // Convert back to hex string
                *hex = Self::color32_to_hex(color);
            }

            // Hex input field (still editable)
            ui.add(
                egui::TextEdit::singleline(hex)
                    .desired_width(90.0)
                    .font(mono_font(11.0)),
            );
        });

        ui.end_row();
    }

    fn render_bottom_bar(
        ui: &mut egui::Ui,
        shared_state: &Arc<PreferencesSharedState>,
        command_tx: &Sender<PreferencesCommand>,
        visible: &Arc<AtomicBool>,
        theme: &RuntimeTheme,
        ctx: &egui::Context,
    ) {
        Frame::NONE
            .fill(theme.surface)
            .inner_margin(Margin::symmetric(16, 12))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        // Save button
                        let save_btn = Button::new(
                            RichText::new(" Save ")
                                .font(mono_font(13.0))
                                .color(theme.background),
                        )
                        .fill(theme.secondary)
                        .stroke(Stroke::NONE)
                        .corner_radius(4.0);

                        if ui.add(save_btn).clicked() {
                            let config = {
                                let temp = shared_state.temp_config.lock().unwrap();
                                temp.clone()
                            };
                            let _ = command_tx.send(PreferencesCommand::SaveAndClose(config));
                            visible.store(false, Ordering::SeqCst);
                            ctx.send_viewport_cmd(ViewportCommand::Close);
                        }

                        ui.add_space(8.0);

                        // Apply button
                        let apply_btn = Button::new(
                            RichText::new(" Apply ")
                                .font(mono_font(13.0))
                                .color(theme.background),
                        )
                        .fill(theme.primary)
                        .stroke(Stroke::NONE)
                        .corner_radius(4.0);

                        if ui.add(apply_btn).clicked() {
                            let config = {
                                let temp = shared_state.temp_config.lock().unwrap();
                                temp.clone()
                            };
                            let _ = command_tx.send(PreferencesCommand::ApplyConfig(config));
                        }

                        ui.add_space(8.0);

                        // Cancel button
                        let cancel_btn = Button::new(
                            RichText::new(" Cancel ")
                                .font(mono_font(13.0))
                                .color(theme.text),
                        )
                        .fill(theme.surface_light)
                        .stroke(Stroke::new(1.0, theme.border))
                        .corner_radius(4.0);

                        if ui.add(cancel_btn).clicked() {
                            let _ = command_tx.send(PreferencesCommand::Cancel);
                            visible.store(false, Ordering::SeqCst);
                            ctx.send_viewport_cmd(ViewportCommand::Close);
                        }

                        // Spacer to push buttons right
                        ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                            ui.label(
                                RichText::new("Changes will be applied immediately")
                                    .font(mono_font(11.0))
                                    .color(theme.text_dim),
                            );
                        });
                    });
                });
            });
    }
}
