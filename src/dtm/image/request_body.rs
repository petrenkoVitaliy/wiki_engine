use rocket::serde::Deserialize;
use rocket_okapi::okapi::schemars::JsonSchema;

use super::trait_common::DtoConvert;

use super::dto::ImageCreateDto;

#[derive(Deserialize, JsonSchema, Clone)]
pub struct ImageCreateBody {
    id: i32,
    base64: String,
    format: String,
}

impl DtoConvert<ImageCreateDto> for ImageCreateBody {
    type TParams = ();

    fn into_dto(self, _params: Self::TParams) -> ImageCreateDto {
        ImageCreateDto {
            id: self.id,
            base64: self.base64,
            format: self.format,
        }
    }
}
