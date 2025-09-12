//! Contains utility macros for the `field` module.
//!
//! These macros help reduce boilerplate code when implementing common traits
//! for the various field enums.

/// Implements `std::fmt::Display` for a field enum with complex (nested) variants.
///
/// This macro handles field enums that contain both simple variants (e.g., `Title`)
/// and complex variants that hold a `Vec` of sub-fields (e.g., `User(Vec<UserField>)`).
///
/// - For simple variants, it relies on `strum`'s `AsRefStr` trait to produce the `camelCase` string.
/// - For complex variants, it formats them as `fieldName(subField1,subField2,...)`.
///
/// # Arguments
/// * `$enum_name:ident` - The name of the enum to implement `Display` for.
/// * `$( $variant:ident => $name:literal ),*` - A comma-separated list of the complex variants.
///   Each entry specifies the `VariantName` and the exact string literal to use for its name.
///
/// # Example
/// ```
///
/// // An enum with simple and complex variants
/// use wp_mini::impl_field_display;
///
/// #[derive(strum_macros::AsRefStr)]
/// #[strum(serialize_all = "camelCase")]
/// enum StoryField {
///     Title,
///     VoteCount,
///     // A complex variant with sub-fields
///     User(Vec<UserField>),
/// }
///
/// #[derive(strum_macros::AsRefStr)]
/// #[strum(serialize_all = "camelCase")]
/// enum UserField {
///     Username,
///     Avatar,
/// }
///
/// // Generate the Display implementation using the macro
/// impl_field_display!(StoryField, User => "user");
///
/// // --- Verification ---
/// // Simple field
/// let simple_field = StoryField::VoteCount;
/// assert_eq!(simple_field.to_string(), "voteCount");
///
/// // Complex field
/// let complex_field = StoryField::User(vec![UserField::Username, UserField::Avatar]);
/// assert_eq!(complex_field.to_string(), "user(username,avatar)");
/// ```
#[macro_export]
macro_rules! impl_field_display {
    // Top-level entry point for the macro.
    // It takes the enum name and a comma-separated list of complex variants.
    // Each complex variant is specified as `VariantName => "string_to_use"`.
    ($enum_name:ident, $( $variant:ident => $name:literal ),* ) => {
        impl std::fmt::Display for $enum_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    // This part iterates through each `Variant => "name"` pair provided.
                    $(
                        // For each pair, it generates a match arm.
                        $enum_name::$variant(sub_fields) => {
                            // This logic is the same as your original implementation.
                            let sub_fields_str = sub_fields
                                .iter()
                                .map(|field| field.to_string())
                                .collect::<Vec<String>>()
                                .join(",");
                            // It writes the specified string name, not the variant name.
                            write!(f, "{}({})", $name, sub_fields_str)
                        },
                    )*

                    // The fallback arm for all simple variants remains the same.
                    // It relies on the `AsRefStr` derive from the `strum` crate.
                    _ => write!(f, "{}", self.as_ref()),
                }
            }
        }
    };
}