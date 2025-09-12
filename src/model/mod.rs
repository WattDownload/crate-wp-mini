//! Contains the data structures that represent objects from the Wattpad API.
//!
//! Each submodule defines a specific model, such as `Story`, `User`, or `Part`.
//! These models are `pub(crate)`, meaning they are intended for internal use within
//! the library. They are exposed to the end-user through the public type aliases
//! in the `crate::types` module.

// mod category;
mod language;
mod part;
mod part_content;
mod part_reference;
mod part_stub;
mod story;
mod text_url;
mod user;
mod user_stub;

pub(crate) use language::*;
pub(crate) use part::*;
pub(crate) use part_content::*;
pub(crate) use part_reference::*;
pub(crate) use part_stub::*;
pub(crate) use story::*;
pub(crate) use text_url::*;
pub(crate) use user::*;
pub(crate) use user_stub::*;