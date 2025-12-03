
use serde::Deserialize;
use crate::model::Story;
use crate::pagination::PaginatedResponse;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserStories {
    pub stories: Vec<Story>,
    pub next_url: Option<String>,
}

impl PaginatedResponse<Story> for UserStories {
    fn into_items(self) -> Vec<Story> {
        self.stories
    }

    fn has_next_page(&self) -> bool {
        self.next_url.is_some()
    }
}