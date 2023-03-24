use std::collections::HashSet;
use std::sync::{Arc, Mutex};

use poll_promise::Promise;
use serde::{Deserialize, Serialize};

use crate::github::github_client::{GitHubApi, Repositories, Teams};
use crate::github::teams::Team;

impl Repositories for GitHubApi {

    // TODO: hvis man vet team-id trenger man ikke finne den ved å skumme igjennom alle teamsa
    fn repositories(
        &self,
        token: &mut String,
        team_name: &str,
        callback: impl 'static + Send + FnOnce(HashSet<Repo>),
    ) {
        let teams_acc: Arc<Mutex<HashSet<Team>>> = Arc::new(Mutex::new(HashSet::new()));
        let base_url = String::from("https://api.github.com/orgs/navikt/teams?per_page=100&page=");

        for i in 1..=3 {
            let url = format!("{}{}", base_url, i);
            let teams = teams_acc.clone();
            let teams_to_add = self.teams(&url, token).block_and_take();
            teams.lock().unwrap().extend(teams_to_add.into_iter());
        }

        match teams_acc.lock().unwrap().clone().into_iter().find(|team| { team.name == team_name }) {
            None => println!("Fant ikke ditt team"),
            Some(team) => {
                println!("Fant teamet: {}", team);
                let team_repos = self.repos(&team.repositories_url, token).block_and_take();
                callback(team_repos);
            }
        };
    }

    fn repos(&self, url: &str, token: &str) -> Promise<HashSet<Repo>> {
        let paginated_url = format!("{}{}", url, "?per_page=100");
        println!("Fetching: {}", &paginated_url);

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
