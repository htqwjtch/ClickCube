use crate::clients::click;

pub struct ClickController {}

impl ClickController {
    pub fn detect_colors(images: Vec<String>) -> Vec<Vec<String>> {
        click::ClickClient::detect_colors(images)
    }
}
