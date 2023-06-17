use rocket::serde::Deserialize;
use rocket_okapi::okapi::schemars::JsonSchema;

#[derive(Deserialize, JsonSchema)]
pub struct ArticleLanguageCreateBody {
    pub content: String,
    pub name: String,
}

#[derive(Debug)]
pub struct ArticleLanguageCreateDto {
    pub name: String,
    pub article_id: i32,
    pub language_id: i32,
}

#[derive(Debug)]
pub struct ArticleLanguageCreateRelationsDto {
    pub content: String,
    pub name: String,
    pub language_code: String,
    pub article_id: i32,
}

#[derive(Deserialize, JsonSchema)]
pub struct ArticleLanguagePatchBody {
    pub enabled: Option<bool>,
    pub name: Option<String>,
}

#[derive(Debug)]
pub struct ArticleLanguagePatchDto {
    pub enabled: Option<bool>,
    pub archived: Option<bool>,
    pub name: Option<String>,
}
