use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::vec::IntoIter;

use egui::{ScrollArea, Ui};
use egui::util::hash;
use egui_extras::{Size, StripBuilder, TableBuilder};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::github::github_client::{Fetch, GitHubApi};
use crate::github::github_models;
use crate::ui::table::StatusTable;

#[derive(Default, Clone)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct PullRequestsPanel {
    pull_requests: Arc<Mutex<Vec<PullRequest>>>,
}

impl PullRequestsPanel {
    pub fn clear_pull_requests(&self) {
        self.pull_requests.lock().unwrap().clear()
    }

    fn pull_requests(&self) -> IntoIter<PullRequest> {
        self.pull_requests.lock().unwrap().clone().into_iter()
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

impl PullRequestsPanel {
    pub fn paint(&self, ui: &mut Ui) {
        StripBuilder::new(ui).size(Size::remainder().at_least(100.0)).vertical(|mut strip| strip.cell(|ui| {
            ScrollArea::horizontal().show(ui, |ui| {
                ui.push_id(hash("pull_request"), |ui| {
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
        }));
    }

    pub fn fetch(&self, repo: String, github: GitHubApi) {
        let _pulls = self.pull_requests.clone();
        github.fetch_path(&format!("/repos/navikt/{}/pulls", repo), move |response| {
            match serde_json::from_slice::<HashSet<github_models::PullRequest>>(&response) {
                Err(error) => eprintln!("error: {}", error),
                Ok(response) => {
                    let pull_request = response.into_iter()
                        .map(|pr| PullRequest::parse(repo.clone(), pr))
                        .collect_vec();

                    _pulls.lock().unwrap().extend(pull_request);
                }
            }
        });
    }
}
