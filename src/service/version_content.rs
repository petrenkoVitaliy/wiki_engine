use std::collections::{hash_map::Entry, HashMap};

use super::repository::entity::article_version::ArticleVersion;
use super::repository::entity::version_content::VersionContent;

use super::schema::version_content::ContentType;

use super::diff_handler::diff_handler::DiffHandler;
use super::error::error_wrapper::ErrorWrapper;
use super::error::formatted_error::FmtError;

pub struct VersionContentService {}

impl VersionContentService {
    pub fn get_contents_map_by_ids(
        article_versions_with_contents: &Vec<(ArticleVersion, VersionContent)>,
    ) -> Result<HashMap<i32, String>, ErrorWrapper> {
        let mut contents_map: HashMap<i32, String> = HashMap::new();

        let versions_contents_by_language_map = article_versions_with_contents.into_iter().fold(
            HashMap::new(),
            |mut acc, (article_version, version_content)| {
                match acc.entry(article_version.article_language_id) {
                    Entry::Vacant(acc) => {
                        acc.insert(vec![(article_version, version_content)]);
                    }
                    Entry::Occupied(mut acc) => {
                        acc.get_mut().push((article_version, version_content));
                    }
                };

                acc
            },
        );

        for (_, article_versions_with_contents) in versions_contents_by_language_map {
            match Self::update_contents_map(&mut contents_map, &article_versions_with_contents) {
                Err(e) => return Err(e),
                _ => (),
            }
        }

        return Ok(contents_map);
    }

    fn update_contents_map<'s>(
        contents_map: &'s mut HashMap<i32, String>,
        article_versions_with_contents: &Vec<(&ArticleVersion, &VersionContent)>,
    ) -> Result<&'s HashMap<i32, String>, ErrorWrapper> {
        let full_content = match article_versions_with_contents.last() {
            None => return FmtError::NotFound("article_version").error(),
            Some((article_version, version_content)) => {
                if !matches!(version_content.content_type, ContentType::Full) {
                    return FmtError::FailedToProcess("version_content").error();
                }

                (article_version, version_content)
            }
        };

        let (content_map, _) = article_versions_with_contents
            [0..article_versions_with_contents.len() - 1]
            .into_iter()
            .rev()
            .map(|(_, version_content)| version_content)
            .fold(
                (
                    contents_map,
                    DiffHandler::get_string_from_bytes(&full_content.1.content),
                ),
                |(content_map, previous_full_content), version_content| {
                    let content = DiffHandler::get_patch(
                        &version_content.content,
                        version_content.content_length,
                        previous_full_content,
                    );

                    content_map.insert(version_content.id, String::from(&content));

                    return (content_map, content);
                },
            );

        Ok(content_map)
    }
}
