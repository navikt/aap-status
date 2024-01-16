use std::sync::{Arc, Mutex};
use std::vec::IntoIter;

use egui::Ui;
use egui_extras::TableBuilder;
use serde::{Deserialize, Serialize};

use http::github;
use model::pull_request::PullRequest;
use model::repository::Repository;

use crate::{FixedField, Scroll, Scrollbar, Table};
use crate::panel::Panel;

#[derive(Deserialize, Serialize, Default)]
pub struct PullRequestsPanel {
    repositories: Vec<Repository>,
    pull_requests: Arc<Mutex<Vec<PullRequest>>>,
    client: github::Client,
}

impl Panel for PullRequestsPanel {
    fn set_repositories(&mut self, repositories: Vec<Repository>) { self.repositories = repositories }
    fn set_client(&mut self, client: github::Client) { self.client = client }

    fn paint(&mut self, ui: &mut Ui, token: &str) {
        ui.heading("Pull Requests");

        if ui.button("Refresh").clicked() {
            self.clear_pull_requests();
            self.repositories().for_each(|repo| {
                let _pulls = self.pull_requests.clone();
                let mut client = self.client.clone();
                self.client.get(token, repo.pulls_url(), move |response| {
                    if let Ok(response) = response {
                        
                        if let Some(remaining) = response.headers.get("x-ratelimit-remaining") {
                            let remaining = remaining.parse::<usize>().unwrap();
                            client.set_rate_limit(remaining);
                        }

                        let pull_requests = serde_json::from_slice::<Vec<PullRequest>>(&response.bytes).unwrap_or_default(); 
                        _pulls.lock().unwrap().extend(pull_requests);
                    }
                });
            })
        }

        FixedField::minimum_width(100.0, ui, |ui| {
            Scrollbar::horizontal(ui, |ui| {
                TableBuilder::create(ui, vec!["Repo", "Title", "Author", "Last Update"]).body(|mut body| {
                    self.pull_requests().for_each(|pull| {
                        body.row(18.0, |mut row| {
                            row.col(|ui| { ui.label(&pull.repo()); });
                            row.col(|ui| { ui.hyperlink_to(&pull.title(), &pull.html()); });
                            row.col(|ui| { ui.label(&pull.updated_at()); });
                            row.col(|ui| { ui.label(&pull.user()); });
                        });
                    });
                });
            });
        });
    }
}

// fn path(repo: &str) -> &str { &format!("/repos/navikt/{}/pulls", repo) }

impl PullRequestsPanel {
    fn clear_pull_requests(&self) {
        self.pull_requests.lock().unwrap().clear()
    }

    fn pull_requests(&self) -> IntoIter<PullRequest> {
        self.pull_requests.lock().unwrap().clone().into_iter()
    }

    fn repositories(&self) -> IntoIter<Repository> {
        self.repositories.clone().into_iter()
    }
}
