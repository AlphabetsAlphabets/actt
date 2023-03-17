use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Preferences {
    /// Can be either `"random"` (default) or `"choice"`.
    /// `"random"` -  Assign a random color to avoid the clash. Which means only a text edit to change the name of the tag will appear.
    /// `"choice"` A window pops up containing a text edit asking for the user to input a new tag name, along with a color picker to change the name of the tag.
    /// This is needed when a tag is renamed and the color of the tag already exists.
    /// It occurs when there are a group of activities with the same tag, and one of them has their tag changed.
    pub tag_assign_behavior: String,
}

impl Default for Preferences {
    fn default() -> Self {
        Self {
            tag_assign_behavior: "random".to_string(),
        }
    }
}
