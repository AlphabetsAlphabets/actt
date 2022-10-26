use std::time::Instant;

use crate::App;
use egui::{Color32, RichText, Ui};

#[derive(PartialEq)]
pub enum Screen {
    Start,
    Tracking,
    Pause,
    History,
}

pub fn horizontal_menu(app: &mut App, ui: &mut Ui) {
    ui.horizontal_top(|ui| {
        ui.selectable_value(&mut app.screen, Screen::Start, "Home");
        ui.selectable_value(&mut app.screen, Screen::History, "History");
    });

    ui.separator();
}

pub fn history_screen(app: &mut App, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
        horizontal_menu(app, ui);
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.heading("History");
            match &app.warning {
                None => ui.label("\n\n\n"),
                Some(msg) => {
                    let msg = format!("\n{}\n", msg);
                    ui.label(msg)
                }
            };

            ui.label("A history of all your activities, and how long you've spent on each one!");
        });
    });
}

pub fn start_screen(app: &mut App, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    // There's nothing wrong with the return type. It's just that `CentralPanel` is also a function
    // Which means that the return type needs to cover that as well.
    egui::CentralPanel::default().show(ctx, |ui| {
        horizontal_menu(app, ui);
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.heading("Home");
            match &app.warning {
                None => ui.label("\n\n\n"),
                Some(msg) => {
                    let msg = format!("\n{}\n", msg);
                    ui.label(msg)
                }
            };

            ui.horizontal(|ui| {
                ui.label("Activity");
                ui.text_edit_singleline(&mut app.activity)
                    .on_hover_text("What do you want to track?");
            });

            ui.horizontal(|ui| {
                ui.label("Tag       ");
                ui.text_edit_singleline(&mut app.tag)
                    .on_hover_text("What category is this activity under?");
            });

            if ui.button("Start").clicked() {
                app.screen = Screen::Tracking;
                app.now = Some(Instant::now());
            }
        });
    });
}

pub fn tracking_screen(app: &mut App, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.label("\n\n\n");
            let header = format!("Tracking activity '{}'", app.activity);
            ui.heading(header);
            match &app.warning {
                None => ui.label("\n\n"),
                Some(msg) => {
                    let msg = format!("\n{}\n", msg);
                    ui.label(msg)
                }
            };

            let now = app.now.unwrap().elapsed().as_secs();
            let header = if now < 60 {
                format!("{}s", now)
            } else {
                format!("{}m", now)
            };
            ctx.request_repaint();

            ui.heading(header);

            ui.label("\n");

            ui.columns(2, |columns| {
                if columns[0].button("Stop").clicked() {
                    app.screen = Screen::Start;
                }

                columns[1].button("Pause");
            });
        });
    });
}

pub fn pause_screen(app: &mut App, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.heading("Pause");
        });
    });
}
