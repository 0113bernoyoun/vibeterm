//! Native macOS Menu Bar
//!
//! Uses muda crate for cross-platform native menu support

use muda::{
    accelerator::Accelerator,
    Menu, MenuEvent, MenuItem, PredefinedMenuItem, Submenu,
};
use std::sync::mpsc;
use std::sync::{Mutex, OnceLock};

/// Menu action events
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuAction {
    NewTab,
    NewWindow,
    CloseTab,
    CloseWindow,
    SplitHorizontal,
    SplitVertical,
    ToggleSidebar,
    Preferences,
    About,
    Quit,
}

/// Global menu event receiver (wrapped in Mutex for Sync)
static MENU_RECEIVER: OnceLock<Mutex<mpsc::Receiver<MenuAction>>> = OnceLock::new();
static MENU_SENDER: OnceLock<mpsc::Sender<MenuAction>> = OnceLock::new();

/// Set up the native menu bar
pub fn setup_menu_bar() {
    // Create channel for menu events
    let (sender, receiver) = mpsc::channel();
    let _ = MENU_SENDER.set(sender);
    let _ = MENU_RECEIVER.set(Mutex::new(receiver));

    // Create menu bar
    let menu_bar = Menu::new();

    // === VibeTerm menu (App menu on macOS) ===
    let app_menu = Submenu::new("VibeTerm", true);

    let about_item = MenuItem::with_id(
        "about",
        "About VibeTerm",
        true,
        None::<Accelerator>,
    );

    let preferences_item = MenuItem::with_id(
        "preferences",
        "Preferences...                  ⌘,",
        true,
        None::<Accelerator>,
    );

    let _ = app_menu.append(&about_item);
    let _ = app_menu.append(&PredefinedMenuItem::separator());
    let _ = app_menu.append(&preferences_item);
    let _ = app_menu.append(&PredefinedMenuItem::separator());
    let _ = app_menu.append(&PredefinedMenuItem::services(None));
    let _ = app_menu.append(&PredefinedMenuItem::separator());
    let _ = app_menu.append(&PredefinedMenuItem::hide(None));
    let _ = app_menu.append(&PredefinedMenuItem::hide_others(None));
    let _ = app_menu.append(&PredefinedMenuItem::show_all(None));
    let _ = app_menu.append(&PredefinedMenuItem::separator());
    let _ = app_menu.append(&PredefinedMenuItem::quit(None));

    // === File menu ===
    let file_menu = Submenu::new("File", true);

    // Note: Accelerators shown in label only - actual shortcuts handled in egui
    let new_tab_item = MenuItem::with_id(
        "new_tab",
        "New Tab                              ⌘T",
        true,
        None::<Accelerator>,
    );

    let new_window_item = MenuItem::with_id(
        "new_window",
        "New Window                      ⇧⌘N",
        true,
        None::<Accelerator>,
    );

    let close_tab_item = MenuItem::with_id(
        "close_tab",
        "Close Tab                            ⌘W",
        true,
        None::<Accelerator>,
    );

    let _ = file_menu.append(&new_tab_item);
    let _ = file_menu.append(&new_window_item);
    let _ = file_menu.append(&PredefinedMenuItem::separator());
    let _ = file_menu.append(&close_tab_item);
    let _ = file_menu.append(&PredefinedMenuItem::close_window(None));

    // === Edit menu ===
    let edit_menu = Submenu::new("Edit", true);
    let _ = edit_menu.append(&PredefinedMenuItem::undo(None));
    let _ = edit_menu.append(&PredefinedMenuItem::redo(None));
    let _ = edit_menu.append(&PredefinedMenuItem::separator());
    let _ = edit_menu.append(&PredefinedMenuItem::cut(None));
    let _ = edit_menu.append(&PredefinedMenuItem::copy(None));
    let _ = edit_menu.append(&PredefinedMenuItem::paste(None));
    let _ = edit_menu.append(&PredefinedMenuItem::select_all(None));

    // === View menu ===
    let view_menu = Submenu::new("View", true);

    let toggle_sidebar_item = MenuItem::with_id(
        "toggle_sidebar",
        "Toggle Sidebar                   ⌘B",
        true,
        None::<Accelerator>,
    );

    let split_horizontal_item = MenuItem::with_id(
        "split_horizontal",
        "Split Pane Horizontally      ⌘D",
        true,
        None::<Accelerator>,
    );

    let split_vertical_item = MenuItem::with_id(
        "split_vertical",
        "Split Pane Vertically         ⇧⌘D",
        true,
        None::<Accelerator>,
    );

    let _ = view_menu.append(&toggle_sidebar_item);
    let _ = view_menu.append(&PredefinedMenuItem::separator());
    let _ = view_menu.append(&split_horizontal_item);
    let _ = view_menu.append(&split_vertical_item);
    let _ = view_menu.append(&PredefinedMenuItem::separator());
    let _ = view_menu.append(&PredefinedMenuItem::fullscreen(None));

    // === Shell menu ===
    let shell_menu = Submenu::new("Shell", true);

    let new_shell_item = MenuItem::with_id(
        "new_shell",
        "New Shell",
        true,
        None::<Accelerator>,
    );

    let _ = shell_menu.append(&new_shell_item);

    // === Window menu ===
    let window_menu = Submenu::new("Window", true);
    let _ = window_menu.append(&PredefinedMenuItem::minimize(None));
    let _ = window_menu.append(&PredefinedMenuItem::maximize(None));
    let _ = window_menu.append(&PredefinedMenuItem::separator());
    let _ = window_menu.append(&PredefinedMenuItem::bring_all_to_front(None));

    // === Help menu ===
    let help_menu = Submenu::new("Help", true);

    let help_item = MenuItem::with_id(
        "help",
        "VibeTerm Help",
        true,
        None::<Accelerator>,
    );

    let _ = help_menu.append(&help_item);

    // Add all menus to menu bar
    let _ = menu_bar.append(&app_menu);
    let _ = menu_bar.append(&file_menu);
    let _ = menu_bar.append(&edit_menu);
    let _ = menu_bar.append(&view_menu);
    let _ = menu_bar.append(&shell_menu);
    let _ = menu_bar.append(&window_menu);
    let _ = menu_bar.append(&help_menu);

    // Initialize menu bar on macOS
    #[cfg(target_os = "macos")]
    {
        let _ = menu_bar.init_for_nsapp();
    }

    // Set up menu event handler
    std::thread::spawn(move || {
        loop {
            if let Ok(event) = MenuEvent::receiver().recv() {
                if let Some(sender) = MENU_SENDER.get() {
                    let action = match event.id().0.as_str() {
                        "new_tab" => Some(MenuAction::NewTab),
                        "new_window" => Some(MenuAction::NewWindow),
                        "close_tab" => Some(MenuAction::CloseTab),
                        "toggle_sidebar" => Some(MenuAction::ToggleSidebar),
                        "split_horizontal" => Some(MenuAction::SplitHorizontal),
                        "split_vertical" => Some(MenuAction::SplitVertical),
                        "preferences" => Some(MenuAction::Preferences),
                        "about" => Some(MenuAction::About),
                        _ => None,
                    };
                    if let Some(action) = action {
                        let _ = sender.send(action);
                    }
                }
            }
        }
    });

    log::info!("Native menu bar initialized");
}

/// Poll for menu events (non-blocking)
pub fn poll_menu_event() -> Option<MenuAction> {
    MENU_RECEIVER.get()?.lock().ok()?.try_recv().ok()
}
