use crate::screens::*;

pub struct App {
    /// Name of the current activity.
    pub activity_name: String,
    /// The tag of the current activity.
    pub tag_name: String,

    pub screen: Screen,
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for App {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.screen {
            Screen::Start => self.start_screen(ctx, _frame),
            Screen::Tracking => self.start_screen(ctx, _frame),
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            activity_name: "".to_string(),
            tag_name: "".to_string(),
            screen: Screen::Start,
        }
    }
}

impl App {
    /// Assign a new name to an activity
    pub fn assign_name(&mut self, ui: &mut egui::Ui, name: &String, index: usize) {}
}
