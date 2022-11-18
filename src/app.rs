use crate::screens::{Activity, Screen, *};

use std::path::{Path, PathBuf};
use std::{
    fs,
    time::{Duration, Instant},
};

use dirs::config_dir;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct App {
    pub activity_name: String,
    pub tag: String,
    pub buf: String,

    #[serde(skip)]
    pub config_file: PathBuf,
    #[serde(skip)]
    pub activity_history: Activity,

    #[serde(skip)]
    pub pause_time: Option<Instant>,
    #[serde(skip)]
    pub total_pause_time: Duration,
    #[serde(skip)]
    pub total_time: Option<Instant>,
    #[serde(skip)]
    pub work_time: Duration,

    // This group of tags is used in the `activity_history` function.
    // This is used when the user wishes to create a new tag and assign it
    // to an activity that does not have a tag.
    #[serde(skip)]
    pub target_name: String,
    #[serde(skip)]
    pub focus: bool,
    #[serde(skip)]
    pub show_name_assign_dialog: bool,
    #[serde(skip)]
    pub show_tag_assign_dialog: bool,
    #[serde(skip)]
    pub new_name: String,
    #[serde(skip)]
    pub new_tag: String,
    #[serde(skip)]
    pub display: Vec<String>,

    #[serde(skip)]
    pub screen: Screen,
    #[serde(skip)]
    pub warning: Option<String>,
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
            buf: "".to_string(),

            config_file: config_file.to_path_buf(),
            activity_history: Activity::default(),

            total_time: None,
            pause_time: None,
            total_pause_time: Duration::from_secs(0),
            work_time: Duration::from_secs(0),

            target_name: "".to_string(),
            new_name: "".to_string(),
            new_tag: "".to_string(),
            focus: false,
            show_name_assign_dialog: false,
            show_tag_assign_dialog: false,
            display: vec![],

            screen: Screen::Start,
            warning: None,
        }
    }
}

impl App {
    pub fn write_config_file(&self) {
        let json = serde_json::to_string(&self.activity_history).unwrap();
        fs::write(&self.config_file, json).unwrap();
    }

    pub fn read_config_file(&self) -> Activity {
        let file = fs::read(&self.config_file).unwrap();
        let contents = std::str::from_utf8(&file[..]).unwrap();
        match serde_json::from_str(contents) {
            Ok(act) => act,
            Err(_) => Activity::default(),
        }
    }

    /// Assign a new name to an activity
    pub fn assign_name(&mut self, ui: &mut egui::Ui, name: &String, index: usize) {
        if self.show_name_assign_dialog && self.target_name == *name {
            let r = ui.text_edit_singleline(&mut self.new_name);
            if !self.focus {
                r.request_focus();
                self.focus = true;
            }

            let lost_focus = r.lost_focus();
            let key_pressed = |key: egui::Key| ui.input().key_pressed(key);

            if lost_focus && key_pressed(egui::Key::Enter) {
                if !self.new_name.trim().is_empty() {
                    self.activity_history.name[index] = self.new_name.clone();
                    self.write_config_file();
                }
                self.show_name_assign_dialog = false;
                self.focus = false;
            } else if lost_focus {
                self.show_name_assign_dialog = false;
                self.focus = false;
            }
        } else {
            let btn = egui::Button::new(name).frame(false);
            if ui.add(btn).clicked() {
                self.target_name = name.clone();
                self.show_name_assign_dialog = true;
            };
        }
    }

    /// Used to create new tags or change name of current tag
    pub fn assign_tag(&mut self, ui: &mut egui::Ui, name: &String, index: usize) {
        if self.show_tag_assign_dialog && *name == self.target_name {
            let r = ui.text_edit_singleline(&mut self.new_tag);
            if !self.focus {
                r.request_focus();
                self.focus = true;
            }

            let lost_focus = r.lost_focus();
            let key_pressed = |key: egui::Key| ui.input().key_pressed(key);

            if lost_focus && key_pressed(egui::Key::Enter) {
                self.activity_history.tag[index] = self.new_tag.clone();
                self.write_config_file();
                self.show_tag_assign_dialog = false;
                self.focus = false;
                ui.close_menu();
            } else if r.lost_focus() {
                self.show_tag_assign_dialog = false;
                self.focus = false;
                ui.close_menu();
            }
        } else {
            let btn = egui::Button::new("Change/Assign tag").frame(false);
            if ui.add(btn).clicked() {
                self.target_name = name.clone();
                self.show_tag_assign_dialog = true;
            };
        }
    }

    pub fn delete_tag(&mut self, ui: &mut egui::Ui, tag: String) {
        let btn = egui::Button::new("Delete tag").frame(false);
        if ui.add(btn).clicked() {
            for user_gen_tag in self.activity_history.tag.iter_mut() {
                if *user_gen_tag == tag {
                    *user_gen_tag = "  ".to_string();
                }
            }

            self.write_config_file();
            ui.close_menu();
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
            Screen::Tracking | Screen::Pause => tracking_screen(self, ctx, _frame),
            Screen::History => history_screen(self, ctx, _frame),
        }
    }
}
