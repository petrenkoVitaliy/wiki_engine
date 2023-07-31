pub struct ArticleLanguageCreateDto {
    pub name: String,
    pub article_id: i32,
    pub language_id: i32,
    pub user_id: i32,
}

pub struct ArticleLanguageCreateRelationsDto {
    pub content: String,
    pub name: String,
    pub language_code: String,
    pub article_id: i32,
    pub user_id: i32,
}

pub struct ArticleLanguagePatchDto {
    pub enabled: Option<bool>,
    pub archived: Option<bool>,
    pub name: Option<String>,
    pub user_id: i32,
}
