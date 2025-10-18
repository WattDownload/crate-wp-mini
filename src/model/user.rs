use serde::Deserialize;

/// Represents a full user object from the Wattpad API.
///
/// This struct contains all the publicly available fields for a user's profile.
/// Some fields, noted in the comments, are only available when making an authenticated
/// request for the current user's own profile.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    /// The user's unique, public username.
    pub username: Option<String>,
    /// The URL for the user's profile picture (avatar).
    pub avatar: Option<String>,
    /// A boolean flag indicating if the user's profile is private.
    pub is_private: Option<bool>,
    /// The URL for the user's profile background image.
    pub background_url: Option<String>,
    /// The user's display name.
    pub name: Option<String>,
    /// The user's full display name.
    #[serde(rename = "fullname")]
    pub full_name: Option<String>,
    /// The "About Me" or biography section of the user's profile.
    pub description: Option<String>,
    /// A list of badges the user has earned.
    pub badges: Option<Vec<String>>,
    /// The user's current status message.
    pub status: Option<String>,
    /// The user's specified gender as a string.
    pub gender: Option<String>,
    /// A numerical code representing the user's gender.
    pub gender_code: Option<String>,
    /// The numerical identifier for the user's preferred language.
    pub language: Option<i64>,
    /// The user's locale string (e.g., "en_US").
    pub locale: Option<String>,
    /// The timestamp when the user's account was created.
    pub create_date: Option<String>,
    /// The timestamp when the user's profile was last modified.
    pub modify_date: Option<String>,
    /// The user's self-reported location.
    pub location: Option<String>,
    /// A boolean flag indicating if the user is a verified account (e.g., a celebrity or public figure).
    pub verified: Option<bool>,
    /// A boolean flag indicating if the user is a Wattpad Ambassador.
    pub ambassador: Option<bool>,
    /// A link to the user's Facebook profile.
    pub facebook: Option<String>,
    /// A link to the user's personal website.
    pub website: Option<String>,
    /// A link to the user's Lulu profile.
    pub lulu: Option<String>,
    /// A link to the user's Smashwords profile.
    pub smashwords: Option<String>,
    /// A link to the user's Bubok profile.
    pub bubok: Option<String>,
    /// The total number of votes received across all of the user's stories.
    pub votes_received: Option<i64>,
    /// The number of stories the user has published.
    pub num_stories_published: Option<i64>,
    /// The number of other users this user is following.
    pub num_following: Option<i64>,
    /// The number of followers this user has.
    pub num_followers: Option<i64>,
    /// The number of public messages on the user's profile.
    pub num_messages: Option<i64>,
    /// The number of public reading lists the user has created.
    pub num_lists: Option<i64>,
    /// A boolean flag indicating if the user has verified their email address.
    #[serde(rename = "verified_email")]
    pub verified_email: Option<bool>,
    /// A list of the user's preferred reading categories.
    #[serde(rename = "preferred_categories")]
    pub preferred_categories: Option<Vec<String>>,
    /// A boolean flag indicating if the user allows search engine crawlers to index their profile.
    pub allow_crawler: Option<bool>,
    /// A deep link URL for the user's profile, often used for mobile app integration.
    pub deeplink: Option<String>,

    // // Authenticated fields (only available for the currently logged-in user)
    // pub follower: Option<bool>,
    // pub is_muted: Option<bool>,
    // pub following: Option<bool>,
    // pub birthdate: Option<String>,
    // pub inbox: Option<Inbox>,
    // pub notifications: Option<Notifications>,
    // #[serde(rename = "connectedServices")]
    // pub connected_services: Option<ConnectedServices>,
    // pub age: Option<i64>,
    // pub email: Option<String>,
    // pub has_password: Option<bool>,
}