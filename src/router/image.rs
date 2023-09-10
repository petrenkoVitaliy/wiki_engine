use rocket::{post, response::status, serde::json::Json};
use rocket_okapi::{
    okapi::schemars::gen::SchemaSettings, openapi, openapi_get_routes, settings::OpenApiSettings,
};

use super::trait_common::DtoConvert;

use super::aggregation::image::ImageAggregation;
use super::dtm::image::request_body::ImageCreateBody;

use super::service::image::ImageService;

#[openapi]
#[post("/", data = "<images_body>")]
async fn create_image(
    images_body: Json<Vec<ImageCreateBody>>,
) -> Result<Json<Vec<ImageAggregation>>, status::Custom<String>> {
    let dtos = images_body
        .0
        .into_iter()
        .map(|body| body.into_dto(()))
        .collect();

    match ImageService::upload_images(dtos).await {
        Ok(created_images) => Ok(Json(created_images)),
        Err(e) => Err(e.custom()),
    }
}

pub fn routes() -> Vec<rocket::Route> {
    let settings = OpenApiSettings {
        json_path: "/image.json".to_owned(),
        schema_settings: SchemaSettings::openapi3(),
    };

    openapi_get_routes![settings: create_image]
}
