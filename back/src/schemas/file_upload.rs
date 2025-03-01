use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct FileUpload {
    #[schema(format = "binary")]
    file: String,
}
