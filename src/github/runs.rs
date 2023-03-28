use std::collections::HashSet;
use std::hash::{Hash, Hasher};

use poll_promise::Promise;
use serde::{Deserialize, Serialize};

use crate::github::github_client::{GitHubApi, Runs};
use crate::github::pulls::PullRequest;

impl Runs for GitHubApi {
    fn runs(&self, token: &mut String, repo: &str) -> Promise<HashSet<WorkflowRun>> {
        let request = ehttp::Request {
            headers: ehttp::headers(&[
                ("Accept", "application/vnd.github+json"),
                ("User-Agent", "rust web-api-client demo"),
                ("Authorization", format!("Bearer {}", token.trim()).as_str()),
            ]),
            ..ehttp::Request::get(format!("https://api.github.com/repos/navikt/{}/actions/runs?status=failure&per_page=10", repo))
        };

        let (sender, promise) = Promise::new();

        ehttp::fetch(request, move |response| {
            match response {
                Ok(res) => {
                    match serde_json::from_slice::<WorkflowRuns>(&res.bytes) {
                        Ok(runs) => sender.send(runs.workflow_runs),
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

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct WorkflowRuns {
    pub total_count: i32,
    pub workflow_runs: HashSet<WorkflowRun>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct WorkflowRun {
    pub id: i64,
    pub name: Option<String>,
    check_suite_id: Option<i64>,
    check_suite_node_id: Option<String>,
    head_sha: String,
    path: String,
    run_number: i32,
    pub run_attempt: i32,
    pub event: String,
    status: Option<String>,
    pub conclusion: Option<String>,
    pub workflow_id: i64,
    url: String,
    pub html_url: String,
    pull_requests: HashSet<PullRequest>,
    created_at: String,
    updated_at: String,
    actor: Option<Actor>,
    triggering_actor: Option<Actor>,
    pub run_started_at: Option<String>,
    jobs_url: String,
    logs_url: String,
    check_suite_url: String,
    artifacts_url: String,
    cancel_url: String,
    rerun_url: String,
    workflow_url: String,
    display_title: String,
}

impl Hash for WorkflowRun {
    fn hash<H: Hasher>(&self, state: &mut H) { self.id.hash(state) }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Actor {
    name: Option<String>,
    email: Option<String>,
    login: String,
    id: i64,
    node_id: String,
    avatar_url: String,
    gravatar_id: String,
    url: String,
    html_url: String,
    #[serde(rename = "type")]
    _type: String,
}
