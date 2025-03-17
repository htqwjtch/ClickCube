pub struct Paint;

impl Paint {
    pub fn upload_colors(
        current_colors: Vec<Vec<String>>,
        colors_to_upload: Vec<Option<Vec<String>>>,
    ) -> Vec<Vec<String>> {
        let mut uploaded_colors = Vec::new();

        for i in 0..colors_to_upload.len() {
            let colors = match colors_to_upload[i].clone() {
                Some(colors) => colors,
                None => current_colors[i].clone(),
            };
            uploaded_colors.push(colors);
        }

        uploaded_colors
    }
}
