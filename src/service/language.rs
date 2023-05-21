use super::repository::connection;
use super::repository::module::language::{model, LanguageRepository};

use super::schema::language::LanguageAggregation;

pub struct LanguageService {}

impl LanguageService {
    pub async fn get_aggregations(
        connection: &connection::PgConnection,
    ) -> Vec<LanguageAggregation> {
        let languages = LanguageRepository::get_many(connection).await;

        LanguageService::map_to_aggregations(languages)
    }

    pub fn map_to_aggregations(languages: Vec<model::Language>) -> Vec<LanguageAggregation> {
        languages
            .into_iter()
            .map(|language| LanguageAggregation {
                id: language.id,
                code: language.code,
            })
            .collect()
    }
}
