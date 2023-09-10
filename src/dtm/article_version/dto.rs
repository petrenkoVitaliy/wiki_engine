use super::repository::entity::article_language::ArticleLanguage;

pub struct ArticleVersionCreateRelationsDto {
    pub content: String,
    pub user_id: i32,
    pub name: Option<String>,
}

pub struct ArticleVersionCreateDto {
    pub version: i32,
    pub content_id: i32,
    pub article_language_id: i32,
    pub user_id: i32,
    pub name: String,
}

pub struct ArticleVersionPatchDto {
    pub enabled: bool,
    pub user_id: i32,
}

pub struct ArticleVersionsJoinSearchDto {
    pub article_languages_ids: Vec<i32>,
    pub version_ge: i32,
}

pub struct LanguageSearchDto {
    pub article_languages_ids: Option<Vec<i32>>,

    pub article_language: Option<ArticleLanguage>,

    pub article_id: Option<i32>,
    pub language_code: Option<String>,

    pub article_language_key: Option<String>,
}
