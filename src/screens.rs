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
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
pub struct Entry {
    pub name: String,
    pub tag_index: usize,
    pub color_index: usize,
}

impl Entry {
    pub fn new(name: String, tag_index: usize, color_index: usize) -> Self {
        Self {
            name,
            tag_index,
            color_index,
        }
    }
}

#[derive(Deserialize, Serialize, Default, Clone)]
struct Preferences {
    /// Can be either `"random"` (default) or `"choice"`.
    /// `"random"` -  Assign a random color to avoid the clash. Which means only a text edit to change the name of the tag will appear.
    /// `"choice"` A window pops up containing a text edit asking for the user to input a new tag name, along with a color picker to change the name of the tag.  
    ///
    /// This is needed when a tag is renamed and the color of the tag already exists.
    /// It occurs when there are a group of activities with the same tag, and one of them has their tag changed.
    tag_assign_behavior: String,
}

// When a new field is added remember to add the change in the delete logic.
// This also applies to the stop logic for adding entries to the config file.
#[derive(Deserialize, Serialize, Default)]
pub struct Config {
    // Activity entry
    pub entry: Vec<Entry>,
    pub total_time: Vec<Duration>,
    pub tag_list: Vec<String>,
    pub colors: Vec<Color32>,
    preferences: Preferences,
}

impl Config {
    /// `value` can be either `"random"` or `"picker"`
    pub fn tag_assign_behavior(&self) -> &String {
        &self.preferences.tag_assign_behavior
    }

    pub fn set_tag_assign_behavior(&mut self, value: String) {
        self.preferences.tag_assign_behavior = value;
    }

    /// Returns `usize::MAX` if tag doesn't exist. Otherwise, returns the tag index.
    pub fn find_tag(&self, tag_list: &[String], tag_to_find: &String) -> usize {
        tag_list
            .iter()
            .position(|e| e == tag_to_find)
            .unwrap_or(usize::MAX)
    }

    /// Returns `usize::MAX` if that color doesn't exist. Otherwise, returns the index of the color
    pub fn find_color(&self, colors: &[Color32], color_to_find: &Color32) -> usize {
        colors
            .iter()
            .position(|e| e == color_to_find)
            .unwrap_or(usize::MAX)
    }

    fn does_color_exist(&self, colors: &[Color32], color: &Color32) -> bool {
        if colors.contains(&color) {
            true
        } else {
            false
        }
    }

    pub fn random_color(
        &self,
        list_of_colors: &[Color32],
        color: &Color32,
        count: Option<usize>,
    ) -> Color32 {
        let limit = 256 ^ 3;
        let count = count.unwrap_or(0) + 1;
        let limit_not_reached = !(limit == count);
        let color_exists = self.does_color_exist(list_of_colors, color);

        if color_exists && limit_not_reached {
            let r = rand::thread_rng().gen_range(0..=255);
            let g = rand::thread_rng().gen_range(0..=255);
            let b = rand::thread_rng().gen_range(0..=255);

            self.random_color(list_of_colors, &Color32::from_rgb(r, g, b), Some(count))
        } else {
            color.clone()
        }
    }
}

#[derive(PartialEq)]
pub enum Screen {
    Start,
    Tracking,
    Pause,
    History,
    Settings,
    Tags,
}

pub fn horizontal_menu(app: &mut App, ui: &mut Ui) {
    ui.horizontal_top(|ui| {
        ui.selectable_value(&mut app.screen, Screen::Start, "Home");
        ui.selectable_value(&mut app.screen, Screen::History, "History");
        ui.selectable_value(&mut app.screen, Screen::Tags, "Tags");
        ui.selectable_value(&mut app.screen, Screen::Settings, "Settings");
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

            if config.entry.len() == 0 || config.tag_list.is_empty() {
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
        let list_of_colors = app.config.colors.clone();

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

                    ui.columns(2, |column| {
                        column[0].vertical_centered_justified(|ui| ui.label("Tag color"));
                        column[1].vertical_centered_justified(|ui| {
                            color_picker_color32(ui, &mut app.color, Alpha::Opaque);
                            let tag_index =
                                app.config.find_tag(&app.config.tag_list, &app.tag_name);
                            let tag_exist = tag_index == usize::MAX;

                            if !tag_exist {
                                app.color = app.config.colors[tag_index];
                            }

                            let does_color_exist =
                                app.config.does_color_exist(&list_of_colors, &app.color);

                            if tag_exist && does_color_exist {
                                app.color =
                                    app.config.random_color(&list_of_colors, &app.color, None);
                            }
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
                    app.add_entry();
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
                        // This is false when the tag is delete, since the tag doesn't exist it
                        // gets an empty string instead.
                        let current_tag = if let Some(current_tag) = tag_list.get_mut(*tag_index) {
                            current_tag.trim()
                        } else {
                            ""
                        };

                        let tag_is_empty = current_tag.is_empty();
                        let text = if tag_is_empty {
                            RichText::new(current_tag.clone()).color(Color32::TRANSPARENT)
                        } else {
                            RichText::new(current_tag.clone()).color(colors[*color_index])
                        };

                        let button = Button::new(text).frame(false);
                        let r = ui.add(button);

                        if !app.show_tag_assign_window {
                            r.context_menu(|ui| {
                                let btn_text = if tag_is_empty {
                                    "Create tag"
                                } else {
                                    "Change tag"
                                };

                                if ui.button(btn_text).clicked() {
                                    app.show_tag_assign_window = true;
                                    app.target_tag_index = index;
                                    ui.close_menu();
                                }
                            });
                        }

                        if app.show_tag_assign_window && index == app.target_tag_index {
                            app.change_or_assign_tag(ctx, index, &colors);
                        }
                    });

                    let minutes = total_time / 60;
                    let seconds = total_time % 60;
                    let hours = minutes / 60;
                    let minutes = minutes % 60;

                    let total_time = format!("{}h {}m {}s", hours, minutes, seconds);

                    // Total time
                    let time_btn = Button::new(total_time).frame(false);
                    column[2].vertical_centered_justified(|ui| ui.add(time_btn));

                    // Delete
                    column[3].vertical_centered_justified(|ui| {
                        if ui.button("X").clicked() {
                            app.config.entry.remove(index);
                            app.config.total_time.remove(index);
                            app.write_config_file();
                        }
                    });
                }
            });
        });
    });

    ctx.request_repaint();
}

pub fn tags_screen(app: &mut App, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
        horizontal_menu(app, ui);

        ui.columns(2, |column| {
            column[0].vertical_centered_justified(|ui| ui.label(blue_text("Tags")));
            column[1].vertical_centered_justified(|ui| ui.label(red_text("Delete")));

            let mut config = app.read_config_file();
            for tag in config.tag_list.iter_mut() {
                column[0].vertical_centered_justified(|ui| ui.label(blue_text(tag)));
                let del_btn = Button::new(red_text("X"));
                column[1].vertical_centered_justified(|ui| {
                    let r = ui.add(del_btn);
                    if r.clicked() {
                        app.delete_tag(tag.clone());
                    }
                    r
                });
            }
        });
    });
}
