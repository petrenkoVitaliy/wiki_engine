use rocket::{get, response::status, serde::json::Json};
use rocket_okapi::{
    okapi::schemars::gen::SchemaSettings, openapi, openapi_get_routes, settings::OpenApiSettings,
};

use super::repository::PgConnection;

use super::aggregation::language::LanguageAggregation;

use super::service::language::LanguageService;

#[openapi]
#[get("/")]
async fn get_languages(
    connection: PgConnection,
) -> Result<Json<Vec<LanguageAggregation>>, status::Custom<String>> {
    let aggregations = LanguageService::get_aggregations(&connection).await;

    Ok(Json(aggregations))
}

pub fn routes() -> Vec<rocket::Route> {
    let settings = OpenApiSettings {
        json_path: "/languages.json".to_owned(),
        schema_settings: SchemaSettings::openapi3(),
    };

    openapi_get_routes![settings: get_languages,]
}
