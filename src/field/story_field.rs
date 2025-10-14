use crate::field::user_stub_field::UserStubField;
use crate::field::{part_reference_field::PartReferenceField, part_stub_field::PartStubField, LanguageField};
use crate::field::{AuthRequiredFields, DefaultableFields};
use crate::impl_field_display;
use strum_macros::AsRefStr;

/// Represents the fields that can be requested for a `Story` object from the Wattpad API.
#[derive(Debug, Clone, AsRefStr, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[strum(serialize_all = "camelCase")]
pub enum StoryField {
    /// The unique numerical identifier of the story.
    Id,
    /// The title of the story.
    Title,
    /// The estimated reading time of the story in seconds.
    Length,
    /// The timestamp when the story was created.
    CreatedDate,
    /// The timestamp when the story was last modified.
    ModifyDate,
    /// The total number of votes the story has received.
    VoteCount,
    /// The total number of reads the story has received.
    ReadCount,
    /// The total number of comments on the story.
    CommentCount,
    /// The numerical identifier for the story's language.
    Language(Vec<LanguageField>),

    /// A complex field representing the author of the story, with selectable sub-fields.
    #[strum(disabled)]
    User(Vec<UserStubField>),

    /// The story's description or synopsis.
    Description,
    /// The URL for the story's cover image.
    Cover,

    /// The timestamp when the cover image was last updated.
    #[strum(serialize = "cover_timestamp")]
    CoverTimestamp,

    /// A boolean flag indicating whether the story is marked as complete.
    Completed,
    /// A list of category IDs that the story belongs to.
    Categories,
    /// A list of user-defined tags associated with the story.
    Tags,
    /// The content rating of the story (e.g., Everyone, Mature).
    Rating,
    /// A boolean flag indicating if the story is intended for a mature audience.
    Mature,
    /// The copyright or license identifier for the story.
    Copyright,
    /// A direct URL to the story on the Wattpad website.
    Url,
    /// The total number of published parts in the story.
    NumParts,
    /// The unique identifier of the first part of the story.
    FirstPartId,

    /// A complex field for a lightweight reference to the first published part, with sub-fields.
    #[strum(disabled)]
    FirstPublishedPart(Vec<PartReferenceField>),

    /// A complex field for a lightweight reference to the last published part, with sub-fields.
    #[strum(disabled)]
    LastPublishedPart(Vec<PartReferenceField>),

    /// A complex field for the list of parts in the story, with selectable sub-fields for each part.
    #[strum(disabled)]
    Parts(Vec<PartStubField>),

    /// A boolean flag indicating whether the story has been deleted.
    Deleted,
}

impl_field_display!(
    StoryField,
    User => "user",
    FirstPublishedPart => "firstPublishedPart",
    LastPublishedPart => "lastPublishedPart",
    Parts => "parts"
);

impl AuthRequiredFields for StoryField {}

impl DefaultableFields for StoryField {
    fn default_fields() -> Vec<Self> {
        vec![
            Self::Id,
            Self::Title,
            Self::Length,
            Self::CreatedDate,
            Self::ModifyDate,
            Self::VoteCount,
            Self::ReadCount,
            Self::CommentCount,
            Self::Language(vec![LanguageField::Id]),
            Self::Description,
            Self::Cover,
            Self::CoverTimestamp,
            Self::Completed,
            Self::Categories,
            Self::Tags,
            Self::Rating,
            Self::Mature,
            Self::Copyright,
            Self::Url,
            Self::NumParts,
            Self::FirstPublishedPart(vec![PartReferenceField::Id]),
            Self::LastPublishedPart(vec![PartReferenceField::Id]),
            Self::Parts(vec![PartStubField::Id]),
            Self::Deleted,
            Self::User(vec![UserStubField::Username]),
        ]
    }
}