use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use egui::{CentralPanel, SelectableLabel, SidePanel, TextEdit, TopBottomPanel};

use crate::github::github_client::*;
use crate::github::github_models::*;
use crate::ui::deployments::DeploymentPanel;
use crate::ui::panels::{Panel, Panels};
use crate::ui::table::Tables;
use crate::ui::workflows::WorkflowPanel;

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
            // github,
            token,
            tables,
            panels,
            team_name,
            teams: _,
            deployments: _,
            deployment_statuses: _,
            workflows: _,
            workflow_runs: _,
            repository_envs: _,
        } = self;

        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label("PAT").on_hover_text("Get your Personal Access Token from Github and select repo and workflow persmissions");
                if (ui.add(TextEdit::singleline(token).password(!panels.github.token_visible))).lost_focus() {
                    println!("token changed:{}", &token);
                    panels.update_token(token.clone());
                };
                if ui.add(SelectableLabel::new(panels.github.token_visible, "👁")).on_hover_text("Show/hide token").clicked() {
                    panels.github.toggle_token_visibility();
                };
            });
        });

        SidePanel::left("side_panel").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("GitHub Status");
                ui.group(|ui| {
                    ui.separator();
                    if ui.button("  Pull Requests  ").clicked() {
                        panels.selected = Panel::PullRequests
                    }
                    ui.separator();
                    if ui.button("  Deployments  ").clicked() {
                        panels.selected = Panel::Deployments;
                    }
                    ui.separator();
                    if ui.button("  Workflows  ").clicked() {
                        panels.selected = Panel::WorkflowRuns;
                    }
                    ui.separator();
                    if ui.button("  Repositories  ").clicked() {
                        panels.selected = Panel::Repositories;
                    }
                    ui.separator();
                });
            });
        });

        CentralPanel::default().show(ctx, |ui| {
            match panels.selected {
                Panel::PullRequests => {
                    ui.heading("Pull Requests");
                    if ui.button("Refresh").clicked() {
                        panels.clear_pull_requests();

                        panels.repositories()
                            .into_iter()
                            .for_each(|repo| panels.add_pull_requests_for_repo(repo.name))
                    }

                    panels.paint_pull_requests(ui);
                }

                Panel::Deployments => {
                    ui.heading("Deployments");

                    ui.horizontal_wrapped(|ui| {
                        if ui.button("Refresh").clicked() {
                            self.deployments.lock().unwrap().clear();
                            panels.repositories().into_iter().for_each(|_repo| {
                                let _deployments = self.deployments.clone();
                                panels.github.fetch_url(&_repo.deployments_url.clone(), move |response| {
                                    if let Ok(deployments) = serde_json::from_slice::<Vec<Deployment>>(&response) {
                                        *_deployments.lock().unwrap()
                                            .entry(_repo.clone().name)
                                            .or_insert(Vec::default()) = deployments
                                    }
                                });
                            });
                        }

                        if ui.button("Refresh status").clicked() {
                            self.deployment_statuses.lock().unwrap().clear();
                            self.deployments.clone().lock().unwrap().clone().into_iter().for_each(|(_, deploys)| {
                                deploys.into_iter().for_each(|deployment| {
                                    let _deployment_statuses = self.deployment_statuses.clone();
                                    panels.github.fetch_url(&deployment.statuses_url, move |response| {
                                        if let Ok(statuses) = serde_json::from_slice::<Vec<Status>>(&response) {
                                            *_deployment_statuses.lock().unwrap()
                                                .entry(deployment.id)
                                                .or_insert(Vec::default()) = statuses
                                        }
                                    });
                                })
                            });
                        }

                        if ui.button("Refresh environments").clicked() {
                            self.repository_envs.lock().unwrap().clear();
                            panels.repositories().into_iter().for_each(|repository| {
                                if self.repository_envs.lock().unwrap().clone().get(&repository.name).is_none() {
                                    let repository_envs = self.repository_envs.clone();
                                    panels.github.fetch_path(&format!("/repos/navikt/{}/environments", repository.name), move |response| {
                                        if let Ok(environments) = serde_json::from_slice::<Environments>(&response) {
                                            *repository_envs.lock().unwrap()
                                                .entry(repository.clone().name)
                                                .or_insert(Vec::default()) = environments.environments;
                                        }
                                    });
                                }
                            });
                        }
                    });

                    panels.others.draw_deployments(
                        ui,
                        &panels.repositories(),
                        &self.deployments.clone().lock().unwrap(),
                        &self.deployment_statuses.clone().lock().unwrap(),
                        &self.repository_envs.clone().lock().unwrap(),
                    );
                }

                Panel::WorkflowRuns => {
                    ui.heading("Failed Workflows");
                    ui.horizontal_wrapped(|ui| {
                        if ui.button("Refresh").clicked() {
                            self.workflow_runs.lock().unwrap().clear();
                            panels.repositories().into_iter().for_each(|_repo| {
                                let _workflow_runs = self.workflow_runs.clone();
                                panels.github.fetch_path(&format!("/repos/navikt/{}/actions/runs?per_page=15", _repo.name), move |response| {
                                    if let Ok(workflow_runs) = serde_json::from_slice::<WorkflowRuns>(&response) {
                                        *_workflow_runs.lock().unwrap()
                                            .entry(_repo.clone().name)
                                            .or_insert(Vec::default()) = workflow_runs.workflow_runs;
                                    }
                                });
                            });
                        }

                        if ui.add(SelectableLabel::new(tables.show_pull_requests_runs(), "Show pull-requests")).clicked() {
                            *tables = tables.toggle_show_workflow_pulls();
                        };

                        if ui.add(SelectableLabel::new(tables.show_successful_runs(), "Show successful")).clicked() {
                            *tables = tables.toggle_show_workflow_success();
                        };
                    });

                    panels.others.draw_workflows(
                        ui,
                        &self.workflow_runs.lock().unwrap().clone(),
                    );
                }
                Panel::Repositories => {
                    ui.heading("Repositories");
                    ui.horizontal_wrapped(|ui| {
                        ui.label("Team");
                        if ui.text_edit_singleline(team_name).lost_focus() {
                            *team_name = team_name.to_string();
                            panels.select_team(team_name.clone());
                        }
                        if ui.button("Fetch").clicked() {
                            panels.find_repositories();
                        }
                    });
                    ui.separator();
                    panels.paint_repositories(ui);
                }
            };
        });
    }

    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)]
pub struct Application {
    token: String,
    tables: Tables,
    panels: Panels,
    team_name: String,
    teams: Arc<Mutex<Vec<Team>>>,
    deployments: Arc<Mutex<BTreeMap<String, Vec<Deployment>>>>,
    deployment_statuses: Arc<Mutex<BTreeMap<i64, Vec<Status>>>>,
    workflows: Arc<Mutex<BTreeMap<String, Vec<Workflow>>>>,
    workflow_runs: Arc<Mutex<BTreeMap<String, Vec<WorkflowRun>>>>,
    repository_envs: Arc<Mutex<BTreeMap<String, Vec<Environment>>>>,
}
