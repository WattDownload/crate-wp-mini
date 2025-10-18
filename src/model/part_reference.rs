use crate::types::PartResponse;
use crate::{WattpadClient, WattpadError};
use serde::Deserialize;

/// Represents a lightweight reference to a story part.
///
/// This struct is often used in lists (e.g., a story's list of parts) where sending
/// the full part data for each item would be inefficient. It can be "upgraded" to a
/// full `Part` object using the `fetch_full_part` method.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PartReference {
    /// The unique numerical identifier of the story part.
    pub id: Option<u64>,
    /// The timestamp when the part was created.
    pub create_date: Option<String>,
}

impl PartReference {
    /// Fetches the full `Part` object corresponding to this reference.
    ///
    /// This is a convenience method that uses the `id` from the reference to make
    /// a new API call and retrieve the complete data for the story part.
    ///
    /// # Arguments
    /// * `client` - An instance of `WattpadClient` to use for the API request.
    ///
    /// # Returns
    /// A `Result` containing the full `PartResponse` on success.
    ///
    /// # Errors
    /// Returns a `WattpadError` if the `id` field on this reference is `None`, or if the
    /// underlying API request fails.
    ///
    /// # Examples
    /// ```no_run
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), wp_mini::WattpadError> {
    /// // Assume `part_ref` is a PartReference obtained from a story object.
    /// use wp_mini::types::PartReferenceResponse;
    /// use wp_mini::WattpadClient;
    /// let part_ref = PartReferenceResponse { id: Some(12345), create_date: None };
    /// let client = WattpadClient::new();
    ///
    /// // Fetch the full details for the part
    /// let full_part_info = part_ref.fetch_full_part(&client).await?;
    ///
    /// if let Some(title) = full_part_info.title {
    ///     println!("Fetched part title: {}", title);
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