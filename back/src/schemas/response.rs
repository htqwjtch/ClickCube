use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct Response {
    pub code: u16,
    pub message: String,
    pub data: Option<String>,
}
