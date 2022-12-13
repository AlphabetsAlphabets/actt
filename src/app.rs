use crate::constants::*;
use crate::screens::{Config, Screen, *};
use rand::{random, Rng};

use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    time::{Duration, Instant},
};

use dirs::config_dir;
use egui::{Button, Color32, Context};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct App {
    pub activity_name: String,
    pub tag_name: String,

    #[serde(skip)]
    pub config_file: PathBuf,
    // This is only used for updating the tags the user has made.
    #[serde(skip)]
    pub config_file_updated: bool,
    #[serde(skip)]
    pub config: Config,

    // This isn't used but I plan to use it to show how much time you spent relaxing.
    // I'm unsure of the exact implementation details though. Cuz I'll have to go through the
    // code for time again.
    #[serde(skip)]
    pub total_pause_time: Duration,
    #[serde(skip)]
    pub pause_time: Option<Instant>,
    #[serde(skip)]
    pub total_time: Option<Instant>,
    #[serde(skip)]
    pub work_time: Duration,

    // This group is for changing the name of an activity.
    #[serde(skip)]
    pub show_name_assign_dialog: bool,
    #[serde(skip)]
    pub new_name: String,
    #[serde(skip)]
    pub target_name_index: usize,

    // This group of tags is used in the `activity_history` function.
    // This is used when the user wishes to create a new tag and assign it
    // to an activity that does not have a tag.
    #[serde(skip)]
    pub show_tag_assign_window: bool,
    #[serde(skip)]
    pub new_tag: String,
    #[serde(skip)]
    pub target_tag: String,
    #[serde(skip)]
    pub target_tag_index: usize,
    #[serde(skip)]
    pub show_color_picker: bool,

    // User preferences
    /// Can be either `"random"` (default) or `"choice"`.
    /// `"random"` -  Assign a random color to avoid the clash. Which means only a text edit to change the name of the tag will appear.
    /// `"choice"` A window pops up containing a text edit asking for the user to input a new tag name, along with a color picker to change the name of the tag.  
    ///
    /// This is needed when a tag is renamed and the color of the tag already exists.
    /// It occurs when there are a group of activities with the same tag, and one of them has their tag changed.
    pub tag_assign_behavior: String,

    // Misc. Ungrouped fields that don't belong to a particular group.
    #[serde(skip)]
    pub screen: Screen,
    #[serde(skip)]
    pub warning: Option<String>,

    /// This is used for visual distinction plus the sunburst.
    pub color: Color32,
    #[serde(skip)]
    pub focus: bool,
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
            Screen::Tracking | Screen::Pause => tracking_screen(self, ctx, _frame),
            Screen::History => history_screen(self, ctx, _frame),
            Screen::Tags => todo!("Tags screen."),
            Screen::Settings => todo!("Settings screen."),
        }
    }
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
            tag_name: "".to_string(),

            config_file: config_file.to_path_buf(),
            config_file_updated: true,
            config: Config::default(),

            total_pause_time: Duration::from_secs(0),
            pause_time: None,
            total_time: None,
            work_time: Duration::from_secs(0),

            show_name_assign_dialog: false,
            new_name: "".to_string(),
            target_name_index: usize::MAX,

            show_tag_assign_window: false,
            target_tag: "".to_string(),
            target_tag_index: 0,
            new_tag: "".to_string(),
            show_color_picker: false,

            // user preferences
            tag_assign_behavior: "random".to_string(),

            screen: Screen::Start,
            warning: None,

            color: Color32::BLACK,
            focus: false,
        }
    }
}

impl App {
    pub fn write_config_file(&mut self) {
        let json = serde_json::to_string(&self.config).unwrap();
        fs::write(&self.config_file, json).unwrap();
        self.config_file_updated = true;
    }

    pub fn read_config_file(&self) -> Config {
        let file = fs::read(&self.config_file).unwrap();
        let contents = std::str::from_utf8(&file[..]).unwrap();
        match serde_json::from_str(contents) {
            Ok(act) => act,
            Err(_) => Config::default(),
        }
    }

    /// Assign a new name to an activity
    pub fn assign_name(&mut self, ui: &mut egui::Ui, name: &String, index: usize) {
        let same_index = self.target_name_index != usize::MAX && index == self.target_name_index;

        if self.show_name_assign_dialog && same_index {
            let r = ui.text_edit_singleline(&mut self.new_name);
            if !self.focus {
                r.request_focus();
                self.focus = true;
            }

            let lost_focus = r.lost_focus();
            let key_pressed = |key: egui::Key| ui.input().key_pressed(key);

            if lost_focus && key_pressed(egui::Key::Enter) {
                if !self.new_name.trim().is_empty() {
                    self.config.entry[index].name = self.new_name.clone();
                    self.write_config_file();
                }
                self.show_name_assign_dialog = false;
                self.focus = false;
                self.target_name_index = usize::MAX;
            } else if lost_focus {
                self.show_name_assign_dialog = false;
                self.focus = false;
                self.target_name_index = usize::MAX;
            }
        } else {
            let btn = egui::Button::new(name).frame(false);
            if ui.add(btn).clicked() {
                self.target_name_index = index;
                self.show_name_assign_dialog = true;
            };
        }
    }

    /// The user can change or assign new tags based on the the cirumstance.
    pub fn change_assign_tag(&mut self, ctx: &Context, index: usize, list_of_colors: &[Color32]) {
        egui::Window::new("").title_bar(false).show(ctx, |ui| {
            ui.label(
                "You can create new tags as well, just type the name of a tag that doesn't exist.",
            );
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("New name");
                ui.text_edit_singleline(&mut self.new_tag);
            });

            // TODO: Add a color picker.
            if self.tag_assign_behavior == "picker" {
                todo!("Add a color picker!");
                // Create a color picker right here.
            } else {
                self.color = self::random_color(&list_of_colors, &self.color, None);
            }

            ui.vertical_centered(|ui| {
                let done_btn = ui.button("Done");
                if !done_btn.clicked() {
                    return;
                }

                if self.new_tag.is_empty() {
                    return;
                }

                let mut config_file = self.read_config_file();
                if config_file.tag_list.contains(&self.new_tag) {
                    todo!("Warn user that tag exists, choose another one.");
                }

                config_file.tag_list.push(self.new_tag.clone());
                self.color = self::random_color(&list_of_colors, &self.color, None);
                config_file.colors.push(self.color.clone());

                if let Some(entry) = config_file.entry.get_mut(index) {
                    entry.tag_index = config_file.tag_list.len() - 1;
                    entry.color_index = config_file.colors.len() - 1;
                }

                self.config = config_file;
                self.write_config_file();

                if self.config_file_updated {
                    self.show_tag_assign_window = false;
                }
            });
        });
    }

    /// target_tag: The tag that is to be deleted.
    pub fn delete_tag(&mut self, ui: &mut egui::Ui, target_tag: String, index: usize) {
        let btn = egui::Button::new("Delete tag").frame(false);
        // FIXME: delete_tag
        // if ui.add(btn).clicked() {
        //     for tag in self.activity.tag.iter_mut() {
        //         if *tag == target_tag {
        //             // Sets the tag to an empty tag. Which signifies "deleted".
        //             *tag = EMPTY_TAG.to_string();
        //             if let Some(color) = self.activity.color.get_mut(index) {
        //                 *color = DEFAULT_TAG_COLOR;
        //             }
        //         }
        //     }

        // self.write_config_file();
        ui.close_menu();
        // }
    }

    /// Adds the details of an activity to `Config`.
    pub fn add_entry(&mut self) {
        // Logic for adding entries to the config file.
        self.screen = Screen::History;
        match self.pause_time {
            Some(pause_time) => {
                // Because `total_time` is an Instant adding it with a Duration makes
                // it so that the Instant began by Duration. Explanation by Dr Nefario:
                // let's say you have an instant for the time of 6AM, and it's currently 7AM.
                // the elapsed time will be 1 hour.
                // but if you add a 5 minute duration to the instant, making it 6:05AM, the elapsed time will now be 55 minutes
                self.total_time = Some(self.total_time.unwrap() + pause_time.elapsed());
            }
            _ => (),
        }

        // TODO: Find a way to make checks for if preferences were changed
        let mut config = self.read_config_file();
        config.total_time.push(self.total_time.unwrap().elapsed());

        if self.does_tag_exist(&config.tag_list, &self.tag_name) {
            let existing_tag_index = self.find_tag(&config.tag_list, &self.tag_name);
            let mut color_index = self.find_color(&config.colors, &self.color);

            if color_index == usize::MAX {
                config.colors.push(self.color.clone());
                color_index = config.colors.len() - 1;
            }

            let new_entry = Entry::new(self.activity_name.clone(), existing_tag_index, color_index);
            config.entry.push(new_entry);
            config.total_time.push(self.total_time.unwrap().elapsed());
        } else {
            // If true means a color already exists. There can't be clashing colors for
            // tags. Therefore a random one will be assigned.
            if self.config.preferences.tag_assign_behavior == "random" {
                if self.find_color(&config.colors, &self.color) != usize::MAX {
                    self.color = self::random_color(&config.colors, &self.color, None);
                }
            }

            config.colors.push(self.color.clone());
            let new_color_index = config.colors.len() - 1;
            config.total_time.push(self.total_time.unwrap().elapsed());

            let new_tag_index = config.tag_list.len();
            let entry = Entry::new(self.activity_name.clone(), new_tag_index, new_color_index);
            config.entry.push(entry);

            config.tag_list.push(self.tag_name.clone());
        }

        config.preferences.tag_assign_behavior = self.tag_assign_behavior.clone();

        self.config = config;
        self.write_config_file();

        self.pause_time = None;
        self.total_pause_time = Duration::default();
        self.work_time = Duration::default();
    }

    pub fn does_tag_exist(&self, tag_list: &[String], cur_tag: &String) -> bool {
        if tag_list.contains(cur_tag) {
            true
        } else {
            false
        }
    }

    pub fn find_tag(&self, tag_list: &[String], tag_to_find: &String) -> usize {
        tag_list.iter().position(|e| e == tag_to_find).unwrap()
    }

    /// Returns `usize::MAX` if that color doesn't exist.
    pub fn find_color(&self, colors: &[Color32], color_to_find: &Color32) -> usize {
        colors
            .iter()
            .position(|e| e == color_to_find)
            .unwrap_or(usize::MAX)
    }
}

fn does_color_exist(colors: &[Color32], color: &Color32) -> bool {
    if colors.contains(&color) {
        true
    } else {
        false
    }
}

pub fn random_color(list_of_colors: &[Color32], color: &Color32, count: Option<usize>) -> Color32 {
    let limit = 256 ^ 3;
    let count = count.unwrap_or(0) + 1;
    let limit_not_reached = !(limit == count);
    let color_exists = does_color_exist(list_of_colors, color);

    if color_exists && limit_not_reached {
        let r = rand::thread_rng().gen_range(0..=255);
        let g = rand::thread_rng().gen_range(0..=255);
        let b = rand::thread_rng().gen_range(0..=255);

        random_color(list_of_colors, &Color32::from_rgb(r, g, b), Some(count))
    } else {
        color.clone()
    }
}
