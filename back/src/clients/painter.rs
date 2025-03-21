use crate::modules::{color_adapter::ColorAdapter, painter::Paint};

pub struct PaintClient {}

impl PaintClient {
    pub fn update_colors(colors_to_upload: Vec<Option<Vec<String>>>) -> Vec<Vec<String>> {
        let current_colors = ColorAdapter::get_raw_colors();
        let all_colors = Paint::update_colors(current_colors, colors_to_upload);
        ColorAdapter::set_raw_colors(all_colors.clone());
        ColorAdapter::receive_raw_colors(all_colors.clone());
        all_colors
    }
}
