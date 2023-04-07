use egui::Ui;
use crate::github::github_client::GitHubApi;
use crate::github::github_models::Repo;
use crate::ui::pull_requests::PullRequestsPanel;
use crate::ui::repositories::RepositoriesPanel;
use crate::ui::table::Tables;

#[derive(Default, Clone)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct PanelUI {
    pub tables: Tables,
}

#[derive(Clone)]
#[derive(serde::Deserialize, serde::Serialize)]
pub enum Panel {
    PullRequests,
    Deployments,
    WorkflowRuns,
    Repositories,
}

#[derive(Clone)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Panels {
    pub selected: Panel,
    pub github: GitHubApi,
    pull_requests: PullRequestsPanel,
    repositories: RepositoriesPanel,
    pub others: PanelUI,
}

impl Panels {
    pub fn update_token(&mut self, token: String) {
        self.github.update_token(token)
    }

    pub fn add_pull_requests_for_repo(&self, repo_name: String) {
        self.pull_requests.fetch(repo_name, self.github.clone())
    }

    pub fn paint_pull_requests(&self, ui: &mut Ui) {
        self.pull_requests.paint(ui)
    }

    pub fn paint_repositories(&self, ui: &mut Ui) {
        self.repositories.paint(ui)
    }

    pub fn clear_pull_requests(&self) {
        self.pull_requests.clear_pull_requests()
    }

    pub fn repositories(&self) -> Vec<Repo> {
        self.repositories.repositories()
    }

    pub fn select_team(&self, team_name: String) {
        self.repositories.fetch_team(team_name, self.github.clone())
    }

    pub fn find_repositories(&self) {
        self.repositories.fetch(self.github.clone())
    }
}

impl Default for Panels {
    fn default() -> Self {
        Panels {
            selected: Panel::Repositories,
            github: GitHubApi::default(),
            pull_requests: PullRequestsPanel::default(),
            repositories: RepositoriesPanel::default(),
            others: PanelUI::default(),
        }
    }
}
