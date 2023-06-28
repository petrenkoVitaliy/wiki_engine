use super::aggregation::{
    article_language::ArticleLanguageAggregation, language::LanguageAggregation,
};

use super::article_version::ArticleVersionAssertHandler;

pub struct ArticleLanguageAssertHandler;

impl ArticleLanguageAssertHandler {
    pub fn assert_article_languages_aggregation(
        received_language: &ArticleLanguageAggregation,
        expected_language: &ArticleLanguageAggregation,
    ) -> () {
        assert_eq!(received_language.name, expected_language.name);
        assert_eq!(received_language.enabled, expected_language.enabled);
        assert_eq!(received_language.archived, expected_language.archived);
        assert_eq!(received_language.updated_at, expected_language.updated_at);

        Self::validate_language_aggregation(
            &received_language.language,
            &expected_language.language,
        );

        assert_eq!(
            received_language.versions.len(),
            expected_language.versions.len()
        );

        for i in 0..received_language.versions.len() {
            ArticleVersionAssertHandler::assert_article_version_aggregation(
                &received_language.versions[i],
                &expected_language.versions[i],
            )
        }
    }

    fn validate_language_aggregation(
        received_language: &LanguageAggregation,
        expected_language: &LanguageAggregation,
    ) -> () {
        assert_eq!(received_language.code, expected_language.code);
    }
}
