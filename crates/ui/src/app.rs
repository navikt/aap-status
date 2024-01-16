use eframe::Frame;
use egui::{CentralPanel, Context, SelectableLabel, SidePanel, TextEdit, TopBottomPanel};

use crate::panel::{Panels, SelectedPanel};

impl Application {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for Application {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        let Self {
            token,
            token_visible,
            panels,
        } = self;

        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label("PAT").on_hover_text("Get your Personal Access Token from Github and select repo and workflow persmissions");
                ui.add(TextEdit::singleline(token).password(!*token_visible));
                if ui.add(SelectableLabel::new(*token_visible, "👁")).on_hover_text("Show/hide token").clicked() {
                    *token_visible = !*token_visible;
                };
            
                ui.label(format!("{} API requests remaining until {}", panels.rate_limit(), panels.rate_limit_reset()));
            });
        });

        SidePanel::left("side_panel").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("GitHub Status");
                ui.group(|ui| {
                    ui.separator();
                    let client = panels.selected.get_client();

                    if ui.button("  Repositories  ").clicked() {
                        panels.selected = SelectedPanel::Repositories(client.clone())
                    }
                    ui.separator();
                    if ui.button("  Pull Requests ").clicked() {
                        panels.selected = SelectedPanel::PullRequests(client.clone())
                    }
                    ui.separator();
                    if ui.button("   Deployments  ").clicked() {
                        panels.selected = SelectedPanel::Deployments(client.clone())
                    }
                    ui.separator();
                    if ui.button("    Workflows   ").clicked() {
                        panels.selected = SelectedPanel::WorkflowRuns(client.clone())
                    }
                    ui.separator();
                });
            });
        });

        CentralPanel::default().show(ctx, |ui| {
            match panels.selected {
                SelectedPanel::Repositories(_) => panels.paint_repositories(ui, token),
                SelectedPanel::PullRequests(_) => panels.paint_pull_requests(ui, token),
                SelectedPanel::Deployments(_) => panels.paint_deployments(ui, token),
                SelectedPanel::WorkflowRuns(_) => panels.paint_workflows(ui, token),
            }
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct Application {
    token: String,
    token_visible: bool,
    panels: Panels,
}
