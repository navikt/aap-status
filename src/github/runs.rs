use serde::{Deserialize, Serialize};

use crate::github::github_client::{GitHubApi, Runs};
use crate::github::pulls::PullRequest;

impl Runs for GitHubApi {
    fn runs(
        &self,
        token: &mut String,
        repo: &str,
        callback: impl 'static + Send + FnOnce(WorkflowRuns),
    ) {
        let url = format!("https://api.github.com/repos/navikt/{}/actions/runs", repo);

        let request = ehttp::Request {
            headers: ehttp::headers(&[
                ("Accept", "application/vnd.github+json"),
                ("User-Agent", "rust web-api-client demo"),
                ("Authorization", format!("Bearer {}", token.trim()).as_str()),
            ]),
            ..ehttp::Request::get(&url)
        };

        ehttp::fetch(request, move |result: ehttp::Result<ehttp::Response>| {
            match result {
                Ok(res) => {
                    match serde_json::from_slice(&res.bytes) {
                        Ok(runs) => callback(runs),
                        Err(e) => println!("error: {:?} when parsing runs with content {:?}", e, res)
                    }
                }
                Err(e) => println!("Error {:?} from {:?}", e, &url)
            }
        });
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct WorkflowRuns {
    pub total_count: i32,
    pub workflow_runs: Vec<WorkflowRun>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WorkflowRun {
    id: i64,
    pub name: Option<String>,
    check_suite_id: Option<i64>,
    check_suite_node_id: Option<String>,
    head_sha: String,
    path: String,
    run_number: i32,
    pub run_attempt: i32,
    pub event: String,
    pub status: Option<String>,
    pub conclusion: Option<String>,
    pub workflow_id: i64,
    url: String,
    html_url: String,
    pull_requests: Vec<PullRequest>,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
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
