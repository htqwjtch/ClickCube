use axum::extract::Multipart;
use std::{fs, path::Path};
use tokio::{fs::File, io::AsyncWriteExt};

use crate::modules::{color_adapter::ColorAdapter, click};

pub struct ClickClient {}

impl ClickClient {
    pub async fn upload_images(mut multipart: Multipart) -> Result<usize, String> {
        let mut uploaded_files_number = 0;

        for i in 1..=6 {
            let field_name = format!("cube{}", i);
            if let Ok(Some(field)) = multipart.next_field().await {
                let content_type = field.content_type().unwrap_or("unknown").to_string();
                let allowed_types = ["image/png", "image/jpeg", "image/webp"];

                if !allowed_types.contains(&content_type.as_str()) {
                    return Err(format!("Unsupported file type for {}", field_name));
                }

                let filename = field_name.clone() + ".jpg";
                let filepath = format!("./assets/{}", filename);

                let dir = Path::new("./assets");
                if !dir.exists() {
                    fs::create_dir_all(dir)
                        .map_err(|e| format!("Failed to create directory: {}", e))?;
                }

                let mut file = File::create(&filepath)
                    .await
                    .map_err(|e| format!("Failed to create file: {}", e))?;

                let data = field
                    .bytes()
                    .await
                    .map_err(|e| format!("Failed to read file {}: {}", field_name, e))?;

                file.write_all(&data)
                    .await
                    .map_err(|e| format!("Failed to write file: {}", e))?;

                uploaded_files_number += 1;
            } else {
                return Err(format!("File does not exist for {}", field_name));
            }
        }

        Ok(uploaded_files_number)
    }

    pub fn detect_colors() -> Vec<Vec<String>> {
        let detected_colors = click::detect_colors();
        ColorAdapter::set_raw_colors(detected_colors.clone());
        ColorAdapter::receive_raw_colors(detected_colors.clone());
        
        detected_colors
    }
}
