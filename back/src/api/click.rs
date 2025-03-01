use axum::{extract::Multipart, http::StatusCode, response::IntoResponse, Json};
use tokio::{fs::File, io::AsyncWriteExt};

use crate::schemas::response::Response;

#[utoipa::path(
    post,
    path = "/upload",
    request_body(
        content = String,
        description = "Multipart form-data containing an image file",
        content_type = "image/png, image/jpeg, image/webp"
    ),
    responses(
        (status = 200, description = "Upload images", body = Response)
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
        message: "Image has been uploaded successfully".to_string(),
        data: Some("Image has been uploaded successfully".to_string()),
    })
}
