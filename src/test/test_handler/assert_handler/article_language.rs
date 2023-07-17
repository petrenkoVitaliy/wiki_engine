use super::aggregation::{
    article_language::ArticleLanguageAggregation, language::LanguageAggregation,
};

use super::article_version::{ArticleVersionAssertHandler, ArticleVersionAssertOptions};

pub struct ArticleLanguageAssertOptions {
    pub is_updated: bool,
}

pub struct ArticleLanguageAssertHandler;

impl ArticleLanguageAssertHandler {
    pub fn assert_article_languages_aggregation(
        received_language: &ArticleLanguageAggregation,
        expected_language: &ArticleLanguageAggregation,
        assert_options: ArticleLanguageAssertOptions,
    ) -> () {
        assert_eq!(received_language.name, expected_language.name);
        assert_eq!(received_language.enabled, expected_language.enabled);
        assert_eq!(received_language.archived, expected_language.archived);

        if assert_options.is_updated {
            assert_eq!(received_language.updated_at.is_some(), true);
        } else {
            assert_eq!(received_language.updated_at, expected_language.updated_at);
        }

        Self::validate_language_aggregation(
            &received_language.language,
            &expected_language.language,
        );

        ArticleVersionAssertHandler::assert_article_version_aggregation(
            &received_language.version,
            &expected_language.version,
            ArticleVersionAssertOptions { is_updated: false },
        )
    }

    fn validate_language_aggregation(
        received_language: &LanguageAggregation,
        expected_language: &LanguageAggregation,
    ) -> () {
        assert_eq!(received_language.code, expected_language.code);
    }
}
