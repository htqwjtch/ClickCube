use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema, Debug)]
pub struct UpdateColorsContent {
    pub _front: Option<Vec<String>>,

    pub _back: Option<Vec<String>>,

    pub _up: Option<Vec<String>>,

    pub _down: Option<Vec<String>>,

    pub _left: Option<Vec<String>>,

    pub _right: Option<Vec<String>>,
}
