use std::collections::{BTreeMap, HashSet};

use eframe::epaint::{Color32, FontId};
use eframe::epaint::text::LayoutJob;
use egui::{ScrollArea, Ui};
use egui_extras::{Column, Size, StripBuilder, TableBuilder};

use crate::app::PanelUI;
use crate::github::github_models::WorkflowRun;
use crate::ui::table::{Table, TableUI};

pub trait WorkflowPanel {
    fn draw_workflows(&mut self, ui: &mut Ui, table: &mut TableUI, pull_requests: &BTreeMap<String, HashSet<WorkflowRun>>);
}

impl WorkflowPanel for PanelUI {
    fn draw_workflows(&mut self, ui: &mut Ui, table: &mut TableUI, workflows: &BTreeMap<String, HashSet<WorkflowRun>>) {
        StripBuilder::new(ui).size(Size::remainder().at_least(100.0)).vertical(|mut strip| strip.cell(|ui| {
            ScrollArea::horizontal().show(ui, |ui|
                table.render(ui, workflows),
            );
        }));
    }
}

impl Table<BTreeMap<String, HashSet<WorkflowRun>>> for TableUI {
    fn render(&mut self, ui: &mut Ui, data: &BTreeMap<String, HashSet<WorkflowRun>>) {
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
                for (repo_name, runs) in data.iter() {
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
                        .filter(|workflow_run| workflow_run.event.clone() != "pull_request" || self.is_show_pr())
                        .filter(|workflow_run| workflow_run.conclusion.clone().unwrap_or_default() == "failure" || self.is_show_success())
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
    }
}
