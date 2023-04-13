use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default)]
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
    pub run_attempt: i32,
    pub event: String,
    pub conclusion: Option<String>,
    pub workflow_id: i64,
    pub html_url: String,
    pub run_started_at: Option<String>,
    pub actor: Option<Actor>,
    pub triggering_actor: Option<Actor>,
    pub jobs_url: String,
    pub logs_url: String,
    pub check_suite_url: String,
    pub artifacts_url: String,
    pub cancel_url: String,
    pub rerun_url: String,
    pub workflow_url: String,
    pub display_title: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Actor {
    pub name: Option<String>,
    pub email: Option<String>,
    pub login: String,
    pub id: i64,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    #[serde(rename = "type")]
    pub actor_type: String,
}