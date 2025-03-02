use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct UploadFiles {
    #[schema(value_type = String, format = Binary)]
    pub _image_with_orange_center: Option<Vec<u8>>,

    #[schema(value_type = String, format = Binary)]
    pub _image_with_red_center: Option<Vec<u8>>,

    #[schema(value_type = String, format = Binary)]
    pub _image_with_yellow_center: Option<Vec<u8>>,

    #[schema(value_type = String, format = Binary)]
    pub _image_with_white_center: Option<Vec<u8>>,

    #[schema(value_type = String, format = Binary)]
    pub _image_with_green_center: Option<Vec<u8>>,

    #[schema(value_type = String, format = Binary)]
    pub _image_with_blue_center: Option<Vec<u8>>,
}
