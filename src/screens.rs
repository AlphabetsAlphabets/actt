use crate::app::App;

use egui::{Ui, Vec2};

#[derive(PartialEq)]
pub enum Screen {
    Start,
    Tracking,
}

impl App {
    pub fn horizontal_menu(&mut self, ui: &mut Ui) {
        ui.horizontal_top(|ui| {
            ui.selectable_value(&mut self.screen, Screen::Start, "Home");
        });

        ui.separator();
    }

    /// The start screen is where metadata about an activity is set.
    pub fn start_screen(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // There's nothing wrong with the return type. It's just that `CentralPanel` is also a function
        // Which means that the return type needs to cover that as well.
        egui::CentralPanel::default().show(ctx, |ui| {
            self.horizontal_menu(ui);
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.heading("Home");
                ui.label("\n\n\n");

                ui.allocate_ui_with_layout(
                    Vec2::new(200.0, 200.0),
                    egui::Layout::top_down(egui::Align::Center),
                    |ui| {
                        // Activity column
                        ui.columns(2, |column| {
                            column[0].vertical_centered_justified(|ui| ui.label("Activity"));
                            column[1].vertical_centered_justified(|ui| {
                                ui.text_edit_singleline(&mut self.activity_name)
                                    .on_hover_text("What do you want to track?")
                            });
                        });

                        // Tag column
                        ui.columns(2, |column| {
                            column[0].vertical_centered_justified(|ui| ui.label("Tag"));
                            column[1].vertical_centered_justified(|ui| {});
                        });

                        // Color column
                        ui.columns(2, |column| {
                            column[0].vertical_centered_justified(|ui| ui.label("Tag color"));
                            column[1].vertical_centered_justified(|ui| {
                                ui.label("Insert color picker here.");
                            });
                        });
                    },
                );

                ui.label("\n");
                if ui.button("Start").clicked() {
                    self.screen = Screen::Tracking;
                }
            });
        });
    }
}
