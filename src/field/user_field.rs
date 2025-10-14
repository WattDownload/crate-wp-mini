use crate::field::{AuthRequiredFields, DefaultableFields};
use strum_macros::{AsRefStr, Display};

/// Represents the fields that can be requested for a `User` object from the Wattpad API.
#[derive(Debug, Clone, Display, AsRefStr, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[strum(serialize_all = "camelCase")]
pub enum UserField {
    /// The user's unique, public username.
    Username,
    /// The URL for the user's profile picture (avatar).
    Avatar,
    /// A boolean flag indicating if the user's profile is private.
    IsPrivate,
    /// The URL for the user's profile background image.
    BackgroundUrl,
    /// The user's full display name.
    Name,
    /// The "About Me" or biography section of the user's profile.
    Description,
    /// A list of badges the user has earned.
    Badges,
    /// The user's current status message.
    Status,
    /// The user's specified gender as a string.
    Gender,
    /// A numerical code representing the user's gender.
    GenderCode,
    /// The numerical identifier for the user's preferred language.
    Language,
    /// The user's locale string (e.g., "en_US").
    Locale,
    /// The timestamp when the user's account was created.
    CreateDate,
    /// The timestamp when the user's profile was last modified.
    ModifyDate,
    /// The user's self-reported location.
    Location,
    /// A boolean flag indicating if the user is a verified account (e.g., a celebrity or public figure).
    Verified,
    /// A boolean flag indicating if the user is a Wattpad Ambassador.
    Ambassador,
    /// A link to the user's Facebook profile.
    Facebook,
    /// A link to the user's personal website.
    Website,
    /// A link to the user's Lulu profile.
    Lulu,
    /// A link to the user's Smashwords profile.
    Smashwords,
    /// A link to the user's Bubok profile.
    Bubok,
    /// The total number of votes received across all of the user's stories.
    VotesReceived,
    /// The number of stories the user has published.
    NumStoriesPublished,
    /// The number of other users this user is following.
    NumFollowing,
    /// The number of followers this user has.
    NumFollowers,
    /// The number of public messages on the user's profile.
    NumMessages,
    /// The number of public reading lists the user has created.
    NumLists,
    /// A boolean flag indicating if the user has verified their email address.
    #[strum(serialize = "verified_email")]
    VerifiedEmail,
    /// A list of the user's preferred reading categories.
    #[strum(serialize = "preferred_categories")]
    PreferredCategories,
    /// A boolean flag indicating if the user allows search engine crawlers to index their profile.
    AllowCrawler,
    /// A deep link URL for the user's profile, often used for mobile app integration.
    Deeplink,
}

impl DefaultableFields for UserField {
    fn default_fields() -> Vec<Self> {
        vec![
            Self::Username,
            Self::Avatar,
            Self::BackgroundUrl,
            Self::Name,
            Self::Description,
            Self::CreateDate,
            Self::ModifyDate,
            Self::VotesReceived,
            Self::NumStoriesPublished,
            Self::NumFollowing,
            Self::NumFollowers,
            Self::NumMessages,
            Self::NumLists,
        ]
    }
}

impl AuthRequiredFields for UserField {}