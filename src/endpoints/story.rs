use crate::client::WattpadRequestBuilder;
use crate::field::{PartField, StoryField};
use crate::types::{PartContentResponse, PartResponse, StoryResponse};
use crate::WattpadError;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use bytes::Bytes;

/// Contains methods for story-related API endpoints.
///
/// This client provides access to fetching information about stories, story parts,
/// and their content in various formats.
pub struct StoryClient {
    /// The shared `reqwest` client for making HTTP requests.
    pub(crate) http: reqwest::Client,
    /// A flag indicating whether the main client is authenticated.
    pub(crate) is_authenticated: Arc<AtomicBool>,
}

impl StoryClient {
    /// Returns detailed information about a story.
    ///
    /// # Arguments
    /// * `story_id` - The unique identifier of the story to fetch.
    /// * `fields` - An optional slice of `StoryField` specifying which fields to retrieve.
    ///   If `None`, a comprehensive list of all known fields will be requested by default.
    ///
    /// # Returns
    /// A `Result` containing a `StoryResponse` struct with the story's metadata on success.
    ///
    /// # Errors
    /// Returns a `WattpadError` if the network request fails or the API returns an error.
    ///
    /// # Examples
    /// ```no_run
    /// # use wattpad::{WattpadClient, field::StoryField};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), wattpad::WattpadError> {
    /// let client = WattpadClient::new();
    /// let story_id = 12345678; // Example story ID
    /// let fields = &[StoryField::Title, StoryField::VoteCount];
    ///
    /// let story_info = client.story.get_story_info(story_id, Some(fields)).await?;
    ///
    /// println!("Title: {}", story_info.title);
    /// println!("Votes: {}", story_info.vote_count);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_story_info(
        &self,
        story_id: u64,
        fields: Option<&[StoryField]>,
    ) -> Result<StoryResponse, WattpadError> {
        WattpadRequestBuilder::new(
            &self.http,
            &self.is_authenticated,
            reqwest::Method::GET,
            &format!("/api/v3/stories/{}", story_id),
        )
            .fields(fields)?
            .execute()
            .await
    }

    /// Returns detailed information about a single story part.
    ///
    /// # Arguments
    /// * `part_id` - The unique identifier of the story part to fetch.
    /// * `fields` - An optional slice of `PartField` specifying which fields to retrieve.
    ///   If `None`, a default set of fields will be requested.
    ///
    /// # Returns
    /// A `Result` containing a `PartResponse` struct with the part's metadata on success.
    ///
    /// # Errors
    /// Returns a `WattpadError` if the network request fails or the API returns an error.
    ///
    /// # Examples
    /// ```no_run
    /// # use wattpad::{WattpadClient, field::PartField};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), wattpad::WattpadError> {
    /// let client = WattpadClient::new();
    /// let part_id = 87654321; // Example part ID
    /// let fields = &[PartField::Title, PartField::WordCount];
    ///
    /// let part_info = client.story.get_part_info(part_id, Some(fields)).await?;
    ///
    /// println!("Part Title: {}", part_info.title);
    /// println!("Word Count: {}", part_info.word_count);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_part_info(
        &self,
        part_id: u64,
        fields: Option<&[PartField]>,
    ) -> Result<PartResponse, WattpadError> {
        WattpadRequestBuilder::new(
            &self.http,
            &self.is_authenticated,
            reqwest::Method::GET,
            &format!("/api/v3/story_parts/{}", part_id),
        )
            .fields(fields)?
            .execute()
            .await
    }

    /// Fetches the raw text content of a single story part.
    ///
    /// This endpoint is useful for getting the plain story text without any metadata.
    ///
    /// # Arguments
    /// * `part_id` - The unique identifier for the story part.
    ///
    /// # Returns
    /// A `Result` containing a `String` with the raw story text on success.
    ///
    /// # Errors
    /// Returns a `WattpadError` if the network request fails.
    ///
    /// # Examples
    /// ```no_run
    /// # use wattpad::WattpadClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), wattpad::WattpadError> {
    /// let client = WattpadClient::new();
    /// let part_id = 87654321;
    ///
    /// let content = client.story.get_part_content_raw(part_id).await?;
    /// println!("Fetched content snippet: {}...", content.chars().take(100).collect::<String>());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_part_content_raw(&self, part_id: u64) -> Result<String, WattpadError> {
        WattpadRequestBuilder::new(
            &self.http,
            &self.is_authenticated,
            reqwest::Method::GET,
            "/apiv2/",
        )
            .param("m", Some("storytext"))
            .param("id", Some(part_id))
            .execute_raw_text()
            .await
    }

    /// Fetches the content of a story part as a structured JSON object.
    ///
    /// # Arguments
    /// * `part_id` - The unique identifier for the story part.
    ///
    /// # Returns
    /// A `Result` containing a `PartContentResponse` struct with the parsed story content on success.
    ///
    /// # Errors
    /// Returns a `WattpadError` if the network request fails or the JSON response cannot be parsed.
    ///
    /// # Examples
    /// ```no_run
    /// # use wattpad::WattpadClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), wattpad::WattpadError> {
    /// let client = WattpadClient::new();
    /// let part_id = 87654321;
    ///
    /// let content_json = client.story.get_part_content_json(part_id).await?;
    /// println!("Text from JSON: {}...", content_json.text.chars().take(100).collect::<String>());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_part_content_json(
        &self,
        part_id: u64,
    ) -> Result<PartContentResponse, WattpadError> {
        WattpadRequestBuilder::new(
            &self.http,
            &self.is_authenticated,
            reqwest::Method::GET,
            "/apiv2/",
        )
            .param("m", Some("storytext"))
            .param("id", Some(part_id))
            .param("output", Some("json"))
            .execute()
            .await
    }

    /// Downloads the text content of an entire story as a single ZIP archive.
    ///
    /// The archive contains the story text, typically organized by parts.
    ///
    /// # Arguments
    /// * `story_id` - The unique identifier for the story (not a part).
    ///
    /// # Returns
    /// A `Result` containing a `Bytes` object with the binary data of the ZIP file on success.
    ///
    /// # Errors
    /// Returns a `WattpadError` if the network request or download fails.
    ///
    /// # Examples
    /// ```no_run
    /// # use wattpad::WattpadClient;
    /// # use std::fs::File;
    /// # use std::io::Write;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = WattpadClient::new();
    /// let story_id = 12345678;
    ///
    /// let zip_bytes = client.story.get_story_content_zip(story_id).await?;
    ///
    /// // Example: Save the ZIP file to disk
    /// let mut file = File::create(format!("{}.zip", story_id))?;
    /// file.write_all(&zip_bytes)?;
    ///
    /// println!("Successfully downloaded and saved {}.zip", story_id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_story_content_zip(&self, story_id: u64) -> Result<Bytes, WattpadError> {
        WattpadRequestBuilder::new(
            &self.http,
            &self.is_authenticated,
            reqwest::Method::GET,
            "/apiv2/",
        )
            .param("m", Some("storytext"))
            .param("group_id", Some(story_id))
            .param("output", Some("zip"))
            .execute_bytes()
            .await
    }
}