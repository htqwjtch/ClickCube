use crate::clients::paint::PaintClient;

pub struct PaintController {}

impl PaintController {
    pub fn upload_colors(colors_to_upload: Vec<Option<Vec<String>>>) -> Vec<Vec<String>> {
        PaintClient::upload_colors(colors_to_upload)
    }
}
