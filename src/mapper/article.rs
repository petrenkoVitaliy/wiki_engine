use super::schema::article::ArticleAggregation;
use super::schema::article_language::ArticleLanguageAggregation;

use super::repository::module::{
    article::model::Article, article_language::model::ArticleLanguage,
    article_version::model::ArticleVersion, language::model::Language,
};

use super::article_language::ArticleLanguageMapper;
use super::article_version::ArticleVersionMapper;
use super::language::LanguageMapper;

pub struct ArticleMapper {}

impl ArticleMapper {
    pub fn map_to_full_aggregation(
        article: &Article,
        article_language_aggregations: Vec<ArticleLanguageAggregation>,
    ) -> ArticleAggregation {
        ArticleAggregation {
            id: article.id,
            enabled: article.enabled,
            archived: article.archived,
            updated_at: article.updated_at,
            created_at: article.created_at,

            languages: article_language_aggregations,
        }
    }

    pub fn map_relations_to_aggregation(
        article: Article,
        article_language: ArticleLanguage,
        article_version: ArticleVersion,
        language: Language,
    ) -> ArticleAggregation {
        let article_versions_aggregations =
            ArticleVersionMapper::map_to_aggregations(vec![article_version]);

        let languages_aggregation = LanguageMapper::map_to_aggregations(vec![language]);
        let article_language_aggregations = ArticleLanguageMapper::map_to_aggregations(
            vec![article_language],
            article_versions_aggregations,
            languages_aggregation,
        );

        ArticleMapper::map_to_full_aggregation(&article, article_language_aggregations)
    }
}
