use egui::Ui;
use serde::{Deserialize, Serialize};

use model::repository::Repository;

use crate::panel_deployment::DeploymentPanel;
use crate::panel_pull_request::PullRequestsPanel;
use crate::panel_repository::RepositoriesPanel;
use crate::panel_workflows::WorkflowPanel;

#[derive(Deserialize, Serialize, Default)]
pub enum SelectedPanel {
    PullRequests,
    Deployments,
    WorkflowRuns,
    #[default]
    Repositories,
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
    fn paint(&mut self, ui: &mut Ui, token: &str);
}