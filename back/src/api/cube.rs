use axum::Json;

use crate::controllers::cube;

#[utoipa::path(
    get,
    path = "/resolve_cube",
    responses(
        (status = 200, description = "Resolve cube", body = Vec<[[String; 3]; 3]>)
    )
)]
pub async fn resolve_cube() -> Json<Vec<[[String; 3]; 3]>> {
    let colors = Vec::new();
    Json(cube::CubeController::solve(colors))
}
