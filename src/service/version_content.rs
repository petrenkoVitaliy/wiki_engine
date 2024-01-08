use std::collections::{hash_map::Entry, HashMap};

use super::diff_handler::DiffHandler;
use super::error::FmtError;

use super::repository::entity::{
    article_version::ArticleVersion,
    auth::UserAccount,
    version_content::{ContentType, VersionContent},
};

pub struct VersionContentService;

impl VersionContentService {
    pub fn get_contents_map_by_ids(
        article_versions_with_contents: &Vec<(ArticleVersion, VersionContent, UserAccount)>,
    ) -> HashMap<i32, String> {
        let mut contents_map: HashMap<i32, String> = HashMap::new();

        let versions_contents_by_language_map = article_versions_with_contents.into_iter().fold(
            HashMap::new(),
            |mut acc, (article_version, version_content, _)| {
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
            Self::update_contents_map(&mut contents_map, &article_versions_with_contents);
        }

        return contents_map;
    }

    fn update_contents_map<'s>(
        contents_map: &'s mut HashMap<i32, String>,
        article_versions_with_contents: &Vec<(&ArticleVersion, &VersionContent)>,
    ) -> &'s HashMap<i32, String> {
        let full_content = article_versions_with_contents
            .get(0)
            .expect(&FmtError::NotFound("article_version").fmt());

        if !matches!(full_content.1.content_type, ContentType::Full) {
            panic!("{}", &FmtError::FailedToProcess("version_content").fmt());
        }

        let (content_map, _) = article_versions_with_contents
            [1..article_versions_with_contents.len()]
            .into_iter()
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

        content_map
    }
}
