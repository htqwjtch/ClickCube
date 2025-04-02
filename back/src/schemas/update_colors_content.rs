use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema, Debug)]
pub struct UpdateColorsContent {
    pub _front: Vec<String>,

    pub _back: Vec<String>,

    pub _up: Vec<String>,

    pub _down: Vec<String>,

    pub _left: Vec<String>,

    pub _right: Vec<String>,
}
