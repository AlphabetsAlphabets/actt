use egui::{Color32, RichText};

pub fn blue_text(text: &str) -> RichText {
    RichText::new(text).color(Color32::LIGHT_BLUE)
}

pub fn red_text(text: &str) -> RichText {
    RichText::new(text).color(Color32::LIGHT_RED)
}
