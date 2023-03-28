use std::collections::HashSet;
use std::hash::{Hash, Hasher};

use poll_promise::Promise;
use serde::{Deserialize, Serialize};

use crate::github::github_client::{GitHubApi, Workflows};

impl Workflows for GitHubApi {
    fn workflows(&self, token: &mut String, repo: &str) -> Promise<HashSet<Workflow>> {

        let request = ehttp::Request {
            headers: ehttp::headers(&[
                ("Accept", "application/vnd.github+json"),
                ("User-Agent", "rust web-api-client demo"),
                ("Authorization", format!("Bearer {}", token.trim()).as_str()),
            ]),
            ..ehttp::Request::get(format!("https://api.github.com/repos/navikt/{}/actions/workflows", repo))
        };

        let (sender, promise) = Promise::new();

        ehttp::fetch(request, move |response| {
            match response {
                Ok(res) => {
                    match serde_json::from_slice::<WorkflowsResponse>(&res.bytes) {
                        Ok(workflows) => sender.send(workflows.workflows),
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

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
struct WorkflowsResponse {
    pub total_count: i32,
    pub workflows: HashSet<Workflow>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Workflow {
    pub id: i64,
    pub node_id: String,
    pub name: String,
    pub path: String,
    pub state: String,
}

impl Hash for Workflow {
    fn hash<H: Hasher>(&self, state: &mut H) { self.id.hash(state) }
}