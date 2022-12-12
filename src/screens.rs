use crate::app;
use crate::colors::*;
use crate::constants::*;
use crate::App;

use std::time::{Duration, Instant};

use egui::Button;
use egui::{
    color_picker::{color_picker_color32, Alpha},
    Color32, RichText, ScrollArea, Ui, Vec2,
};

use egui_dropdown::DropDownBox;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
pub struct Entry {
    pub name: String,
    pub tag_index: usize,
    pub color_index: usize,
}

impl Entry {
    fn new(name: String, tag_index: usize, color_index: usize) -> Self {
        Self {
            name,
            tag_index,
            color_index,
        }
    }
}

#[derive(Deserialize, Serialize, Default)]
struct Preferences {
    /// Can be `"random"` (default), or `"picker"`.
    /// - `random` - A color is assigned at random when there are matching colors.
    /// - `picker` - A color wheel is shown to the user, allowing them to pick and choose a color.
    tag_assign_behavior: String,
}

// When a new field is added remember to add the change in the delete logic.
// This also applies to the stop logic for adding entries to the config file.
#[derive(Deserialize, Serialize, Default)]
pub struct Config {
    // Activity entry
    pub entry: Vec<Entry>,
    total_time: Vec<Duration>,
    pub tag_list: Vec<String>,
    pub colors: Vec<Color32>,
    preferences: Preferences,
}

impl Config {
    pub fn get_tags(&self) -> Vec<String> {
        self.tag_list.clone()
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

            let mut config = app.read_config_file();

            if config.entry.len() == 0 {
                ui.label("It's empty!");
            } else {
                activity_listing(app, &mut config, ctx, _frame, ui);
            }
        });
    });
}

/// The start screen is where metadata about an activity is set.
pub fn start_screen(app: &mut App, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    // There's nothing wrong with the return type. It's just that `CentralPanel` is also a function
    // Which means that the return type needs to cover that as well.
    egui::CentralPanel::default().show(ctx, |ui| {
        app.config = app.read_config_file();

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
                    // 1 column per action.
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
                            // This is needed for when a tag is delete. Since an empty tag is two
                            // blank spaces, in the tag text edit there will be two spaces
                            // inserted. This is a fix for it.
                            if app.tag_name == EMPTY_TAG {
                                app.tag_name = "".to_string();
                            }

                            ui.add(DropDownBox::from_iter(
                                &app.config.tag_list.clone(),
                                "tags",
                                &mut app.tag_name,
                                |ui, text| {
                                    let r = ui.selectable_label(false, text);
                                    r
                                },
                            ))
                            .on_hover_text("What category is this activity under?");
                        });
                    });

                    let tag_list = &mut app.config.tag_list;
                    let mut tag_list_iter = tag_list.iter_mut();
                    if let Some(color_index) =
                        tag_list_iter.position(|e| *e == app.tag_name)
                    {
                        app.color = app.config.colors[color_index];
                    }

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
                } else if app.config.preferences.tag_assign_behavior == "picker" {
                    app.warning = Some(
                        "Please pick a different color, that one has already been chosen."
                            .to_string(),
                    );
                } else {
                    if app.tag_name.is_empty() {
                        app.tag_name = EMPTY_TAG.to_string();
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

            let header = RichText::new(app.activity_name.clone()).size(32.0);
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
                    // Logic for adding entries to the config file.
                    app.screen = Screen::History;
                    match app.pause_time {
                        Some(pause_time) => {
                            // Because `total_time` is an Instant adding it with a Duration makes
                            // it so that the Instant began by Duration. Explanation by Dr Nefario:
                            // let's say you have an instant for the time of 6AM, and it's currently 7AM.
                            // the elapsed time will be 1 hour.
                            // but if you add a 5 minute duration to the instant, making it 6:05AM, the elapsed time will now be 55 minutes
                            app.total_time = Some(app.total_time.unwrap() + pause_time.elapsed());
                        }
                        _ => (),
                    }

                    // TODO: Find a way to make checks for if preferences were changed
                    let mut config = app.read_config_file();
                    config.total_time.push(app.total_time.unwrap().elapsed());

                    if app.does_tag_exist(&config.tag_list, &app.tag_name) {
                        let existing_tag_index = app.find_tag(&config.tag_list, &app.tag_name);
                        let mut color_index = app.find_color(&config.colors, &app.color);

                        if color_index == usize::MAX {
                            config.colors.push(app.color.clone());
                            color_index = config.colors.len() - 1;
                        }

                        let new_entry =
                            Entry::new(app.activity_name.clone(), existing_tag_index, color_index);
                        config.entry.push(new_entry);
                        config.total_time.push(app.total_time.unwrap().elapsed());
                    } else {
                        // If true means a color already exists. There can't be clashing colors for
                        // tags. Therefore a random one will be assigned.
                        if app.config.preferences.tag_assign_behavior == "random" {
                            if app.find_color(&config.colors, &app.color) != usize::MAX {
                                app.color = app::random_color(&config.colors, &app.color, None);
                            }
                        }

                        config.colors.push(app.color.clone());
                        let new_color_index = config.colors.len() - 1;
                        config.total_time.push(app.total_time.unwrap().elapsed());

                        let new_tag_index = config.tag_list.len();
                        let entry =
                            Entry::new(app.activity_name.clone(), new_tag_index, new_color_index);
                        config.entry.push(entry);

                        config.tag_list.push(app.tag_name.clone());
                    }

                    config.preferences.tag_assign_behavior = app.tag_assign_behavior.clone();

                    app.config = config;
                    app.write_config_file();

                    app.pause_time = None;
                    app.total_pause_time = Duration::default();
                    app.work_time = Duration::default();
                }

                match app.screen {
                    Screen::Pause => {
                        if columns[1].button("Resume").clicked() {
                            app.screen = Screen::Tracking;
                            // Total run time of a task.
                            app.total_time =
                                Some(app.total_time.unwrap() + app.pause_time.unwrap().elapsed());
                            app.pause_time = None;
                        }
                    }
                    _ => {
                        if columns[1].button("Pause").clicked() {
                            app.screen = Screen::Pause;
                            match app.pause_time {
                                // Some means paused before, don't want that. Reset the value.
                                Some(_) => app.pause_time = None,
                                // None means first pause.
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
    config: &mut Config,
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

                let Config {
                    total_time,
                    tag_list,
                    colors,
                    ..
                } = config;

                for (index, entry) in config.entry.iter().enumerate() {
                    let total_time = total_time[index].as_secs();
                    let Entry {
                        name,
                        tag_index,
                        color_index,
                    } = entry;

                    // Name
                    column[0].vertical_centered_justified(|ui| {
                        app.assign_name(ui, name, index);
                    });

                    // Tag
                    column[1].vertical_centered_justified(|ui| {
                        let current_tag = &tag_list[*tag_index].trim().to_string();
                        let text = RichText::new(current_tag.clone()).color(colors[*color_index]);
                        let button = Button::new(text).frame(false);
                        let r = ui.add(button);

                        if app.show_tag_assign_window {
                            egui::Window::new("").title_bar(false).show(ctx, |ui| {
                                ui.horizontal(|ui| {
                                    ui.label("New name");
                                    ui.text_edit_singleline(&mut app.new_tag);
                                });

                                // TODO: Add a color picker.
                                if app.tag_assign_behavior == "picker" {
                                    // Create a color picker right here.
                                } else {
                                    let list_of_colors = &app.config.colors;
                                    app.color = app::random_color(list_of_colors, &app.color, None);
                                }

                                ui.vertical_centered(|ui| {
<<<<<<< HEAD
                                    if ui.button("Done").clicked() {
=======
                                    if app.new_tag.trim().is_empty() {
                                        app.show_tag_assign_window = false;
                                    } else if ui.button("Done").clicked() {
>>>>>>> 4363dbd (You can now change name of tag.)
                                        let mut config_file = app.read_config_file();
                                        let tag_index = config_file.entry[index].tag_index;
                                        config_file.tag_list[tag_index] = app.new_tag.clone();

                                        app.config = config_file;
                                        app.write_config_file();

                                        app.show_tag_assign_window = false;
                                    }
                                });
                            });
                        } else {
                            r.context_menu(|ui| {
                                if ui.button("Change tag").clicked() {
                                    app.show_tag_assign_window = true;
                                    ui.close_menu();
                                }
                            });
                        }
                    });

                    let minutes = total_time / 60;
                    let seconds = total_time % 60;
                    let hours = minutes / 60;
                    let minutes = minutes % 60;

                    let total_time = format!("{}h {}m {}s", hours, minutes, seconds);

                    // Total time
                    column[2].vertical_centered_justified(|ui| ui.label(total_time));

                    column[3].vertical_centered_justified(|ui| {
                        if ui.button("X").clicked() {
                            app.config.entry.remove(index);
                            app.config.total_time.remove(index);
                            app.write_config_file();
                        }
                    });
                }

                // Delete
            });
        });
    });

    ctx.request_repaint();
}
