// This is used to hide the console from popping up on Windows.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;

mod app;
mod screens;

use app::App;

use eframe;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use egui::Vec2;

    let native_options = eframe::NativeOptions {
        min_window_size: Some(Vec2::new(498.0, 394.0)),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "Actt",
        native_options,
        Box::new(|_cc| Box::<App>::default()),
    );
}
