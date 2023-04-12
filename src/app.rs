use egui::{CentralPanel, SelectableLabel, SidePanel, TextEdit, TopBottomPanel};

use crate::ui::panels::{Panels, SelectedPanel};

impl Application {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for Application {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
            });
        });

        SidePanel::left("side_panel").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("GitHub Status");
                ui.group(|ui| {
                    if ui.button("  Repositories  ").clicked() {
                        panels.selected = SelectedPanel::Repositories;
                    }
                    ui.separator();


                    ui.separator();
                    if ui.button("  Pull Requests ").clicked() {
                        panels.selected = SelectedPanel::PullRequests
                    }

                    ui.separator();
                    if ui.button("   Deployments  ").clicked() {
                        panels.selected = SelectedPanel::Deployments;
                    }

                    ui.separator();
                    if ui.button("    Workflows   ").clicked() {
                        panels.selected = SelectedPanel::WorkflowRuns;
                    }
                    ui.separator();
                });
            });
        });

        CentralPanel::default().show(ctx, |ui| {
            match panels.selected {
                SelectedPanel::PullRequests => panels.paint_pull_requests(ui, token),
                SelectedPanel::Deployments => panels.paint_deployments(ui, token),
                SelectedPanel::WorkflowRuns => panels.paint_workflows(ui, token),
                SelectedPanel::Repositories => panels.paint_repositories(ui, token)
            };
        });
    }

    /// Called by the frame work to save state before shutdown.
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
