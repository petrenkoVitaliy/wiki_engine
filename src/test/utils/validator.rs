use super::aggregation::{
    article::ArticleAggregation, article_language::ArticleLanguageAggregation,
    article_version::ArticleVersionAggregation, language::LanguageAggregation,
    version_content::VersionContentAggregation,
};

pub struct ArticleResponseValidator {}

impl ArticleResponseValidator {
    pub fn validate_article_aggregation(
        received_aggregation: ArticleAggregation,
        expected_aggregation: ArticleAggregation,
    ) -> () {
        assert_eq!(received_aggregation.archived, expected_aggregation.archived);
        assert_eq!(received_aggregation.enabled, expected_aggregation.enabled);
        assert_eq!(
            received_aggregation.updated_at,
            expected_aggregation.updated_at
        );

        Self::validate_article_languages_aggregation(
            &received_aggregation.languages,
            &expected_aggregation.languages,
        )
    }

    pub fn validate_version_content_aggregation(
        received_version_content: &VersionContentAggregation,
        expected_version_content: &VersionContentAggregation,
    ) -> () {
        assert_eq!(
            received_version_content.content,
            expected_version_content.content
        );
    }

    pub fn validate_language_aggregation(
        received_language: &LanguageAggregation,
        expected_language: &LanguageAggregation,
    ) -> () {
        assert_eq!(received_language.code, expected_language.code);
    }

    pub fn validate_article_versions_aggregation(
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

            Self::validate_version_content_aggregation(
                &received_versions[i].content,
                &expected_versions[i].content,
            );
        }
    }

    pub fn validate_article_languages_aggregation(
        received_languages: &Vec<ArticleLanguageAggregation>,
        expected_languages: &Vec<ArticleLanguageAggregation>,
    ) -> () {
        assert_eq!(received_languages.len(), expected_languages.len());

        for i in 0..received_languages.len() {
            assert_eq!(received_languages[i].name, expected_languages[i].name);
            assert_eq!(received_languages[i].enabled, expected_languages[i].enabled);
            assert_eq!(
                received_languages[i].archived,
                expected_languages[i].archived
            );
            assert_eq!(
                received_languages[i].updated_at,
                expected_languages[i].updated_at
            );

            Self::validate_article_versions_aggregation(
                &received_languages[i].versions,
                &expected_languages[i].versions,
            );

            Self::validate_language_aggregation(
                &received_languages[i].language,
                &expected_languages[i].language,
            );
        }
    }
}
