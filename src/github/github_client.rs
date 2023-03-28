use std::collections::{BTreeMap, HashSet};
use std::sync::{Arc, Mutex};

use ehttp::{Request, Response};
use poll_promise::Promise;

use crate::github::pulls::PullRequest;
use crate::github::repositories::Repo;
use crate::github::runs::WorkflowRun;
use crate::github::teams::Team;
use crate::github::workflows::Workflow;

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

pub trait Runs {
    fn runs(&self, token: &mut String, repo: &str) -> Promise<HashSet<WorkflowRun>>;
}

pub trait Workflows {
    fn workflows(&self, token: &mut String, repo: &str) -> Promise<HashSet<Workflow>>;
}

pub trait Teams {
    fn team(&self, name: &str, token: &str) -> Promise<Option<Team>>;
    fn teams(&self, url: &str, token: &str) -> Promise<HashSet<Team>>;
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
