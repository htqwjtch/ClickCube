use crate::modules::{adapter::Adapter, paint::Paint};

pub struct PaintClient {}

impl PaintClient {
    pub fn upload_colors(colors_to_upload: Vec<Option<Vec<String>>>) -> Vec<Vec<String>> {
        let current_colors = Adapter::get_raw_colors();
        let all_colors = Paint::upload_colors(current_colors, colors_to_upload);
        Adapter::set_raw_colors(all_colors.clone());
        Adapter::adapt_raw_colors(all_colors.clone());
        all_colors
    }
}
