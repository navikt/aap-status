use std::sync::{Arc, Mutex};

use egui::{Color32, FontId, Ui};
use egui::text::LayoutJob;
use http::github::{Client, self};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use model::repository::Repository;
use model::team::Team;

use crate::panel::Panel;

#[derive(Deserialize, Serialize, Default)]
pub struct RepositoriesPanel {
    repositories: Arc<Mutex<Vec<Repository>>>,
    blacklisted: Arc<Mutex<Vec<Repository>>>,
    archived: Arc<Mutex<Vec<Repository>>>,
    team: Arc<Mutex<Option<Team>>>,
    team_name: String,
    client: Client,
}

impl Panel for RepositoriesPanel {
    fn set_repositories(&mut self, _: Vec<Repository>) {}
    fn set_client(&mut self, client: github::Client) { self.client = client }

    fn paint(&mut self, ui: &mut Ui, token: &str) {
        ui.heading("Repositories");

        ui.horizontal_wrapped(|ui| {
            ui.label("Team");
            if ui.text_edit_singleline(&mut self.team_name).lost_focus() {
                println!("team edit field lost focus");
            }
            if ui.button("Select").clicked() {
                self.fetch_team(token, self.team_name.clone());
            }
        });
        ui.separator();

        self.team_info(ui, token);
        ui.separator();

        ui.horizontal_top(|ui| {
            ui.group(|ui| self.whitelisted(ui));
            ui.group(|ui| self.blacklisted(ui));
            ui.group(|ui| self.archived(ui));
            // ui.group(|ui| self.team_info(ui));
        });
    }
}

impl RepositoriesPanel {
    pub fn repositories(&self) -> Vec<Repository> {
        self.repositories.lock().unwrap().clone()
    }

    pub fn blacklisted_repositories(&self) -> Vec<Repository> { self.blacklisted.lock().unwrap().clone() }

    pub fn whitelist_repository(&self, repo: Repository) {
        self.repositories.lock().unwrap().push(repo.clone());
        self.blacklisted.clone().lock().unwrap().retain(|it| *it != repo.clone());
    }

    pub fn blacklist_repository(&self, repo: Repository) {
        self.blacklisted.lock().unwrap().push(repo.clone());
        self.repositories.lock().unwrap().retain(|it| *it != repo.clone());
    }

    fn whitelisted(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.heading(format!("Selected: {}", self.repositories.lock().unwrap().len()));
            self.repositories().into_iter().for_each(|repo| {
                ui.horizontal_wrapped(|ui| {
                    let blacklist_button = LayoutJob::simple_singleline("➡".into(), FontId::default(), Color32::LIGHT_RED);

                    if ui.button(blacklist_button).clicked() {
                        self.blacklist_repository(repo.clone())
                    }

                    ui.label(&repo.name);
                });
            });
        });
    }

    fn blacklisted(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.heading(format!("Blacklisted: {}", self.blacklisted.lock().unwrap().len()));

            self.blacklisted_repositories().into_iter().for_each(|repo| {
                ui.horizontal_wrapped(|ui| {
                    let whitelist_button = LayoutJob::simple_singleline("⬅".into(), FontId::default(), Color32::LIGHT_GREEN);

                    if ui.button(whitelist_button).clicked() {
                        self.whitelist_repository(repo.clone())
                    }

                    ui.label(&repo.name);
                });
            });
        });
    }

    fn archived(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.heading(format!("Archived: {}", self.archived.lock().unwrap().len()));
    
            self.archived.lock().unwrap().iter().for_each(|repo| {
                ui.label(&repo.name);
            });
        });
    }

    fn team_info(&mut self, ui: &mut Ui, token: &str) {
        let team = self.team.lock().unwrap().clone();
        match team {
            Some(team) => {
                ui.vertical(|ui| {
                    ui.hyperlink_to("Members", format!("https://github.com/orgs/navikt/teams/{}/members", team.name));
                    ui.label(team.description.unwrap_or_default());
                    if ui.button("Fetch repositories").clicked() {
                        self.fetch_repositories(token);
                    }
                });
            },
            None => {
                ui.label("No team selected.");
            }
        }
    }

    fn fetch_team(&mut self, token: &str, team_name: String) {
        let _team = self.team.clone();
        let url = format!("/orgs/navikt/teams/{}", &team_name);
        let mut client = self.client.clone();
        self.client.get_path(token, &url, move |response| {
            if let Ok(response) = response {
                if let Some(remaining) = response.headers.get("x-ratelimit-remaining") {
                    let remaining = remaining.parse::<usize>().unwrap();
                    client.set_rate_limit(remaining);
                }

                let team = serde_json::from_slice::<Team>(&response.bytes).unwrap_or_default();
                *_team.lock().unwrap() = Some(team);
            }
        });
    }

    fn fetch_repositories(&mut self, token: &str) {
        let _repositories = self.repositories.clone();
        let _team = self.team.lock().unwrap().clone().unwrap(); // button is only visible if this is Some
        let _blacklisted = self.blacklisted.lock().unwrap().clone();
        let _archived = self.archived.clone();
        let url = format!("{}{}", &_team.repositories_url, "?per_page=100");
        let mut client = self.client.clone();
        self.client.get(token, &url, move |response| {
            if let Ok(response) = response {
                
                if let Some(remaining) = response.headers.get("x-ratelimit-remaining") {
                    let remaining = remaining.parse::<usize>().unwrap();
                    client.set_rate_limit(remaining);
                }

                let repositories = serde_json::from_slice::<Vec<Repository>>(&response.bytes).unwrap_or_default();
                let repos = repositories
                    .clone()
                    .into_iter()
                    .filter(|repo| !_blacklisted.contains(repo))
                    .filter(|repo| !repo.archived)
                    .collect_vec();

                *_repositories.lock().unwrap() = repos;

                let archived = repositories
                    .clone()
                    .into_iter()
                    .filter(|repo| !_blacklisted.contains(repo))
                    .filter(|repo| repo.archived)
                    .collect_vec();

                *_archived.lock().unwrap() = archived;
            }
        });
    }
}
