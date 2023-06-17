use super::repository::connection;
use super::repository::models::language::{model, LanguageRepository};

use super::aggregation::language::LanguageAggregation;

pub struct LanguageService {}

impl LanguageService {
    pub async fn get_one(
        connection: &connection::PgConnection,
        code: String,
    ) -> Option<model::Language> {
        LanguageRepository::get_one(connection, code).await
    }

    pub async fn get_aggregation(
        connection: &connection::PgConnection,
        code: String,
    ) -> Option<LanguageAggregation> {
        let language = match LanguageRepository::get_one(connection, code).await {
            None => return None,
            Some(language) => language,
        };

        Some(LanguageAggregation::from_model(language))
    }

    pub async fn get_aggregations(
        connection: &connection::PgConnection,
    ) -> Vec<LanguageAggregation> {
        let languages = LanguageRepository::get_many(connection).await;

        LanguageAggregation::from_model_list(languages)
    }
}
