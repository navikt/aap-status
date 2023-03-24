use std::collections::{BTreeMap, HashSet};
use std::sync::{Arc, Mutex};

use eframe::epaint::Color32;
use eframe::epaint::text::TextFormat;

use crate::github::github_client::{GitHubApi, Pulls, Repositories, Runs, Teams};
use crate::github::pulls::PullRequest;
use crate::github::repositories::Repo;
use crate::github::runs::WorkflowRuns;
use crate::github::teams::Team;
use crate::github::workflows::Workflow;
use crate::ui::table::Table;

impl eframe::App for TemplateApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            token,
            show_token,
            pr_table,
            run_table,
            state,
            team,
            github,
            pulls: _,
            workflows: _,
            runs: _,
            repos: _,
            teams: _,
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
            ui.heading("GitHub Status");

            ui.separator();
            ui.label("Overview of pull requests");

            if ui.button("Pull Requests").clicked() {
                *state = State::Pulls
            }

            ui.separator();
            ui.label("Latest runs in GitHub Actions");

            if ui.button("Workflows").clicked() {
                *state = State::Runs
            }

            ui.separator();
            if ui.button("Repositories").clicked() {
                *state = State::Repositories
            }

            ui.separator();
            ui.label("Fetch data from GitHub");

            if ui.button("Refresh").clicked() {
                match state {
                    State::Pulls => {
                        let _repos = self.repos.clone();
                        for repo in _repos.lock().unwrap().clone().into_iter() {
                            let _pulls = self.pulls.clone();
                            github.pull_requests(token, &repo.clone().name, move |response: Vec<PullRequest>| {
                                *_pulls.lock().unwrap().entry(repo.name).or_default() = response;
                            });
                        }
                    }
                    State::Runs => {
                        let _repos = self.repos.clone();
                        for repo in _repos.lock().unwrap().clone().into_iter() {
                            let _runs = self.runs.clone();
                            github.runs(token, &repo.clone().name, move |response: WorkflowRuns| {
                                *_runs.lock().unwrap().entry(repo.name).or_insert(WorkflowRuns::default()) = response;
                            });
                        }
                    }
                    State::Repositories => {
                        let _repos = self.repos.clone();
                        github.repositories(token, &team.clone(), move |response| {
                            *_repos.lock().unwrap() = response
                        });
                    }
                }
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            use egui_extras::{Size, StripBuilder};
            match state {
                State::Pulls => {
                    ui.heading("Pull Requests");

                    StripBuilder::new(ui)
                        .size(Size::remainder().at_least(100.0))
                        .vertical(|mut strip| {
                            strip.cell(|ui| {
                                egui::ScrollArea::horizontal().show(ui, |ui| {
                                    pr_table.pull_requests_ui(ui, &self.pulls.lock().unwrap().clone())
                                });
                            });
                        });
                }
                State::Runs => {
                    ui.heading("Workflow Runs");

                    StripBuilder::new(ui)
                        .size(Size::remainder().at_least(100.0))
                        .vertical(|mut strip| {
                            strip.cell(|ui| {
                                egui::ScrollArea::horizontal().show(ui, |ui| {
                                    let _runs = &self.runs.lock().unwrap().clone();
                                    let _workflows = &self.workflows.lock().unwrap().clone();
                                    run_table.workflow_runs_ui(ui, _runs)
                                });
                            });
                        });
                }
                State::Repositories => {
                    ui.heading("Repositories");

                    ui.horizontal_wrapped(|ui| {
                        if ui.text_edit_singleline(team).ctx.input().key_pressed(egui::Key::Enter) {
                            *team = team.to_string();
                        }

                        if ui.button("fetch repos").clicked() {
                            let _repos = self.repos.clone();
                            github.repositories(token, &team.clone(), move |response| {
                                *_repos.lock().unwrap() = response
                            });
                        }
                    });

                    ui.label(format!("Teams in navikt: {}", self.teams.clone().lock().unwrap().len()));
                    ui.horizontal_wrapped(|ui| {
                        egui::ComboBox::from_label("")
                            .selected_text(format!("{:?}", team))
                            .show_ui(ui, |ui| {
                                ui.style_mut().wrap = Some(false);
                                ui.set_min_width(60.0);

                                self.teams.lock().unwrap().clone().into_iter().for_each(|fetched_team| {
                                    ui.selectable_value(team, fetched_team.name.clone(), fetched_team.name);
                                });
                            });
                        if ui.button("fetch teams from github/navikt").clicked() {
                            let base_url = String::from("https://api.github.com/orgs/navikt/teams?per_page=100&page=");

                            for i in 1..=3 {
                                let url = format!("{}{}", base_url, i);
                                let teams = self.teams.clone();
                                let teams_to_add = github.teams(&url, token).block_and_take();
                                teams.lock().unwrap().extend(teams_to_add.into_iter());
                            }
                        }
                    });

                    ui.separator();

                    ui.label(format!("Repositories in your selected team {}: {}", team, self.repos.clone().lock().unwrap().len()));
                    let _repos = self.repos.lock().unwrap().clone();

                    _repos.into_iter().for_each(|repo| {
                        ui.horizontal_wrapped(|ui| {
                            use egui::text::LayoutJob;
                            let mut job = LayoutJob::default();
                            let red_text = TextFormat {
                                color: Color32::from_rgb(255, 100, 100),
                                ..Default::default()
                            };
                            job.append("❌", 0.0, red_text);
                            if ui.button(job).clicked() {
                                println!("button remove for {:?} clicked", &repo.name);
                                self.repos.clone().lock().unwrap().remove(&repo);
                            };
                            ui.label(&repo.name);
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
            token: String::from("<GitHub PAT>"),
            show_token: false,
            pr_table: Table::default(),
            run_table: Table::default(),
            state: State::Repositories,
            team: String::from("aap"),
            github: GitHubApi::default(),
            pulls: Arc::new(Mutex::new(BTreeMap::new())),
            workflows: Arc::new(Mutex::new(BTreeMap::new())),
            runs: Arc::new(Mutex::new(BTreeMap::new())),
            repos: Arc::new(Mutex::new(HashSet::new())),
            teams: Arc::new(Mutex::new(HashSet::new())),
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

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TemplateApp {
    token: String,
    show_token: bool,
    pr_table: Table,
    run_table: Table,
    state: State,
    team: String,

    #[serde(skip)]
    github: GitHubApi,

    // #[serde(skip)]
    pulls: Arc<Mutex<BTreeMap<String, Vec<PullRequest>>>>,

    // #[serde(skip)]
    workflows: Arc<Mutex<BTreeMap<String, Vec<Workflow>>>>,

    // #[serde(skip)]
    runs: Arc<Mutex<BTreeMap<String, WorkflowRuns>>>,

    // #[serde(skip)]
    repos: Arc<Mutex<HashSet<Repo>>>,

    // #[serde(skip)]
    teams: Arc<Mutex<HashSet<Team>>>,
}
