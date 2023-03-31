use std::collections::{BTreeMap, HashSet};

use egui::{ScrollArea, Ui};
use egui::util::hash;
use egui_extras::{Column, Size, StripBuilder, TableBuilder};

use crate::app::PanelUI;
use crate::github::github_models::PullRequest;
use crate::ui::table::{Table, TableUI};

pub trait PullRequestPanel {
    fn draw_pull_requests(&mut self, ui: &mut Ui, data: &BTreeMap<String, HashSet<PullRequest>>);
}

impl PullRequestPanel for PanelUI {
    fn draw_pull_requests(&mut self, ui: &mut Ui, data: &BTreeMap<String, HashSet<PullRequest>>) {
        StripBuilder::new(ui)
            .size(Size::remainder().at_least(100.0))
            .vertical(|mut strip| strip.cell(|ui| {
                ScrollArea::horizontal()
                    .show(ui, |ui| {
                        self.tables.pull_requests().render(ui, data)
                    });
            }));
    }
}

impl Table<BTreeMap<String, HashSet<PullRequest>>> for TableUI {
    fn render(&mut self, ui: &mut Ui, data: &BTreeMap<String, HashSet<PullRequest>>) {
        ui.push_id(hash("pull_request"), |ui| {
            let table = TableBuilder::new(ui)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .min_scrolled_height(0.0);

            table
                .header(20.0, |mut header| {
                    header.col(|ui| { ui.strong("Repo"); });
                    header.col(|ui| { ui.strong("Title"); });
                    header.col(|ui| { ui.strong("Author"); });
                    header.col(|ui| { ui.strong("Last Update"); });
                })
                .body(|mut body| {
                    for (name, prs) in data.iter() {
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
}
