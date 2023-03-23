use std::collections::HashSet;

use poll_promise::Promise;

use crate::github::pulls::PullRequest;
use crate::github::repositories::Repo;
use crate::github::runs::WorkflowRuns;
use crate::github::teams::Team;
use crate::github::workflows::Workflow;

pub struct GitHubApi {}

impl Default for GitHubApi {
    fn default() -> Self { Self {} }
}

pub trait Pulls {
    fn pull_requests(
        &self,
        token: &mut String,
        repo: &String,
        callback: impl 'static + Send + FnOnce(Vec<PullRequest>),
    );
}

pub trait Repositories {
    fn repositories(
        &self,
        token: &mut String,
        team_name: &String,
        callback: impl 'static + Send + FnOnce(HashSet<Repo>),
    );

    fn repos(
        &self,
        url: &String,
        token: &String,
    ) -> Promise<HashSet<Repo>>;
}

pub trait Runs {
    fn runs(
        &self,
        token: &mut String,
        repo: &String,
        callback: impl 'static + Send + FnOnce(WorkflowRuns),
    );
}

pub trait Workflows {
    fn workflows(
        &self,
        token: &mut String,
        repo: &String,
        callback: impl 'static + Send + FnOnce(Vec<Workflow>),
    );
}

pub trait Teams {
    fn teams(
        &self,
        url: &String,
        token: &String,
    ) -> Promise<HashSet<Team>>;
}
