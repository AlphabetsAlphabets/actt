use crate::app::App;

use crate::constants::*;

use egui::{
    color_picker::{color_picker_color32, Alpha},
    Ui, Vec2,
};

#[derive(PartialEq)]
pub enum Screen {
    Start,
    Tracking,
}

pub fn horizontal_menu(app: &mut App, ui: &mut Ui) {
    ui.horizontal_top(|ui| {
        ui.selectable_value(&mut app.screen, Screen::Start, "Home");
    });

    ui.separator();
}

/// The start screen is where metadata about an activity is set.
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
                unimplemented!();
            }
        });
    });
}
