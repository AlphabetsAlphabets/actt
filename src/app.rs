#![feature(path_try_exists)]

use super::screens::{Activity, Screen, *};

use std::path::{Path, PathBuf};
use std::{
    fs,
    time::{Duration, Instant},
};

use dirs::config_dir;
use egui::{Color32, RichText};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct App {
    pub activity_name: String,
    pub tag: String,

    #[serde(skip)]
    pub config_file: PathBuf,
    #[serde(skip)]
    pub activity_history: Activity,
    #[serde(skip)]
    pub total_time: Duration,
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
        let home = config_dir().unwrap();
        let home = format!("{}/actt", home.display());
        // This is ok
        let home = Path::new(&home).to_owned();
        if !Path::try_exists(&home).unwrap() {
            fs::create_dir(&home).unwrap();
        }

        let config_file = format!("{}/actt.json", home.display());
        let config_file = Path::new(&config_file);
        if !Path::try_exists(config_file).unwrap() {
            fs::File::create(config_file).unwrap();
        }

        Self {
            activity_name: "".to_string(),
            tag: "".to_string(),
            warning: None,

            now: None,
            config_file: config_file.to_path_buf(),
            activity_history: Activity::default(),
            total_time: Duration::from_secs(0),
            pause_time: Duration::from_secs(0),
            work_time: Duration::from_secs(0),

            screen: Screen::Start,
        }
    }
}

impl App {
    pub fn read_config_file(&self) -> Option<Activity> {
        let file = fs::read(&self.config_file).unwrap();
        let contents = std::str::from_utf8(&file[..]).unwrap();
        match serde_json::from_str(contents) {
            Ok(act) => Some(act),
            Err(_) => None,
        }
    }

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
            Screen::Tracking => tracking_screen(self, ctx, _frame),
            Screen::Pause => pause_screen(self, ctx, _frame),
            Screen::History => history_screen(self, ctx, _frame),
        }
    }
}
