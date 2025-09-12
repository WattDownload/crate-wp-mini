use crate::field::DefaultableFields;
use strum_macros::{Display, EnumIter};

/// Represents the fields for a `PartReference` object.
///
/// A `PartReference` is typically a lightweight link to a story part,
/// often found within a list of parts in a story object.
#[derive(Debug, Clone, Copy, Display, EnumIter, PartialEq, Eq)]
#[strum(serialize_all = "camelCase")]
pub enum PartReferenceField {
    /// The unique numerical identifier of the story part.
    Id,
    /// The timestamp when the part was created.
    CreateDate,
}

impl DefaultableFields for PartReferenceField {
    fn default_fields() -> Vec<Self> {
        vec![Self::Id]
    }
}