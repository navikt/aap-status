use std::collections::BTreeMap;

use egui::Ui;

use crate::PullRequest;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Table {
    striped: bool,
    resizable: bool,
    num_rows: usize,
    scroll_to_row_slider: usize,
    scroll_to_row: Option<usize>,
}

impl Default for Table {
    fn default() -> Self {
        Self {
            striped: true,
            resizable: true,
            num_rows: 10_000,
            scroll_to_row_slider: 0,
            scroll_to_row: None,
        }
    }
}

impl Table {
    pub fn table_ui(&mut self, ui: &mut Ui, pulls: &BTreeMap<String, Vec<PullRequest>>) {
        use egui_extras::{Column, TableBuilder};

        let mut table = TableBuilder::new(ui)
            .striped(self.striped)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto())
            .column(
                Column::initial(100.0)
                    .at_least(40.0)
                    .resizable(true)
                    .clip(true),
            )
            .min_scrolled_height(0.0);

        if let Some(row_nr) = self.scroll_to_row.take() {
            table = table.scroll_to_row(row_nr, None);
        }

        table.header(20.0, |mut header| {
            header.col(|ui| { ui.strong("ID"); });
            header.col(|ui| { ui.strong("Title"); });
            header.col(|ui| { ui.strong("Last Update"); });
            header.col(|ui| { ui.strong("Author"); });
        })
            .body(|mut body| {
                for (name, prs) in pulls.into_iter() {

                    if !prs.is_empty() {
                        body.row(40.0, |mut row| {
                            row.col(|ui| { ui.heading(""); });
                            row.col(|ui| { ui.heading(name); });
                            row.col(|ui| { ui.heading(""); });
                            row.col(|ui| { ui.heading(""); });
                        });
                    }

                    prs.into_iter().for_each(|pr| {
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
}