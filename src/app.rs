//! VibeTerm Application
//!
//! Main application state and egui integration

use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender};
use egui::{CentralPanel, Context, Event, Frame, ImeEvent, Key, SidePanel, TopBottomPanel, Widget};
use egui_term::{BackendCommand, BackendSettings, PtyEvent, TerminalBackend, TerminalView};
use crate::config::{Config, RuntimeTheme};
use crate::menu::{self, MenuAction};
use crate::theme;
use crate::ui::{FileEntry, Sidebar, StatusBar, TabBar, TabInfo};

/// Content type for a tab
#[derive(Debug)]
enum TabContent {
    /// Terminal emulator
    Terminal(TerminalInstance),
    /// File viewer
    FileViewer {
        path: PathBuf,
        content: String,
        scroll_offset: f32,
    },
}

/// Terminal instance with its backend
struct TerminalInstance {
    backend: TerminalBackend,
    id: u64,
}

impl std::fmt::Debug for TerminalInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TerminalInstance")
            .field("id", &self.id)
            .finish()
    }
}

/// A pane within a workspace (can be terminal or file viewer)
#[derive(Debug)]
struct Pane {
    content: TabContent,
    /// Relative width (0.0 - 1.0), used for split panes
    width_ratio: f32,
}

/// Workspace containing panes
struct Workspace {
    name: String,
    panes: Vec<Pane>,
    focused_pane: usize,
}

impl Workspace {
    fn new(
        name: impl Into<String>,
        id: u64,
        ctx: &Context,
        pty_sender: Sender<(u64, PtyEvent)>,
    ) -> anyhow::Result<Self> {
        let name = name.into();
        let backend = create_terminal_backend(id, ctx, pty_sender)?;

        Ok(Self {
            name,
            panes: vec![Pane {
                content: TabContent::Terminal(TerminalInstance { backend, id }),
                width_ratio: 1.0,
            }],
            focused_pane: 0,
        })
    }

    /// Add a new terminal pane (split)
    fn add_terminal_pane(&mut self, id: u64, ctx: &Context, pty_sender: Sender<(u64, PtyEvent)>) {
        if let Ok(backend) = create_terminal_backend(id, ctx, pty_sender) {
            // Redistribute widths equally
            let new_width = 1.0 / (self.panes.len() + 1) as f32;
            for pane in &mut self.panes {
                pane.width_ratio = new_width;
            }
            self.panes.push(Pane {
                content: TabContent::Terminal(TerminalInstance { backend, id }),
                width_ratio: new_width,
            });
        }
    }

    /// Add a file viewer pane
    fn add_file_pane(&mut self, path: PathBuf) {
        let content = std::fs::read_to_string(&path).unwrap_or_else(|e| format!("Error: {}", e));
        let name = path.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "Unknown".to_string());

        // Redistribute widths
        let new_width = 1.0 / (self.panes.len() + 1) as f32;
        for pane in &mut self.panes {
            pane.width_ratio = new_width;
        }

        self.panes.push(Pane {
            content: TabContent::FileViewer {
                path,
                content,
                scroll_offset: 0.0,
            },
            width_ratio: new_width,
        });

        self.focused_pane = self.panes.len() - 1;
        self.name = name;
    }
}

/// Main application state
pub struct VibeTermApp {
    /// Configuration
    config: Config,
    /// Runtime theme (parsed colors)
    theme: RuntimeTheme,
    /// All workspaces (tabs)
    workspaces: Vec<Workspace>,
    /// Currently active workspace
    active_workspace: usize,
    /// Terminal ID counter
    next_terminal_id: u64,
    /// Sidebar visibility
    sidebar_visible: bool,
    /// Sidebar entries
    sidebar_entries: Vec<FileEntry>,
    /// Selected sidebar entry
    sidebar_selected: Option<usize>,
    /// Project root path
    project_root: Option<PathBuf>,
    /// PTY event channel
    pty_sender: Sender<(u64, PtyEvent)>,
    pty_receiver: Receiver<(u64, PtyEvent)>,
    /// egui context for creating new terminals
    ctx: Context,
    /// Divider being dragged (workspace_idx, divider_idx)
    dragging_divider: Option<(usize, usize)>,
    /// Show preferences window
    show_preferences: bool,
    /// IME is currently composing (preedit active)
    ime_composing: bool,
}

impl VibeTermApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Load configuration
        let config = Config::load();
        let theme = RuntimeTheme::from(&config.theme);

        // Apply VibeTerm theme
        crate::theme::apply_theme(&cc.egui_ctx, &theme);
        crate::theme::configure_fonts(&cc.egui_ctx);

        // Create PTY event channel
        let (pty_sender, pty_receiver) = std::sync::mpsc::channel();

        // Create initial workspace
        let workspace = Workspace::new("shell", 0, &cc.egui_ctx, pty_sender.clone())
            .expect("Failed to create initial workspace");

        // Load sidebar entries from current directory
        let project_root = std::env::current_dir().ok();
        let sidebar_entries = project_root
            .as_ref()
            .map(|p| load_directory_entries(p, 0))
            .unwrap_or_default();

        Self {
            config,
            theme,
            workspaces: vec![workspace],
            active_workspace: 0,
            next_terminal_id: 1,
            sidebar_visible: true,
            sidebar_entries,
            sidebar_selected: None,
            project_root,
            pty_sender,
            pty_receiver,
            ctx: cc.egui_ctx.clone(),
            dragging_divider: None,
            show_preferences: false,
            ime_composing: false,
        }
    }

    /// Get current workspace
    fn current_workspace(&self) -> &Workspace {
        &self.workspaces[self.active_workspace]
    }

    /// Get current workspace mutably
    fn current_workspace_mut(&mut self) -> &mut Workspace {
        &mut self.workspaces[self.active_workspace]
    }

    /// Get tab info for UI
    fn get_tabs(&self) -> Vec<TabInfo> {
        self.workspaces
            .iter()
            .map(|ws| TabInfo::new(&ws.name))
            .collect()
    }

    /// Create a new workspace/tab with terminal
    fn create_new_tab(&mut self) {
        let id = self.next_terminal_id;
        self.next_terminal_id += 1;

        let name = format!("shell-{}", self.workspaces.len() + 1);
        if let Ok(workspace) = Workspace::new(name, id, &self.ctx, self.pty_sender.clone()) {
            self.workspaces.push(workspace);
            self.active_workspace = self.workspaces.len() - 1;
        }
    }

    /// Create a new workspace/tab with file
    fn create_file_tab(&mut self, path: PathBuf) {
        let name = path.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "File".to_string());

        let content = std::fs::read_to_string(&path).unwrap_or_else(|e| format!("Error: {}", e));

        // Create a new workspace with a file viewer
        let workspace = Workspace {
            name,
            panes: vec![Pane {
                content: TabContent::FileViewer {
                    path,
                    content,
                    scroll_offset: 0.0,
                },
                width_ratio: 1.0,
            }],
            focused_pane: 0,
        };

        self.workspaces.push(workspace);
        self.active_workspace = self.workspaces.len() - 1;
    }

    /// Close a tab
    fn close_tab(&mut self, index: usize) {
        if self.workspaces.len() > 1 {
            self.workspaces.remove(index);
            if self.active_workspace >= self.workspaces.len() {
                self.active_workspace = self.workspaces.len() - 1;
            }
        }
    }

    /// Move tab from one position to another
    fn move_tab(&mut self, from: usize, to: usize) {
        if from != to && from < self.workspaces.len() && to < self.workspaces.len() {
            let workspace = self.workspaces.remove(from);
            self.workspaces.insert(to, workspace);
            if self.active_workspace == from {
                self.active_workspace = to;
            } else if from < self.active_workspace && to >= self.active_workspace {
                self.active_workspace -= 1;
            } else if from > self.active_workspace && to <= self.active_workspace {
                self.active_workspace += 1;
            }
        }
    }

    /// Split current pane (add new terminal)
    fn split_pane(&mut self) {
        let id = self.next_terminal_id;
        self.next_terminal_id += 1;

        // Clone before mutable borrow to satisfy borrow checker
        let ctx = self.ctx.clone();
        let pty_sender = self.pty_sender.clone();
        self.current_workspace_mut().add_terminal_pane(id, &ctx, pty_sender);
    }

    /// Close current pane
    fn close_current_pane(&mut self) {
        let ws = self.current_workspace_mut();
        if ws.panes.len() > 1 {
            ws.panes.remove(ws.focused_pane);
            // Redistribute widths
            let new_width = 1.0 / ws.panes.len() as f32;
            for pane in &mut ws.panes {
                pane.width_ratio = new_width;
            }
            if ws.focused_pane >= ws.panes.len() {
                ws.focused_pane = ws.panes.len() - 1;
            }
        } else if self.workspaces.len() > 1 {
            self.close_tab(self.active_workspace);
        }
    }

    /// Handle keyboard shortcuts
    fn handle_shortcuts(&mut self, ctx: &Context) {
        let modifiers = ctx.input(|i| i.modifiers);

        ctx.input(|i| {
            // Cmd+T: New tab
            if i.key_pressed(Key::T) && modifiers.command {
                self.create_new_tab();
            }

            // Cmd+W: Close pane or tab
            if i.key_pressed(Key::W) && modifiers.command {
                self.close_current_pane();
            }

            // Cmd+D: Split pane
            if i.key_pressed(Key::D) && modifiers.command && !modifiers.shift {
                self.split_pane();
            }

            // Cmd+B: Toggle sidebar
            if i.key_pressed(Key::B) && modifiers.command {
                self.sidebar_visible = !self.sidebar_visible;
            }

            // Cmd+,: Preferences
            if i.key_pressed(Key::Comma) && modifiers.command {
                self.show_preferences = true;
            }

            // Cmd+1-9: Switch tabs
            for n in 1..=9 {
                let key = match n {
                    1 => Key::Num1,
                    2 => Key::Num2,
                    3 => Key::Num3,
                    4 => Key::Num4,
                    5 => Key::Num5,
                    6 => Key::Num6,
                    7 => Key::Num7,
                    8 => Key::Num8,
                    9 => Key::Num9,
                    _ => continue,
                };
                if i.key_pressed(key) && modifiers.command {
                    if n - 1 < self.workspaces.len() {
                        self.active_workspace = n - 1;
                    }
                }
            }

            // Ctrl+Tab: Next pane
            if i.key_pressed(Key::Tab) && modifiers.ctrl && !modifiers.shift {
                let ws = &mut self.workspaces[self.active_workspace];
                ws.focused_pane = (ws.focused_pane + 1) % ws.panes.len();
            }

            // Ctrl+Shift+Tab: Previous pane
            if i.key_pressed(Key::Tab) && modifiers.ctrl && modifiers.shift {
                let ws = &mut self.workspaces[self.active_workspace];
                if ws.focused_pane == 0 {
                    ws.focused_pane = ws.panes.len() - 1;
                } else {
                    ws.focused_pane -= 1;
                }
            }
        });
    }

    /// Handle IME (Input Method Editor) events for Korean/Japanese/Chinese input
    fn handle_ime_events(&mut self, ctx: &Context) {
        let events = ctx.input(|i| i.events.clone());

        // Track if we're in IME composition mode
        let mut in_ime_composition = false;

        for event in &events {
            match event {
                Event::Ime(ime_event) => {
                    match ime_event {
                        ImeEvent::Enabled => {
                            in_ime_composition = true;
                            self.ime_composing = true;
                        }
                        ImeEvent::Preedit(text) => {
                            in_ime_composition = !text.is_empty();
                            self.ime_composing = in_ime_composition;
                        }
                        ImeEvent::Commit(text) => {
                            log::info!("IME Commit: '{}'", text);
                            // Send committed text to terminal
                            if let Some(ws) = self.workspaces.get_mut(self.active_workspace) {
                                if let Some(pane) = ws.panes.get_mut(ws.focused_pane) {
                                    if let TabContent::Terminal(terminal) = &mut pane.content {
                                        terminal.backend.process_command(
                                            BackendCommand::Write(text.clone().into_bytes())
                                        );
                                    }
                                }
                            }
                            self.ime_composing = false;
                        }
                        ImeEvent::Disabled => {
                            self.ime_composing = false;
                        }
                    }
                }
                _ => {}
            }
        }

        // If IME is composing, filter out Text events to prevent double input
        if self.ime_composing {
            ctx.input_mut(|i| {
                i.events.retain(|e| !matches!(e, Event::Text(_)));
            });
        }
    }

    /// Handle menu bar events
    fn handle_menu_events(&mut self) {
        while let Some(action) = menu::poll_menu_event() {
            match action {
                MenuAction::NewTab => self.create_new_tab(),
                MenuAction::NewWindow => {
                    // TODO: Open new window
                    log::info!("New window requested");
                }
                MenuAction::CloseTab => self.close_current_pane(),
                MenuAction::CloseWindow => {
                    // Handled by system
                }
                MenuAction::SplitHorizontal | MenuAction::SplitVertical => self.split_pane(),
                MenuAction::ToggleSidebar => self.sidebar_visible = !self.sidebar_visible,
                MenuAction::Preferences => self.show_preferences = true,
                MenuAction::About => {
                    log::info!("About VibeTerm v{}", env!("CARGO_PKG_VERSION"));
                }
                MenuAction::Quit => {
                    // Handled by system
                }
            }
        }
    }

    /// Process PTY events
    fn process_pty_events(&mut self) {
        while let Ok((terminal_id, event)) = self.pty_receiver.try_recv() {
            match event {
                PtyEvent::Exit => {
                    log::info!("Terminal {} exited", terminal_id);
                    // Find and remove the terminal
                    for workspace in &mut self.workspaces {
                        if let Some(idx) = workspace.panes.iter().position(|p| {
                            if let TabContent::Terminal(t) = &p.content {
                                t.id == terminal_id
                            } else {
                                false
                            }
                        }) {
                            if workspace.panes.len() > 1 {
                                workspace.panes.remove(idx);
                                let new_width = 1.0 / workspace.panes.len() as f32;
                                for pane in &mut workspace.panes {
                                    pane.width_ratio = new_width;
                                }
                                if workspace.focused_pane >= workspace.panes.len() {
                                    workspace.focused_pane = workspace.panes.len() - 1;
                                }
                            }
                            break;
                        }
                    }
                }
                _ => {}
            }
        }
    }

    /// Toggle directory expansion
    fn toggle_directory(&mut self, idx: usize) {
        if let Some(entry) = self.sidebar_entries.get_mut(idx) {
            if entry.is_dir {
                entry.is_expanded = !entry.is_expanded;

                if entry.is_expanded {
                    let children = load_directory_entries(&entry.path, entry.depth + 1);
                    let insert_pos = idx + 1;
                    for (i, child) in children.into_iter().enumerate() {
                        self.sidebar_entries.insert(insert_pos + i, child);
                    }
                } else {
                    let depth = entry.depth;
                    let mut remove_count = 0;
                    for i in (idx + 1)..self.sidebar_entries.len() {
                        if self.sidebar_entries[i].depth > depth {
                            remove_count += 1;
                        } else {
                            break;
                        }
                    }
                    for _ in 0..remove_count {
                        self.sidebar_entries.remove(idx + 1);
                    }
                }
            }
        }
    }

    /// Show preferences window
    fn show_preferences_window(&mut self, ctx: &Context) {
        egui::Window::new("Preferences")
            .open(&mut self.show_preferences)
            .resizable(true)
            .default_size([500.0, 400.0])
            .show(ctx, |ui| {
                ui.heading("Theme Colors");
                ui.separator();

                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Background:");
                        ui.text_edit_singleline(&mut self.config.theme.background);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Text:");
                        ui.text_edit_singleline(&mut self.config.theme.text);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Primary:");
                        ui.text_edit_singleline(&mut self.config.theme.primary);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Border:");
                        ui.text_edit_singleline(&mut self.config.theme.border);
                    });

                    ui.separator();
                    ui.heading("Terminal Colors");

                    ui.horizontal(|ui| {
                        ui.label("Black:");
                        ui.text_edit_singleline(&mut self.config.theme.black);
                        ui.label("Red:");
                        ui.text_edit_singleline(&mut self.config.theme.red);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Green:");
                        ui.text_edit_singleline(&mut self.config.theme.green);
                        ui.label("Yellow:");
                        ui.text_edit_singleline(&mut self.config.theme.yellow);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Blue:");
                        ui.text_edit_singleline(&mut self.config.theme.blue);
                        ui.label("Magenta:");
                        ui.text_edit_singleline(&mut self.config.theme.magenta);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Cyan:");
                        ui.text_edit_singleline(&mut self.config.theme.cyan);
                        ui.label("White:");
                        ui.text_edit_singleline(&mut self.config.theme.white);
                    });

                    ui.separator();

                    if ui.button("Save & Apply").clicked() {
                        // Update runtime theme
                        self.theme = RuntimeTheme::from(&self.config.theme);
                        // Apply to egui
                        crate::theme::apply_theme(&self.ctx, &self.theme);
                        // Save to file
                        if let Err(e) = self.config.save() {
                            log::error!("Failed to save config: {}", e);
                        }
                    }

                    ui.label("Config file: ~/.config/vibeterm/config.toml");
                });
            });
    }

    /// Render panes with proper divider drag support
    fn render_panes(&mut self, ui: &mut egui::Ui) {
        let terminal_theme = theme::get_terminal_theme(&self.config);
        let pane_count = self.current_workspace().panes.len();
        let focused_idx = self.current_workspace().focused_pane;

        if pane_count == 0 {
            return;
        }

        let available_width = ui.available_width();
        let available_height = ui.available_height();
        let divider_width = 4.0; // Make divider wider for easier dragging
        let total_divider_width = (pane_count - 1) as f32 * divider_width;
        let content_width = available_width - total_divider_width;

        // Check for click to focus pane BEFORE rendering (check pointer position)
        let clicked_primary = ui.input(|i| i.pointer.button_clicked(egui::PointerButton::Primary));
        let pointer_pos = ui.input(|i| i.pointer.latest_pos());

        // Handle divider dragging
        if let Some((_, divider_idx)) = self.dragging_divider {
            if ui.input(|i| i.pointer.any_released()) {
                self.dragging_divider = None;
            } else if let Some(pos) = pointer_pos {
                // Calculate new ratios based on drag position
                let start_x = ui.min_rect().left();
                let relative_x = (pos.x - start_x).max(0.0);

                // Minimum pane width as ratio (prevent collapse)
                let min_ratio = 0.1;

                // Calculate target ratio for left panes (0..=divider_idx)
                let left_panes = divider_idx + 1;
                let right_panes = pane_count - left_panes;

                // Target ratio based on mouse position
                let target_left_total = (relative_x / content_width).clamp(
                    min_ratio * left_panes as f32,  // minimum for left panes
                    1.0 - min_ratio * right_panes as f32  // leave room for right panes
                );

                let ws = &mut self.workspaces[self.active_workspace];

                // Distribute left ratio evenly among left panes
                let left_each = (target_left_total / left_panes as f32).max(min_ratio);
                // Distribute remaining ratio evenly among right panes
                let right_total = (1.0 - left_each * left_panes as f32).max(min_ratio * right_panes as f32);
                let right_each = (right_total / right_panes as f32).max(min_ratio);

                for (i, pane) in ws.panes.iter_mut().enumerate() {
                    if i <= divider_idx {
                        pane.width_ratio = left_each;
                    } else {
                        pane.width_ratio = right_each;
                    }
                }
            }
        }

        // Calculate pane rects first to determine which pane was clicked
        let start_x = ui.min_rect().left();
        let mut pane_rects: Vec<egui::Rect> = Vec::new();
        let mut current_x = start_x;
        let min_pane_width = 50.0; // Minimum pane width in pixels
        for idx in 0..pane_count {
            let pane_width = (self.workspaces[self.active_workspace].panes[idx].width_ratio * content_width).max(min_pane_width);
            let rect = egui::Rect::from_min_size(
                egui::pos2(current_x, ui.min_rect().top()),
                egui::vec2(pane_width, available_height.max(1.0)),
            );
            pane_rects.push(rect);
            current_x += pane_width + divider_width;
        }

        // Check which pane was clicked and switch focus
        if clicked_primary {
            if let Some(pos) = pointer_pos {
                for (idx, rect) in pane_rects.iter().enumerate() {
                    if rect.contains(pos) && idx != focused_idx {
                        self.workspaces[self.active_workspace].focused_pane = idx;
                        break;
                    }
                }
            }
        }

        // Update focused_idx after potential change
        let focused_idx = self.current_workspace().focused_pane;

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;

            for idx in 0..pane_count {
                let is_focused = idx == focused_idx;
                let pane_width = (self.workspaces[self.active_workspace].panes[idx].width_ratio * content_width).max(min_pane_width);

                // Pane area - use NONE sense since we handle clicks manually above
                let (rect, _response) = ui.allocate_exact_size(
                    egui::vec2(pane_width, available_height.max(1.0)),
                    egui::Sense::hover(),
                );

                // Focus border
                if is_focused {
                    ui.painter().rect_stroke(
                        rect,
                        0.0,
                        egui::Stroke::new(2.0, self.theme.primary),
                        egui::StrokeKind::Inside,
                    );
                } else {
                    ui.painter().rect_stroke(
                        rect,
                        0.0,
                        egui::Stroke::new(1.0, self.theme.border),
                        egui::StrokeKind::Inside,
                    );
                }

                // Render pane content
                let inner_rect = rect.shrink(2.0);
                ui.allocate_new_ui(
                    egui::UiBuilder::new().max_rect(inner_rect),
                    |ui| {
                        match &mut self.workspaces[self.active_workspace].panes[idx].content {
                            TabContent::Terminal(terminal) => {
                                TerminalView::new(ui, &mut terminal.backend)
                                    .set_theme(terminal_theme.clone())
                                    .set_focus(is_focused)
                                    .set_size(inner_rect.size())
                                    .ui(ui);
                            }
                            TabContent::FileViewer { content, .. } => {
                                // Simple file viewer
                                ui.painter().rect_filled(inner_rect, 0.0, self.theme.background);
                                egui::ScrollArea::vertical()
                                    .id_salt(format!("file_scroll_{}", idx))
                                    .show(ui, |ui| {
                                        ui.add(egui::Label::new(
                                            egui::RichText::new(content.as_str())
                                                .font(theme::mono_font(12.0))
                                                .color(self.theme.text)
                                        ).wrap());
                                    });
                            }
                        }
                    },
                );

                // Divider (except for last pane)
                if idx < pane_count - 1 {
                    let (divider_rect, divider_response) = ui.allocate_exact_size(
                        egui::vec2(divider_width, available_height),
                        egui::Sense::click_and_drag(),
                    );

                    // Start dragging
                    if divider_response.drag_started() {
                        self.dragging_divider = Some((self.active_workspace, idx));
                    }

                    // Visual feedback
                    let divider_color = if divider_response.dragged() || divider_response.hovered() {
                        self.theme.primary
                    } else {
                        self.theme.border
                    };
                    ui.painter().rect_filled(divider_rect, 0.0, divider_color);

                    // Cursor change
                    if divider_response.hovered() || divider_response.dragged() {
                        ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeHorizontal);
                    }
                }
            }
        });
    }
}

impl eframe::App for VibeTermApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // Enable IME for Korean/Japanese/Chinese input
        ctx.send_viewport_cmd(egui::ViewportCommand::IMEAllowed(true));

        // Handle keyboard shortcuts
        self.handle_shortcuts(ctx);

        // Handle IME events (Korean/Japanese/Chinese input)
        self.handle_ime_events(ctx);

        // Handle menu events
        self.handle_menu_events();

        // Process PTY events
        self.process_pty_events();

        // Show preferences window if open
        if self.show_preferences {
            self.show_preferences_window(ctx);
        }

        // Request continuous repaint for terminal updates
        ctx.request_repaint();

        // Tab bar (top)
        TopBottomPanel::top("tab_bar")
            .exact_height(theme::TAB_BAR_HEIGHT)
            .frame(Frame::NONE)
            .show(ctx, |ui| {
                let tabs = self.get_tabs();
                let tab_bar = TabBar::new(&tabs, self.active_workspace, &self.theme);
                let response = tab_bar.show(ui);

                if let Some(idx) = response.selected_tab {
                    self.active_workspace = idx;
                    // Reset focused pane to first pane when switching tabs
                    self.workspaces[idx].focused_pane = 0;
                }
                if let Some(idx) = response.closed_tab {
                    self.close_tab(idx);
                }
                if response.new_tab_requested {
                    self.create_new_tab();
                }
            });

        // Status bar (bottom)
        TopBottomPanel::bottom("status_bar")
            .exact_height(theme::STATUS_BAR_HEIGHT)
            .frame(Frame::NONE)
            .show(ctx, |ui| {
                let pane_count = self.current_workspace().panes.len();
                let focused = self.current_workspace().focused_pane;
                StatusBar::new(pane_count, focused, &self.theme).show(ui);
            });

        // Sidebar (left)
        if self.sidebar_visible {
            SidePanel::left("sidebar")
                .exact_width(self.config.ui.sidebar_width)
                .frame(Frame::NONE)
                .resizable(true)
                .show(ctx, |ui| {
                    let root_name = self
                        .project_root
                        .as_ref()
                        .and_then(|p| p.file_name())
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or_else(|| "Project".to_string());

                    let sidebar = Sidebar::new(
                        &self.sidebar_entries,
                        self.sidebar_selected,
                        &root_name,
                        &self.theme,
                    );
                    let response = sidebar.show(ui);

                    if let Some(idx) = response.selected {
                        self.sidebar_selected = Some(idx);
                    }
                    if let Some(idx) = response.toggled_dir {
                        self.toggle_directory(idx);
                    }
                    // Double-click file opens in new tab
                    if let Some(idx) = response.opened_file {
                        if let Some(entry) = self.sidebar_entries.get(idx) {
                            if !entry.is_dir {
                                self.create_file_tab(entry.path.clone());
                            }
                        }
                    }
                });
        }

        // Main content area (center)
        CentralPanel::default()
            .frame(Frame::NONE.fill(self.theme.background))
            .show(ctx, |ui| {
                self.render_panes(ui);
            });
    }
}

/// Create a new terminal backend
fn create_terminal_backend(
    id: u64,
    ctx: &Context,
    pty_sender: Sender<(u64, PtyEvent)>,
) -> anyhow::Result<TerminalBackend> {
    let shell = std::env::var("SHELL").unwrap_or_else(|_| {
        if cfg!(target_os = "windows") {
            "cmd.exe".to_string()
        } else {
            "/bin/bash".to_string()
        }
    });

    let settings = BackendSettings {
        shell,
        args: vec![],
        working_directory: std::env::current_dir().ok(),
    };

    let backend = TerminalBackend::new(id, ctx.clone(), pty_sender, settings)?;
    Ok(backend)
}

/// Load directory entries for sidebar
fn load_directory_entries(path: &PathBuf, depth: usize) -> Vec<FileEntry> {
    let mut entries = Vec::new();

    if let Ok(read_dir) = std::fs::read_dir(path) {
        let mut items: Vec<_> = read_dir.filter_map(|e| e.ok()).collect();

        // Sort: directories first, then alphabetically
        items.sort_by(|a, b| {
            let a_is_dir = a.file_type().map(|t| t.is_dir()).unwrap_or(false);
            let b_is_dir = b.file_type().map(|t| t.is_dir()).unwrap_or(false);

            match (a_is_dir, b_is_dir) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.file_name().cmp(&b.file_name()),
            }
        });

        let total = items.len();
        for (i, item) in items.into_iter().enumerate() {
            let name = item.file_name().to_string_lossy().to_string();

            // Skip hidden files
            if name.starts_with('.') {
                continue;
            }

            let is_dir = item.file_type().map(|t| t.is_dir()).unwrap_or(false);
            let is_last = i == total - 1;

            let mut entry = FileEntry::new(name, item.path(), is_dir, depth);
            entry.is_last = is_last;
            entries.push(entry);
        }
    }

    entries
}
