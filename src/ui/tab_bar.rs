//! Tab Bar Component
//!
//! TUI-style workspace tabs using box-drawing characters

use egui::{Button, Frame, PointerButton, RichText, Ui};
use crate::config::RuntimeTheme;
use crate::theme::{tui, mono_font};

/// Tab bar with TUI aesthetic
pub struct TabBar<'a> {
    tabs: &'a [TabInfo],
    active_tab: usize,
    theme: &'a RuntimeTheme,
}

/// Information about a tab
#[derive(Debug, Clone)]
pub struct TabInfo {
    pub name: String,
    pub is_modified: bool,
}

impl TabInfo {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            is_modified: false,
        }
    }
}

impl<'a> TabBar<'a> {
    pub fn new(tabs: &'a [TabInfo], active_tab: usize, theme: &'a RuntimeTheme) -> Self {
        Self {
            tabs,
            active_tab,
            theme,
        }
    }

    /// Show the tab bar and return user actions
    pub fn show(&self, ui: &mut Ui) -> TabBarResponse {
        let mut response = TabBarResponse::default();

        // TUI-style frame with border
        Frame::NONE
            .fill(self.theme.surface)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;

                    // Track tab rectangles for drag-and-drop
                    let mut tab_rects = Vec::new();

                    // Draw tabs
                    for (idx, tab) in self.tabs.iter().enumerate() {
                        let is_active = idx == self.active_tab;

                        // Tab number (1-9 for keyboard shortcuts)
                        let number = if idx < 9 {
                            format!("{}", idx + 1)
                        } else {
                            " ".to_string()
                        };

                        // Tab text with TUI indicators
                        let indicator = if is_active { tui::TAB_ACTIVE } else { tui::TAB_INACTIVE };
                        let modified = if tab.is_modified { tui::TAB_MODIFIED } else { "" };
                        let text = format!(" {}{} {}{} ", indicator, number, tab.name, modified);

                        let text_color = if is_active {
                            self.theme.text
                        } else {
                            self.theme.text_dim
                        };

                        let bg_color = if is_active {
                            self.theme.background
                        } else {
                            self.theme.surface
                        };

                        // Create clickable tab button
                        let tab_btn = Button::new(RichText::new(&text).font(mono_font(12.0)).color(text_color))
                            .fill(bg_color)
                            .frame(false);

                        let tab_response = ui.add(tab_btn);

                        // Store tab rectangle for drag detection
                        tab_rects.push((idx, tab_response.rect));

                        // Track hovered tab
                        if tab_response.hovered() {
                            response.tab_hovered = Some(idx);
                            if !is_active {
                                let rect = tab_response.rect;
                                ui.painter().rect_filled(rect, 0.0, self.theme.surface_light);
                            }
                        }

                        // Active tab bottom indicator
                        if is_active {
                            let rect = tab_response.rect;
                            let indicator_rect = egui::Rect::from_min_max(
                                egui::pos2(rect.left(), rect.bottom() - 2.0),
                                rect.right_bottom(),
                            );
                            ui.painter().rect_filled(indicator_rect, 0.0, self.theme.primary);
                        }

                        // Handle clicks - use clicked() for left click
                        if tab_response.clicked() {
                            response.selected_tab = Some(idx);
                        }

                        // Middle-click to close
                        if tab_response.clicked_by(PointerButton::Middle) {
                            response.closed_tab = Some(idx);
                        }

                        // Separator between tabs
                        ui.label(RichText::new(format!("{}", tui::VERTICAL)).font(mono_font(12.0)).color(self.theme.border));
                    }

                    // New tab button [+]
                    let plus_btn = Button::new(RichText::new(" + ").font(mono_font(12.0)).color(self.theme.text_dim))
                        .fill(self.theme.surface)
                        .frame(false);

                    if ui.add(plus_btn).clicked() {
                        response.new_tab_requested = true;
                    }

                    // Fill remaining space
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Right side info (optional)
                        ui.label(RichText::new(format!("{}", tui::VERTICAL)).font(mono_font(12.0)).color(self.theme.border));
                    });

                    // Store tab rectangles in response
                    response.tab_rects = tab_rects;
                });

                // Bottom border line
                let rect = ui.max_rect();
                ui.painter().line_segment(
                    [rect.left_bottom(), rect.right_bottom()],
                    egui::Stroke::new(1.0, self.theme.border),
                );
            });

        response
    }
}

/// Response from tab bar interaction
#[derive(Debug, Default)]
pub struct TabBarResponse {
    pub selected_tab: Option<usize>,
    pub closed_tab: Option<usize>,
    pub new_tab_requested: bool,
    pub tab_rects: Vec<(usize, egui::Rect)>,
    pub tab_hovered: Option<usize>,
}
