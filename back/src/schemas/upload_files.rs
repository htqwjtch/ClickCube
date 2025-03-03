use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct UploadFiles {
    #[schema(value_type = String, format = Binary)]
    pub _image1: Option<Vec<u8>>,

    #[schema(value_type = String, format = Binary)]
    pub _image2: Option<Vec<u8>>,

    #[schema(value_type = String, format = Binary)]
    pub _image3: Option<Vec<u8>>,

    #[schema(value_type = String, format = Binary)]
    pub _image4: Option<Vec<u8>>,

    #[schema(value_type = String, format = Binary)]
    pub _image5: Option<Vec<u8>>,

    #[schema(value_type = String, format = Binary)]
    pub _image6: Option<Vec<u8>>,
}
