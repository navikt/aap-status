use egui::Ui;
use http::github::Client;
use serde::{Deserialize, Serialize};

use model::repository::Repository;

use crate::panel_deployment::DeploymentPanel;
use crate::panel_pull_request::PullRequestsPanel;
use crate::panel_repository::RepositoriesPanel;
use crate::panel_workflows::WorkflowPanel;

#[derive(Deserialize, Serialize, Clone)]
pub enum SelectedPanel {
    Repositories(Client),
    PullRequests(Client),
    Deployments(Client),
    WorkflowRuns(Client),
}

impl Default for SelectedPanel {
    fn default() -> Self {
        SelectedPanel::Repositories(Client::default())
    }
}

impl SelectedPanel {
    pub fn get_client(&self) -> Client {
        match self {
            SelectedPanel::Repositories(client) => client.clone(),
            SelectedPanel::PullRequests(client) => client.clone(),
            SelectedPanel::Deployments(client) => client.clone(),
            SelectedPanel::WorkflowRuns(client) => client.clone(),
        }
    }
}

#[derive(Deserialize, Serialize, Default)]
pub struct Panels {
    pub selected: SelectedPanel,
    pub repositories: RepositoriesPanel,
    pub pull_requests: PullRequestsPanel,
    pub deployment: DeploymentPanel,
    pub workflow: WorkflowPanel,
}

impl Panels {
    pub fn rate_limit(&self) -> usize {
        self.selected.get_client().get_rate_limit()
    }

    pub fn rate_limit_reset(&self) -> u64 {
        self.selected.get_client().get_rate_reset()
    }

    pub fn paint_repositories(&mut self, ui: &mut Ui, token: &str) {
        self.repositories.paint(ui, token);
    }

    pub fn paint_pull_requests(&mut self, ui: &mut Ui, token: &str) {
        self.pull_requests.set_repositories(self.repositories.repositories());
        self.pull_requests.paint(ui, token);
    }

    pub fn paint_deployments(&mut self, ui: &mut Ui, token: &str) {
        self.deployment.set_repositories(self.repositories.repositories());
        self.deployment.paint(ui, token);
    }

    pub fn paint_workflows(&mut self, ui: &mut Ui, token: &str) {
        self.workflow.set_repositories(self.repositories.repositories());
        self.workflow.paint(ui, token);
    }
}

pub trait Panel {
    fn set_repositories(&mut self, repositories: Vec<Repository>);
    fn set_client(&mut self, client: Client);
    fn paint(&mut self, ui: &mut Ui, token: &str);
}
