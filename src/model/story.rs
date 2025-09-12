use super::Language;
use crate::types::{PartReferenceResponse, PartStubResponse, UserStubResponse};
use serde::Deserialize;

/// Represents a full story object from the Wattpad API.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Story {
    /// The unique identifier of the story.
    pub id: Option<String>,
    /// The title of the story.
    pub title: Option<String>,
    /// The estimated reading time of the story in seconds.
    pub length: Option<i64>,
    /// The timestamp when the story was created.
    pub create_date: Option<String>,
    /// The timestamp when the story was last modified.
    pub modify_date: Option<String>,
    /// The total number of votes the story has received.
    pub vote_count: Option<i64>,
    /// The total number of reads the story has received.
    pub read_count: Option<i64>,
    /// The total number of comments on the story.
    pub comment_count: Option<i64>,
    /// The language the story is written in.
    pub language: Option<Language>,
    /// A stub object representing the author of the story.
    pub user: Option<UserStubResponse>,
    /// The story's description or synopsis.
    pub description: Option<String>,
    /// The URL for the story's cover image.
    pub cover: Option<String>,
    /// The timestamp when the cover image was last updated.
    #[serde(rename = "cover_timestamp")]
    pub cover_timestamp: Option<String>,
    /// A boolean flag indicating whether the story is marked as complete.
    pub completed: Option<bool>,
    /// A list of category IDs that the story belongs to.
    pub categories: Option<Vec<i64>>,
    /// A list of user-defined tags associated with the story.
    pub tags: Option<Vec<String>>,
    /// The content rating of the story (e.g., Everyone, Mature).
    pub rating: Option<i64>,
    /// A boolean flag indicating if the story is intended for a mature audience.
    pub mature: Option<bool>,
    /// The copyright or license identifier for the story.
    pub copyright: Option<i64>,
    /// A direct URL to the story on the Wattpad website.
    pub url: Option<String>,
    /// The total number of published parts in the story.
    pub num_parts: Option<i64>,
    /// The unique identifier of the first part of the story.
    pub first_part_id: Option<i64>,
    /// A lightweight reference to the first published part of the story.
    pub first_published_part: Option<PartReferenceResponse>,
    /// A lightweight reference to the last published part of the story.
    pub last_published_part: Option<PartReferenceResponse>,
    /// A list of the parts belonging to this story, often as stubs.
    pub parts: Option<Vec<PartStubResponse>>,
    /// A boolean flag indicating whether the story has been deleted.
    pub deleted: Option<bool>,
}