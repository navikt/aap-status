use std::collections::BTreeMap;

use egui::{Color32, TextFormat, Ui};

use crate::github::pulls::PullRequest;
use crate::github::runs::WorkflowRun;
use crate::github::runs::WorkflowRuns;

#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Table {
    striped: bool,
}

impl Table {
    pub fn pull_requests_ui(&mut self, ui: &mut Ui, pulls: &BTreeMap<String, Vec<PullRequest>>) {
        use egui_extras::{Column, TableBuilder};

        let table = TableBuilder::new(ui)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto())
            .column(Column::auto().resizable(true).clip(true))
            .column(Column::auto())
            .column(Column::auto())
            .min_scrolled_height(0.0);

        table.header(20.0, |mut header| {
            header.col(|ui| { ui.strong("ID"); });
            header.col(|ui| { ui.strong("Title"); });
            header.col(|ui| { ui.strong("Last Update"); });
            header.col(|ui| { ui.strong("Author"); });
        })
            .body(|mut body| {
                for (name, prs) in pulls.iter() {
                    if !prs.is_empty() {
                        body.row(40.0, |mut row| {
                            row.col(|ui| { ui.heading(""); });
                            row.col(|ui| { ui.heading(name); });
                            row.col(|ui| { ui.heading(""); });
                            row.col(|ui| { ui.heading(""); });
                        });
                    }

                    prs.iter().for_each(|pr| {
                        let _pr = pr.clone();
                        body.row(18.0, |mut row| {
                            row.col(|ui| { ui.label(format!("{}", &_pr.number)); });
                            row.col(|ui| { ui.hyperlink_to(&_pr.title.unwrap(), &_pr.html_url.unwrap()); });
                            row.col(|ui| { ui.label(&_pr.updated_at.unwrap()); });
                            row.col(|ui| { ui.label(&_pr.user.unwrap().login); });
                        });
                    });
                }
            });
    }

    pub fn workflow_runs_ui(
        &mut self,
        ui: &mut Ui,
        repo_with_runs: &BTreeMap<String, WorkflowRuns>,
    ) {
        use egui_extras::{Column, TableBuilder};

        let table = TableBuilder::new(ui)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto())
            .min_scrolled_height(0.0);

        table.header(20.0, |mut header| {
            header.col(|ui| { ui.strong("Repo"); });
            header.col(|ui| { ui.strong("Workflow"); });
            header.col(|ui| { ui.strong("Event"); });
            header.col(|ui| { ui.strong("Status"); });
            header.col(|ui| { ui.strong("Conclusion"); });
            header.col(|ui| { ui.strong("Attempts"); });
            header.col(|ui| { ui.strong("Timestamp"); });
        }).body(|mut body| {
            for (repo_name, runs) in repo_with_runs.iter() {
                // Group workflow runs by id
                let group_by_workflow_id = runs.workflow_runs.clone().into_iter().fold(BTreeMap::new(), |mut acc: BTreeMap<i64, Vec<WorkflowRun>>, wr| {
                    acc.entry(wr.workflow_id).or_default().push(wr);
                    acc
                });

                group_by_workflow_id.into_iter().for_each(|(_, workflow_runs)| {
                    workflow_runs.into_iter().take(1).for_each(|workflow_run| {
                        let conclusion = &workflow_run.conclusion.unwrap_or(String::new());

                        if conclusion == &String::from("failure") {
                            body.row(18.0, |mut row| {
                                row.col(|ui| { ui.label(repo_name); });
                                row.col(|ui| { ui.label(&workflow_run.name.unwrap_or(String::new())); });
                                row.col(|ui| { ui.label(&workflow_run.event); });
                                row.col(|ui| { ui.label(&workflow_run.status.unwrap_or(String::new())); });

                                row.col(|ui| {
                                    use egui::text::LayoutJob;
                                    let green = TextFormat { color: Color32::from_rgb(100, 255, 146), ..Default::default() };
                                    let red = TextFormat { color: Color32::from_rgb(255, 100, 100), ..Default::default() };
                                    let mut job = LayoutJob::default();
                                    let color = if conclusion == "success" { green } else { red };
                                    job.append(conclusion, 0.0, color);
                                    ui.label(job);
                                });

                                row.col(|ui| { ui.label(format!("{}", &workflow_run.run_attempt)); });
                                row.col(|ui| { ui.label(&workflow_run.run_started_at.unwrap_or(String::new())); });
                            });
                        }
                    });
                });
                // }
            };
        })
    }
}
