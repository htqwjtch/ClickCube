use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema, Debug)]
pub struct TurnCubeFaceContent {
    pub colors: Vec<Vec<String>>,

    pub command: String,
}
