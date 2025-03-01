use axum::Json;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Item {
    pub id: i32,
    pub name: String,
}

#[utoipa::path(
    get,
    path = "/resolve_cube",
    responses(
        (status = 200, description = "Resolve cube", body = [Item])
    )
)]
pub async fn resolve_cube() -> Json<Vec<Item>> {
    Json(vec![
        Item {
            id: 1,
            name: "Item1".to_string(),
        },
        Item {
            id: 2,
            name: "Item2".to_string(),
        },
    ])
}
