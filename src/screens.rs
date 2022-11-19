use std::{
    fmt::{self, Debug},
    time::{Duration, Instant},
};

use crate::colors::*;
use crate::App;
use egui::{
    color_picker::{color_picker_color32, Alpha},
    stroke_ui, Button, Color32, Label, RichText, ScrollArea, Sense, Ui, Vec2,
};

use egui_dropdown::DropDownBox;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
pub struct Activity {
    pub name: Vec<String>,
    total_time: Vec<Duration>,
    color: Vec<Color32>,
    pub tag: Vec<String>,
}

impl Debug for Activity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // name: <name> #<tag> @ <time>
        for (index, name) in self.name.iter().enumerate() {
            let time = &self.total_time[index].as_secs();
            let tag = &self.tag[index];

            writeln!(f, "Name: {} #{} - {}s", name, tag, time)?;
        }

        Ok(())
    }
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

            let mut act = app.read_config_file();

            if act.name.len() == 0 {
                ui.label("It's empty!");
            } else {
                activity_listing(app, &mut act, ctx, _frame, ui);
            }
        });
    });
}

pub fn start_screen(app: &mut App, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    // There's nothing wrong with the return type. It's just that `CentralPanel` is also a function
    // Which means that the return type needs to cover that as well.
    egui::CentralPanel::default().show(ctx, |ui| {
        app.activity_history = app.read_config_file();

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
                            let display = prepare_tag_for_display(&app.activity_history.tag[..]);

                            ui.add(DropDownBox::from_iter(
                                &display,
                                "tags",
                                &mut app.tag,
                                |ui, text| ui.selectable_label(false, text),
                            ))
                            .on_hover_text("What category is this activity under?");
                        });
                    });

                    ui.columns(2, |column| {
                        column[0].vertical_centered_justified(|ui| ui.label("Tag color"));
                        column[1].vertical_centered_justified(|ui| {
                            color_picker_color32(ui, &mut app.color, Alpha::Opaque);
                        });
                    });
                },
            );

            ui.label("\n");
            if ui.button("Start").clicked() {
                if app.activity_name.len() <= 0 {
                    app.warning = Some("Activity name cannot be empty!".to_string());
                } else {
                    if app.tag.is_empty() {
                        app.tag = "  ".to_string();
                    }

                    app.warning = None;
                    app.screen = Screen::Tracking;
                    app.total_time = Some(Instant::now());
                }
            }
        });
    });
}

pub fn tracking_screen(app: &mut App, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.label("\n\n\n");
            let r = app.color.r() as u8;
            let g = app.color.g() as u8;
            let b = app.color.b() as u8;

            let header = RichText::new(app.activity_name.clone())
                .color(Color32::from_rgb(r, g, b))
                .size(32.0);
            ui.heading(header);
            match &app.warning {
                None => ui.label("\n\n"),
                Some(msg) => {
                    let msg = format!("\n{}\n", msg);
                    let msg = red_text(msg.as_str());
                    ui.label(msg)
                }
            };

            let total_time: u64 = match app.pause_time {
                Some(pause_time) => pause_time.duration_since(app.total_time.unwrap()).as_secs(),
                None => app.total_time.unwrap().elapsed().as_secs(),
            };

            // 5741s
            let m = total_time / 60;
            let s = total_time % 60;
            let h = m / 60;
            let m = m % 60;

            let header = format!("{}h {}m {}s", h, m, s);
            match app.screen {
                Screen::Pause => {
                    ui.heading("Paused");
                }
                _ => {
                    ctx.request_repaint();
                    ui.heading(header);
                }
            }

            ui.label("\n");

            ui.columns(2, |columns| {
                if columns[0].button("Stop").clicked() {
                    app.screen = Screen::History;
                    match app.pause_time {
                        Some(pause_time) => {
                            app.total_time = Some(app.total_time.unwrap() + pause_time.elapsed());
                        }
                        _ => (),
                    }

                    let mut act = app.read_config_file();
                    if act.name.len() == 0 {
                        act.name = vec![app.activity_name.clone()];
                        act.total_time = vec![app.total_time.unwrap().elapsed()];
                        act.tag = vec![app.tag.clone()];
                        act.color = vec![app.color];
                    } else {
                        act.name.push(app.activity_name.clone());
                        act.total_time.push(app.total_time.unwrap().elapsed());
                        act.tag.push(app.tag.clone());
                        act.color.push(app.color);
                    }

                    app.activity_history = act;
                    app.write_config_file();

                    app.pause_time = None;
                    app.total_pause_time = Duration::default();
                    app.work_time = Duration::default();
                }

                match app.screen {
                    Screen::Pause => {
                        if columns[1].button("Resume").clicked() {
                            app.screen = Screen::Tracking;
                            app.total_time =
                                Some(app.total_time.unwrap() + app.pause_time.unwrap().elapsed());
                            app.pause_time = None;
                        }
                    }
                    _ => {
                        if columns[1].button("Pause").clicked() {
                            app.screen = Screen::Pause;
                            match app.pause_time {
                                Some(_) => app.pause_time = None,
                                None => app.pause_time = Some(Instant::now()),
                            };
                        }
                    }
                }
            });
        });
    });
}

fn activity_listing(
    app: &mut App,
    act: &mut Activity,
    ctx: &egui::Context,
    _frame: &mut eframe::Frame,
    ui: &mut Ui,
) {
    let scroll_area = ScrollArea::vertical().auto_shrink([false; 2]);
    scroll_area.show(ui, |ui| {
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.columns(4, |column| {
                column[0].vertical_centered_justified(|ui| ui.label(blue_text("Name")));
                column[1].vertical_centered_justified(|ui| ui.label(blue_text("Tag")));
                column[2].vertical_centered_justified(|ui| ui.label(blue_text("Time spent")));
                column[3].vertical_centered_justified(|ui| ui.label(red_text("Delete")));
            });

            for (index, name) in act.name.iter().enumerate() {
                ui.columns(4, |column| {
                    let tag = act.tag[index].clone();
                    app.tag = tag.clone();
                    let total_time = &act.total_time[index].as_secs();

                    column[0].vertical_centered_justified(|ui| {
                        app.assign_name(ui, name, index);
                    });
                    column[1].vertical_centered_justified(|ui| {
                        let text = RichText::new(app.tag.clone()).color(act.color[index]);
                        let label = Label::new(text).sense(Sense::click());
                        let r = ui.add(label);
                        r.context_menu(|ui| {
                            app.assign_tag(ui, name, index);
                            if tag != "  " {
                                app.delete_tag(ui, tag);
                            }
                        });
                    });

                    let m = total_time / 60;
                    let s = total_time % 60;
                    let h = m / 60;
                    let m = m % 60;

                    let total_time = format!("{}h {}m {}s", h, m, s);

                    column[2].vertical_centered_justified(|ui| ui.label(total_time));
                    column[3].vertical_centered_justified(|ui| {
                        if ui.button("X").clicked() {
                            app.activity_history = app.read_config_file();

                            let Activity {
                                name,
                                total_time,
                                color,
                                tag,
                            } = &mut app.activity_history;

                            name.remove(index);
                            total_time.remove(index);
                            tag.remove(index);
                            color.remove(index);

                            app.write_config_file();
                        }
                    });
                });
            }
        });
    });

    ctx.request_repaint();
}

fn prepare_tag_for_display(tags: &[String]) -> Vec<String> {
    let mut prev = vec![];
    let mut display: Vec<String> = vec![];
    for tag in tags {
        if prev.contains(tag) || *tag == "  " {
            continue;
        }

        display.push(tag.clone());
        prev.push(tag.clone());
    }

    display
}
