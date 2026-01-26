//! Command Palette for quick actions

use egui::{Frame, Key, RichText, ScrollArea};
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use crate::config::RuntimeTheme;
use crate::theme::mono_font;

/// A command in the palette
#[derive(Debug, Clone)]
pub struct Command {
    pub id: &'static str,
    pub label: &'static str,
    pub shortcut: Option<&'static str>,
    pub keywords: &'static [&'static str],
}

/// All available commands
pub static COMMANDS: &[Command] = &[
    Command {
        id: "new_tab",
        label: "New Tab",
        shortcut: Some("Cmd+T"),
        keywords: &["new", "tab", "create", "workspace"],
    },
    Command {
        id: "close_tab",
        label: "Close Tab",
        shortcut: Some("Cmd+W"),
        keywords: &["close", "tab", "remove", "workspace"],
    },
    Command {
        id: "split_horizontal",
        label: "Split Horizontally",
        shortcut: Some("Cmd+D"),
        keywords: &["split", "horizontal", "pane", "divide"],
    },
    Command {
        id: "split_vertical",
        label: "Split Vertically",
        shortcut: Some("Cmd+Shift+D"),
        keywords: &["split", "vertical", "pane", "divide"],
    },
    Command {
        id: "close_pane",
        label: "Close Pane",
        shortcut: Some("Cmd+Shift+W"),
        keywords: &["close", "pane", "remove"],
    },
    Command {
        id: "toggle_sidebar",
        label: "Toggle Sidebar",
        shortcut: Some("Cmd+B"),
        keywords: &["sidebar", "toggle", "hide", "show"],
    },
    Command {
        id: "settings",
        label: "Open Settings",
        shortcut: None,
        keywords: &["settings", "config", "preferences"],
    },
    Command {
        id: "next_tab",
        label: "Next Tab",
        shortcut: Some("Cmd+]"),
        keywords: &["next", "tab", "switch"],
    },
    Command {
        id: "prev_tab",
        label: "Previous Tab",
        shortcut: Some("Cmd+["),
        keywords: &["previous", "tab", "switch"],
    },
];

/// Command with match score
#[derive(Debug, Clone)]
struct CommandMatch {
    command: &'static Command,
    score: i64,
}

/// Command palette state
pub struct CommandPalette {
    visible: bool,
    query: String,
    filtered: Vec<CommandMatch>,
    selected: usize,
    matcher: SkimMatcherV2,
}

impl CommandPalette {
    pub fn new() -> Self {
        let matcher = SkimMatcherV2::default();
        let filtered = COMMANDS
            .iter()
            .map(|cmd| CommandMatch { command: cmd, score: 0 })
            .collect();

        Self {
            visible: false,
            query: String::new(),
            filtered,
            selected: 0,
            matcher,
        }
    }

    /// Toggle visibility
    pub fn toggle(&mut self) {
        self.visible = !self.visible;
        if self.visible {
            self.query.clear();
            self.update_filter();
            self.selected = 0;
        }
    }

    /// Is palette visible?
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Update filtered commands based on query
    fn update_filter(&mut self) {
        if self.query.is_empty() {
            self.filtered = COMMANDS
                .iter()
                .map(|cmd| CommandMatch { command: cmd, score: 0 })
                .collect();
        } else {
            let mut matches: Vec<CommandMatch> = COMMANDS
                .iter()
                .filter_map(|cmd| {
                    // Match against label and keywords
                    let label_score = self.matcher.fuzzy_match(&cmd.label, &self.query);
                    let keyword_score = cmd.keywords.iter()
                        .filter_map(|kw| self.matcher.fuzzy_match(kw, &self.query))
                        .max();

                    let score = label_score.or(keyword_score)?;
                    Some(CommandMatch { command: cmd, score })
                })
                .collect();

            matches.sort_by_key(|m| -m.score);
            self.filtered = matches;
        }

        // Reset selection
        self.selected = 0;
    }

    /// Show palette and return selected command ID
    pub fn show(&mut self, ctx: &egui::Context, theme: &RuntimeTheme) -> Option<&'static str> {
        if !self.visible {
            return None;
        }

        let mut executed = None;

        egui::Window::new("command_palette")
            .title_bar(false)
            .fixed_pos(egui::pos2(ctx.screen_rect().width() * 0.5 - 300.0, 100.0))
            .fixed_size(egui::vec2(600.0, 400.0))
            .frame(Frame::window(&ctx.style())
                .fill(theme.surface)
                .stroke(egui::Stroke::new(1.0, theme.border)))
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    // Search input
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("❯").font(mono_font(14.0)).color(theme.primary));

                        let text_edit = egui::TextEdit::singleline(&mut self.query)
                            .font(mono_font(14.0))
                            .desired_width(550.0)
                            .hint_text("Type to search commands...");

                        let response = ui.add(text_edit);

                        // Auto-focus on open
                        if response.changed() {
                            self.update_filter();
                        }

                        response.request_focus();
                    });

                    ui.separator();

                    // Command list
                    ScrollArea::vertical()
                        .max_height(320.0)
                        .show(ui, |ui| {
                            for (idx, cmd_match) in self.filtered.iter().enumerate() {
                                let is_selected = idx == self.selected;

                                let bg_color = if is_selected {
                                    theme.selection
                                } else {
                                    theme.surface
                                };

                                let text_color = if is_selected {
                                    theme.text
                                } else {
                                    theme.text_dim
                                };

                                let frame = Frame::NONE
                                    .fill(bg_color)
                                    .inner_margin(egui::Margin { left: 8, right: 8, top: 4, bottom: 4 });

                                frame.show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        ui.label(RichText::new(cmd_match.command.label)
                                            .font(mono_font(12.0))
                                            .color(text_color));

                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                            if let Some(shortcut) = cmd_match.command.shortcut {
                                                ui.label(RichText::new(shortcut)
                                                    .font(mono_font(10.0))
                                                    .color(theme.text_dim));
                                            }
                                        });
                                    });

                                    if ui.interact(ui.max_rect(), ui.id().with(idx), egui::Sense::click()).clicked() {
                                        executed = Some(cmd_match.command.id);
                                    }
                                });
                            }
                        });
                });

                // Keyboard navigation
                if ui.input(|i| i.key_pressed(Key::ArrowDown)) {
                    if self.selected < self.filtered.len().saturating_sub(1) {
                        self.selected += 1;
                    }
                }
                if ui.input(|i| i.key_pressed(Key::ArrowUp)) {
                    if self.selected > 0 {
                        self.selected -= 1;
                    }
                }
                if ui.input(|i| i.key_pressed(Key::Enter)) {
                    if let Some(cmd_match) = self.filtered.get(self.selected) {
                        executed = Some(cmd_match.command.id);
                    }
                }
                if ui.input(|i| i.key_pressed(Key::Escape)) {
                    self.visible = false;
                }
            });

        if executed.is_some() {
            self.visible = false;
        }

        executed
    }
}

impl Default for CommandPalette {
    fn default() -> Self {
        Self::new()
    }
}
