use crate::client::WattpadRequestBuilder;
use crate::field::UserField;
use crate::types::UserResponse;
use crate::WattpadError;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

/// Provides access to user-related API endpoints.
///
/// This client allows you to fetch public information about Wattpad users.
pub struct UserClient {
    /// The shared `reqwest` client for making HTTP requests.
    pub(crate) http: reqwest::Client,
    /// A flag indicating whether the main client is authenticated.
    pub(crate) is_authenticated: Arc<AtomicBool>,
}

impl UserClient {
    /// Fetches detailed public information about a specific user.
    ///
    /// This function retrieves a user's profile data, such as their follower count,
    /// stories they've written, and more.
    ///
    /// # Arguments
    /// * `username` - The username of the user to fetch.
    /// * `fields` - An optional slice of `UserField` specifying which fields to retrieve.
    ///   If `None`, a default set of fields will be requested.
    ///
    /// # Returns
    /// A `Result` containing a `UserResponse` struct with the user's data on success.
    ///
    /// # Errors
    /// Returns a `WattpadError` if the network request fails, the API returns an error
    /// (e.g., user not found), or a requested field requires authentication when the
    /// client is unauthenticated.
    ///
    /// # Examples
    /// ```no_run
    /// # use wattpad::{WattpadClient, field::UserField};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), wattpad::WattpadError> {
    /// let client = WattpadClient::new();
    /// let username = "test";
    /// let fields = &[UserField::Username, UserField::FollowerCount];
    ///
    /// let user_info = client.user.get_user_info(username, Some(fields)).await?;
    ///
    /// println!("User: {}", user_info.username);
    /// println!("Followers: {}", user_info.follower_count);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_user_info(
        &self,
        username: &str,
        fields: Option<&[UserField]>,
    ) -> Result<UserResponse, WattpadError> {
        WattpadRequestBuilder::new(
            &self.http,
            &self.is_authenticated,
            reqwest::Method::GET,
            &format!("/api/v3/users/{}", username),
        )
            .fields(fields)?
            .execute()
            .await
    }
}
