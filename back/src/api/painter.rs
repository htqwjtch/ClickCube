use axum::{response::IntoResponse, Json};

use crate::{
    controllers::painter::PainterController, schemas::update_colors_content::UpdateColorsContent,
};

#[utoipa::path(
    post,
    path = "/update-colors",
    description = "Update colors on cube faces",
    request_body(
        content = UpdateColorsContent,
        description = "Vectors of String for colors of cube faces",
        content_type = "application/json"
    ),
    responses(
        (status = 200, description = "Colors has been updated successfully!", body = Vec<Vec<String>>)
    )
)]
pub async fn update_colors(Json(colors): Json<UpdateColorsContent>) -> impl IntoResponse {
    let colors_to_upload = vec![
        colors._front,
        colors._back,
        colors._up,
        colors._down,
        colors._left,
        colors._right,
    ];
    Json(PainterController::update_colors(colors_to_upload))
}
