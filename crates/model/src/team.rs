use std::fmt::Formatter;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Default)]
pub struct Team {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub privacy: String,
    pub url: String,
    pub html_url: String,
    pub members_url: String,
    pub repositories_url: String,
    pub permission: String,
}

impl std::fmt::Display for Team {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}",
               self.name,
               self.description.clone().unwrap_or_default(),
        )
    }
}