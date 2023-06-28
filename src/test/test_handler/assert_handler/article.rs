use super::aggregation::article::ArticleAggregation;

use super::article_language::ArticleLanguageAssertHandler;

pub struct ArticleAssertHandler;

impl ArticleAssertHandler {
    pub fn assert_article_aggregation(
        received_aggregation: ArticleAggregation,
        expected_aggregation: ArticleAggregation,
    ) -> () {
        assert_eq!(received_aggregation.archived, expected_aggregation.archived);
        assert_eq!(received_aggregation.enabled, expected_aggregation.enabled);
        assert_eq!(
            received_aggregation.updated_at,
            expected_aggregation.updated_at
        );

        assert_eq!(
            received_aggregation.languages.len(),
            expected_aggregation.languages.len()
        );

        for i in 0..received_aggregation.languages.len() {
            ArticleLanguageAssertHandler::assert_article_languages_aggregation(
                &received_aggregation.languages[i],
                &expected_aggregation.languages[i],
            )
        }
    }
}
