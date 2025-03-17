use axum::{extract::Multipart, http::StatusCode, response::IntoResponse, Json};

use crate::clients::click::ClickClient;
use crate::schemas::response::Response;

pub struct ClickController {}

impl ClickController {
    pub async fn upload_images(multipart: Multipart) -> impl IntoResponse {
        return match ClickClient::upload_images(multipart).await {
            Ok(num) => Json(Response {
                code: StatusCode::OK.as_u16(),
                message: StatusCode::OK.to_string(),
                info: format!("{} images have been uploaded successfully!", num),
            }),
            Err(e) => {
                if e.contains("Unsupported file type") {
                    Json(Response {
                        code: StatusCode::UNSUPPORTED_MEDIA_TYPE.as_u16(),
                        message: StatusCode::UNSUPPORTED_MEDIA_TYPE.to_string(),
                        info: format!("Unsupported file type for {}", e),
                    })
                } else if e.contains("Failed to create directory")
                    || e.contains("Failed to create file")
                    || e.contains("Failed to read file")
                    || e.contains("Failed to write file")
                    || e.contains("File does not exist")
                {
                    Json(Response {
                        code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                        message: StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                        info: e,
                    })
                } else {
                    Json(Response {
                        code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                        message: StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                        info: "Something went wrong".to_string(),
                    })
                }
            }
        };
    }

    pub fn detect_colors() -> Vec<Vec<String>> {
        ClickClient::detect_colors()
    }
}
