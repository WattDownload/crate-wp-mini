//! Defines the primary error handling types for the library.
//!
//! This module contains the central `WattpadError` enum, which consolidates all
//! possible errors that can occur, from network issues to specific API responses.
//! It also includes the logic for parsing raw error messages from the Wattpad API
//! into more specific, user-friendly error variants.

use reqwest;
use serde::Deserialize;
use thiserror::Error;

/// The primary error type for all operations in the `wattpad` crate.
#[derive(Error, Debug)]
pub enum WattpadError {
    /// An error occurred during a network request (e.g., connection timeout, DNS failure).
    #[error("Network or request error: {0}")]
    RequestError(#[from] reqwest::Error),

    /// An error occurred while parsing the JSON response from the API.
    #[error("Failed to parse JSON response: {0}")]
    ParseError(#[from] serde_json::Error),

    /// An authentication attempt failed, likely due to invalid credentials.
    #[error("Authentication failed: Invalid credentials or missing cookies.")]
    AuthenticationFailed,

    /// A requested field or endpoint requires authentication, but the client is not logged in.
    #[error("Authentication required for '{field}': {context}")]
    AuthenticationRequired {
        /// The name of the field or endpoint that requires authentication.
        field: String,
        /// Additional context about the requirement.
        context: String,
    },

    /// A required field was missing from the API response.
    #[error("Missing a required field: '{field}'. Context: {context}")]
    MissingRequiredField {
        /// The name of the field that was expected.
        field: String,
        /// Additional context about where the field was expected.
        context: String,
    },

    /// A specific API error (code 1014) indicating the requested user was not found.
    #[error("API Error 1014: User not found.")]
    UserNotFound,

    /// A specific API error (code 1017) indicating the requested story was not found.
    #[error("API Error 1017: Story not found.")]
    StoryNotFound,

    /// A specific API error (code 1018) indicating permission was denied because the user is not logged in.
    #[error("API Error 1018: Permission Denied. User not logged in.")]
    PermissionDeniedNotLoggedIn,

    /// A specific API error (code 1154) indicating access to a resource was denied.
    #[error("API Error 1154: Access Denied.")]
    AccessDenied,

    /// A catch-all for any other error returned by the Wattpad API.
    #[error("API Error {code} ({error_type}): {message}")]
    ApiError {
        /// The numerical error code from the API.
        code: i64,
        /// The string error type from the API.
        error_type: String,
        /// The descriptive error message from the API.
        message: String,
    },
}

/// An internal struct to deserialize the raw error response from the Wattpad API.
///
/// This is a temporary representation that gets converted into a `WattpadError`.
#[derive(Deserialize)]
pub(crate) struct ApiErrorResponse {
    pub code: i64,
    #[serde(rename = "error")]
    pub error_type: String,
    pub message: String,
}

impl From<ApiErrorResponse> for WattpadError {
    /// Converts the raw API error response into a more specific and user-friendly `WattpadError`.
    ///
    /// This implementation maps known error codes to specific enum variants, providing clearer
    /// error types to the end-user. Unknown codes are mapped to the generic `ApiError` variant.
    fn from(res: ApiErrorResponse) -> Self {
        match res.code {
            1014 => WattpadError::UserNotFound,
            1017 => WattpadError::StoryNotFound,
            1018 => WattpadError::PermissionDeniedNotLoggedIn,
            1154 => WattpadError::AccessDenied,
            _ => WattpadError::ApiError {
                code: res.code,
                error_type: res.error_type,
                message: res.message,
            },
        }
    }
}