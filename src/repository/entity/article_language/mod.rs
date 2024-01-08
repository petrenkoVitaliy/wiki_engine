use super::article;
use super::connection;
use super::db_schema;
use super::dtm;
use super::dtm_common;
use super::error;
use super::language;

mod model;
mod repository;

pub use self::model::ArticleLanguage;
pub use self::repository::ArticleLanguageRepository;
