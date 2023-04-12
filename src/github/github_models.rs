use std::fmt::Formatter;

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(untagged)]
enum DataOrEmpty<T> {
    Data(T),
    Empty {},
}

#[derive(Deserialize, Serialize, Clone, Eq, PartialEq)]
pub struct Repo {
    id: i64,
    pub name: String,
    full_name: String,
    html_url: String,
    pub deployments_url: String,
    releases_url: String,
    pulls_url: String,
    description: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct PullRequests {
    pub prs: Vec<PullRequest>
}

#[derive(Serialize, Deserialize, Clone, Default, Eq, PartialEq, Hash)]
pub struct PullRequest {
    id: i64,
    number: i64,
    pub url: String,
    pub html_url: Option<String>,
    pub title: Option<String>,
    // body: Option<String>,
    state: Option<String>,
    pub user: Option<User>,
    created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default, Eq, PartialEq, Hash)]
pub struct User {
    pub login: String,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct RepoMeta {
    id: i64,
    url: String,
    name: String,
}

#[derive(Deserialize, Serialize, Clone, Default)]
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

#[derive(Serialize, Deserialize, Clone)]
struct WorkflowsResponse {
    pub total_count: i32,
    pub workflows: Vec<Workflow>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Workflow {
    pub id: i64,
    pub node_id: String,
    pub name: String,
    pub path: String,
    pub state: String,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct WorkflowRuns {
    pub total_count: i32,
    pub workflow_runs: Vec<WorkflowRun>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
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

#[derive(Serialize, Deserialize, Clone)]
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


#[derive(Serialize, Deserialize, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Status {
    url: String,
    pub id: i64,
    node_id: String,
    pub state: State,
    pub description: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum State {
    Error,
    Failure,
    Inactive,
    Pending,
    Success,
    Queued,
    InProgress,
}

impl Status {
    pub fn colored_state(&self) -> egui::text::LayoutJob {
        use eframe::epaint::Color32;
        use eframe::epaint::FontId;

        let color = match self.state {
            State::Error => Color32::LIGHT_RED,
            State::Failure => Color32::LIGHT_RED,
            State::Inactive => Color32::LIGHT_GRAY,
            State::Pending => Color32::LIGHT_BLUE,
            State::Success => Color32::LIGHT_GREEN,
            State::Queued => Color32::LIGHT_BLUE,
            State::InProgress => Color32::LIGHT_RED
        };

        egui::text::LayoutJob::simple_singleline(
            format!("{:?}", self.state.clone()),
            FontId::default(),
            color,
        )
    }
}


#[derive(Serialize, Deserialize, Clone)]
pub struct Environments {
    pub total_count: i32,
    pub environments: Vec<Environment>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Environment {
    url: String,
    pub id: i64,
    node_id: String,
    pub name: String,
    pub html_url: String,
}
