use crate::App;
use egui::{Color32, RichText};

pub enum Screen {
    Start,
    Tracking,
    Pause,
}

pub fn start_screen(
    app: &mut App,
    ctx: &egui::Context,
    _frame: &mut eframe::Frame,
) -> Option<Screen> {
    // There's nothing wrong with the return type. It's just that `CentralPanel` is also a function
    // Which means that the return type needs to cover that as well.
    let mut screen = None;
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.heading("Actt");
            ui.hyperlink_to(
                "Made by AlphabetsAlphabets",
                "https://github.com/AlphabetsAlphabets",
            );

            match &app.warning {
                None => ui.label("\n\n\n"),
                Some(msg) => {
                    let msg = format!("\n{}\n", msg);
                    ui.label(msg)
                }
            };

            ui.label("Activity");

            ui.text_edit_singleline(&mut app.activity)
                .on_hover_text("What do you want to track?");

            ui.label("Tag");
            ui.text_edit_singleline(&mut app.tag)
                .on_hover_text("What category is this activity under?");

            if ui.button("Start").clicked() {
                screen = Some(Screen::Tracking);
            }
        });
    });

    screen
}

pub fn tracking_screen(app: &mut App, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.heading("Actt");
            ui.hyperlink_to(
                "Made by AlphabetsAlphabets",
                "https://github.com/AlphabetsAlphabets",
            );

            match &app.warning {
                None => ui.label("\n\n\n"),
                Some(msg) => {
                    let msg = format!("\n{}\n", msg);
                    ui.label(msg)
                }
            };

            ui.label("Activity");

            ui.text_edit_singleline(&mut app.activity)
                .on_hover_text("What do you want to track?");

            ui.label("Tag");
            ui.text_edit_singleline(&mut app.tag)
                .on_hover_text("What category is this activity under?");
        });
    });
}
