use std::collections::{BTreeMap, HashSet};
use std::sync::{Arc, Mutex};

use eframe::epaint::{Color32, FontId};
use egui::{CentralPanel, ScrollArea, SelectableLabel, SidePanel, TextEdit, TopBottomPanel};
use egui::text::LayoutJob;
use egui_extras::{Size, StripBuilder};

use crate::github::github_client::*;
use crate::github::github_models::*;
use crate::ui::table::Table;

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
            github,
            token,
            workflows_options,
            pr_table,
            run_table,
            panel,
            team_name,
            team: _,
            teams: _,
            pull_requests: _,
            workflows: _,
            workflow_runs: _,
            repositories: _,
            blacklisted_repositories: _,
        } = self;

        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label("PAT").on_hover_text("Get your Personal Access Token from Github and select repo and workflow persmissions");
                ui.add(TextEdit::singleline(&mut token.value).password(!token.show));
                if ui.add(SelectableLabel::new(token.show, "👁")).on_hover_text("Show/hide token").clicked() {
                    *token = token.toggle();
                };
            });
        });

        SidePanel::left("side_panel").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("GitHub Status");
                ui.group(|ui| {
                    ui.separator();
                    if ui.button("  Pull Requests  ").clicked() { *panel = Panel::PullRequests }
                    ui.separator();
                    if ui.button("  Workflows  ").clicked() { *panel = Panel::WorkflowRuns }
                    ui.separator();
                    if ui.button("  Repositories  ").clicked() { *panel = Panel::Repositories }
                    ui.separator();
                });
            });
        });

        CentralPanel::default().show(ctx, |ui| {
            match panel {
                Panel::PullRequests => {
                    ui.heading("Pull Requests");
                    if ui.button("Refresh").clicked() {
                        self.pull_requests.lock().unwrap().clear();
                        let _repos = self.repositories.clone();

                        _repos.lock().unwrap().clone().into_iter().for_each(|_repo| {
                            let _pulls = self.pull_requests.clone();
                            github.fetch_path(&mut token.value, &format!("/repos/navikt/{}/pulls", _repo.name), move |response| {
                                if let Ok(pull_requests) = serde_json::from_slice::<HashSet<PullRequest>>(&response) {
                                    *_pulls.lock().unwrap()
                                        .entry(_repo.clone().name)
                                        .or_insert(HashSet::default()) = pull_requests;
                                }
                            });
                        });
                    }

                    StripBuilder::new(ui)
                        .size(Size::remainder().at_least(100.0))
                        .vertical(|mut strip| {
                            strip.cell(|ui| {
                                ScrollArea::horizontal().show(ui, |ui| {
                                    pr_table.pull_requests_ui(ui, &self.pull_requests.lock().unwrap())
                                });
                            });
                        });
                }
                Panel::WorkflowRuns => {
                    ui.heading("Failed Workflows");
                    ui.horizontal_wrapped(|ui| {
                        if ui.button("Refresh").clicked() {
                            self.workflow_runs.lock().unwrap().clear();
                            let _repos = self.repositories.clone();

                            _repos.lock().unwrap().clone().into_iter().for_each(|_repo| {
                                let _workflow_runs = self.workflow_runs.clone();

                                github.fetch_path(&mut token.value, &format!("/repos/navikt/{}/actions/runs?per_page=15", _repo.name), move |response| {
                                    if let Ok(workflow_runs) = serde_json::from_slice::<WorkflowRuns>(&response) {
                                        *_workflow_runs.lock().unwrap()
                                            .entry(_repo.clone().name)
                                            .or_insert(HashSet::default()) = workflow_runs.workflow_runs;
                                    }
                                });
                            });
                        }

                        if ui.add(SelectableLabel::new(workflows_options.show_prs, "Show pull-requests")).clicked() {
                            *workflows_options = workflows_options.toggle_prs();
                        };

                        if ui.add(SelectableLabel::new(workflows_options.show_success, "Show successful")).clicked() {
                            *workflows_options = workflows_options.toggle_success();
                        };
                    });

                    StripBuilder::new(ui)
                        .size(Size::remainder().at_least(100.0))
                        .vertical(|mut strip| {
                            strip.cell(|ui| {
                                ScrollArea::horizontal().show(ui, |ui| {
                                    let _runs = &self.workflow_runs.lock().unwrap();
                                    run_table.workflow_runs_ui(ui, workflows_options.show_prs, workflows_options.show_success, _runs)
                                });
                            });
                        });
                }
                Panel::Repositories => {
                    ui.heading("Repositories");

                    ui.horizontal_wrapped(|ui| {
                        ui.label("Team");
                        if ui.text_edit_singleline(team_name).lost_focus() {
                            *team_name = team_name.to_string();
                            let _team = self.team.clone();
                            github.fetch_path(&mut token.value, &format!("/orgs/navikt/teams/{}", &team_name), move |response| {
                                if let Ok(team) = serde_json::from_slice::<Team>(&response) {
                                    *_team.lock().unwrap() = team;
                                }
                            });
                        }

                        if ui.button("Fetch").clicked() {
                            let _repositories = self.repositories.clone();
                            let _team = self.team.lock().unwrap().clone();
                            let _blacklisted = self.blacklisted_repositories.lock().unwrap().clone();
                            github.fetch_url(&mut token.value, format!("{}{}", &_team.repositories_url, "?per_page=100").as_str(), move |response| {
                                if let Ok(repositories) = serde_json::from_slice::<HashSet<Repo>>(&response) {
                                    *_repositories.lock().unwrap() = repositories.into_iter()
                                        .filter(|repo| !_blacklisted.contains(repo))
                                        .collect::<HashSet<Repo>>();
                                }
                            });
                        }
                    });

                    ui.separator();
                    ui.horizontal_top(|ui| {
                        ui.group(|ui| {
                            ui.vertical(|ui| {
                                let _repos = self.repositories.lock().unwrap().clone();

                                ui.heading(format!("Selected: {}", &_repos.len()));

                                _repos.into_iter().for_each(|repo| {
                                    ui.horizontal_wrapped(|ui| {
                                        let blacklist_button = LayoutJob::simple_singleline("➡".into(), FontId::default(), Color32::LIGHT_RED);

                                        if ui.button(blacklist_button).clicked() {
                                            self.repositories.lock().unwrap().remove(&repo);
                                            self.blacklisted_repositories.lock().unwrap().insert(repo.clone());
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
                                        let whitelist_button = LayoutJob::simple_singleline("⬅".into(), FontId::default(), Color32::LIGHT_GREEN);

                                        if ui.button(whitelist_button).clicked() {
                                            self.repositories.lock().unwrap().insert(repo.clone());
                                            self.blacklisted_repositories.clone().lock().unwrap().remove(&repo);
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
            };
        });
    }

    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

#[derive(PartialEq, serde::Deserialize, serde::Serialize)]
enum Panel {
    Repositories,
    PullRequests,
    WorkflowRuns,
}

impl Default for Panel {
    fn default() -> Self { Self::Repositories }
}

#[derive(serde::Deserialize, serde::Serialize, Default, Clone)]
struct Token {
    value: String,
    show: bool,
}

impl Token {
    fn toggle(&self) -> Self {
        Token {
            show: !self.show,
            ..self.clone()
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Default, Clone)]
struct WorkflowsOptions {
    show_prs: bool,
    show_success: bool,
}

impl WorkflowsOptions {
    fn toggle_success(&self) -> Self {
        WorkflowsOptions {
            show_success: !self.show_success,
            ..self.clone()
        }
    }

    fn toggle_prs(&self) -> Self {
        WorkflowsOptions {
            show_prs: !self.show_prs,
            ..self.clone()
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)]
pub struct Application {
    #[serde(skip)]
    github: GitHubApi,
    token: Token,
    workflows_options: WorkflowsOptions,
    pr_table: Table,
    run_table: Table,
    panel: Panel,
    team_name: String,

    team: Arc<Mutex<Team>>,
    teams: Arc<Mutex<HashSet<Team>>>,
    pull_requests: Arc<Mutex<BTreeMap<String, HashSet<PullRequest>>>>,
    workflows: Arc<Mutex<BTreeMap<String, HashSet<Workflow>>>>,
    workflow_runs: Arc<Mutex<BTreeMap<String, HashSet<WorkflowRun>>>>,
    repositories: Arc<Mutex<HashSet<Repo>>>,
    blacklisted_repositories: Arc<Mutex<HashSet<Repo>>>,
}
