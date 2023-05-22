use super::repository::module::language::model::Language;

use super::schema::language::LanguageAggregation;

pub struct LanguageMapper {}

impl LanguageMapper {
    pub fn map_to_aggregations(languages: Vec<Language>) -> Vec<LanguageAggregation> {
        languages
            .into_iter()
            .map(|language| LanguageMapper::map_to_aggregation(language))
            .collect()
    }

    pub fn map_to_aggregation(language: Language) -> LanguageAggregation {
        LanguageAggregation {
            id: language.id,
            code: language.code,
        }
    }
}
