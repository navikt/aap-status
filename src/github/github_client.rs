use std::collections::HashSet;

use poll_promise::Promise;

use crate::github::pulls::PullRequest;
use crate::github::repositories::Repo;
use crate::github::runs::WorkflowRun;
use crate::github::teams::Team;
use crate::github::workflows::Workflow;

#[derive(Default)]
pub struct GitHubApi {}

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
