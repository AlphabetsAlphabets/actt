use crate::constants::*;
use crate::screens::{Activity, Screen, *};
use rand::{random, Rng};

use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    time::{Duration, Instant},
};

use dirs::config_dir;
use egui::{Color32, Context};

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
    pub activity: Activity,

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
    pub show_tag_assign_dialog: bool,
    #[serde(skip)]
    pub new_tag: String,
    #[serde(skip)]
    pub target_tag: String,
    #[serde(skip)]
    pub target_tag_index: usize,
    #[serde(skip)]
    pub show_color_picker: bool,

    #[serde(skip)]
    pub display_ready_tags: HashMap<String, Vec<usize>>,

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

    // This is used for visual distinction plus the sunburst.
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
            activity: Activity::default(),

            total_pause_time: Duration::from_secs(0),
            pause_time: None,
            total_time: None,
            work_time: Duration::from_secs(0),

            show_name_assign_dialog: false,
            new_name: "".to_string(),
            target_name_index: usize::MAX,

            show_tag_assign_dialog: false,
            target_tag: "".to_string(),
            target_tag_index: 0,
            new_tag: "".to_string(),
            show_color_picker: false,

            display_ready_tags: HashMap::default(),

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
        let json = serde_json::to_string(&self.activity).unwrap();
        fs::write(&self.config_file, json).unwrap();
        self.config_file_updated = true;
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
                    self.activity.entry[index].name = self.new_name.clone();
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

    /// Used to create new tags or change name of current tag
    pub fn assign_tag(&mut self, ctx: &Context, ui: &mut egui::Ui, tag: &String, index: usize) {
        // TODO: Turn this into a window instead. To do two things.
        // 1. Assign/Change tag as usual. When changing tags, only the *specific* tag is changed.
        //    Everything else remains the same. It just makes more sense. Changing multiple tags is
        //    with the check boxes.
        // 2. Pick a color for a tag.
        // Reason being that when you delete a tag, you can't reassign a color.
        if self.activity.colors[index] == Color32::TRANSPARENT {
            self.assign_tag_after_deletion(ctx, ui);
        } else if self.show_tag_assign_dialog && index == self.target_tag_index {
            let text_edit = ui.text_edit_singleline(&mut self.new_tag);
            if !self.focus {
                text_edit.request_focus();
                self.focus = true;
            }

            let lost_focus = text_edit.lost_focus();
            let key_pressed = |key: egui::Key| ui.input().key_pressed(key);

            if lost_focus && key_pressed(egui::Key::Enter) {
                // TODO: This needs to be reworked to accomodate changes in the config file.

                // This check is to see if there are any clashing colors. When renaming a tag, a
                // new color must be chosen. I can go three routes.
                // 1. Ask the user what color is to be chosen.
                // 2. The color is randomly assigned.
                // 3. Implement both (a preference to be set in the options menu).

                // Randomly assigns a tag color if two tags with the same color exists.
                let Self { activity, .. } = self;
                let Activity { colors, .. } = activity;
                if self.tag_assign_behavior == "random" {
                    if let Some(cur_color) = colors.get(index) {
                        if does_color_exist(&colors, cur_color) {
                            colors[index] = random_color(&colors, cur_color, None);
                        }
                    }
                }

                self.write_config_file();
                self.show_tag_assign_dialog = false;
                self.focus = false;

                ui.close_menu();
            } else if text_edit.lost_focus() {
                self.show_tag_assign_dialog = false;
                self.focus = false;

                ui.close_menu();
            }
        } else {
            let btn = egui::Button::new("Change/Assign tag").frame(false);
            if ui.add(btn).clicked() {
                self.target_tag_index = index;
                self.target_tag = tag.clone();
                self.show_tag_assign_dialog = true;
            };
        }
    }

    /// color - Check if the color exists.
    /// Returns a new color that is unique. If `color` is unique, returns `color`.
    pub fn assign_tag_after_deletion(&mut self, ctx: &Context, ui: &mut egui::Ui) {
        egui::Window::new("Title").show(ctx, |ui| ui.label("Hello"));
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

fn random_color(colors: &[Color32], color: &Color32, count: Option<usize>) -> Color32 {
    let limit = 255 ^ 3;
    let count = count.unwrap_or(0) + 1;
    let limit_not_reached = !(limit == count);

    if limit_not_reached {
        let r = rand::thread_rng().gen_range(0..=255);
        let g = rand::thread_rng().gen_range(0..=255);
        let b = rand::thread_rng().gen_range(0..=255);

        random_color(colors, &Color32::from_rgb(r, g, b), Some(count))
    } else {
        color.clone()
    }
}
