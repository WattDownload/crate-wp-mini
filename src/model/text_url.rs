use serde::Deserialize;

/// Represents a `text_url` object from the Wattpad API.
///
/// This object provides URLs and tokens for accessing the actual text content
/// of a story part, which is often served from a separate, temporary URL.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TextUrl {
    /// The direct, often temporary and expiring, URL to the story part's text content.
    pub text: Option<String>,
    /// A token that can be used to obtain a new URL for the text content,
    /// likely after the original one expires.
    #[serde(rename = "refresh_token")]
    pub refresh_token: Option<String>,
}