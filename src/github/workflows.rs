use serde::{Deserialize, Serialize};

use crate::github::github_client::{GitHubApi, Workflows};

impl Workflows for GitHubApi {
    fn workflows(
        &self,
        token: &mut String,
        repo: &str,
        callback: impl 'static + Send + FnOnce(Vec<Workflow>),
    ) {
        let url = format!("https://api.github.com/repos/navikt/{}/actions/workflows", repo);

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
                    match serde_json::from_slice::<WorkflowsResponse>(&res.bytes) {
                        Ok(workflows) => callback(workflows.workflows),
                        Err(e) => println!("error: {:?} when parsing runs with content {:?}", e, res)
                    }
                }
                Err(e) => println!("Error {:?} from {:?}", e, &url)
            }
        });
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct WorkflowsResponse {
    pub total_count: i32,
    pub workflows: Vec<Workflow>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Workflow {
    pub id: i64,
    pub node_id: String,
    pub name: String,
    pub path: String,
    pub state: String,
}
