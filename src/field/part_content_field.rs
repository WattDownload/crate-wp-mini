use crate::field::{AuthRequiredFields, DefaultableFields};
use strum_macros::{Display, EnumIter};

/// Represents the fields that can be requested for a `PartContent` object from the Wattpad API.
#[derive(Debug, Clone, Copy, Display, EnumIter, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[strum(serialize_all = "camelCase")]
pub enum PartContentField {
    /// The full text content of the story part.
    Text,

    /// A hash of the text content, likely for caching or integrity checks.
    #[strum(serialize = "text_hash")]
    TextHash,
}

impl AuthRequiredFields for PartContentField {}

impl DefaultableFields for PartContentField {
    fn default_fields() -> Vec<Self> {
        vec![Self::Text]
    }
}