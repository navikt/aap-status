use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use std::vec::IntoIter;

use eframe::epaint::{Color32, FontId};
use eframe::epaint::text::LayoutJob;
use egui::{ScrollArea, SelectableLabel, Ui};
use egui_extras::{Column, Size, StripBuilder, TableBuilder};

use crate::github;
use crate::github::github_models::{Repo, Workflow, WorkflowRun, WorkflowRuns};
use crate::github::HOST;

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct WorkflowPanel {
    repositories: Vec<Repo>,
    workflows: Arc<Mutex<BTreeMap<String, Vec<Workflow>>>>,
    workflow_runs: Arc<Mutex<BTreeMap<String, Vec<WorkflowRun>>>>,
    show_pull_requests: bool,
    show_successfuls: bool,
}

impl WorkflowPanel {
    fn repositories(&self) -> IntoIter<Repo> {
        self.repositories.clone().into_iter()
    }
}

impl WorkflowPanel {
    pub fn set_repos(&mut self, repos: Vec<Repo>) {
        self.repositories = repos
    }

    pub fn paint(&mut self, ui: &mut Ui, token: &str) {
        ui.heading("Failed Workflows");

        ui.horizontal_wrapped(|ui| {
            if ui.button("Refresh").clicked() {
                self.repositories().for_each(|_repo| {
                    let _workflow_runs = self.workflow_runs.clone();
                    let url = &format!("{}/repos/navikt/{}/actions/runs?per_page=15", HOST, _repo.name);
                    github::fetch_lifetime::<WorkflowRuns>(token, url, move |response| {
                        if let Ok(workflow_runs) = response {
                            *_workflow_runs.lock().unwrap()
                                .entry(_repo.clone().name)
                                .or_insert(Vec::default()) = workflow_runs.workflow_runs;
                        }
                    });
                });
            }

            if ui.add(SelectableLabel::new(self.show_pull_requests, "Show pull-requests")).clicked() {
                self.show_pull_requests = !self.show_pull_requests;
            };

            if ui.add(SelectableLabel::new(self.show_successfuls, "Show successful")).clicked() {
                self.show_successfuls = !self.show_successfuls;
            };
        });

        StripBuilder::new(ui).size(Size::remainder().at_least(100.0)).vertical(|mut strip| strip.cell(|ui| {
            ScrollArea::horizontal().show(ui, |ui| {
                let workflows = self.workflow_runs.lock().unwrap().clone();

                let table = TableBuilder::new(ui)
                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                    .column(Column::auto())
                    .column(Column::auto())
                    .column(Column::auto())
                    .column(Column::auto())
                    .column(Column::auto())
                    .column(Column::auto())
                    .min_scrolled_height(0.0);

                table
                    .header(20.0, |mut header| {
                        header.col(|ui| { ui.strong("Repo"); });
                        header.col(|ui| { ui.strong("Conclusion"); });
                        header.col(|ui| { ui.strong("Workflow"); });
                        header.col(|ui| { ui.strong("Event"); });
                        header.col(|ui| { ui.strong("Attempts"); });
                        header.col(|ui| { ui.strong("Timestamp"); });
                    })
                    .body(|mut body| {
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
                                .filter(|workflow_run| workflow_run.conclusion.clone().unwrap_or_default() == "failure" || self.show_successfuls)
                                .for_each(|workflow_run| {
                                    body.row(18.0, |mut row| {
                                        row.col(|ui| { ui.label(repo_name); });
                                        row.col(|ui| {
                                            let color = match &workflow_run.conclusion {
                                                Some(conclusion) if conclusion == "failure" => Color32::LIGHT_RED,
                                                _ => Color32::LIGHT_GREEN
                                            };

                                            let conclusion = LayoutJob::simple_singleline(workflow_run.conclusion.clone().unwrap_or_default(), FontId::default(), color);

                                            ui.label(conclusion);
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
        }));
    }
}
