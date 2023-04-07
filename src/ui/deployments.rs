use std::collections::BTreeMap;

use egui::{ScrollArea, Ui};
use egui::util::hash;
use egui_extras::{Size, StripBuilder};
use itertools::Itertools;

use crate::github::github_models::{Deployment, Environment, Repo, Status};
use crate::ui::panels::PanelUI;

pub trait DeploymentPanel {
    fn draw_deployments(
        &mut self,
        ui: &mut Ui,
        repositories: &Vec<Repo>,
        deployments: &BTreeMap<String, Vec<Deployment>>,
        deployments_status: &BTreeMap<i64, Vec<Status>>,
        envs: &BTreeMap<String, Vec<Environment>>,
    );
}

impl DeploymentPanel for PanelUI {
    fn draw_deployments(
        &mut self,
        ui: &mut Ui,
        repositories: &Vec<Repo>,
        deployments: &BTreeMap<String, Vec<Deployment>>,
        deployments_status: &BTreeMap<i64, Vec<Status>>,
        envs: &BTreeMap<String, Vec<Environment>>,
    ) {
        ui.horizontal_top(|ui| {
            StripBuilder::new(ui).size(Size::remainder()).vertical(|mut strip| {
                strip.cell(|ui| {
                    // vertical_scroll("deploy-vertical", ui, |ui| {
                    ScrollArea::vertical().id_source(hash("deploy-vertical")).show(ui, |ui| {

                        StripBuilder::new(ui).size(Size::remainder()).vertical(|mut strip| {
                            strip.cell(|ui| {
                                ScrollArea::horizontal().id_source(hash("deploy-horizontal")).show(ui, |ui| {
                                    envs.values().flatten().unique_by(|env| &env.name).for_each(|env| {
                                        StripBuilder::new(ui).size(Size::exact(450.0)).horizontal(|mut strip| {
                                            strip.cell(|ui| {
                                                ui.group(|ui| {
                                                    ui.vertical(|ui| {
                                                        ui.heading(&env.name);

                                                        repositories.iter().for_each(|repository| {
                                                            if let Some(deployments) = deployments.get(&repository.name) {
                                                                let last_deployment = deployments.iter()
                                                                    .filter(|deployment| deployment.environment == env.name)
                                                                    .sorted_by(|cur, next| Ord::cmp(&next.id, &cur.id))
                                                                    .next();

                                                                if let Some(deployment) = last_deployment {
                                                                    if let Some(statuses) = deployments_status.get(&deployment.id) {
                                                                        let last_status = statuses.iter()
                                                                            .sorted_by(|cur, next| Ord::cmp(&next.id, &cur.id))
                                                                            .next();

                                                                        if let Some(status) = last_status {
                                                                            ui.horizontal_wrapped(|ui| {
                                                                                StripBuilder::new(ui).size(Size::exact(150.0)).horizontal(|mut strip| {
                                                                                    strip.cell(|ui| {
                                                                                        ui.label(&repository.name.clone());
                                                                                    });
                                                                                });
                                                                                StripBuilder::new(ui).size(Size::exact(60.0)).horizontal(|mut strip| {
                                                                                    strip.cell(|ui| {
                                                                                        ui.label(status.colored_state());
                                                                                    });
                                                                                });
                                                                                StripBuilder::new(ui).size(Size::exact(200.0)).horizontal(|mut strip| {
                                                                                    strip.cell(|ui| {
                                                                                        ui.label(status.description.clone());
                                                                                    });
                                                                                });
                                                                            });
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        });
                                                    });
                                                });
                                            });
                                        });
                                    });
                                });
                            });
                        });
                    });
                });
            });
        });
    }
}
