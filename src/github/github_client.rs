use std::collections::HashSet;

use ehttp::Request;
use poll_promise::Promise;

use crate::github::pulls::PullRequest;
use crate::github::repositories::Repo;
use crate::github::teams::Team;
use crate::github::workflows::{Workflow, WorkflowRun};

#[derive(Default)]
pub struct GitHubApi {}

pub trait Fetcher {
    fn fetch(&self, token: &mut String, url: &str, callback: impl 'static + Send + FnOnce(Vec<u8>));
}

pub trait Pulls {
    fn pull_requests(&self, token: &mut String, repo: &str) -> Promise<HashSet<PullRequest>>;
}

pub trait Repositories {
    fn repositories(&self, token: &mut String, team: &Team) -> Promise<HashSet<Repo>>;
}

pub trait Workflows {
    fn workflows(&self, token: &mut String, repo: &str) -> Promise<HashSet<Workflow>>;
    fn workflow_runs(&self, token: &mut String, repo: &str) -> Promise<HashSet<WorkflowRun>>;
}

pub trait Teams {
    fn team(&self, name: &str, token: &str) -> Promise<Option<Team>>;
}

impl Fetcher for GitHubApi {
    fn fetch(&self, token: &mut String, url: &str, callback: impl 'static + Send + FnOnce(Vec<u8>)) {
        let request = Request {
            headers: ehttp::headers(&[
                ("Accept", "application/vnd.github+json"),
                ("User-Agent", "rust web-api-client demo"),
                ("Authorization", format!("Bearer {}", token.trim()).as_str()),
            ]),
            ..Request::get(url)
        };

        ehttp::fetch(request, |response| {
            if let Ok(res) = response { callback(res.bytes) }
        });
    }
}
