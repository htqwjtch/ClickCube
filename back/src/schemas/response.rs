use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct Response<T> {
    pub code: u16,
    pub message: String,
    pub info: T,
}
