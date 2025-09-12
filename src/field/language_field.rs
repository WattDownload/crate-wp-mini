use crate::field::{AuthRequiredFields, DefaultableFields};
use strum_macros::{Display, EnumIter};

/// Represents the fields that can be requested for a `Language` object from the Wattpad API.
#[derive(Debug, Clone, Copy, Display, EnumIter, PartialEq, Eq)]
#[strum(serialize_all = "camelCase")]
pub enum LanguageField {
    /// The unique numerical identifier of the language.
    Id,
    /// The full name of the language (e.g., "English").
    Name,
}

impl AuthRequiredFields for LanguageField {}

impl DefaultableFields for LanguageField {
    fn default_fields() -> Vec<Self> {
        vec![Self::Id]
    }
}