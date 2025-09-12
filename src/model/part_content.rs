use serde::Deserialize;

/// Represents the content of a story part, typically returned in a structured JSON format.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PartContent {
    /// The full text content of the story part.
    pub text: Option<String>,
    /// A hash of the text content, likely used for caching or integrity checks.
    #[serde(rename = "text_hash")]
    pub text_hash: Option<String>,
}