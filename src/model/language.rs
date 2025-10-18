use serde::Deserialize;

/// Represents a language object from the Wattpad API.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Language {
    /// The unique numerical identifier of the language.
    pub id: Option<u64>,
    /// The full name of the language (e.g., "English").
    pub name: Option<String>,
}