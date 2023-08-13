use crate::screens::*;

use egui::{Context, Ui};

#[derive(Default)]
pub struct App {
    /// Name of the current activity.
    pub activity_name: String,
    /// The tag of the current activity.
    pub tag_name: String,

    pub screen: Screen,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        match self.screen {
            Screen::Start => self.start_screen(ctx, _frame),
            Screen::Tracking => self.start_screen(ctx, _frame),
        }
    }
}

impl App {
    /// Assign a new name to an activity
    pub fn assign_name(&mut self, ui: &mut Ui, name: &String, index: usize) {}
}
