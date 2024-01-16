use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use egui::{Color32, SelectableLabel, Ui};
use egui_extras::TableBuilder;
use serde::{Deserialize, Serialize};
use http::github;
use model::repository::Repository;
use model::workflow::{Workflow, WorkflowRun, WorkflowRuns};

use crate::{FixedField, Scroll, Scrollbar, Table};
use crate::panel::Panel;

#[derive(Deserialize, Serialize, Default)]
pub struct WorkflowPanel {
    repositories: Vec<Repository>,
    workflows: Arc<Mutex<BTreeMap<String, Vec<Workflow>>>>,
    workflow_runs: Arc<Mutex<BTreeMap<String, Vec<WorkflowRun>>>>,
    show_pull_requests: bool,
    show_successfuls: bool,
    client: github::Client,
}

impl Panel for WorkflowPanel {
    fn set_repositories(&mut self, repositories: Vec<Repository>) { self.repositories = repositories }
    fn set_client(&mut self, client: github::Client) { self.client = client }

    fn paint(&mut self, ui: &mut Ui, token: &str) {
        ui.heading("Failed Workflows");

        ui.horizontal_wrapped(|ui| {
            if ui.button("Refresh").clicked() {
                self.refresh(token);
            }

            if ui.add(SelectableLabel::new(self.show_pull_requests, "Show pull-requests")).clicked() {
                self.show_pull_requests = !self.show_pull_requests;
            };

            if ui.add(SelectableLabel::new(self.show_successfuls, "Show successful")).clicked() {
                self.show_successfuls = !self.show_successfuls;
            };
        });

        FixedField::minimum_width(100.0, ui, |ui| {
            Scrollbar::horizontal(ui, |ui| {
                let workflows = self.workflow_runs.lock().unwrap().clone();

                TableBuilder::create(ui, vec!["Repo", "Conclusion", "Workflow", "Event", "Attempts", "Timestamp"]).body(|mut body| {
                    for (repo_name, runs) in workflows.iter() {
                        let newest_workflow_runs = runs.iter()
                            .fold(BTreeMap::new(), |mut acc: BTreeMap<i64, WorkflowRun>, next| {
                                let existing_or_new = acc.entry(next.workflow_id).or_default();
                                if next.id > existing_or_new.id {
                                    acc.insert(next.workflow_id, next.clone());
                                }
                                acc
                            })
                            .into_values();

                        newest_workflow_runs
                            .filter(|workflow_run| workflow_run.event.clone() != "pull_request" || self.show_pull_requests)
                            .filter(|workflow_run| workflow_run.conclusion.clone().unwrap_or_default() != "success" || self.show_successfuls)
                            .for_each(|workflow_run| {
                                body.row(18.0, |mut row| {
                                    row.col(|ui| { ui.label(repo_name); });
                                    row.col(|ui| {
                                        let color = match &workflow_run.conclusion {
                                            Some(conclusion) if conclusion == "success" => Color32::LIGHT_GREEN,
                                            Some(conclusion) if conclusion == "failure" => Color32::LIGHT_RED,
                                            _ => Color32::LIGHT_GRAY
                                        };

                                        ui.colored_label(color, workflow_run.conclusion.unwrap_or_default());
                                    });
                                    row.col(|ui| { ui.hyperlink_to(&workflow_run.name.clone().unwrap_or_default(), &workflow_run.html_url.clone()); });
                                    row.col(|ui| { ui.label(&workflow_run.event.clone()); });
                                    row.col(|ui| { ui.label(format!("{}", &workflow_run.run_attempt.clone())); });
                                    row.col(|ui| { ui.label(&workflow_run.run_started_at.clone().unwrap_or_default()); });
                                });
                            });
                    }
                });
            });
        });
    }
}

impl WorkflowPanel {
    fn refresh(&mut self, token: &str) {
        self.workflow_runs.lock().unwrap().clear();
        self.repositories.clone().into_iter().for_each(|_repo| {
            let _workflow_runs = self.workflow_runs.clone();
            let url = format!("/repos/navikt/{}/actions/runs?per_page=15", _repo.name);
            let mut client = self.client.clone();
            self.client.get_path(token, &url, move |response| {
                if let Ok(response) = response {

                    if let Some(remaining) = response.headers.get("x-ratelimit-remaining") {
                        let remaining = remaining.parse::<usize>().unwrap();
                        client.set_rate_limit(remaining);
                    }

                    let workflow_runs = serde_json::from_slice::<WorkflowRuns>(&response.bytes).unwrap_or_default();

                    *_workflow_runs.lock().unwrap()
                        .entry(_repo.clone().name)
                        .or_default() = workflow_runs.workflow_runs;
                }
            });
        });
    }
}
