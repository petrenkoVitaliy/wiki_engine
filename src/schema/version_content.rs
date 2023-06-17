pub use super::ContentType;

pub struct VersionContentDto {
    pub content: Vec<u8>,
    pub content_type: ContentType,
}
