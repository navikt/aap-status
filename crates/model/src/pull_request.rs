use std::string::ToString;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct PullRequest {
    pub id: i64,
    pub url: String,
    html_url: Option<String>,
    title: Option<String>,
    user: Option<User>,
    head: Head,
    updated_at: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
struct Head {
    repo: Repo,
}

#[derive(Serialize, Deserialize, Clone, Default)]
struct Repo {
    name: String,
}

#[derive(Serialize, Deserialize, Clone, Default)]
struct User {
    login: String,
}

const UNKNOWN: &str = "unknown";

impl PullRequest {
    pub fn html(&self) -> String { self.html_url.clone().unwrap_or(self.url.clone()) }
    pub fn title(&self) -> String { self.title.clone().unwrap_or(UNKNOWN.to_string()) }
    pub fn user(&self) -> String { self.user.clone().unwrap_or(User { login: UNKNOWN.to_string() }).login }
    pub fn updated_at(&self) -> String { self.updated_at.clone().unwrap_or(UNKNOWN.to_string()) }
    pub fn repo(&self) -> String { self.head.repo.name.clone() }
}

// #[derive(Serialize, Deserialize)]
// #[serde(default)]
// pub struct PullRequest {
//     id: i64,
//     url: String,
//     html_url: String,
//     title: String,
//     user: User,
//     updated_at: String,
// }
//
// #[derive(Serialize, Deserialize)]
// pub struct User {
//     login: String,
// }


