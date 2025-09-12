//! Defines the public type aliases for the data structures returned by API endpoints.
//!
//! These aliases provide a clear and consistent naming convention (e.g., `UserResponse`)
//! for the types that consumers of this library will receive, distinguishing them from
//! the internal `model` structs. This makes the library's public API more explicit.

use crate::model;

/// Represents the response data for a full user object. Alias for [`model::User`].
pub type UserResponse = model::User;

/// Represents the response data for a lightweight user stub. Alias for [`model::UserStub`].
pub type UserStubResponse = model::UserStub;

/// Represents the response data for a full story object. Alias for [`model::Story`].
pub type StoryResponse = model::Story;

/// Represents the response data for a lightweight story part reference. Alias for [`model::PartReference`].
pub type PartReferenceResponse = model::PartReference;

/// Represents the response data for a lightweight story part stub. Alias for [`model::PartStub`].
pub type PartStubResponse = model::PartStub;

/// Represents the response data for a full story part object. Alias for [`model::Part`].
pub type PartResponse = model::Part;

/// Represents the response data for a text URL object. Alias for [`model::TextUrl`].
pub type TextUrlResponse = model::TextUrl;

/// Represents the response data for a story part's content. Alias for [`model::PartContent`].
pub type PartContentResponse = model::PartContent;