use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex};

use ehttp::Request;
use poll_promise::Promise;
use serde::{Deserialize, Serialize};

use crate::github::github_client::{GitHubApi, Pulls};

impl Pulls for GitHubApi {
    fn pull_requests(&self, token: &mut String, repo: &str) -> Promise<HashSet<PullRequest>> {
        let (sender, promise) = Promise::new();
        let request = Request {
            headers: ehttp::headers(&[
                ("Accept", "application/vnd.github+json"),
                ("User-Agent", "rust web-api-client demo"),
                ("Authorization", format!("Bearer {}", token.trim()).as_str()),
            ]),
            ..Request::get(format!("https://api.github.com/repos/navikt/{}/pulls", repo))
        };

        ehttp::fetch(request, move |response| {
            match response {
                Ok(res) => {
                    match serde_json::from_slice::<HashSet<PullRequest>>(&res.bytes) {
                        Ok(value) => sender.send(value),
                        Err(e) => {
                            println!("Failed to parse from slice: {:?}", e);
                            sender.send(HashSet::default());
                        }
                    }
                }
                Err(e) => {
                    println!("Failed to fetch: {}", e);
                    sender.send(HashSet::default());
                }
            };
        });

        promise
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum DataOrEmpty<T> {
    Data(T),
    Empty {},
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct PullsResponse {
    pub pull_requests: HashSet<PullRequest>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct PullRequest {
    id: i32,
    number: i32,
    url: String,
    head: Head,
    base: Base,
    pub html_url: Option<String>,
    pub title: Option<String>,
    body: Option<String>,
    state: Option<String>,
    pub user: Option<User>,
    created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl Hash for PullRequest {
    fn hash<H: Hasher>(&self, state: &mut H) { self.id.hash(state) }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct User {
    pub login: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Base {
    #[serde(rename = "ref")]
    _ref: String,
    sha: String,
    repo: Repo,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Head {
    #[serde(rename = "ref")]
    _ref: String,
    sha: String,
    repo: Repo,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Repo {
    id: i64,
    url: String,
    name: String,
}
