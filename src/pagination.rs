use serde::de::DeserializeOwned;
use std::marker::PhantomData;
use crate::client::WattpadRequestBuilder;
use crate::WattpadError;

/// A trait that response wrappers must implement to work with the Paginator.
/// T is the type of the item inside the list (e.g., Story, UserStub).
pub trait PaginatedResponse<T> {
    /// Consumes the response and returns the vector of items.
    fn into_items(self) -> Vec<T>;

    /// If nextUrl present
    fn has_next_page(&self) -> bool;
}

/// A generic builder for fetching paginated resources.
///
/// * `T`: The type of item being fetched (e.g., `Story`).
/// * `R`: The API response wrapper (e.g., `UserStoriesResponse`).
pub struct Paginator<'a, T, R> {
    req_builder: WattpadRequestBuilder<'a>,
    limit: usize,
    offset: usize,
    // Use PhantomData because T and R are used in methods, not as fields
    _marker: PhantomData<(T, R)>,
}

impl<'a, T, R> Paginator<'a, T, R>
where
    T: Send + Sync,
    R: DeserializeOwned + PaginatedResponse<T> + Send + Sync,
{
    pub(crate) fn new(req_builder: WattpadRequestBuilder<'a>) -> Self {
        Self {
            req_builder,
            limit: 10, // Default Wattpad limit
            offset: 0,
            _marker: PhantomData,
        }
    }

    /// Set the number of items per page (Max 100).
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = limit.min(100);
        self
    }

    /// Set the starting offset.
    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = offset;
        self
    }

    /// Execute the request for a SINGLE page.
    /// Returns the full response wrapper (so you can see nextUrl if needed).
    pub async fn execute(self) -> Result<R, WattpadError> {
        self.req_builder
            .param("limit", Some(self.limit))
            .param("offset", Some(self.offset))
            .execute::<R>() // Generic execute defined in client
            .await
    }

    /// Automatically fetch ALL items by iterating through pages.
    /// Returns a simple vector of items, discarding the wrapper.
    pub async fn execute_all(mut self) -> Result<Vec<T>, WattpadError> {
        // Initial Setup: Maximize efficiency
        self.limit = 100;
        self.offset = 0;

        let mut all_items = Vec::new();

        loop {
            // Request
            let response_wrapper = self.req_builder.clone()
                .param("limit", Some(self.limit))
                .param("offset", Some(self.offset))
                .execute::<R>()
                .await?;

            // Check the "Next" signal BEFORE consuming items
            let has_next = response_wrapper.has_next_page();

            // Extract items
            let items = response_wrapper.into_items();

            all_items.extend(items);

            // If nextUrl exists, advance the offset.
            // If it doesn't, we are done.
            if has_next {
                // Add the number of items we actually received to the offset.
                // This less safe, what if the API returned 99 for some reason, but also sent a nextUrl (unlikely so it's ok)
                self.offset += 100;
            } else {
                break;
            }
        }

        Ok(all_items)
    }
}