use std::collections::HashSet;

use poll_promise::Promise;
use serde::{Deserialize, Serialize};

use crate::github::github_client::{GitHubApi, Repositories};
use crate::github::teams::Team;

impl Repositories for GitHubApi {
    fn repositories(
        &self,
        token: &mut String,
        team: &Team,
    ) -> Promise<HashSet<Repo>> {

        let paginated_url = format!("{}{}", &team.repositories_url, "?per_page=100");
        println!("forsøker å hente {}", &paginated_url);

        let request = ehttp::Request {
            headers: ehttp::headers(&[
                ("Accept", "application/vnd.github+json"),
                ("User-Agent", "Rust-wasm-App"),
                ("Authorization", format!("Bearer {}", token.trim()).as_str()),
            ]),
            ..ehttp::Request::get(paginated_url)
        };

        let (sender, promise) = Promise::new();

        ehttp::fetch(request, move |response| {
            println!("response: {:?}", &response);
            match response {
                Ok(res) => {
                    match serde_json::from_slice::<HashSet<Repo>>(&res.bytes) {
                        Ok(teams) => sender.send(teams),
                        Err(_) => sender.send(HashSet::new()),
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

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct Repo {
    id: i64,
    pub name: String,
    full_name: String,
    // org/name filter by prefix navikt/aap
    html_url: String,
    deployments_url: String,
    // per_page=2 (dev,prod)
    releases_url: String,
    // per_page=1 (latest)
    pulls_url: String,
    // remove suffix {/number}
    description: Option<String>,
}
