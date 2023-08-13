use crate::screens::*;

use std::time::{Duration, Instant};

use dirs::config_dir;
use egui::Color32;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
pub struct App {
    /// Even if it is unsued, ignore this warning. This is useful for passing in data.
    /// This way I won't need to keep redeclaring it.
    pub tmp: String,

    /// Name of the current activity.
    pub activity_name: String,
    /// The tag of the current activity.
    pub tag_name: String,

    /// A tally of how long the user paused an activity throughout the
    /// entire run of the activity.
    pub total_pause_time: Duration,
    /// The amount of time the user is pause for each time, instead of throughout
    /// the entire run of the activity.
    pub pause_time: Option<Instant>,
    /// The amount of time passed when conducting the activity.
    /// `total_time = pause_time + work_time`
    pub total_time: Option<Instant>,
    /// The amount of time the user is worked on an activity.
    pub work_time: Duration,

    // This group is for changing the name of an activity.
    /// The dialog box to be shown when the user when they want
    /// to change the name of an activity in the history screen.
    pub show_name_assign_dialog: bool,
    // The new name of the activity.
    pub new_name: String,
    /// The index of `new_name`. Check how `Config` stores data.
    pub target_name_index: usize,

    // This group of tags is used in the `activity_history` function.
    // This is used when the user wishes to create a new tag and assign it
    // to an activity that does not have a tag.
    /// Check to see if a new tag needs to be created.
    pub create_tag: bool,
    /// Check to see if the user wants to switch to a new tag.
    /// Determines whether the dialogue to create a tag will appear on screen.
    pub show_create_tag_win: bool,

    pub change_tag: bool,
    /// Determines whether the dialogue to change a tag will appear on screen.
    pub show_change_tag_win: bool,

    /// The new tag the user creates.
    pub new_tag: String,
    /// The original tag to be changed
    pub target_tag: String,
    /// The index of `target_tag`
    pub target_tag_index: usize,
    /// UNUSED.
    /// Boolean to determine whether to show the color picker on screen.
    pub show_color_picker: bool,

    // Misc. Ungrouped fields that don't belong to a particular group.
    /// Keeps track of which screen the user is currently on.
    pub screen: Screen,
    /// The message to display when there is an error of some sort.
    pub warning: Option<String>,

    /// Color for tags
    pub color: Color32,
    /// Identifies which text box should be focused.
    pub focus: bool,
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
            Screen::Start => start_screen(self, ctx, _frame),
            Screen::Tracking => start_screen(self, ctx, _frame),
        }
    }
}

impl Default for App {
    fn default() -> Self {
        // Gets the home directory (platform independent)
        let home = config_dir().unwrap();
        let home = format!("{}/actt", home.display());

        Self {
            tmp: "".to_string(),
            activity_name: "".to_string(),
            tag_name: "".to_string(),

            total_pause_time: Duration::from_secs(0),
            pause_time: None,
            total_time: None,
            work_time: Duration::from_secs(0),

            show_name_assign_dialog: false,
            new_name: "".to_string(),
            target_name_index: usize::MAX,

            create_tag: false,
            show_create_tag_win: false,

            change_tag: false,
            show_change_tag_win: false,

            target_tag: "".to_string(),
            target_tag_index: 0,
            new_tag: "".to_string(),
            show_color_picker: false,

            screen: Screen::Start,
            warning: None,

            color: Color32::BLACK,
            focus: false,
        }
    }
}

impl App {
    /// Assign a new name to an activity
    pub fn assign_name(&mut self, ui: &mut egui::Ui, name: &String, index: usize) {}
}
