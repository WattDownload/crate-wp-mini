use crate::types::{StoryResponse, TextUrlResponse};
use serde::Deserialize;

/// Represents a full story part object from the Wattpad API.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Part {
    /// The unique numerical identifier of the story part.
    pub id: Option<u64>,
    /// The title of the story part.
    pub title: Option<String>,
    /// A direct URL to the story part on the Wattpad website.
    pub url: Option<String>,

    /// An object containing URLs for accessing the part's text content.
    #[serde(rename = "text_url")]
    pub text_url: Option<TextUrlResponse>,

    /// The content rating of the story part.
    pub rating: Option<i64>,
    /// A boolean flag indicating whether the part is a draft.
    pub draft: Option<bool>,
    /// The timestamp when the part was last modified.
    pub modify_date: Option<String>,
    /// The timestamp when the part was created.
    pub create_date: Option<String>,
    /// A boolean flag indicating if the part contains images that have been banned.
    pub has_banned_images: Option<bool>,
    /// The length of the story part, often representing an estimated reading time in seconds.
    pub length: Option<i64>,
    /// The ID of any video associated with the part.
    pub video_id: Option<String>,
    /// The URL for the part's cover image.
    pub photo_url: Option<String>,
    /// The total number of comments on the part.
    pub comment_count: Option<i64>,
    /// The total number of votes the part has received.
    pub vote_count: Option<i64>,
    /// The total number of reads the part has received.
    pub read_count: Option<i64>,
    /// The unique identifier of the parent story.
    pub group_id: Option<String>,
    /// A boolean flag indicating if the currently authenticated user has voted for this part.
    pub voted: Option<bool>,
    /// The parent story object that this part belongs to.
    ///
    /// This is heap-allocated using `Box` to prevent an infinitely sized struct,
    /// as a `Story` can contain a list of `Part`s.
    pub group: Option<Box<StoryResponse>>,
    /// A boolean flag indicating whether the part has been deleted.
    pub deleted: Option<bool>,
}