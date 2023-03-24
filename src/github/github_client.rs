use std::collections::HashSet;

use poll_promise::Promise;

use crate::github::pulls::PullRequest;
use crate::github::repositories::Repo;
use crate::github::runs::WorkflowRuns;
use crate::github::teams::Team;
use crate::github::workflows::Workflow;

#[derive(Default)]
pub struct GitHubApi {}

pub trait Pulls {
    fn pull_requests(
        &self,
        token: &mut String,
        repo: &str,
        callback: impl 'static + Send + FnOnce(Vec<PullRequest>),
    );
}

pub trait Repositories {
    fn repositories(
        &self,
        token: &mut String,
        team_name: &str,
        callback: impl 'static + Send + FnOnce(HashSet<Repo>),
    );

    fn repos(
        &self,
        url: &str,
        token: &str,
    ) -> Promise<HashSet<Repo>>;
}

pub trait Runs {
    fn runs(
        &self,
        token: &mut String,
        repo: &str,
        callback: impl 'static + Send + FnOnce(WorkflowRuns),
    );
}

pub trait Workflows {
    fn workflows(
        &self,
        token: &mut String,
        repo: &str,
        callback: impl 'static + Send + FnOnce(Vec<Workflow>),
    );
}

pub trait Teams {
    fn teams(
        &self,
        url: &str,
        token: &str,
    ) -> Promise<HashSet<Team>>;
}
