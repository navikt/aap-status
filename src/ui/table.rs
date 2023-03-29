use std::collections::{BTreeMap, HashSet};

use eframe::epaint::FontId;
use eframe::epaint::text::LayoutJob;
use egui::{Color32, Ui};
use egui::util::hash;
use egui_extras::{Column, TableBuilder};

use crate::github::pulls::PullRequest;
use crate::github::workflows::WorkflowRun;

#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Table {
    striped: bool,
}

impl Table {
    pub fn pull_requests_ui(&mut self, ui: &mut Ui, pull_requests: &BTreeMap<String, HashSet<PullRequest>>) {
        ui.push_id(hash("pull_request"), |ui| {
            let table = TableBuilder::new(ui)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .min_scrolled_height(0.0);

            table.header(20.0, |mut header| {
                header.col(|ui| { ui.strong("Repo"); });
                header.col(|ui| { ui.strong("Title"); });
                header.col(|ui| { ui.strong("Author"); });
                header.col(|ui| { ui.strong("Last Update"); });
            })
                .body(|mut body| {
                    for (name, prs) in pull_requests.iter() {
                        prs.iter().for_each(|pr| {
                            body.row(18.0, |mut row| {
                                row.col(|ui| { ui.label(name); });
                                row.col(|ui| { ui.hyperlink_to(&pr.title.clone().unwrap_or_default(), &pr.html_url.clone().unwrap_or_default()); });
                                row.col(|ui| { ui.label(&pr.updated_at.clone().unwrap_or_default()); });
                                row.col(|ui| { ui.label(&pr.user.clone().unwrap_or_default().login); });
                            });
                        });
                    }
                });
        });
    }

    pub fn workflow_runs_ui(
        &mut self,
        ui: &mut Ui,
        show_prs: bool,
        show_success: bool,
        map_of_runs: &BTreeMap<String, HashSet<WorkflowRun>>,
    ) {
        let table = TableBuilder::new(ui)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto())
            .min_scrolled_height(0.0);

        table.header(20.0, |mut header| {
            header.col(|ui| { ui.strong("Repo"); });
            header.col(|ui| { ui.strong("Conclusion"); });
            header.col(|ui| { ui.strong("Workflow"); });
            header.col(|ui| { ui.strong("Event"); });
            header.col(|ui| { ui.strong("Attempts"); });
            header.col(|ui| { ui.strong("Timestamp"); });
        }).body(|mut body| {
            for (repo_name, runs) in map_of_runs.iter() {
                let newest_workflow_runs = runs.iter().fold(BTreeMap::new(), |mut acc: BTreeMap<i64, WorkflowRun>, next| {
                    let existing_or_new = acc.entry(next.workflow_id).or_default();
                    if next.id > existing_or_new.id { acc.insert(next.workflow_id, next.clone()); }
                    acc
                }).into_values();

                newest_workflow_runs
                    .filter(|workflow_run| workflow_run.event.clone() != "pull_request" || show_prs)
                    .filter(|workflow_run| workflow_run.conclusion.clone().unwrap_or_default() == "failure" || show_success)
                    .for_each(|workflow_run| {
                        body.row(18.0, |mut row| {
                            row.col(|ui| { ui.label(repo_name); });
                            row.col(|ui| {
                                let color = if let Some(conclusion) = &workflow_run.conclusion {
                                    if conclusion == "failure" {
                                        Color32::LIGHT_RED
                                    } else {
                                        Color32::LIGHT_GREEN
                                    }
                                } else {
                                    Color32::LIGHT_GREEN
                                };

                                let conclusion = LayoutJob::simple_singleline(
                                    workflow_run.conclusion.clone().unwrap_or_default(),
                                    FontId::default(),
                                    color,
                                );

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
    }
}
