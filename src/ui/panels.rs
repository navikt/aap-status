use egui::Ui;

use crate::ui::deployments::DeploymentPanel;
use crate::ui::pull_requests::PullRequestsPanel;
use crate::ui::repositories::RepositoriesPanel;
use crate::ui::workflows::WorkflowPanel;

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub enum Panel {
    PullRequests,
    Deployments,
    WorkflowRuns,
    #[default]
    Repositories,
}

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct Panels {
    pub selected: Panel,
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
        self.pr_panel.set_repos(self.repo_panel.repos());
        self.pr_panel.paint(ui, token);
    }

    pub fn paint_deployments(&mut self, ui: &mut Ui, token: &str) {
        self.deployment_panel.set_repos(self.repo_panel.repos());
        self.deployment_panel.paint(ui, token);
    }

    pub fn paint_workflows(&mut self, ui: &mut Ui, token: &str) {
        self.workflow_panel.set_repos(self.repo_panel.repos());
        self.workflow_panel.paint(ui, token);
    }
}
