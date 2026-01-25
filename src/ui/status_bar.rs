//! Status Bar Component
//!
//! TUI-style bottom status bar with box-drawing characters

use egui::{Frame, RichText, Ui};
use crate::config::RuntimeTheme;
use crate::theme::{tui, mono_font};

/// Status bar at the bottom of the window
pub struct StatusBar<'a> {
    pane_count: usize,
    focused_pane: usize,
    theme: &'a RuntimeTheme,
}

impl<'a> StatusBar<'a> {
    pub fn new(pane_count: usize, focused_pane: usize, theme: &'a RuntimeTheme) -> Self {
        Self {
            pane_count,
            focused_pane,
            theme,
        }
    }

    /// Show the status bar
    pub fn show(&self, ui: &mut Ui) {
        Frame::NONE
            .fill(self.theme.surface)
            .show(ui, |ui| {
                // Top border line
                let rect = ui.max_rect();
                ui.painter().line_segment(
                    [rect.left_top(), rect.right_top()],
                    egui::Stroke::new(1.0, self.theme.border),
                );

                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;

                    // App name
                    ui.label(RichText::new(" VibeTerm ")
                        .font(mono_font(11.0))
                        .color(self.theme.primary));

                    ui.label(RichText::new(tui::SEPARATOR)
                        .font(mono_font(11.0))
                        .color(self.theme.border));

                    // Pane indicator with TUI symbols
                    let pane_indicators: String = (0..self.pane_count)
                        .map(|i| {
                            if i == self.focused_pane {
                                tui::PANE_FOCUSED
                            } else {
                                tui::PANE_UNFOCUSED
                            }
                        })
                        .collect::<Vec<_>>()
                        .join(" ");

                    ui.label(RichText::new(format!("Panes: {} ", pane_indicators))
                        .font(mono_font(11.0))
                        .color(self.theme.text_dim));

                    ui.label(RichText::new(tui::SEPARATOR)
                        .font(mono_font(11.0))
                        .color(self.theme.border));

                    // Keyboard shortcuts
                    ui.label(RichText::new("^D:Split ^W:Close ^Tab:Switch ")
                        .font(mono_font(11.0))
                        .color(self.theme.text_dim));

                    // Right-aligned version
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(RichText::new(format!(" v{} ", env!("CARGO_PKG_VERSION")))
                            .font(mono_font(11.0))
                            .color(self.theme.text_dim));

                        ui.label(RichText::new(tui::SEPARATOR)
                            .font(mono_font(11.0))
                            .color(self.theme.border));
                    });
                });
            });
    }
}
