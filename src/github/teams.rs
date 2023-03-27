use std::collections::HashSet;
use std::fmt::Formatter;

use poll_promise::Promise;
use serde::{Deserialize, Serialize};

use crate::github::github_client::{GitHubApi, Teams};

impl Teams for GitHubApi {
    fn team(&self, name: &str, token: &str) -> Promise<Option<Team>> {
        let request = ehttp::Request {
            headers: ehttp::headers(&[
                ("Accept", "application/vnd.github+json"),
                ("User-Agent", "Rust-wasm-App"),
                ("Authorization", format!("Bearer {}", token.trim()).as_str()),
            ]),
            ..ehttp::Request::get(format!("https://api.github.com/orgs/navikt/teams/{name}").as_str())
        };

        let (sender, promise) = Promise::new();

        ehttp::fetch(request, move |response| {
            match response {
                Ok(res) => {
                    match serde_json::from_slice::<Team>(&res.bytes) {
                        Ok(team) => sender.send(Some(team)),
                        Err(e) => {
                            println!("Failed to parse from slice: {:?}", e);
                            sender.send(None);
                        }
                    }
                }
                Err(e) => {
                    println!("Failed to fetch: {}", e);
                    sender.send(None);
                }
            };
        });

        promise
    }

    fn teams(&self, url: &str, token: &str) -> Promise<HashSet<Team>> {
        println!("Fetching: {}", &url);

        let request = ehttp::Request {
            headers: ehttp::headers(&[
                ("Accept", "application/vnd.github+json"),
                ("User-Agent", "Rust-wasm-App"),
                ("Authorization", format!("Bearer {}", token.trim()).as_str()),
            ]),
            ..ehttp::Request::get(url)
        };

        let (sender, promise) = Promise::new();

        ehttp::fetch(request, move |response| {
            println!("response: {:?}", &response);
            match response {
                Ok(res) => {
                    match serde_json::from_slice::<HashSet<Team>>(&res.bytes) {
                        Ok(teams) => {
                            println!("Parsed {} bytes from slice", teams.len());
                            sender.send(teams);
                        }
                        Err(e) => {
                            println!("Failed to parse from slice: {:?}", e);
                            sender.send(HashSet::new());
                        }
                    }
                }
                Err(e) => {
                    println!("Failed to fetch: {}", e);
                    sender.send(HashSet::new());
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
