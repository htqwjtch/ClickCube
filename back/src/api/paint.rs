use axum::Json;

use crate::{controllers::paint::PaintController, schemas::upload_colors::UploadColors};

#[utoipa::path(
    post,
    path = "/upload-colors",
    description = "Upload colors on cube faces",
    request_body(
        content = UploadColors,
        description = "Vectors of String for colors of cube faces",
        content_type = "application/json"
    ),
    responses(
        (status = 200, description = "Colors has been uploaded successfully!", body = Vec<Vec<String>>)
    )
)]
pub async fn upload_colors(Json(colors): Json<UploadColors>) -> Json<Vec<Vec<String>>> {
    let colors_to_upload = vec![
        colors._front,
        colors._back,
        colors._up,
        colors._down,
        colors._left,
        colors._right,
    ];
    Json(PaintController::upload_colors(colors_to_upload))
}
