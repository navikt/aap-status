use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use std::vec::IntoIter;

use egui::{ScrollArea, Ui};
use egui::util::hash;
use egui_extras::{Size, StripBuilder};
use itertools::Itertools;

use crate::github;
use crate::github::github_models::{Deployment, Environment, Environments, Repo, Status};

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct DeploymentPanel {
    repositories: Vec<Repo>,
    deployments: Arc<Mutex<BTreeMap<String, Vec<Deployment>>>>,
    statuses: Arc<Mutex<BTreeMap<i64, Status>>>,
    environments: Arc<Mutex<Vec<Environment>>>,
}

impl DeploymentPanel {
    pub fn set_repos(&mut self, repos: Vec<Repo>) {
        self.repositories = repos
    }

    fn repositories(&self) -> IntoIter<Repo> {
        self.repositories.clone().into_iter()
    }

    fn deployment(&self, repo: &str, env: &str) -> Option<Deployment> {
        let deployments = self.deployments.lock().unwrap().clone();
        if let Some(repo_deployments) = deployments.get(repo) {
            repo_deployments.clone().into_iter().find(|deployment| deployment.environment == env)
        } else {
            None
        }
    }

    fn environments(&self) -> IntoIter<Environment> {
        self.environments.lock().unwrap().clone().into_iter()
    }

    #[allow(dead_code)]
    fn clear_deployments(&self) {
        self.deployments.lock().unwrap().clear()
    }

    #[allow(dead_code)]
    fn clear_deployment_statuses(&self) {
        self.statuses.lock().unwrap().clear()
    }
}

impl DeploymentPanel {
    pub fn paint(&self, ui: &mut Ui, token: &str) {
        ui.heading("Deployments");

        ui.horizontal_wrapped(|ui| {
            if ui.button("Refresh").clicked() {
                self.environments.lock().unwrap().clear();
                self.deployments.lock().unwrap().clear();

                self.repositories().for_each(|repository| {
                    let environments = self.environments.clone();
                    refresh_environment(token, &repository, move |response| {
                        let new_envs = response.into_iter()
                            .filter(|env| { !environments.lock().unwrap().iter().any(|cur| cur.name == env.name) })
                            .collect_vec();

                        environments.lock().unwrap().extend(new_envs);
                    });

                    let repository_name = repository.clone().name;
                    let deployments = self.deployments.clone();
                    refresh_deployment(token, &repository, move |response| {
                        let last_deployments_per_environment = response.into_iter()
                            .group_by(|deps| deps.environment.clone())
                            .into_iter()
                            .map(|(_, group)| group.max_by(|x, y| x.id.cmp(&y.id)).unwrap()).collect_vec();

                        *deployments.lock().unwrap().entry(repository_name).or_insert(Vec::default()) = last_deployments_per_environment;
                    });
                });
            }

            if ui.button("Refresh statuses").clicked() {
                self.statuses.lock().unwrap().clear();
                let repo_to_deployments = self.deployments.lock().unwrap().clone();
                repo_to_deployments.into_values().for_each(|deployments| {
                    println!("Deployments found: {}", deployments.len());
                    deployments.into_iter().for_each(|deployment| {
                        let statuses = self.statuses.clone();
                        refresh_status(token, &deployment, move |response| {
                            println!("found {} statuses", response.len());
                            let last_status = response.into_iter()
                                .sorted_by(|cur, next| Ord::cmp(&next.id, &cur.id))
                                .next();

                            if let Some(status) = last_status {
                                println!("keeps status {}", &status.id);
                                statuses.lock().unwrap().insert(deployment.id, status);
                            }
                        });
                    });
                });
            }
        });

        ui.horizontal_top(|ui| {
            StripBuilder::new(ui).size(Size::remainder()).vertical(|mut strip| {
                strip.cell(|ui| {
                    ScrollArea::vertical().id_source(hash("deploy-vertical")).show(ui, |ui| {
                        StripBuilder::new(ui).size(Size::remainder()).vertical(|mut strip| {
                            strip.cell(|ui| {
                                ScrollArea::horizontal().id_source(hash("deploy-horizontal")).show(ui, |ui| {
                                    self.environments().for_each(|env| {
                                        StripBuilder::new(ui).size(Size::exact(450.0)).horizontal(|mut strip| {
                                            strip.cell(|ui| {
                                                ui.group(|ui| {
                                                    ui.vertical(|ui| {
                                                        ui.heading(&env.name);
                                                        self.repositories().for_each(|repository| {
                                                            if let Some(deployment) = self.deployment(&repository.name, &env.name) {
                                                                if let Some(status) = self.statuses.lock().unwrap().get(&deployment.id) {
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

fn refresh_deployment(
    token: &str,
    repo: &Repo,
    on_refreshed: impl FnOnce(Vec<Deployment>) + Send + 'static,
) {
    github::fetch_lifetime::<Vec<Deployment>>(token, &repo.deployments_url.clone(), |response| {
        if let Ok(deployments) = response {
            on_refreshed(deployments)
        }
    })
}

fn refresh_status(
    token: &str,
    deployment: &Deployment,
    on_refreshed: impl FnOnce(Vec<Status>) + Send + 'static,
) {
    github::fetch_lifetime::<Vec<Status>>(token, &deployment.statuses_url, |response| {
        if let Ok(statuses) = response {
            on_refreshed(statuses)
        }
    })
}

fn refresh_environment(
    token: &str,
    repository: &Repo,
    on_refreshed: impl FnOnce(Vec<Environment>) + Send + 'static,
) {
    github::fetch_lifetime::<Environments>(token, &format!("{}/repos/navikt/{}/environments", github::HOST, repository.name), |response| {
        if let Ok(environments) = response {
            on_refreshed(environments.environments);
        }
    });
}
