use crate::field::{AuthRequiredFields, DefaultableFields};
use strum_macros::{Display, EnumIter};

/// Represents the sub-fields for a `text_url` object.
///
/// This object typically provides expiring URLs for accessing story part content.
#[derive(Debug, Clone, Copy, Display, EnumIter, PartialEq, Eq)]
#[strum(serialize_all = "camelCase")]
pub enum TextUrlField {
    /// The direct, often temporary, URL to the story part's text content.
    Text,
    /// A token that can be used to refresh or obtain a new URL for the text content.
    #[strum(serialize = "refresh_token")]
    RefreshToken,
}

impl AuthRequiredFields for TextUrlField {}

impl DefaultableFields for TextUrlField {
    fn default_fields() -> Vec<Self> {
        vec![Self::Text]
    }
}