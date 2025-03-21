use axum::{response::IntoResponse, Json};

use crate::{
    controllers::handmade::HandMadeController, schemas::turn_cube_face_content::TurnCubeFaceContent,
};

#[utoipa::path(
    post,
    path = "/turn-cube-face",
    description = "Turn cube face",
    request_body(
        content = TurnCubeFaceContent,
        description = "Vector of Strings for colors of cube faces, String for command to turn cube face",
        content_type = "application/json"
    ),
    responses(
        (status = 200, description = "Cube face has been turned successfully!", body = Vec<Vec<String>>)
    )
)]
pub async fn turn_cube_face(Json(payload): Json<TurnCubeFaceContent>) -> impl IntoResponse {
    Json(HandMadeController::turn_cube_face(
        payload.colors,
        payload.command,
    ))
}
