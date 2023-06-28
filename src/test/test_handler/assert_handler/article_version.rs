use super::aggregation::{
    article_version::ArticleVersionAggregation, version_content::VersionContentAggregation,
};

pub struct ArticleVersionAssertHandler;

impl ArticleVersionAssertHandler {
    pub fn assert_article_version_aggregation(
        received_version: &ArticleVersionAggregation,
        expected_version: &ArticleVersionAggregation,
    ) -> () {
        assert_eq!(received_version.version, expected_version.version);
        assert_eq!(received_version.enabled, expected_version.enabled);
        assert_eq!(received_version.updated_at, received_version.updated_at);

        Self::assert_version_content_aggregation(
            &received_version.content,
            &expected_version.content,
        );
    }

    fn assert_version_content_aggregation(
        received_version_content: &VersionContentAggregation,
        expected_version_content: &VersionContentAggregation,
    ) -> () {
        assert_eq!(
            received_version_content.content,
            expected_version_content.content
        );
    }
}
