//! Defines the field enums used for type-safe field selection in API requests.
//!
//! This module contains enums that represent the data fields available for different
//! Wattpad API objects (e.g., `StoryField`, `UserField`). Using these enums ensures
//! that only valid fields can be requested from the API.
//!
//! It also provides traits to manage default fields and authentication requirements.

// Private modules for each field type.
mod language_field;
mod macros;
mod part_content_field;
mod part_field;
mod part_reference_field;
mod part_stub_field;
mod story_field;
mod text_url_field;
mod user_field;
mod user_stub_field;

// Publicly export the field enums for use throughout the crate.
pub use language_field::LanguageField;
pub use part_content_field::PartContentField;
pub use part_field::PartField;
pub use part_reference_field::PartReferenceField;
pub use part_stub_field::PartStubField;
pub use story_field::StoryField;
pub use text_url_field::TextUrlField;
pub use user_field::UserField;
pub use user_stub_field::UserStubField;

/// A trait for field enums that can provide a default set of fields.
///
/// This is used to request a standard, useful set of data when the user
/// doesn't specify which fields they want.
pub trait DefaultableFields {
    /// Returns a vector of the enum's variants that should be used as the default.
    fn default_fields() -> Vec<Self>
    where
        Self: Sized;
}

/// A trait to identify which fields in an enum require user authentication.
pub trait AuthRequiredFields: Sized + PartialEq {
    /// Returns a vector of enum variants that require authentication.
    ///
    /// By default, this returns an empty vector, assuming most fields are public.
    /// This method should be overridden for enums that have protected fields.
    fn auth_required_fields() -> Vec<Self> {
        vec![]
    }

    /// Checks if a specific field instance requires authentication.
    ///
    /// Returns `true` if `self` is present in the list of authentication-required fields.
    fn auth_required(&self) -> bool {
        Self::auth_required_fields().contains(self)
    }
}