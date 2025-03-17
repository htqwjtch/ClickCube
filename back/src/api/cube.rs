use axum::Json;

use crate::controllers::cube::CubeController;

#[utoipa::path(
    get,
    path = "/solve",
    description = "Solve cube",
    responses(
        (status = 200, description = "Cube has been solved successfully!", body = Vec<String>)
    )
)]
pub async fn solve() -> Json<Vec<String>> {
    Json(CubeController::solve())
}
