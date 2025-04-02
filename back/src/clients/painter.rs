use crate::modules::color_adapter::ColorAdapter;

pub struct PainterClient {}

impl PainterClient {
    pub fn update_colors(colors_to_upload: Vec<Vec<String>>) -> Vec<Vec<String>> {
        ColorAdapter::receive_raw_colors(colors_to_upload.clone());
        println!("{:?}", colors_to_upload);
        colors_to_upload
    }
}
