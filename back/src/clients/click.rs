use crate::modules::click;

pub struct ClickClient {}

impl ClickClient {
    pub fn detect_colors(_images: Vec<String>) -> Vec<Vec<String>> {
        click::detect_colors()
    }
}
