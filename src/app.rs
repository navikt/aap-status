use std::collections::{BTreeMap, HashSet};
use std::sync::{Arc, Mutex};

use eframe::epaint::{Color32, FontId};
use egui::ScrollArea;

use crate::github::github_client::{Fetcher, GitHubApi, Pulls, Repositories, Teams, Workflows};
use crate::github::pulls::PullRequest;
use crate::github::repositories::Repo;
use crate::github::teams::Team;
use crate::github::workflows::{Workflow, WorkflowRun};
use crate::ui::table::Table;

impl eframe::App for TemplateApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            github,
            token,
            show_token,
            pr_table,
            run_table,
            state,
            team_name,
            team: _,
            teams: _,
            pull_requests: _,
            workflows: _,
            workflow_runs: _,
            repositories: _,
            blacklisted_repositories: _,
        } = self;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label("Personal Access Token:");
                ui.add(egui::TextEdit::singleline(token).password(*show_token));

                if ui.add(egui::SelectableLabel::new(*show_token, "👁"))
                    .on_hover_text("Show/hide token")
                    .clicked() { *show_token = !*show_token; };
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("GitHub Status");
                ui.group(|ui| {
                    ui.separator();
                    if ui.button("Pull Requests").clicked() { *state = State::Pulls }
                    ui.separator();
                    if ui.button("Workflows").clicked() { *state = State::Runs }
                    ui.separator();
                    if ui.button("Repositories").clicked() { *state = State::Repositories }
                    ui.separator();
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            use egui_extras::{Size, StripBuilder};
            match state {
                State::Pulls => {
                    ui.heading("Pull Requests");
                    if ui.button("Refresh").clicked() {
                        self.pull_requests.lock().unwrap().clear();
                        let _pulls = self.pull_requests.clone();
                        let _repos = self.repositories.clone();

                        _repos.lock().unwrap().clone().iter().for_each(|_repo| {
                            let prs = github.pull_requests(token, &_repo.clone().name).block_and_take();
                            *_pulls.lock().unwrap().entry(_repo.name.clone()).or_insert(HashSet::default()) = prs;
                        });
                    }

                    StripBuilder::new(ui)
                        .size(Size::remainder().at_least(100.0))
                        .vertical(|mut strip| {
                            strip.cell(|ui| {
                                ScrollArea::horizontal()
                                    .show(ui, |ui| {
                                        pr_table.pull_requests_ui(ui, &self.pull_requests.lock().unwrap())
                                    });
                            });
                        });
                }
                State::Runs => {
                    ui.heading("Failed Workflows");
                    ui.horizontal_wrapped(|ui| {
                        if ui.button("Refresh").clicked() {
                            self.workflow_runs.lock().unwrap().clear();
                            let _repos = self.repositories.clone();

                            _repos.lock().unwrap().clone().iter().for_each(|_repo| {
                                let runs = github.workflow_runs(token, &_repo.clone().name).block_and_take();
                                *self.workflow_runs.lock().unwrap().entry(_repo.name.clone()).or_insert(HashSet::default()) = runs;
                            });
                        }
                    });

                    StripBuilder::new(ui)
                        .size(Size::remainder().at_least(100.0))
                        .vertical(|mut strip| {
                            strip.cell(|ui| {
                                ScrollArea::horizontal().show(ui, |ui| {
                                    let _runs = &self.workflow_runs.lock().unwrap();
                                    run_table.workflow_runs_ui(ui, _runs)
                                });
                            });
                        });
                }
                State::Repositories => {
                    ui.heading("Repositories");
                    ui.horizontal_wrapped(|ui| {
                        if ui.text_edit_singleline(team_name).lost_focus() {
                            tracing::info!("selected {:?}", &team_name);
                            *team_name = team_name.to_string();
                            if let Some(team) = github.team(&team_name, token).block_and_take() {
                                *self.team.lock().unwrap() = team;
                            }
                        }

                        if ui.button("Fetch async").clicked() {
                            let _team = self.team.lock().unwrap().clone();
                            let _blacklisted = self.blacklisted_repositories.lock().unwrap().clone();
                            let repositories = github.repositories(token, &_team)
                                .block_and_take()
                                .into_iter()
                                .filter(|repo| !_blacklisted.contains(repo))
                                .collect::<HashSet<Repo>>();
                            *self.repositories.lock().unwrap() = repositories;
                        }

                        if ui.button("Fetch").clicked() {
                            let _repositories = self.repositories.clone();
                            let _team = self.team.lock().unwrap().clone();
                            github.fetch(token, format!("{}{}", &_team.repositories_url, "?per_page=100").as_str(), move |response| {
                                if let Ok(repositories) = serde_json::from_slice::<HashSet<Repo>>(&response) {
                                    *_repositories.lock().unwrap() = repositories;
                                }
                            });
                        }
                    });

                    ui.separator();
                    ui.group(|ui| {
                        ui.label(format!("{}", self.team.lock().unwrap().clone()));
                    });

                    ui.horizontal_wrapped(|ui| {
                        ui.group(|ui| {
                            ui.vertical(|ui| {
                                let _repos = self.repositories.lock().unwrap().clone();

                                ui.heading(format!("Selected: {}", &_repos.len()));


                                _repos.into_iter().for_each(|repo| {
                                    ui.horizontal_wrapped(|ui| {
                                        let blacklist_button = egui::text::LayoutJob::simple_singleline(
                                            String::from("➡"),
                                            FontId::default(),
                                            Color32::LIGHT_RED,
                                        );

                                        if ui.button(blacklist_button).clicked() {
                                            tracing::info!("blacklisted {:?}", &repo.name);
                                            self.repositories.clone().lock().unwrap().remove(&repo);
                                            self.blacklisted_repositories.clone().lock().unwrap().insert(repo.clone());
                                        };

                                        ui.label(&repo.name);
                                    });
                                });
                            });
                        });
                        ui.group(|ui| {
                            ui.vertical(|ui| {
                                let _blacklisted_repos = self.blacklisted_repositories.lock().unwrap().clone();

                                ui.heading(format!("Blacklisted: {}", _blacklisted_repos.len()));

                                _blacklisted_repos.into_iter().for_each(|repo| {
                                    ui.horizontal_wrapped(|ui| {
                                        let whitelist_button = egui::text::LayoutJob::simple_singleline(
                                            String::from("⬅"),
                                            FontId::default(),
                                            Color32::LIGHT_GREEN,
                                        );

                                        if ui.button(whitelist_button).clicked() {
                                            tracing::info!("whitelisted {:?}", &repo.name);
                                            self.repositories.clone().lock().unwrap().insert(repo.clone());
                                            self.blacklisted_repositories.clone().lock().unwrap().remove(&repo);
                                        };

                                        ui.label(&repo.name);
                                    });
                                });
                            });
                        });
                    });
                }
            };
        });
    }

    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            github: GitHubApi::default(),
            token: String::from("<GitHub PAT>"),
            show_token: false,
            pr_table: Table::default(),
            run_table: Table::default(),
            state: State::Repositories,
            team_name: String::from("aap"),
            team: Arc::default(),
            teams: Arc::default(),
            pull_requests: Arc::default(),
            workflows: Arc::default(),
            workflow_runs: Arc::default(),
            repositories: Arc::default(),
            blacklisted_repositories: Arc::default(),
        }
    }
}

impl TemplateApp {
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

#[derive(PartialEq)]
#[derive(serde::Deserialize, serde::Serialize)]
pub enum State {
    Repositories,
    Pulls,
    Runs,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TemplateApp {
    #[serde(skip)]
    github: GitHubApi,

    token: String,
    show_token: bool,
    pr_table: Table,
    run_table: Table,
    state: State,
    team_name: String,

    team: Arc<Mutex<Team>>,
    teams: Arc<Mutex<HashSet<Team>>>,
    pull_requests: Arc<Mutex<BTreeMap<String, HashSet<PullRequest>>>>,
    workflows: Arc<Mutex<BTreeMap<String, HashSet<Workflow>>>>,
    workflow_runs: Arc<Mutex<BTreeMap<String, HashSet<WorkflowRun>>>>,
    repositories: Arc<Mutex<HashSet<Repo>>>,
    blacklisted_repositories: Arc<Mutex<HashSet<Repo>>>,
}
