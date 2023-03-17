use super::Preferences;
use super::Entry;

use std::time::Duration;

use rand::Rng;
use egui::Color32;

use derivative::Derivative;
use serde::{Deserialize, Serialize};

// When a new field is added remember to add the change in the delete logic.
// This also applies to the stop logic for adding entries to the config file.
#[derive(Derivative, Deserialize, Serialize, Default)]
pub struct Config {
    // Activity entry
    pub entry: Vec<Entry>,
    pub total_time: Vec<Duration>,
    pub tag_list: Vec<String>,
    pub colors: Vec<Color32>,

    #[derivative(Default(value = "Preferences::default()"))]
    pub preferences: Preferences,
}

impl Config {
    /// `value` can be either `"random"` or `"picker"`
    pub fn tag_assign_behavior(&self) -> &String {
        &self.preferences.tag_assign_behavior
    }

    /// Returns `usize::MAX` if tag doesn't exist. Otherwise, returns the tag index.
    pub fn find_tag(&self, tag_list: &[String], tag_to_find: &String) -> usize {
        tag_list
            .iter()
            .position(|e| e == tag_to_find)
            .unwrap_or(usize::MAX)
    }

    /// Returns `usize::MAX` if that color doesn't exist. Otherwise, returns the index of the color
    pub fn find_color(&self, colors: &[Color32], color_to_find: &Color32) -> usize {
        colors
            .iter()
            .position(|e| e == color_to_find)
            .unwrap_or(usize::MAX)
    }

    pub fn does_color_exist(&self, colors: &[Color32], color: &Color32) -> bool {
        if colors.contains(&color) {
            true
        } else {
            false
        }
    }

    pub fn random_color(
        &self,
        list_of_colors: &[Color32],
        color: &Color32,
        count: Option<usize>,
    ) -> Color32 {
        let limit = 256 ^ 3;
        let count = count.unwrap_or(0) + 1;
        let limit_not_reached = !(limit == count);
        let color_exists = self.does_color_exist(list_of_colors, color);

        if color_exists && limit_not_reached {
            let r = rand::thread_rng().gen_range(0..=255);
            let g = rand::thread_rng().gen_range(0..=255);
            let b = rand::thread_rng().gen_range(0..=255);

            self.random_color(list_of_colors, &Color32::from_rgb(r, g, b), Some(count))
        } else {
            color.clone()
        }
    }
}
