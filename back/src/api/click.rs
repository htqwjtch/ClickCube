use crate::controllers::click;
use crate::models::response::Response;

use axum::{http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::Multipart;
use tokio::{fs::File, io::AsyncWriteExt};

#[utoipa::path(
    post,
    path = "/upload_images",
    request_body(
        content = String,
        description = "Multipart form-data containing an image file",
        content_type = "multi-part/form-data"
    ),
    responses(
            (status = 200, description = "File uploaded successfully", body = Response),
            (status = 415, description = "Unsupported file type", body = Response),
            (status = 500, description = "Internal server error", body = Response)
        
    )
)]
pub async fn upload_images(mut multipart: Multipart) -> impl IntoResponse {
    while let Ok(Some(field)) = multipart.next_field().await {
        let content_type = field.content_type().unwrap_or("unknown").to_string();

        // Разрешенные типы файлов
        let allowed_types = ["image/png", "image/jpeg", "image/webp"];
        if !allowed_types.contains(&content_type.as_str()) {
            return Json(Response {
                code: StatusCode::UNSUPPORTED_MEDIA_TYPE.as_u16(),
                message: "Unsupported file type".to_string(),
                data: None,
            });
        }

        let filename = field.file_name().unwrap_or("uploaded_image").to_string();
        let filepath = format!("./assets/{}", filename);

        let mut file = File::create(filepath)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
            .unwrap();

        let data = field
            .bytes()
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
            .unwrap();
        file.write_all(&data)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
            .unwrap();
    }

    Json(Response {
        code: StatusCode::OK.as_u16(),
        message: StatusCode::OK.to_string(),
        data: Some("Image has been uploaded successfully".to_string()),
    })
}

#[utoipa::path(
    get,
    path = "/detect_colors",
    responses(
        (status = 200, description = "Detect colors", body = Vec<Vec<String>>)
    )
)]
pub async fn detect_colors() -> Json<Vec<Vec<String>>> {
    let images = Vec::new();
    Json(click::ClickController::detect_colors(images))
}
