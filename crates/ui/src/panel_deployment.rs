use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use std::vec::IntoIter;

use egui::{Color32, Ui};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use http::github;
use model::deployment::{Deployment, State, Status};
use model::environment::{Environment, Environments};
use model::repository::Repository;

use crate::panel::Panel;
use crate::{FixedField, Scroll, Scrollbar};

#[derive(Deserialize, Serialize, Default)]
pub struct DeploymentPanel {
    repositories: Vec<Repository>,
    deployments: Arc<Mutex<BTreeMap<String, Vec<Deployment>>>>,
    statuses: Arc<Mutex<BTreeMap<i64, Status>>>,
    environments: Arc<Mutex<Vec<Environment>>>,
    client: github::Client,
}

impl Panel for DeploymentPanel {
    fn set_repositories(&mut self, repositories: Vec<Repository>) { self.repositories = repositories }
    fn set_client(&mut self, client: github::Client) { self.client = client }

    fn paint(&mut self, ui: &mut Ui, token: &str) {
        ui.heading("Deployments");

        ui.horizontal_wrapped(|ui| {
            if ui.button("Refresh environments").clicked() {
                self.refresh_deployments(token);
            }

            if ui.button("Refresh deployments").clicked() {
                self.refresh_statuses(token);
            }
        });

        ui.horizontal_top(|ui| {
            FixedField::remaining_width(ui, |ui| {
                Scrollbar::vertical(ui, |ui| {
                    FixedField::remaining_width(ui, |ui| {
                        Scrollbar::horizontal(ui, |ui| {
                            self.environments().for_each(|env| {
                                FixedField::height(450.0, ui, |ui| {
                                    ui.group(|ui| {
                                        ui.vertical(|ui| {
                                            ui.heading(&env.name);
                                            self.repositories().for_each(|repository| {
                                                if let Some(deployment) =
                                                    self.deployment(&repository.name, &env.name)
                                                {
                                                    if let Some(status) = self
                                                        .statuses
                                                        .lock()
                                                        .unwrap()
                                                        .get(&deployment.id)
                                                    {
                                                        ui.horizontal_wrapped(|ui| {
                                                            FixedField::height(150.0, ui, |ui| {
                                                                ui.label(&repository.name.clone());
                                                            });
                                                            FixedField::height(60.0, ui, |ui| {
                                                                ui.colored_label(
                                                                    status.color(),
                                                                    status.state.to_string(),
                                                                );
                                                            });
                                                            FixedField::height(200.0, ui, |ui| {
                                                                ui.label(status.description());
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
    }

}

trait StateColor {
    fn color(&self) -> Color32;
}

impl StateColor for Status {
    fn color(&self) -> Color32 {
        match self.state {
            State::Error => Color32::LIGHT_RED,
            State::Failure => Color32::LIGHT_RED,
            State::Inactive => Color32::LIGHT_GRAY,
            State::Pending => Color32::LIGHT_BLUE,
            State::Success => Color32::LIGHT_GREEN,
            State::Queued => Color32::LIGHT_BLUE,
            State::InProgress => Color32::LIGHT_RED,
        }
    }
}

impl DeploymentPanel {
    fn repositories(&self) -> IntoIter<Repository> {
        self.repositories.clone().into_iter()
    }

    fn deployment(&self, repo: &str, env: &str) -> Option<Deployment> {
        let deployments = self.deployments.lock().unwrap().clone();
        if let Some(repo_deployments) = deployments.get(repo) {
            repo_deployments
                .clone()
                .into_iter()
                .find(|deployment| deployment.environment == env)
        } else {
            None
        }
    }

    fn environments(&self) -> IntoIter<Environment> {
        self.environments.lock().unwrap().clone().into_iter()
    }

    fn refresh_statuses(&mut self, token: &str) {
        self.statuses.lock().unwrap().clear();
        let repo_to_deployments = self.deployments.lock().unwrap().clone();
        repo_to_deployments.into_values().for_each(|deployments| {
            deployments.into_iter().for_each(|deployment| {
                let statuses = self.statuses.clone();
                self.refresh_status(token, &deployment, move |response| {
                    let last_status = response
                        .into_iter()
                        .sorted_by(|cur, next| Ord::cmp(&next.id, &cur.id))
                        .next();

                    if let Some(status) = last_status {
                        statuses.lock().unwrap().insert(deployment.id, status);
                    }
                });
            });
        });
    }

    fn refresh_deployments(&mut self, token: &str) {
        self.environments.lock().unwrap().clear();
        self.deployments.lock().unwrap().clear();

        self.repositories().for_each(|repository| {
            let environments = self.environments.clone();
            self.refresh_environment(token, &repository, move |response| {
                let new_envs = response
                    .into_iter()
                    .filter(|env| {
                        !environments
                            .lock()
                            .unwrap()
                            .iter()
                            .any(|cur| cur.name == env.name)
                    })
                    .collect_vec();

                environments.lock().unwrap().extend(new_envs);
            });

            let repository_name = repository.clone().name;
            let deployments = self.deployments.clone();
            self.refresh_deployment(token, &repository, move |response| {
                let last_deployments_per_environment = response
                    .into_iter()
                    .group_by(|deps| deps.environment.clone())
                    .into_iter()
                    .map(|(_, group)| group.max_by(|x, y| x.id.cmp(&y.id)).unwrap())
                    .collect_vec();

                *deployments
                    .lock()
                    .unwrap()
                    .entry(repository_name)
                    .or_default() = last_deployments_per_environment;
            });
        });
    }

    fn refresh_deployment(
        &mut self,
        token: &str,
        repo: &Repository,
        on_refreshed: impl FnOnce(Vec<Deployment>) + Send + 'static,
    ) {
        let mut client = self.client.clone();
        self.client.get(token, &repo.deployments_url.clone(), move |response| {
            if let Ok(response) = response {
                if let Some(remaining) = response.headers.get("x-ratelimit-remaining") {
                    let remaining = remaining.parse::<usize>().unwrap();
                    client.set_rate_limit(remaining);
                }
                
                let deployments = serde_json::from_slice::<Vec<Deployment>>(&response.bytes).unwrap_or_default();
                on_refreshed(deployments);
            }
        })
    }

    fn refresh_status(
        &mut self,
        token: &str,
        deployment: &Deployment,
        on_refreshed: impl FnOnce(Vec<Status>) + Send + 'static,
    ) {
        let mut client = self.client.clone();
        self.client.get(token, &deployment.statuses_url, move |response| {
            if let Ok(response) = response {
                if let Some(remaining) = response.headers.get("x-ratelimit-remaining") {
                    let remaining = remaining.parse::<usize>().unwrap();
                    client.set_rate_limit(remaining);
                }

                let statuses = serde_json::from_slice::<Vec<Status>>(&response.bytes).unwrap_or_default();
                on_refreshed(statuses)
            }
        })
    }

    fn refresh_environment(
        &mut self,
        token: &str,
        repository: &Repository,
        on_refreshed: impl FnOnce(Vec<Environment>) + Send + 'static,
    ) {
        self.client.get_path(
            token,
            &format!("/repos/navikt/{}/environments", repository.name),
            |response| {
                if let Ok(response) = response {
                    let environments = match serde_json::from_slice::<Environments>(&response.bytes) {
                        Ok(environments) => environments.environments,
                        Err(_) => vec![],
                    };
                    on_refreshed(environments);
                }
            },
        );
    }
}
