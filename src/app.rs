use super::screens::{Screen, *};
use egui::{Color32, RichText};
use std::time::{Duration, Instant};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct App {
    pub activity: String,
    pub tag: String,

    #[serde(skip)]
    pub pause_time: Duration,
    #[serde(skip)]
    pub now: Option<Instant>,
    #[serde(skip)]
    pub work_time: Duration,
    #[serde(skip)]
    pub warning: Option<String>,
    #[serde(skip)]
    pub screen: Screen,
}

impl Default for App {
    fn default() -> Self {
        Self {
            activity: "".to_string(),
            tag: "".to_string(),
            warning: None,

            now: None,
            pause_time: Duration::from_secs(0),
            work_time: Duration::from_secs(0),

            screen: Screen::Start,
        }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for App {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.screen {
            Screen::Start => start_screen(self, ctx, _frame),
            _ => None,
        };
    }
}
