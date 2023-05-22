use super::repository::connection;
use super::repository::module::language::{model, LanguageRepository};

use super::schema::language::LanguageAggregation;

use super::mapper::language::LanguageMapper;

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

        Some(LanguageMapper::map_to_aggregation(language))
    }

    pub async fn get_aggregations(
        connection: &connection::PgConnection,
    ) -> Vec<LanguageAggregation> {
        let languages = LanguageRepository::get_many(connection).await;

        LanguageMapper::map_to_aggregations(languages)
    }
}
