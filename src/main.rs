//! VibeTerm - GPU-accelerated terminal emulator with TUI aesthetics
//!
//! Built with egui + egui_term (Alacritty backend)

mod app;
mod config;
mod directory_scanner;
mod layout;
mod menu;
mod project;
mod pty_tracker;
mod theme;
mod ui;

use app::VibeTermApp;

fn main() -> eframe::Result<()> {
    // Initialize logging
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp_millis()
        .init();

    log::info!("VibeTerm v{} starting...", env!("CARGO_PKG_VERSION"));

    // eframe native options
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("VibeTerm")
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([600.0, 400.0])
            .with_transparent(false),
        // Renderer (glow = OpenGL)
        renderer: eframe::Renderer::Glow,
        vsync: true,
        multisampling: 4,
        depth_buffer: 0,
        stencil_buffer: 0,
        hardware_acceleration: eframe::HardwareAcceleration::Required,
        ..Default::default()
    };

    // Run app
    eframe::run_native(
        "VibeTerm",
        native_options,
        Box::new(|cc| {
            // Set up native menu bar
            menu::setup_menu_bar();
            Ok(Box::new(VibeTermApp::new(cc)))
        }),
    )
}
