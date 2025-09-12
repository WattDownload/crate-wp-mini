use crate::field::DefaultableFields;
use strum_macros::{Display, EnumIter};

/// Represents the fields for a `UserStub` object.
///
/// A `UserStub` is a lightweight or summary representation of a user,
/// often embedded in other API objects like stories or comments.
#[derive(Debug, Clone, Copy, Display, EnumIter, PartialEq, Eq)]
#[strum(serialize_all = "camelCase")]
pub enum UserStubField {
    /// The user's unique username.
    #[strum(serialize = "name")]
    Username,
    /// The URL for the user's profile picture (avatar).
    Avatar,
    /// The user's full display name.
    #[strum(serialize = "fullname")]
    FullName,
    /// A boolean flag indicating if the user is a verified account.
    Verified,
}

impl DefaultableFields for UserStubField {
    fn default_fields() -> Vec<Self> {
        vec![Self::Username, Self::Avatar]
    }
}