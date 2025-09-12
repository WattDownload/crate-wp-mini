//! The main module for the Wattpad API client.
//!
//! It contains the primary `WattpadClient`, which serves as the entry point for all API
//! interactions. It also includes the internal `WattpadRequestBuilder` for constructing
//! and executing API calls, and helper functions for handling responses.

use crate::error::{ApiErrorResponse, WattpadError};
use crate::endpoints::story::StoryClient;
use crate::endpoints::user::UserClient;
use crate::field::{AuthRequiredFields, DefaultableFields};
use bytes::Bytes;
use reqwest::Client as ReqwestClient;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, USER_AGENT};
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

// =================================================================================================
// WattpadClientBuilder
// =================================================================================================

/// A builder for creating a `WattpadClient` with custom configuration.
#[derive(Default)]
pub struct WattpadClientBuilder {
    client: Option<ReqwestClient>,
    user_agent: Option<String>,
    headers: Option<HeaderMap>,
}

impl WattpadClientBuilder {
    /// Provide a pre-configured `reqwest::Client`.
    /// If this is used, any other configurations like `.user_agent()` or `.header()` will be ignored,
    /// as the provided client is assumed to be fully configured.
    pub fn reqwest_client(mut self, client: ReqwestClient) -> Self {
        self.client = Some(client);
        self
    }

    /// Set a custom User-Agent string for all requests.
    pub fn user_agent(mut self, user_agent: &str) -> Self {
        self.user_agent = Some(user_agent.to_string());
        self
    }

    /// Add a single custom header to be sent with all requests.
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.get_or_insert_with(HeaderMap::new).insert(key, value);
        self
    }

    /// Builds the `WattpadClient`.
    ///
    /// If a `reqwest::Client` was not provided via the builder, a new default one will be created.
    pub fn build(self) -> WattpadClient {
        let http_client = match self.client {
            // If a client was provided, use it directly.
            Some(client) => client,
            // Otherwise, build a new client using the builder's settings.
            None => {
                let mut headers = self.headers.unwrap_or_default();

                // Set the User-Agent, preferring the custom one, otherwise use the default.
                let ua_string = self.user_agent.unwrap_or_else(||
                    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36".to_string()
                );

                // Insert the user-agent header, it will override any existing one in the map.
                headers.insert(reqwest::header::USER_AGENT, HeaderValue::from_str(&ua_string).expect("Invalid User-Agent string"));

                ReqwestClient::builder()
                    .default_headers(headers)
                    .cookie_store(true)
                    .build()
                    .expect("Failed to build reqwest client")
            }
        };

        // The rest of the logic remains the same
        let auth_flag = Arc::new(AtomicBool::new(false));
        WattpadClient {
            user: UserClient {
                http: http_client.clone(),
                is_authenticated: auth_flag.clone(),
            },
            story: StoryClient {
                http: http_client.clone(),
                is_authenticated: auth_flag.clone(),
            },
            http: http_client,
            is_authenticated: auth_flag,
        }
    }
}

/// The main asynchronous client for interacting with the Wattpad API.
///
/// This client holds the HTTP connection, manages authentication state, and provides
/// access to categorized sub-clients for different parts of the API.
pub struct WattpadClient {
    /// The underlying `reqwest` client used for all HTTP requests.
    http: reqwest::Client,
    /// An atomically-managed boolean flag to track authentication status.
    is_authenticated: Arc<AtomicBool>,
    /// Provides access to user-related API endpoints.
    pub user: UserClient,
    /// Provides access to story and part-related API endpoints.
    pub story: StoryClient,
}

impl WattpadClient {
    /// Creates a new `WattpadClient` with default settings.
    ///
    /// This is now a convenience method that uses the builder.
    pub fn new() -> Self {
        WattpadClientBuilder::default().build()
    }

    /// Creates a new builder for configuring a `WattpadClient`.
    ///
    /// This is the new entry point for custom client creation.
    pub fn builder() -> WattpadClientBuilder {
        WattpadClientBuilder::default()
    }

    /// Authenticates the client using a username and password.
    ///
    /// On a successful login, the API returns session cookies which are automatically
    /// stored in the client's cookie store for use in subsequent requests.
    ///
    /// # Arguments
    /// * `username` - The Wattpad username.
    /// * `password` - The Wattpad password.
    ///
    /// # Returns
    /// An empty `Ok(())` on successful authentication.
    ///
    /// # Errors
    /// Returns `WattpadError::AuthenticationFailed` if login is unsuccessful.
    pub async fn authenticate(&self, username: &str, password: &str) -> Result<(), WattpadError> {
        let url = "https://www.wattpad.com/auth/login?&_data=routes%2Fauth.login";

        let mut payload = HashMap::new();
        payload.insert("username", username);
        payload.insert("password", password);

        let response = self.http.post(url).form(&payload).send().await?;

        // A successful login is indicated by the presence of session cookies in the response.
        if response.cookies().next().is_none() {
            self.is_authenticated.store(false, Ordering::SeqCst);
            return Err(WattpadError::AuthenticationFailed);
        }

        self.is_authenticated.store(true, Ordering::SeqCst);
        Ok(())
    }

    /// Deauthenticates the client by logging out from Wattpad.
    ///
    /// This method sends a request to the logout endpoint, which invalidates the session
    /// cookies. It then sets the client's internal authentication state to `false`.
    ///
    /// # Returns
    /// An empty `Ok(())` on successful logout.
    ///
    /// # Errors
    /// Returns a `WattpadError` if the HTTP request fails.
    pub async fn deauthenticate(&self) -> Result<(), WattpadError> {
        let url = "https://www.wattpad.com/logout";

        // 1. Send a GET request to the logout URL. The reqwest client's cookie store
        //    will automatically handle the updated (cleared) session cookies from the response.
        self.http.get(url).send().await?;

        // 2. Set the local authentication flag to false.
        self.is_authenticated.store(false, Ordering::SeqCst);
        Ok(())
    }

    /// Checks if the client has been successfully authenticated.
    ///
    /// # Returns
    /// `true` if `authenticate` has been called successfully, `false` otherwise.
    pub fn is_authenticated(&self) -> bool {
        self.is_authenticated.load(Ordering::SeqCst)
    }
}

/// Provides a default implementation for `WattpadClient`.
///
/// This is equivalent to calling `WattpadClient::new()`.
impl Default for WattpadClient {
    fn default() -> Self {
        Self::new()
    }
}

/// A private helper function to process a `reqwest::Response`.
///
/// If the response status is successful, it deserializes the JSON body into type `T`.
/// Otherwise, it attempts to parse a specific `ApiErrorResponse` format from the body.
async fn handle_response<T: serde::de::DeserializeOwned>(
    response: reqwest::Response,
) -> Result<T, WattpadError> {
    if response.status().is_success() {
        let json = response.json::<T>().await?;
        Ok(json)
    } else {
        let error_response = response.json::<ApiErrorResponse>().await?;
        Err(error_response.into())
    }
}

// =================================================================================================

/// An internal builder for constructing and executing API requests.
///
/// This struct uses a fluent, chainable interface to build up an API call
/// with its path, parameters, fields, and authentication requirements before sending it.
pub(crate) struct WattpadRequestBuilder<'a> {
    client: &'a reqwest::Client,
    is_authenticated: &'a Arc<AtomicBool>,
    method: reqwest::Method,
    path: String,
    params: Vec<(&'static str, String)>,
    auth_required: bool,
}

impl<'a> WattpadRequestBuilder<'a> {
    /// Creates a new request builder.
    pub(crate) fn new(
        client: &'a reqwest::Client,
        is_authenticated: &'a Arc<AtomicBool>,
        method: reqwest::Method,
        path: &str,
    ) -> Self {
        Self {
            client,
            is_authenticated,
            method,
            path: path.to_string(),
            params: Vec::new(),
            auth_required: false,
        }
    }

    /// A private helper to check for endpoint authentication before sending a request.
    fn check_endpoint_auth(&self) -> Result<(), WattpadError> {
        if self.auth_required && !self.is_authenticated.load(Ordering::SeqCst) {
            return Err(WattpadError::AuthenticationRequired {
                field: "Endpoint".to_string(),
                context: format!("The endpoint at '{}' requires authentication.", self.path),
            });
        }
        Ok(())
    }

    /// Marks the entire request as requiring authentication.
    ///
    /// If this is set, the request will fail with an error if the client is not authenticated.
    pub(crate) fn requires_auth(mut self) -> Self {
        self.auth_required = true;
        self
    }

    /// Adds a query parameter to the request from an `Option`.
    ///
    /// If the value is `Some`, the parameter is added. If `None`, it's ignored.
    pub(crate) fn maybe_param<T: ToString>(mut self, key: &'static str, value: Option<T>) -> Self {
        if let Some(val) = value {
            self.params.push((key, val.to_string()));
        }
        self
    }

    /// Adds the `fields` query parameter for field selection.
    ///
    /// This method handles using default fields if none are provided. It also performs a
    /// crucial check to ensure that if any requested field requires authentication,
    /// the client is currently authenticated.
    ///
    /// # Errors
    /// Returns `WattpadError::AuthenticationRequired` if a field needs authentication
    /// but the client is not logged in.
    pub(crate) fn fields<T>(mut self, fields: Option<&[T]>) -> Result<Self, WattpadError>
    where
        T: ToString + DefaultableFields + AuthRequiredFields + PartialEq + Clone,
    {
        let fields_to_query = match fields {
            Some(f) if !f.is_empty() => Cow::from(f),
            _ => Cow::from(T::default_fields()),
        };

        if !self.is_authenticated.load(Ordering::SeqCst) {
            if let Some(auth_field) = fields_to_query.iter().find(|f| f.auth_required()) {
                return Err(WattpadError::AuthenticationRequired {
                    field: auth_field.to_string(),
                    context: format!(
                        "The field '{}' requires authentication.",
                        auth_field.to_string()
                    ),
                });
            }
        }

        let fields_str = fields_to_query
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>()
            .join(",");

        self.params.push(("fields", fields_str));
        Ok(self)
    }

    /// Adds a query parameter to the request.
    pub(crate) fn param<T: ToString>(mut self, key: &'static str, value: Option<T>) -> Self {
        if let Some(val) = value {
            self.params.push((key, val.to_string()));
        }
        self
    }

    /// Executes the request and deserializes the JSON response into a specified type `T`.
    pub(crate) async fn execute<T: serde::de::DeserializeOwned>(self) -> Result<T, WattpadError> {
        self.check_endpoint_auth()?;

        let url = format!("https://www.wattpad.com{}", self.path);
        let response = self
            .client
            .request(self.method, &url)
            .query(&self.params)
            .send()
            .await?;
        handle_response(response).await
    }

    /// Executes the request and returns the raw response body as a `String`.
    pub(crate) async fn execute_raw_text(self) -> Result<String, WattpadError> {
        self.check_endpoint_auth()?;

        let url = format!("https://www.wattpad.com{}", self.path);
        let response = self
            .client
            .request(self.method, &url)
            .query(&self.params)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.text().await?)
        } else {
            let error_response = response.json::<ApiErrorResponse>().await?;
            Err(error_response.into())
        }
    }

    /// Executes the request and returns the raw response body as `Bytes`.
    ///
    /// This method is ideal for downloading files or other binary content.
    pub(crate) async fn execute_bytes(self) -> Result<Bytes, WattpadError> {
        self.check_endpoint_auth()?;

        let url = format!("https://www.wattpad.com{}", self.path);
        let response = self
            .client
            .request(self.method, &url)
            .query(&self.params)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.bytes().await?)
        } else {
            let error_response = response.json::<ApiErrorResponse>().await?;
            Err(error_response.into())
        }
    }
}
