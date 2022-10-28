// This is used to hide the console from popping up on Windows.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use actt::screens;
use actt::App;
use eframe;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use egui::Vec2;

    let native_options = eframe::NativeOptions {
        min_window_size: Some(Vec2::new(498.0, 394.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Actt",
        native_options,
        Box::new(|cc| Box::new(App::new(cc))),
    );
}
