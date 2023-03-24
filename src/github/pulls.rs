use serde::{Deserialize, Serialize};

use crate::github::github_client::{GitHubApi, Pulls};

impl Pulls for GitHubApi {
    fn pull_requests(
        &self, token:
        &mut String,
        repo: &str,
        callback: impl 'static + Send + FnOnce(Vec<PullRequest>),
    ) {
        let url = format!("https://api.github.com/repos/navikt/{}/pulls", repo);

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
                    match serde_json::from_slice::<DataOrEmpty<Vec<PullRequest>>>(&res.bytes) {
                        Ok(pulls) => {
                            match pulls {
                                DataOrEmpty::Data(prs) => callback(prs),
                                DataOrEmpty::Empty {} => callback(Vec::<PullRequest>::default()),
                            };
                        }
                        Err(e) => println!("error: {:?} when parsing pulls with content {:?}", e, res)
                    }
                }
                Err(e) => println!("Error {:?} from {:?}", e, &url)
            }
        });
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum DataOrEmpty<T> {
    Data(T),
    Empty {},
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PullsResponse {
    pub pull_requests: Vec<PullRequest>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PullRequest {
    id: i32,
    pub number: i32,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub login: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Base {
    #[serde(rename = "ref")]
    _ref: String,
    sha: String,
    repo: Repo,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Head {
    #[serde(rename = "ref")]
    _ref: String,
    sha: String,
    repo: Repo,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Repo {
    id: i64,
    url: String,
    name: String,
}