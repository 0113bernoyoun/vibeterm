//! VibeTerm Application
//!
//! Main application state and egui integration

use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender};
use egui::{CentralPanel, Context, Event, Frame, ImeEvent, Key, SidePanel, TopBottomPanel, Widget};
use egui_term::{BackendCommand, BackendSettings, PtyEvent, TerminalBackend, TerminalView};
use crate::config::{Config, RuntimeTheme};
use crate::layout::{LayoutNode, PaneId, SplitDirection, ComputedLayout, DIVIDER_WIDTH, DEFAULT_SPLIT_RATIO};
use crate::menu::{self, MenuAction};
use crate::theme;
use crate::ui::{FileEntry, Sidebar, StatusBar, TabBar, TabInfo};

/// State for pane drag-and-drop repositioning
#[derive(Debug, Clone)]
pub struct PaneDragState {
    /// The pane being dragged
    pub source_pane_id: PaneId,
    /// Cursor position at drag start
    pub start_pos: egui::Pos2,
    /// Current cursor position
    pub current_pos: egui::Pos2,
    /// Has drag exceeded 8px threshold?
    pub drag_active: bool,
}

/// Where a pane can be dropped
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DropZone {
    /// Drop at top edge (creates vertical split, new pane above)
    Top(PaneId),
    /// Drop at bottom edge (creates vertical split, new pane below)
    Bottom(PaneId),
    /// Drop at left edge (creates horizontal split, new pane left)
    Left(PaneId),
    /// Drop at right edge (creates horizontal split, new pane right)
    Right(PaneId),
}

/// Drop zone with rendering info
#[derive(Debug)]
pub struct DropZoneInfo {
    /// The zone type
    pub zone: DropZone,
    /// Hit-test rectangle (25% of edge)
    pub rect: egui::Rect,
    /// Visual highlight rectangle (50% preview)
    pub highlight_rect: egui::Rect,
}

/// Content type for a tab
#[derive(Debug)]
pub enum TabContent {
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

/// Workspace containing panes in a binary split tree
struct Workspace {
    name: String,
    root: LayoutNode<TabContent>,
    focused_pane: PaneId,
    next_pane_id: u64,
}

/// Transform a LayoutNode by splitting a target leaf
fn split_node<T>(
    node: LayoutNode<T>,
    target_id: PaneId,
    direction: SplitDirection,
    new_pane_id: PaneId,
    new_content: Option<T>,
) -> (LayoutNode<T>, Option<T>) {
    match node {
        LayoutNode::Leaf { id, content } if id == target_id => {
            // Found the target - split it, consume new_content
            let new_content = new_content.expect("new_content should be available when target is found");
            (LayoutNode::Split {
                direction,
                ratio: DEFAULT_SPLIT_RATIO,
                first: Box::new(LayoutNode::Leaf { id, content }),
                second: Box::new(LayoutNode::Leaf { id: new_pane_id, content: new_content }),
            }, None)
        }
        LayoutNode::Leaf { id, content } => {
            // Not the target, return unchanged with content passed through
            (LayoutNode::Leaf { id, content }, new_content)
        }
        LayoutNode::Split { direction: dir, ratio, first, second } => {
            // Recurse into first child
            let (new_first, remaining) = split_node(*first, target_id, direction, new_pane_id, new_content);
            // Recurse into second child with whatever content is remaining
            let (new_second, remaining) = split_node(*second, target_id, direction, new_pane_id, remaining);
            (LayoutNode::Split {
                direction: dir,
                ratio,
                first: Box::new(new_first),
                second: Box::new(new_second),
            }, remaining)
        }
    }
}

/// Remove a pane from the tree, promoting its sibling
fn close_node<T>(node: LayoutNode<T>, target_id: PaneId) -> Option<LayoutNode<T>> {
    match node {
        LayoutNode::Leaf { id, .. } if id == target_id => None,
        LayoutNode::Leaf { id, content } => Some(LayoutNode::Leaf { id, content }),
        LayoutNode::Split { direction, ratio, first, second } => {
            // Check if either direct child is the target
            if let LayoutNode::Leaf { id, .. } = first.as_ref() {
                if *id == target_id {
                    return Some(*second);
                }
            }
            if let LayoutNode::Leaf { id, .. } = second.as_ref() {
                if *id == target_id {
                    return Some(*first);
                }
            }

            // Recurse
            let new_first = close_node(*first, target_id);
            let new_second = close_node(*second, target_id);

            match (new_first, new_second) {
                (Some(f), Some(s)) => Some(LayoutNode::Split {
                    direction,
                    ratio,
                    first: Box::new(f),
                    second: Box::new(s),
                }),
                (Some(f), None) => Some(f),
                (None, Some(s)) => Some(s),
                (None, None) => None,
            }
        }
    }
}

impl Workspace {
    fn new(
        name: impl Into<String>,
        terminal_id: u64,
        ctx: &Context,
        pty_sender: Sender<(u64, PtyEvent)>,
    ) -> anyhow::Result<Self> {
        let name = name.into();
        let backend = create_terminal_backend(terminal_id, ctx, pty_sender)?;
        let pane_id = PaneId(0);

        Ok(Self {
            name,
            root: LayoutNode::Leaf {
                id: pane_id,
                content: TabContent::Terminal(TerminalInstance { backend, id: terminal_id }),
            },
            focused_pane: pane_id,
            next_pane_id: 1,
        })
    }

    /// Split focused pane in given direction
    /// Existing content moves to first child (left/top)
    /// New terminal goes to second child (right/bottom)
    fn split_focused(
        &mut self,
        direction: SplitDirection,
        terminal_id: u64,
        ctx: &Context,
        pty_sender: Sender<(u64, PtyEvent)>,
    ) -> anyhow::Result<()> {
        let backend = create_terminal_backend(terminal_id, ctx, pty_sender)?;
        let new_pane_id = PaneId(self.next_pane_id);
        self.next_pane_id += 1;

        let target_id = self.focused_pane;
        let new_content = TabContent::Terminal(TerminalInstance { backend, id: terminal_id });

        // Take ownership, transform, put back
        let old_root = std::mem::replace(&mut self.root, LayoutNode::Leaf {
            id: PaneId(u64::MAX),
            content: TabContent::FileViewer { path: PathBuf::new(), content: String::new(), scroll_offset: 0.0 },
        });
        let (new_root, _) = split_node(old_root, target_id, direction, new_pane_id, Some(new_content));
        self.root = new_root;

        // Focus the new pane
        self.focused_pane = new_pane_id;

        Ok(())
    }

    /// Close a pane by ID, returns true if closed
    fn close_pane(&mut self, pane_id: PaneId) -> bool {
        // Get all pane IDs to find next focus target
        let mut pane_ids = Vec::new();
        self.root.collect_pane_ids(&mut pane_ids);

        if pane_ids.len() <= 1 {
            // Don't close the last pane
            return false;
        }

        // Find index of closing pane
        let closing_idx = match pane_ids.iter().position(|id| *id == pane_id) {
            Some(idx) => idx,
            None => return false,
        };

        // Determine new focus (prefer previous, else next)
        let new_focus = if closing_idx > 0 {
            pane_ids[closing_idx - 1]
        } else {
            pane_ids[1]
        };

        // Close the pane
        let old_root = std::mem::replace(&mut self.root, LayoutNode::Leaf {
            id: PaneId(u64::MAX),
            content: TabContent::FileViewer { path: PathBuf::new(), content: String::new(), scroll_offset: 0.0 },
        });

        if let Some(new_root) = close_node(old_root, pane_id) {
            self.root = new_root;
            self.focused_pane = new_focus;
            true
        } else {
            false
        }
    }

    /// Move focus to next pane (DFS order)
    fn focus_next(&mut self) {
        let mut pane_ids = Vec::new();
        self.root.collect_pane_ids(&mut pane_ids);

        if let Some(idx) = pane_ids.iter().position(|id| *id == self.focused_pane) {
            let next_idx = (idx + 1) % pane_ids.len();
            self.focused_pane = pane_ids[next_idx];
        }
    }

    /// Move focus to previous pane (DFS order)
    fn focus_prev(&mut self) {
        let mut pane_ids = Vec::new();
        self.root.collect_pane_ids(&mut pane_ids);

        if let Some(idx) = pane_ids.iter().position(|id| *id == self.focused_pane) {
            let prev_idx = if idx == 0 { pane_ids.len() - 1 } else { idx - 1 };
            self.focused_pane = pane_ids[prev_idx];
        }
    }

    /// Get mutable reference to content by PaneId
    fn get_content_mut(&mut self, pane_id: PaneId) -> Option<&mut TabContent> {
        self.root.get_content_mut(pane_id)
    }

    /// Get content reference
    fn get_content(&self, pane_id: PaneId) -> Option<&TabContent> {
        self.root.get_content(pane_id)
    }

    /// Find pane by terminal ID
    fn find_pane_by_terminal_id(&self, terminal_id: u64) -> Option<PaneId> {
        fn find_in_node(node: &LayoutNode<TabContent>, terminal_id: u64) -> Option<PaneId> {
            match node {
                LayoutNode::Leaf { id, content } => {
                    if let TabContent::Terminal(t) = content {
                        if t.id == terminal_id {
                            return Some(*id);
                        }
                    }
                    None
                }
                LayoutNode::Split { first, second, .. } => {
                    find_in_node(first, terminal_id)
                        .or_else(|| find_in_node(second, terminal_id))
                }
            }
        }

        find_in_node(&self.root, terminal_id)
    }

    /// Count panes
    fn pane_count(&self) -> usize {
        self.root.pane_count()
    }

    /// Get all pane IDs in DFS order
    fn pane_ids(&self) -> Vec<PaneId> {
        let mut ids = Vec::new();
        self.root.collect_pane_ids(&mut ids);
        ids
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
    /// Pane being dragged for repositioning
    dragging_pane: Option<PaneDragState>,
    /// Show preferences window
    show_preferences: bool,
    /// IME is currently composing (preedit active)
    ime_composing: bool,
    /// Cached terminal theme (regenerated when config changes)
    cached_terminal_theme: egui_term::TerminalTheme,
}

impl VibeTermApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Load configuration
        let config = Config::load();
        let theme = RuntimeTheme::from(&config.theme);
        let cached_terminal_theme = theme::get_terminal_theme(&config);

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
            dragging_pane: None,
            show_preferences: false,
            ime_composing: false,
            cached_terminal_theme,
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
        let pane_id = PaneId(0);

        // Create a new workspace with a file viewer
        let workspace = Workspace {
            name,
            root: LayoutNode::Leaf {
                id: pane_id,
                content: TabContent::FileViewer {
                    path,
                    content,
                    scroll_offset: 0.0,
                },
            },
            focused_pane: pane_id,
            next_pane_id: 1,
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

    /// Split current pane horizontally (add new terminal to the right)
    fn split_pane_horizontal(&mut self) {
        let id = self.next_terminal_id;
        self.next_terminal_id += 1;

        // Clone before mutable borrow to satisfy borrow checker
        let ctx = self.ctx.clone();
        let pty_sender = self.pty_sender.clone();
        let _ = self.current_workspace_mut().split_focused(
            SplitDirection::Horizontal,
            id,
            &ctx,
            pty_sender,
        );
    }

    /// Split current pane vertically (add new terminal below)
    fn split_pane_vertical(&mut self) {
        let id = self.next_terminal_id;
        self.next_terminal_id += 1;

        // Clone before mutable borrow to satisfy borrow checker
        let ctx = self.ctx.clone();
        let pty_sender = self.pty_sender.clone();
        let _ = self.current_workspace_mut().split_focused(
            SplitDirection::Vertical,
            id,
            &ctx,
            pty_sender,
        );
    }

    /// Close current pane
    fn close_current_pane(&mut self) {
        let focused_pane = self.current_workspace().focused_pane;
        let pane_count = self.current_workspace().pane_count();

        if pane_count > 1 {
            self.current_workspace_mut().close_pane(focused_pane);
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

            // Cmd+D: Split pane horizontally (left/right)
            if i.key_pressed(Key::D) && modifiers.command && !modifiers.shift {
                self.split_pane_horizontal();
            }

            // Cmd+Shift+D: Split pane vertically (top/bottom)
            if i.key_pressed(Key::D) && modifiers.command && modifiers.shift {
                self.split_pane_vertical();
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
                self.workspaces[self.active_workspace].focus_next();
            }

            // Ctrl+Shift+Tab: Previous pane
            if i.key_pressed(Key::Tab) && modifiers.ctrl && modifiers.shift {
                self.workspaces[self.active_workspace].focus_prev();
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
                                let focused = ws.focused_pane;
                                if let Some(content) = ws.get_content_mut(focused) {
                                    if let TabContent::Terminal(terminal) = content {
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
                MenuAction::SplitHorizontal => self.split_pane_horizontal(),
                MenuAction::SplitVertical => self.split_pane_vertical(),
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
                        if let Some(pane_id) = workspace.find_pane_by_terminal_id(terminal_id) {
                            if workspace.pane_count() > 1 {
                                workspace.close_pane(pane_id);
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

    /// Compute drop zones for all panes except the source pane
    fn compute_drop_zones(&self, layout: &ComputedLayout, source_id: PaneId) -> Vec<DropZoneInfo> {
        let mut zones = Vec::new();
        let edge_ratio = 0.25;

        for (pane_id, rect) in &layout.pane_rects {
            if *pane_id == source_id {
                continue; // Skip source pane
            }

            let w = rect.width();
            let h = rect.height();

            // Top zone (25% of height from top)
            zones.push(DropZoneInfo {
                zone: DropZone::Top(*pane_id),
                rect: egui::Rect::from_min_size(rect.min, egui::vec2(w, h * edge_ratio)),
                highlight_rect: egui::Rect::from_min_size(rect.min, egui::vec2(w, h * 0.5)),
            });

            // Bottom zone (25% of height from bottom)
            zones.push(DropZoneInfo {
                zone: DropZone::Bottom(*pane_id),
                rect: egui::Rect::from_min_size(
                    egui::pos2(rect.min.x, rect.max.y - h * edge_ratio),
                    egui::vec2(w, h * edge_ratio),
                ),
                highlight_rect: egui::Rect::from_min_size(
                    egui::pos2(rect.min.x, rect.min.y + h * 0.5),
                    egui::vec2(w, h * 0.5),
                ),
            });

            // Left zone (25% of width from left)
            zones.push(DropZoneInfo {
                zone: DropZone::Left(*pane_id),
                rect: egui::Rect::from_min_size(rect.min, egui::vec2(w * edge_ratio, h)),
                highlight_rect: egui::Rect::from_min_size(rect.min, egui::vec2(w * 0.5, h)),
            });

            // Right zone (25% of width from right)
            zones.push(DropZoneInfo {
                zone: DropZone::Right(*pane_id),
                rect: egui::Rect::from_min_size(
                    egui::pos2(rect.max.x - w * edge_ratio, rect.min.y),
                    egui::vec2(w * edge_ratio, h),
                ),
                highlight_rect: egui::Rect::from_min_size(
                    egui::pos2(rect.min.x + w * 0.5, rect.min.y),
                    egui::vec2(w * 0.5, h),
                ),
            });
        }

        zones
    }

    /// Execute a pane drop operation
    fn execute_pane_drop(&mut self, source_id: PaneId, zone: DropZone) {
        let ws = &mut self.workspaces[self.active_workspace];

        // Create a placeholder to swap with
        let placeholder = LayoutNode::Leaf {
            id: PaneId(u64::MAX),
            content: TabContent::FileViewer {
                path: std::path::PathBuf::new(),
                content: String::new(),
                scroll_offset: 0.0,
            },
        };

        // Step 1: Extract source pane from tree
        let old_root = std::mem::replace(&mut ws.root, placeholder);

        if let Some((tree_without_source, extracted_content)) = crate::layout::extract_pane(old_root, source_id) {
            // Step 2: Determine target and direction from zone
            let (target_id, direction, before) = match zone {
                DropZone::Top(id) => (id, SplitDirection::Vertical, true),
                DropZone::Bottom(id) => (id, SplitDirection::Vertical, false),
                DropZone::Left(id) => (id, SplitDirection::Horizontal, true),
                DropZone::Right(id) => (id, SplitDirection::Horizontal, false),
            };

            // Step 3: Insert at new location (keeping same PaneId for PTY connection)
            ws.root = crate::layout::insert_adjacent(
                tree_without_source,
                target_id,
                source_id,
                extracted_content,
                direction,
                before,
            );

            // Keep focus on the moved pane
            ws.focused_pane = source_id;
        } else {
            // Extraction failed (single pane?), restore original
            // This shouldn't happen if drop zones are computed correctly
            log::warn!("Failed to extract pane {} for drop", source_id.0);
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
                        self.cached_terminal_theme = theme::get_terminal_theme(&self.config);
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

    /// Render panes using the binary split tree layout
    fn render_panes(&mut self, ui: &mut egui::Ui) {
        let terminal_theme = self.cached_terminal_theme.clone();
        let focused_pane = self.current_workspace().focused_pane;

        // Compute layout for all panes
        let available_rect = ui.available_rect_before_wrap();
        let mut layout = ComputedLayout::new();
        let mut path = Vec::new();
        self.workspaces[self.active_workspace]
            .root
            .compute_layout(available_rect, DIVIDER_WIDTH, &mut path, &mut layout);

        // Check for click to focus pane (on click release)
        let clicked_primary = ui.input(|i| i.pointer.button_clicked(egui::PointerButton::Primary));
        let button_pressed = ui.input(|i| i.pointer.button_pressed(egui::PointerButton::Primary));
        let pointer_pos = ui.input(|i| i.pointer.latest_pos());
        let pointer_released = ui.input(|i| i.pointer.any_released());

        if clicked_primary {
            if let Some(pos) = pointer_pos {
                for (pane_id, rect) in &layout.pane_rects {
                    if rect.contains(pos) && *pane_id != focused_pane {
                        self.workspaces[self.active_workspace].focused_pane = *pane_id;
                        break;
                    }
                }
            }
        }

        // Handle pane drag-and-drop
        // Start potential drag on button press (not click release)
        if button_pressed && self.dragging_pane.is_none() && self.dragging_divider.is_none() {
            if let Some(pos) = pointer_pos {
                for (pane_id, rect) in &layout.pane_rects {
                    if rect.contains(pos) {
                        self.dragging_pane = Some(PaneDragState {
                            source_pane_id: *pane_id,
                            start_pos: pos,
                            current_pos: pos,
                            drag_active: false,
                        });
                        break;
                    }
                }
            }
        }

        // Update drag state while dragging
        if let Some(ref mut drag_state) = self.dragging_pane {
            if let Some(pos) = pointer_pos {
                drag_state.current_pos = pos;

                // Activate drag after 8px threshold
                if !drag_state.drag_active {
                    let delta = drag_state.current_pos - drag_state.start_pos;
                    if delta.length() >= 8.0 {
                        drag_state.drag_active = true;
                    }
                }
            }

            // Cancel on ESC
            if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                self.dragging_pane = None;
            }
        }

        // Handle drop on button release (separate block to avoid borrow issues)
        if pointer_released {
            if let Some(drag_state) = self.dragging_pane.take() {
                if drag_state.drag_active {
                    let drop_zones = self.compute_drop_zones(&layout, drag_state.source_pane_id);
                    if let Some(zone_info) = drop_zones.iter().find(|z| z.rect.contains(drag_state.current_pos)) {
                        self.execute_pane_drop(drag_state.source_pane_id, zone_info.zone);
                    }
                }
                // dragging_pane is already None from .take()
            }
        }

        // Handle divider dragging
        let mut needs_recompute = false;
        if let Some((_, divider_idx)) = self.dragging_divider {
            if ui.input(|i| i.pointer.any_released()) {
                self.dragging_divider = None;
            } else if let Some(pos) = pointer_pos {
                // Get the divider info
                if let Some(divider) = layout.dividers.get(divider_idx) {
                    // Get the split node at this path and update its ratio
                    if let Some(split_node) = self.workspaces[self.active_workspace]
                        .root
                        .get_split_at_path_mut(&divider.path)
                    {
                        if let LayoutNode::Split { direction, ratio, .. } = split_node {
                            let parent_rect = if divider.path.is_empty() {
                                available_rect
                            } else {
                                // For nested splits, we need the parent rect
                                // For now, use available_rect as approximation
                                available_rect
                            };

                            let new_ratio = match direction {
                                SplitDirection::Horizontal => {
                                    let relative_x = pos.x - parent_rect.left();
                                    (relative_x / (parent_rect.width() - DIVIDER_WIDTH))
                                        .clamp(crate::layout::MIN_SPLIT_RATIO, crate::layout::MAX_SPLIT_RATIO)
                                }
                                SplitDirection::Vertical => {
                                    let relative_y = pos.y - parent_rect.top();
                                    (relative_y / (parent_rect.height() - DIVIDER_WIDTH))
                                        .clamp(crate::layout::MIN_SPLIT_RATIO, crate::layout::MAX_SPLIT_RATIO)
                                }
                            };
                            *ratio = new_ratio;
                            needs_recompute = true;
                        }
                    }
                }
            }
        }

        // CONDITIONAL recompute - only when divider drag changed ratio
        if needs_recompute {
            layout = ComputedLayout::new();
            path.clear();
            self.workspaces[self.active_workspace]
                .root
                .compute_layout(available_rect, DIVIDER_WIDTH, &mut path, &mut layout);
        }

        let focused_pane = self.current_workspace().focused_pane;

        // Render dividers first (background layer)
        for (idx, divider) in layout.dividers.iter().enumerate() {
            let divider_response = ui.allocate_rect(divider.rect, egui::Sense::click_and_drag());

            if divider_response.drag_started() {
                self.dragging_divider = Some((self.active_workspace, idx));
            }

            let divider_color = if divider_response.dragged() || divider_response.hovered() {
                self.theme.primary
            } else {
                self.theme.border
            };
            ui.painter().rect_filled(divider.rect, 0.0, divider_color);

            if divider_response.hovered() || divider_response.dragged() {
                let cursor = match divider.direction {
                    SplitDirection::Horizontal => egui::CursorIcon::ResizeHorizontal,
                    SplitDirection::Vertical => egui::CursorIcon::ResizeVertical,
                };
                ui.ctx().set_cursor_icon(cursor);
            }
        }

        // Render panes
        // We need to collect pane info first to avoid borrow issues
        let pane_info: Vec<(PaneId, egui::Rect)> = layout.pane_rects.iter()
            .map(|(id, rect)| (*id, *rect))
            .collect();

        for (pane_id, rect) in pane_info {
            let is_focused = pane_id == focused_pane;

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
                    if let Some(content) = self.workspaces[self.active_workspace]
                        .root
                        .get_content_mut(pane_id)
                    {
                        match content {
                            TabContent::Terminal(terminal) => {
                                TerminalView::new(ui, &mut terminal.backend)
                                    .set_theme(terminal_theme.clone())
                                    .set_focus(is_focused)
                                    .set_size(inner_rect.size())
                                    .ui(ui);
                            }
                            TabContent::FileViewer { content, .. } => {
                                ui.painter().rect_filled(inner_rect, 0.0, self.theme.background);
                                egui::ScrollArea::vertical()
                                    .id_salt(format!("file_scroll_{}", pane_id.0))
                                    .show(ui, |ui| {
                                        ui.add(egui::Label::new(
                                            egui::RichText::new(content.as_str())
                                                .font(theme::mono_font(12.0))
                                                .color(self.theme.text)
                                        ).wrap());
                                    });
                            }
                        }
                    }
                },
            );
        }

        // Render drag feedback overlay
        if let Some(ref drag_state) = self.dragging_pane {
            if drag_state.drag_active {
                let drop_zones = self.compute_drop_zones(&layout, drag_state.source_pane_id);

                // Find and highlight active zone
                if let Some(zone_info) = drop_zones.iter().find(|z| z.rect.contains(drag_state.current_pos)) {
                    ui.painter().rect_filled(
                        zone_info.highlight_rect,
                        0.0,
                        egui::Color32::from_rgba_unmultiplied(100, 150, 255, 80),
                    );
                }

                // Ghost preview following cursor
                let preview_size = egui::vec2(120.0, 80.0);
                let preview_pos = drag_state.current_pos - preview_size * 0.5;
                ui.painter().rect_filled(
                    egui::Rect::from_min_size(preview_pos, preview_size),
                    4.0,
                    egui::Color32::from_rgba_unmultiplied(
                        self.theme.primary.r(),
                        self.theme.primary.g(),
                        self.theme.primary.b(),
                        100,
                    ),
                );
                ui.painter().rect_stroke(
                    egui::Rect::from_min_size(preview_pos, preview_size),
                    4.0,
                    egui::Stroke::new(2.0, self.theme.primary),
                    egui::StrokeKind::Inside,
                );
            }
        }
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
                    let pane_ids = self.workspaces[idx].pane_ids();
                    if let Some(first_id) = pane_ids.first() {
                        self.workspaces[idx].focused_pane = *first_id;
                    }
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
                let pane_count = self.current_workspace().pane_count();
                let pane_ids = self.current_workspace().pane_ids();
                let focused_pane = self.current_workspace().focused_pane;
                let focused_idx = pane_ids.iter().position(|id| *id == focused_pane).unwrap_or(0);
                StatusBar::new(pane_count, focused_idx, &self.theme).show(ui);
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
