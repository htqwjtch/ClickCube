use axum::Json;

use crate::controllers::cube;

#[utoipa::path(
    get,
    path = "/solve",
    description = "Solve cube",
    responses(
        (status = 200, description = "Cube has been solved successfully!", body = Vec<[[String; 3]; 3]>)
    )
)]
pub async fn solve() -> Json<Vec<[[String; 3]; 3]>> {
    let colors = Vec::new();
    Json(cube::CubeController::solve(colors))
}
