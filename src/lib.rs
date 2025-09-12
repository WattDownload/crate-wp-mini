//! An asynchronous, unofficial Wattpad API wrapper for Rust.

// Declare the modules that make up the library.
mod client;
pub mod endpoints;
pub mod field;
mod error;
mod model;
pub mod types;

// Publicly export the primary types for easy use.
pub use client::WattpadClient;
pub use error::WattpadError;