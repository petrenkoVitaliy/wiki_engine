use super::aggregation::{
    article_language::ArticleLanguageAggregation, article_version::ArticleVersionAggregation,
    language::LanguageAggregation, version_content::VersionContentAggregation,
};

pub struct ArticleLanguageAssertHandler {}

impl ArticleLanguageAssertHandler {
    pub fn assert_article_languages_aggregation(
        received_language: &ArticleLanguageAggregation,
        expected_language: &ArticleLanguageAggregation,
    ) -> () {
        assert_eq!(received_language.name, expected_language.name);
        assert_eq!(received_language.enabled, expected_language.enabled);
        assert_eq!(received_language.archived, expected_language.archived);
        assert_eq!(received_language.updated_at, expected_language.updated_at);

        Self::assert_article_versions_aggregation(
            &received_language.versions,
            &expected_language.versions,
        );

        Self::validate_language_aggregation(
            &received_language.language,
            &expected_language.language,
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

    fn validate_language_aggregation(
        received_language: &LanguageAggregation,
        expected_language: &LanguageAggregation,
    ) -> () {
        assert_eq!(received_language.code, expected_language.code);
    }

    fn assert_article_versions_aggregation(
        received_versions: &Vec<ArticleVersionAggregation>,
        expected_versions: &Vec<ArticleVersionAggregation>,
    ) -> () {
        assert_eq!(received_versions.len(), expected_versions.len());

        for i in 0..received_versions.len() {
            assert_eq!(received_versions[i].version, expected_versions[i].version);
            assert_eq!(received_versions[i].enabled, expected_versions[i].enabled);
            assert_eq!(
                received_versions[i].updated_at,
                expected_versions[i].updated_at
            );

            Self::assert_version_content_aggregation(
                &received_versions[i].content,
                &expected_versions[i].content,
            );
        }
    }
}
