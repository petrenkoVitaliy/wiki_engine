use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars::JsonSchema;

use super::repository::entity::language::Language;

#[derive(Clone, Serialize, Deserialize, JsonSchema)]
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
