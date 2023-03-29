use std::fmt::Formatter;

use poll_promise::Promise;
use serde::{Deserialize, Serialize};

use crate::github::github_client::{GitHubApi, Teams};

impl Teams for GitHubApi {
    fn team(&self, name: &str, token: &str) -> Promise<Option<Team>> {
        let url = format!("https://api.github.com/orgs/navikt/teams/{name}");
        let request = ehttp::Request {
            headers: ehttp::headers(&[
                ("Accept", "application/vnd.github+json"),
                ("User-Agent", "Rust-wasm-App"),
                ("Authorization", format!("Bearer {}", token.trim()).as_str()),
            ]),
            ..ehttp::Request::get(&url)
        };

        let (sender, promise) = Promise::new();

        ehttp::fetch(request, move |response| {
            match response {
                Ok(res) => {
                    match serde_json::from_slice::<Team>(&res.bytes) {
                        Ok(team) => sender.send(Some(team)),
                        Err(e) => {
                            tracing::error!{%e, "Failed to deserialize {url}"}
                            sender.send(None);
                        }
                    }
                }
                Err(e) => {
                    tracing::error!{%e, "Failed to fetch {url}"}
                    sender.send(None);
                }
            };
        });

        promise
    }
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Hash, Default)]
pub struct Team {
    pub name: String,
    id: i64,
    node_id: String,
    slug: String,
    description: Option<String>,
    privacy: String,
    url: String,
    html_url: String,
    members_url: String,
    pub repositories_url: String,
    permission: String,
}

impl std::fmt::Display for Team {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Name:{} ID:{}", self.name, self.id)
    }
}
