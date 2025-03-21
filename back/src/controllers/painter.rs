use crate::clients::painter::PaintClient;

pub struct PainterController {}

impl PainterController {
    pub fn update_colors(colors_to_upload: Vec<Option<Vec<String>>>) -> Vec<Vec<String>> {
        PaintClient::update_colors(colors_to_upload)
    }
}
