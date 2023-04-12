use std::sync::{Arc, Mutex};

use eframe::epaint::{Color32, FontId};
use eframe::epaint::text::LayoutJob;
use egui::Ui;
use itertools::Itertools;
use crate::github;

use crate::github::github_models::{Repo, Team};
use crate::github::HOST;

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct RepositoriesPanel {
    repositories: Arc<Mutex<Vec<Repo>>>,
    blacklisted: Arc<Mutex<Vec<Repo>>>,
    team: Arc<Mutex<Team>>,
    team_name: String,
}

impl RepositoriesPanel {
    pub fn repos(&self) -> Vec<Repo> {
        self.repositories.lock().unwrap().clone()
    }
}

impl RepositoriesPanel {
    pub fn repositories(&self) -> Vec<Repo> {
        self.repositories.lock().unwrap().clone()
    }

    pub fn blacklisted_repositories(&self) -> Vec<Repo> {
        self.blacklisted.lock().unwrap().clone()
    }

    pub fn whitelist_repository(&self, repo: Repo) {
        self.repositories.lock().unwrap().push(repo.clone());
        self.blacklisted.clone().lock().unwrap().retain(|it| *it != repo.clone());
    }

    pub fn blacklist_repository(&self, repo: Repo) {
        self.blacklisted.lock().unwrap().push(repo.clone());
        self.repositories.lock().unwrap().retain(|it| *it != repo.clone());
    }
}

impl RepositoriesPanel {
    pub fn paint(&mut self, ui: &mut Ui, token: &str) {
        ui.heading("Repositories");

        ui.horizontal_wrapped(|ui| {
            ui.label("Team");
            if ui.text_edit_singleline(&mut self.team_name).lost_focus() {
                self.fetch_team(token, self.team_name.clone());
            }
            if ui.button("Fetch").clicked() {
                self.fetch_repositories(token);
            }
        });
        ui.separator();
        ui.horizontal_top(|ui| {
            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.heading(format!("Selected: {}", self.repositories.lock().unwrap().len()));
                    self.repositories().into_iter().for_each(|repo| {
                        ui.horizontal_wrapped(|ui| {
                            let blacklist_button = LayoutJob::simple_singleline("➡".into(), FontId::default(), Color32::LIGHT_RED);

                            if ui.button(blacklist_button).clicked() {
                                self.blacklist_repository(repo.clone())
                            };

                            ui.label(&repo.name);
                        });
                    });
                });
            });
            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.heading(format!("Blacklisted: {}", self.blacklisted.lock().unwrap().len()));

                    self.blacklisted_repositories().into_iter().for_each(|repo| {
                        ui.horizontal_wrapped(|ui| {
                            let whitelist_button = LayoutJob::simple_singleline("⬅".into(), FontId::default(), Color32::LIGHT_GREEN);

                            if ui.button(whitelist_button).clicked() {
                                self.whitelist_repository(repo.clone())
                            };

                            ui.label(&repo.name);
                        });
                    });
                });
            });
            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.heading("Team");
                    ui.label(format!("{}", self.team.lock().unwrap().clone()))
                });
            });
        });
    }
}

impl RepositoriesPanel {
    fn fetch_team(&self, token: &str, team_name: String) {
        let _team = self.team.clone();
        github::fetch_lifetime::<Team>(token, &format!("{}/orgs/navikt/teams/{}", HOST, &team_name), move | response | {
            if let Ok(team) = response {
                *_team.lock().unwrap() = team
            }
        });
    }

    fn fetch_repositories(&mut self, token: &str) {
        let _repositories = self.repositories.clone();
        let _team = self.team.lock().unwrap().clone();
        let _blacklisted = self.blacklisted.lock().unwrap().clone();
        let url = format!("{}{}", &_team.repositories_url, "?per_page=100");
        github::fetch_lifetime::<Vec<Repo>>(token, &url, move |response| {
            if let Ok(repositories) = response {
                let result = repositories.into_iter()
                    .filter(|repo| !_blacklisted.contains(repo))
                    .collect_vec();

                *_repositories.lock().unwrap() = result;
            }
        });
    }
}
