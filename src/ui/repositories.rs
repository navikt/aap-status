use std::sync::{Arc, Mutex};
use eframe::epaint::{Color32, FontId};
use eframe::epaint::text::LayoutJob;
use egui::Ui;
use itertools::Itertools;

use crate::github::github_client::{Fetch, GitHubApi};
use crate::github::github_models::{Repo, Team};

#[derive(Default, Clone)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct RepositoriesPanel {
    repositories: Arc<Mutex<Vec<Repo>>>,
    blacklisted: Arc<Mutex<Vec<Repo>>>,
    team: Arc<Mutex<Team>>,
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
    pub fn paint(&self, ui: &mut Ui) {
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
    pub fn fetch_team(&self, team_name: String, github: GitHubApi) {
        let _team = self.team.clone();
        github.fetch_path(&format!("/orgs/navikt/teams/{}", &team_name), move |response| {
            match serde_json::from_slice::<Team>(&response) {
                Err(error) => eprintln!("error: {}", error),
                Ok(response) => {
                    *_team.lock().unwrap() = response;
                }
            }
        });
    }

    pub fn fetch(&self, github: GitHubApi) {
        let _repositories = self.repositories.clone();
        let _team = self.team.lock().unwrap().clone();
        let _blacklisted = self.blacklisted.lock().unwrap().clone();
        github.fetch_url(format!("{}{}", &_team.repositories_url, "?per_page=100").as_str(), move |response| {
            if let Ok(repositories) = serde_json::from_slice::<Vec<Repo>>(&response) {
                *_repositories.lock().unwrap() = repositories.into_iter()
                    .filter(|repo| !_blacklisted.contains(repo))
                    .collect_vec();
            }
        });
    }
}
