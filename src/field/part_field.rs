use crate::field::text_url_field::TextUrlField;
use crate::field::{AuthRequiredFields, DefaultableFields, StoryField};
use crate::impl_field_display;
use strum_macros::AsRefStr;

/// Represents the fields that can be requested for a `Part` object from the Wattpad API.
#[derive(Debug, Clone, AsRefStr, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[strum(serialize_all = "camelCase")]
pub enum PartField {
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
    /// The timestamp when the part was last modified.
    ModifyDate,
    /// The timestamp when the part was created.
    CreateDate,
    /// A boolean flag indicating if the part contains images that have been banned.
    HasBannedImages,
    /// The length of the story part, often representing an estimated reading time in seconds.
    Length,
    /// The ID of any video associated with the part.
    VideoID,
    /// The URL for the part's cover image.
    PhotoUrl,
    /// The total number of comments on the part.
    CommentCount,
    /// The total number of votes the part has received.
    VoteCount,
    /// The total number of reads the part has received.
    ReadCount,
    /// The unique identifier of the parent story (also known as group ID).
    GroupId,

    /// The parent story.
    #[strum(disabled)]
    Group(Vec<StoryField>),

    /// A boolean flag indicating whether the part has been deleted.
    Deleted,
}

impl_field_display!(
    PartField,
    TextUrl => "text_url",
    Group => "group"
);

impl AuthRequiredFields for PartField {}

impl DefaultableFields for PartField {
    fn default_fields() -> Vec<Self> {
        vec![Self::Id, Self::Title, Self::Url]
    }
}