use rocket::serde::Serialize;
use rocket_okapi::okapi::schemars::JsonSchema;

use super::repository::models::language::model::Language;

#[derive(Clone, Serialize, JsonSchema)]
pub struct LanguageAggregation {
    pub id: i32,
    pub code: String,
}

impl LanguageAggregation {
    pub fn from_model(language: Language) -> Self {
        LanguageAggregation {
            id: language.id,
            code: language.code,
        }
    }

    pub fn from_model_list(languages: Vec<Language>) -> Vec<Self> {
        languages
            .into_iter()
            .map(|language| Self::from_model(language))
            .collect()
    }
}
