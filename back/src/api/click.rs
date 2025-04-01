use axum::{extract::Multipart, response::IntoResponse, Json};

use crate::{
    controllers::click,
    schemas::{response::Response, upload_images_content::UploadImagesContent},
};

#[utoipa::path(
    post,
    path = "/upload-images",
    description = "Upload cube images on server",
    request_body(
        content = UploadImagesContent,
        description = "Multipart form-data containing an image file",
        content_type = "multipart/form-data"
    ),
    responses(
        (status = 200, description = "Images have been uploaded successfully!", body = Response<String>),
        (status = 415, description = "Unsupported file type", body = Response<String>),
        (status = 500, description = "Internal server error", body = Response<String>)
    )
)]
pub async fn upload_images(multipart: Multipart) -> impl IntoResponse {
    Json(click::ClickController::upload_images(multipart).await)
}

#[utoipa::path(
    get,
    path = "/detect-colors",
    description = "Detect colors of cube elements on uploaded images",
    responses(
        (status = 200, description = "Detected colors", body = Vec<Vec<String>>),
    )
)]
pub async fn detect_colors() -> impl IntoResponse {
    Json(click::ClickController::detect_colors())
}
