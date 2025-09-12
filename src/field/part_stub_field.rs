use crate::field::text_url_field::TextUrlField;
use crate::field::{AuthRequiredFields, DefaultableFields};
use crate::impl_field_display;
use strum_macros::AsRefStr;

/// Represents the fields for a `PartStub` object.
///
/// A `PartStub` is typically a lightweight or summary representation of a story part,
/// often used when a full `Part` object is not required.
#[derive(Debug, Clone, AsRefStr, PartialEq, Eq)]
#[strum(serialize_all = "camelCase")]
pub enum PartStubField {
    /// The unique numerical identifier of the story part.
    Id,
    /// The title of the story part.
    Title,
    /// A direct URL to the story part on the Wattpad website.
    Url,

    /// A complex field for URLs related to the part's text content, with selectable sub-fields.
    #[strum(disabled)]
    TextUrl(Vec<TextUrlField>),

    /// The content rating of the story part.
    Rating,
    /// A boolean flag indicating whether the part is a draft.
    Draft,
    /// The timestamp when the part was created.
    CreateDate,
    /// The timestamp when the part was last modified.
    ModifyDate,
    /// A boolean flag indicating if the part contains images that have been banned.
    HasBannedImages,
    /// The length of the story part, often representing an estimated reading time in seconds.
    Length,
    /// The ID of any video associated with the part.
    VideoId,
    /// The URL for the part's cover image.
    PhotoUrl,
    /// The total number of comments on the part.
    CommentCount,
    /// The total number of votes the part has received.
    VoteCount,
    /// The total number of reads the part has received.
    ReadCount,

    /// A boolean flag indicating if the currently authenticated user has voted for this part.
    /// **Requires authentication.**
    Voted,
    /// A boolean flag indicating whether the part has been deleted.
    Deleted,
}

impl_field_display!(
    PartStubField,
    TextUrl => "text_url"
);

impl AuthRequiredFields for PartStubField {
    fn auth_required_fields() -> Vec<Self> {
        vec![Self::Voted]
    }
}

impl DefaultableFields for PartStubField {
    fn default_fields() -> Vec<Self> {
        vec![
            Self::Id,
            Self::Title,
            Self::TextUrl(vec![TextUrlField::Text]),
            Self::Rating,
            Self::VideoId,
            Self::PhotoUrl,
            Self::ModifyDate,
        ]
    }
}

impl PartStubField {
    /// A constant slice of fields that require authentication.
    ///
    /// Note: This is largely superseded by the `AuthRequiredFields` trait implementation
    /// but is kept for potential internal checks.
    pub(crate) const AUTH_REQUIRED_FIELDS: &'static [Self] = &[
        Self::Voted,
        Self::Deleted, // TODO: Find if this is not in the stub or can be retrieved
    ];
}