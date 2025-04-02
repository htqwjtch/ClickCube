use crate::clients::painter::PainterClient;

pub struct PainterController {}

impl PainterController {
    pub fn update_colors(colors_to_upload: Vec<Vec<String>>) -> Vec<Vec<String>> {
        PainterClient::update_colors(colors_to_upload)
    }
}
