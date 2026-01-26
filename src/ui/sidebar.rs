//! Sidebar Component
//!
//! TUI-style file tree browser using box-drawing characters

use egui::{Button, Frame, RichText, ScrollArea, Sense, Ui};
use crate::config::RuntimeTheme;
use crate::layout::PaneId;
use crate::theme::{tui, mono_font};
use std::path::PathBuf;
use crate::context::{FileGitStatus, RepoStatus};

/// File/directory entry for sidebar
#[derive(Debug, Clone)]
pub struct FileEntry {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub is_expanded: bool,
    pub depth: usize,
    pub is_last: bool,  // Is this the last item at this level?
    /// Git status for this file (v0.7.0)
    pub git_status: Option<FileGitStatus>,
    /// Whether this file is pinned (v0.7.0)
    pub is_pinned: bool,
}

impl FileEntry {
    pub fn new(name: impl Into<String>, path: PathBuf, is_dir: bool, depth: usize) -> Self {
        Self {
            name: name.into(),
            path,
            is_dir,
            is_expanded: false,
            depth,
            is_last: false,
            git_status: None,
            is_pinned: false,
        }
    }
}

/// Sidebar file browser
pub struct Sidebar<'a> {
    entries: &'a [FileEntry],
    selected_index: Option<usize>,
    root_name: &'a str,
    theme: &'a RuntimeTheme,
    /// Pane info: (pane_id, current_dir) for all terminal panes
    panes: &'a [(PaneId, PathBuf)],
    /// Currently focused pane
    focused_pane: Option<PaneId>,
    /// Is directory loading in progress?
    loading: bool,
    /// Repository status (v0.7.0)
    repo_status: Option<&'a RepoStatus>,
    /// Enable git status display
    show_git_status: bool,
}

impl<'a> Sidebar<'a> {
    pub fn new(
        entries: &'a [FileEntry],
        selected_index: Option<usize>,
        root_name: &'a str,
        theme: &'a RuntimeTheme,
        panes: &'a [(PaneId, PathBuf)],
        focused_pane: Option<PaneId>,
        loading: bool,
        repo_status: Option<&'a RepoStatus>,
        show_git_status: bool,
    ) -> Self {
        Self {
            entries,
            selected_index,
            root_name,
            theme,
            panes,
            focused_pane,
            loading,
            repo_status,
            show_git_status,
        }
    }

    /// Show the sidebar and return user actions
    pub fn show(&self, ui: &mut Ui) -> SidebarResponse {
        let mut response = SidebarResponse::default();

        Frame::NONE
            .fill(self.theme.surface)
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    // Header with pane indicators
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(format!("{}{}",
                            tui::TOP_LEFT,
                            tui::HORIZONTAL.to_string().repeat(2),
                        )).font(mono_font(12.0)).color(self.theme.border));

                        // Pane mini-tabs
                        for (pane_id, _pane_dir) in self.panes {
                            let is_focused = self.focused_pane == Some(*pane_id);
                            let pane_label = format!(" {} ", pane_id.0);

                            let text_color = if is_focused {
                                self.theme.primary
                            } else {
                                self.theme.text_dim
                            };

                            let btn = Button::new(
                                RichText::new(&pane_label)
                                    .font(mono_font(10.0))
                                    .color(text_color)
                            )
                            .fill(self.theme.surface)
                            .frame(false);

                            if ui.add(btn).clicked() {
                                response.pane_clicked = Some(*pane_id);
                            }
                        }

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(RichText::new(format!(" {}",
                                tui::TOP_RIGHT
                            )).font(mono_font(12.0)).color(self.theme.border));
                        });
                    });

                    // Project root name below pane tabs with collapse/expand buttons
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(" ").font(mono_font(11.0)));
                        ui.label(RichText::new(self.root_name)
                            .font(mono_font(11.0))
                            .color(self.theme.text));

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            // Collapse all button
                            if ui.small_button("⊟")
                                .on_hover_text("Collapse All (Cmd+Shift+C)")
                                .clicked()
                            {
                                response.collapse_all = true;
                            }

                            // Expand all button
                            if ui.small_button("⊞")
                                .on_hover_text("Expand All (Cmd+Shift+E)")
                                .clicked()
                            {
                                response.expand_all = true;
                            }
                        });
                    });

                    // Separator line
                    ui.label(RichText::new(format!("{}{}",
                        tui::T_RIGHT,
                        tui::HORIZONTAL.to_string().repeat(40)
                    )).font(mono_font(12.0)).color(self.theme.border));

                    // Scrollable file list
                    ScrollArea::vertical()
                        .id_salt("sidebar_files")
                        .show(ui, |ui| {
                            ui.vertical(|ui| {
                                // Show loading indicator
                                if self.loading {
                                    ui.horizontal(|ui| {
                                        ui.label(RichText::new(" 🔄 Loading...")
                                            .font(mono_font(11.0))
                                            .color(self.theme.text_dim));
                                    });
                                    return;
                                }

                                for (idx, entry) in self.entries.iter().enumerate() {
                                    let is_selected = self.selected_index == Some(idx);

                                    // Build tree prefix
                                    let prefix = self.build_tree_prefix(entry);

                                    // Git status indicator (v0.7.0)
                                    let git_indicator = if self.show_git_status {
                                        entry.git_status.map(|s| s.indicator()).unwrap_or(" ")
                                    } else {
                                        " "
                                    };

                                    // Pin indicator (v0.7.0)
                                    let pin_indicator = if entry.is_pinned {
                                        "📌"
                                    } else {
                                        ""
                                    };

                                    // Icon based on type
                                    let icon = if entry.is_dir {
                                        if entry.is_expanded {
                                            tui::FOLDER_OPEN
                                        } else {
                                            tui::FOLDER_CLOSED
                                        }
                                    } else {
                                        tui::FILE
                                    };

                                    // Full line text with git/pin indicators
                                    let text = format!("{}{} {}{}{}",
                                        prefix,
                                        git_indicator,
                                        pin_indicator,
                                        icon,
                                        entry.name
                                    );

                                    let text_color = if is_selected {
                                        self.theme.text
                                    } else {
                                        self.theme.text_dim
                                    };

                                    let bg_color = if is_selected {
                                        self.theme.selection
                                    } else {
                                        self.theme.surface
                                    };

                                    // Clickable row
                                    let btn = Button::new(
                                        RichText::new(&text)
                                            .font(mono_font(11.0))
                                            .color(text_color)
                                    )
                                    .fill(bg_color)
                                    .frame(false)
                                    .sense(Sense::click());

                                    let btn_response = ui.add(btn);

                                    // Hover highlight
                                    if btn_response.hovered() && !is_selected {
                                        let rect = btn_response.rect;
                                        ui.painter().rect_filled(rect, 0.0, self.theme.surface_light);
                                    }

                                    // Handle click
                                    if btn_response.clicked() {
                                        if entry.is_dir {
                                            response.toggled_dir = Some(idx);
                                        }
                                        response.selected = Some(idx);
                                    }

                                    // Handle double-click
                                    if btn_response.double_clicked() && !entry.is_dir {
                                        response.opened_file = Some(idx);
                                    }
                                }
                            });
                        });

                    // Bottom border
                    ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                        ui.label(RichText::new(format!("{}{}",
                            tui::BOTTOM_LEFT,
                            tui::HORIZONTAL.to_string().repeat(40)
                        )).font(mono_font(12.0)).color(self.theme.border));
                    });
                });

                // Right border
                let rect = ui.max_rect();
                ui.painter().line_segment(
                    [rect.right_top(), rect.right_bottom()],
                    egui::Stroke::new(1.0, self.theme.border),
                );
            });

        response
    }

    /// Build tree-style prefix for entry
    fn build_tree_prefix(&self, entry: &FileEntry) -> String {
        if entry.depth == 0 {
            return String::new();
        }

        let mut prefix = String::new();
        for _ in 0..entry.depth.saturating_sub(1) {
            prefix.push_str(tui::TREE_PIPE);
        }

        if entry.is_last {
            prefix.push_str(tui::TREE_LAST);
        } else {
            prefix.push_str(tui::TREE_BRANCH);
        }

        prefix
    }

    /// Get color for git status indicator
    fn get_git_status_color(&self, status: FileGitStatus) -> egui::Color32 {
        match status {
            FileGitStatus::Clean => self.theme.text_dim,
            FileGitStatus::Modified | FileGitStatus::StagedModified => self.theme.yellow,
            FileGitStatus::Staged => self.theme.green,
            FileGitStatus::Untracked => self.theme.secondary,
            FileGitStatus::Deleted => self.theme.red,
            FileGitStatus::Renamed => self.theme.cyan,
            FileGitStatus::Conflicted => self.theme.red,
            FileGitStatus::Ignored => self.theme.text_dim,
        }
    }
}

/// Response from sidebar interaction
#[derive(Debug, Default)]
pub struct SidebarResponse {
    /// Item was selected (single click)
    pub selected: Option<usize>,
    /// File was opened (double click)
    pub opened_file: Option<usize>,
    /// Directory expand/collapse toggled
    pub toggled_dir: Option<usize>,
    /// Pane mini-tab was clicked (focus that pane)
    pub pane_clicked: Option<PaneId>,
    /// File pin toggled (v0.7.0)
    pub toggle_pin: Option<usize>,
    /// Collapse all directories requested
    pub collapse_all: bool,
    /// Expand all directories requested
    pub expand_all: bool,
}
