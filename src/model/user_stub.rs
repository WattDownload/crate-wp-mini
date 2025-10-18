use crate::model::User;
use crate::{WattpadClient, WattpadError};
use serde::Deserialize;

/// Represents a lightweight stub of a user.
///
/// A `UserStub` contains a small, essential subset of the fields of a full `User` object.
/// It's often embedded in other API responses (like stories or comments) to avoid
/// sending redundant data. It can be "upgraded" to a full `User` object using the
/// `fetch_full_profile` method.
#[derive(Debug, Deserialize, Clone)]
pub struct UserStub {
    /// The user's unique username.
    #[serde(rename = "name")]
    pub username: Option<String>,
    /// The URL for the user's profile picture (avatar).
    pub avatar: Option<String>,
    /// The user's full display name.
    pub fullname: Option<String>,
    /// A boolean flag indicating if the user is a verified account.
    pub verified: Option<bool>,
}

impl UserStub {
    /// Fetches the full `User` object corresponding to this stub.
    ///
    /// This is a convenience method that uses the `username` from the stub to make a
    /// new API call and retrieve the complete profile data.
    ///
    /// # Arguments
    /// * `client` - An instance of `WattpadClient` to use for the API request.
    ///
    /// # Returns
    /// A `Result` containing the full `User` object on success.
    ///
    /// # Errors
    /// Returns a `WattpadError` if the `username` field on this stub is `None`, or if the
    /// underlying API request fails (e.g., user not found).
    ///
    /// # Examples
    /// ```no_run
    /// # use wp_mini::WattpadClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), wp_mini::WattpadError> {
    /// // Assume `user_stub` is a UserStub obtained from a story object.
    /// use wp_mini::types::UserStubResponse;
    /// let user_stub = UserStubResponse { username: Some("testuser".to_string()), avatar: None, fullname: None, verified: None };
    /// let client = WattpadClient::new();
    ///
    /// // Fetch the full profile for the user
    /// let full_profile = user_stub.fetch_full_profile(&client).await?;
    ///
    /// if let Some(followers) = full_profile.num_followers {
    ///     println!("Fetched follower count: {}", followers);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn fetch_full_profile(&self, client: &WattpadClient) -> Result<User, WattpadError> {
        if let Some(username) = &self.username {
            client.user.get_user_info(username, None).await
        } else {
            Err(WattpadError::MissingRequiredField {
                field: "username".to_string(),
                context: "Cannot fetch full profile without a username.".to_string(),
            })
        }
    }
}