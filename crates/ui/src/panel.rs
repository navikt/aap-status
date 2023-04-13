use egui::Ui;
use model::repository::Repository;
use crate::panel_deployment::DeploymentPanel;
use crate::panel_pull_request::PullRequestsPanel;
use crate::panel_repository::RepositoriesPanel;
use crate::panel_workflows::WorkflowPanel;

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub enum SelectedPanel {
    PullRequests,
    Deployments,
    WorkflowRuns,
    #[default]
    Repositories,
}

pub trait Panel {
    fn set_repositories(&mut self, repositories: Vec<Repository>);
    fn paint(&mut self, ui: &mut Ui, token: &str);
}

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct Panels {
    pub selected: SelectedPanel,
    pr_panel: PullRequestsPanel,
    repo_panel: RepositoriesPanel,
    deployment_panel: DeploymentPanel,
    workflow_panel: WorkflowPanel,
}

impl Panels {
    pub fn paint_repositories(&mut self, ui: &mut Ui, token: &str) {
        self.repo_panel.paint(ui, token);
    }

    pub fn paint_pull_requests(&mut self, ui: &mut Ui, token: &str) {
        self.pr_panel.set_repositories(self.repo_panel.repositories());
        self.pr_panel.paint(ui, token);
    }

    pub fn paint_deployments(&mut self, ui: &mut Ui, token: &str) {
        self.deployment_panel.set_repositories(self.repo_panel.repositories());
        self.deployment_panel.paint(ui, token);
    }

    pub fn paint_workflows(&mut self, ui: &mut Ui, token: &str) {
        self.workflow_panel.set_repositories(self.repo_panel.repositories());
        self.workflow_panel.paint(ui, token);
    }
}
