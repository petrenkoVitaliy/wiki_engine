use super::aggregation::language::LanguageAggregation;

use super::repository::{
    entity::language::{Language, LanguageRepository},
    PgConnection,
};

pub struct LanguageService;

impl LanguageService {
    pub async fn get_one(connection: &PgConnection, code: String) -> Option<Language> {
        LanguageRepository::get_one(connection, code).await
    }

    pub async fn get_aggregation(
        connection: &PgConnection,
        code: String,
    ) -> Option<LanguageAggregation> {
        let language = match LanguageRepository::get_one(connection, code).await {
            None => return None,
            Some(language) => language,
        };

        Some(LanguageAggregation::from_model(language))
    }

    pub async fn get_aggregations(connection: &PgConnection) -> Vec<LanguageAggregation> {
        let languages = LanguageRepository::get_many(connection).await;

        LanguageAggregation::from_model_list(languages)
    }
}
