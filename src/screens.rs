use std::{
    fs,
    time::{Duration, Instant},
};

use crate::App;
use egui::{Color32, RichText, Ui};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
pub struct Activity {
    name: Vec<String>,
    total_time: Vec<Duration>,
    tag: Vec<String>,
}

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
            ui.separator();

            match app.read_config_file() {
                Some(act) => {
                    for (index, name) in act.name.iter().enumerate() {
                        let tag = &act.tag[index];
                        let total_time = &act.total_time[index];

                        let msg = format!("{} | @{} | {}s", name, tag, total_time.as_secs());
                        ui.label(msg);
                    }
                }
                None => {
                    ui.label("It's empty!");
                }
            }
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
                    let msg = format!("\n{}\n\n", msg);
                    ui.label(msg)
                }
            };

            ui.horizontal(|ui| {
                ui.label("Activity");
                ui.text_edit_singleline(&mut app.activity_name)
                    .on_hover_text("What do you want to track?");
            });

            ui.horizontal(|ui| {
                ui.label("Tag       ");
                ui.text_edit_singleline(&mut app.tag)
                    .on_hover_text("What category is this activity under?");
            });

            if ui.button("Start").clicked() {
                if app.activity_name.len() <= 0 {
                    app.warning = Some("Activity cannot be empty!".to_string());
                } else {
                    app.warning = None;
                    app.screen = Screen::Tracking;
                    app.now = Some(Instant::now());
                }
            }
        });
    });
}

pub fn tracking_screen(app: &mut App, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.label("\n\n\n");
            let header = format!("Tracking activity '{}'", app.activity_name);
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
                    app.screen = Screen::History;
                    app.total_time = Duration::from_secs(now);

                    let act = match app.read_config_file() {
                        Some(mut act) => {
                            act.name.push(app.activity_name.clone());
                            act.total_time.push(app.total_time);
                            act.tag.push(app.tag.clone());

                            act
                        }
                        None => Activity {
                            name: vec![app.activity_name.clone()],
                            total_time: vec![app.total_time],
                            tag: vec![app.tag.clone()],
                        },
                    };

                    let json = serde_json::to_string(&act).unwrap();

                    // Could fail because of perms
                    fs::write(&app.config_file, json).unwrap();
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
