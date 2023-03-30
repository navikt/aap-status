use std::collections::HashSet;
use std::fmt::Formatter;
use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(untagged)]
enum DataOrEmpty<T> {
    Data(T),
    Empty {},
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct Repo {
    id: i64,
    pub name: String,
    full_name: String,
    html_url: String,
    pub deployments_url: String,
    // per_page=2 (dev,prod)
    releases_url: String,
    // per_page=1 (latest)
    pulls_url: String,
    // remove suffix {/number}
    description: Option<String>,
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
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
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
    repo: RepoMeta,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Head {
    #[serde(rename = "ref")]
    _ref: String,
    sha: String,
    repo: RepoMeta,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct RepoMeta {
    id: i64,
    url: String,
    name: String,
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
        write!(f, "{}\n{}",
               self.name,
               self.description.clone().unwrap_or_default(),
        )
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
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
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
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
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


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Deployment {
    url: String,
    pub id: i64,
    node_id: String,
    task: String,
    pub environment: String,
    pub created_at: String,
    pub updated_at: String,
    pub statuses_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Status {
    url: String,
    id: i64,
    node_id: String,
    pub state: String,
    pub description: String,
}

