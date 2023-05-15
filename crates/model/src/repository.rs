use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Default)]
#[derive(Eq, PartialEq)] // use binary operation !=
pub struct Repository {
    pub id: i64,
    pub name: String,
    pub full_name: String,
    pub html_url: String,
    pub deployments_url: String,
    pub releases_url: String,
    pulls_url: String,
    pub description: Option<String>,
    pub archived: bool,
}

impl Repository {
    pub fn pulls_url(&self) -> &str {
        self.pulls_url.strip_suffix("{/number}").unwrap_or(&self.pulls_url)
    }
}
