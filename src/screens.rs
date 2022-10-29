use std::{
    cmp::{max, min},
    fs,
    time::{Duration, Instant},
};

use crate::App;
use egui::{Color32, RichText, ScrollArea, Ui, Vec2};

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
                None => ui.label("\r"),
                Some(msg) => {
                    let msg = format!("\n{}\n", msg);
                    ui.label(msg)
                }
            };

            ui.label("A history of all your activities, and how long you've spent on each one!");
            ui.separator();

            let act = match app.read_config_file() {
                Some(act) => act,
                None => Activity::default(),
            };

            if act.name.len() == 0 {
                ui.label("It's empty!");
            } else {
                let scroll_area = ScrollArea::vertical().auto_shrink([false; 2]);
                scroll_area.show(ui, |ui| {
                    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                        ui.columns(4, |column| {
                            let blue_text =
                                |text: &str| RichText::new(text).color(Color32::LIGHT_BLUE);

                            column[0].vertical_centered_justified(|ui| ui.label(blue_text("Name")));
                            column[1].vertical_centered_justified(|ui| ui.label(blue_text("Tag")));
                            column[2].vertical_centered_justified(|ui| {
                                ui.label(blue_text("Time spent"))
                            });
                            column[3]
                                .vertical_centered_justified(|ui| ui.label(blue_text("Delete")));
                        });

                        for (index, name) in act.name.iter().enumerate() {
                            ui.columns(4, |column| {
                                let tag = &act.tag[index];
                                let total_time = &act.total_time[index].as_secs();

                                column[0].vertical_centered_justified(|ui| ui.label(name));
                                column[1].vertical_centered_justified(|ui| ui.label(tag));

                                let total_time = if *total_time < 60 {
                                    format!("{}s", total_time)
                                } else {
                                    format!("{}m", total_time)
                                };

                                column[2].vertical_centered_justified(|ui| ui.label(total_time));
                                column[3].vertical_centered_justified(|ui| {
                                    if ui.button("X").clicked() {
                                        let Activity {
                                            name,
                                            total_time,
                                            tag,
                                        } = &mut app.activity_history;

                                        if name.len() != 0
                                            || total_time.len() != 0
                                            || tag.len() != 0
                                        {
                                            // println!("Name: {} ({})", &name[index], index);
                                            name.remove(index);
                                            total_time.remove(index);
                                            tag.remove(index);
                                            ctx.request_repaint();

                                            app.write_config_file();
                                        }
                                    }
                                });
                            });
                        }
                    });
                });
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

            ui.allocate_ui_with_layout(
                Vec2::new(200.0, 200.0),
                egui::Layout::top_down(egui::Align::Center),
                |ui| {
                    ui.columns(2, |column| {
                        column[0].vertical_centered_justified(|ui| ui.label("Activity"));
                        column[1].vertical_centered_justified(|ui| {
                            ui.text_edit_singleline(&mut app.activity_name)
                                .on_hover_text("What do you want to track?")
                        });
                    });

                    ui.columns(2, |column| {
                        column[0].vertical_centered_justified(|ui| ui.label("Tag"));
                        column[1].vertical_centered_justified(|ui| {
                            ui.text_edit_singleline(&mut app.tag)
                                .on_hover_text("What category is this activity under?")
                        });
                    });
                },
            );

            ui.label("\n");
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

            let mut now = app.now.unwrap().elapsed().as_secs();
            let total_time = app.total_time.as_secs();
            if total_time == 0 {
                app.total_time = Duration::from_secs(now);
            } else {
                app.total_time += Duration::from_secs(now);
            }

            match app.screen {
                Screen::Pause => {
                    ui.heading("Paused");
                }
                _ => {
                    let header = if now < 60 {
                        format!("{}s", app.total_time.as_secs())
                    } else {
                        format!("{}m", app.total_time.as_secs() / 60)
                    };

                    ctx.request_repaint();
                    ui.heading(header);
                }
            }

            ui.label("\n");

            ui.columns(2, |columns| {
                if columns[0].button("Stop").clicked() {
                    app.screen = Screen::History;
                    app.total_time = Duration::from_secs(now);

                    app.activity_history = match app.read_config_file() {
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

                    app.write_config_file();

                    app.work_time = Duration::default();
                }

                match app.screen {
                    Screen::Pause => {
                        if columns[1].button("Resume").clicked() {
                            app.screen = Screen::Tracking;
                            app.now = Some(Instant::now());
                        }
                    }
                    _ => {
                        if columns[1].button("Pause").clicked() {
                            app.screen = Screen::Pause;
                            let work_time = app.work_time.as_secs();
                            app.work_time =
                                Duration::from_secs(max(work_time, now) - min(work_time, now));
                        }
                    }
                }
            });
        });
    });
}
