use super::repository::entity::article::ArticleType;

pub struct ArticlePatchDto {
    pub id: i32,
    pub user_id: i32,
    pub enabled: Option<bool>,
    pub archived: Option<bool>,
    pub article_type: Option<ArticleType>,
}

pub struct ArticleCreateDto {
    pub article_type: ArticleType,
    pub user_id: i32,
}

pub struct ArticleCreateRelationsDto {
    pub content: String,
    pub language: String,
    pub name: String,
    pub article_type: ArticleType,
    pub user_id: i32,
}
