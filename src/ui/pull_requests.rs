use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::vec::IntoIter;

use egui::Ui;
use egui_extras::TableBuilder;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::github;
use crate::github::{github_models, HOST};
use crate::github::github_models::Repo;
use crate::ui::{FixedField, Scroll, Scrollbar, Table};
use crate::ui::panels::Panel;

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct PullRequestsPanel {
    repositories: Vec<Repo>,
    pull_requests: Arc<Mutex<Vec<PullRequest>>>,
}

impl Panel for PullRequestsPanel {
    fn set_repositories(&mut self, repositories: Vec<Repo>) {
        self.repositories = repositories
    }

    fn paint(&mut self, ui: &mut Ui, token: &str) {
        ui.heading("Pull Requests");

        if ui.button("Refresh").clicked() {
            self.clear_pull_requests();
            self.repositories().for_each(|repo| {
                let _pulls = self.pull_requests.clone();
                github::fetch_lifetime::<HashSet<github_models::PullRequest>>(token, &format!("{}/repos/navikt/{}/pulls", HOST, &repo.name), move |response| {
                    if let Ok(pull_requests) = response {
                        let pull_request = pull_requests.into_iter()
                            .map(|pr| PullRequest::parse(repo.clone().name, pr))
                            .collect_vec();

                        _pulls.lock().unwrap().extend(pull_request);
                    }
                });
            })
        }

        FixedField::minimum_width(100.0, ui, |ui| {
            Scrollbar::horizontal(ui, |ui| {
                TableBuilder::create(ui, vec!["Repo", "Title", "Author", "Last Update"]).body(|mut body| {
                    self.pull_requests().for_each(|pull| {
                        body.row(18.0, |mut row| {
                            row.col(|ui| { ui.label(&pull.repo_name); });
                            row.col(|ui| { ui.hyperlink_to(&pull.title, &pull.html_url); });
                            row.col(|ui| { ui.label(&pull.updated_at); });
                            row.col(|ui| { ui.label(&pull.user); });
                        });
                    });
                });
            });
        });
    }
}

impl PullRequestsPanel {
    fn clear_pull_requests(&self) {
        self.pull_requests.lock().unwrap().clear()
    }

    fn pull_requests(&self) -> IntoIter<PullRequest> {
        self.pull_requests.lock().unwrap().clone().into_iter()
    }

    fn repositories(&self) -> IntoIter<Repo> {
        self.repositories.clone().into_iter()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PullRequest {
    repo_name: String,
    url: String,
    html_url: String,
    title: String,
    user: String,
    updated_at: String,
}

impl PullRequest {
    pub fn parse(repo: String, value: github_models::PullRequest) -> Self {
        PullRequest {
            repo_name: repo,
            url: value.url,
            html_url: value.html_url.unwrap_or_default(),
            title: value.title.unwrap_or_default(),
            user: value.user.unwrap_or_default().login,
            updated_at: value.updated_at.unwrap_or_default(),
        }
    }
}