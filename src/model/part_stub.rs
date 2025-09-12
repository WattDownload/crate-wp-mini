use crate::types::{PartResponse, TextUrlResponse};
use crate::{WattpadClient, WattpadError};
use serde::Deserialize;

/// Represents a lightweight stub of a story part.
///
/// A `PartStub` contains a subset of the fields of a full `Part` object. It's
/// often used in lists where sending the complete data for every part would be
/// inefficient. It can be "upgraded" to a full `Part` object using the
/// `fetch_full_part` method.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PartStub {
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
    /// A boolean flag indicating if the currently authenticated user has voted for this part.
    /// **Requires authentication.**
    pub voted: Option<bool>,
    /// A boolean flag indicating whether the part has been deleted.
    pub deleted: Option<bool>,
}

impl PartStub {
    /// Fetches the full `Part` object corresponding to this stub.
    ///
    /// This is a convenience method that uses the `id` from the stub to make
    /// a new API call and retrieve the complete data for the story part.
    ///
    /// # Arguments
    /// * `client` - An instance of `WattpadClient` to use for the API request.
    ///
    /// # Returns
    /// A `Result` containing the full `PartResponse` on success.
    ///
    /// # Errors
    /// Returns a `WattpadError` if the `id` field on this stub is `None`, or if the
    /// underlying API request fails.
    ///
    /// # Examples
    /// ```no_run
    /// # use wp_mini::{WattpadClient};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), wp_mini::WattpadError> {
    /// // Assume `part_stub` is a PartStub obtained from a story object's part list.
    /// use wp_mini::types::PartStubResponse;
    /// let part_stub = PartStubResponse { id: Some(12345), title: Some("Chapter 1".to_string()), /* ... other fields ... */ voted: None, deleted: None, read_count: None, vote_count: None, comment_count: None, photo_url: None, video_id: None, length: None, has_banned_images: None, create_date: None, modify_date: None, draft: None, rating: None, text_url: None, url: None };
    /// let client = WattpadClient::new();
    ///
    /// // Fetch the full details for the part
    /// let full_part_info = part_stub.fetch_full_part(&client).await?;
    ///
    /// if let Some(votes) = full_part_info.vote_count {
    ///     println!("Fetched part vote count: {}", votes);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn fetch_full_part(
        &self,
        client: &WattpadClient,
    ) -> Result<PartResponse, WattpadError> {
        if let Some(id) = self.id {
            client.story.get_part_info(id, None).await
        } else {
            Err(WattpadError::MissingRequiredField {
                field: "id".to_string(),
                context: "Cannot fetch full part without an id.".to_string(),
            })
        }
    }
}