use egui::Ui;

pub trait Table<T: Clone> {
    fn render(&mut self, ui: &mut Ui, data: &T);
}

#[derive(serde::Deserialize, serde::Serialize, Default, Clone)]
pub struct Tables {
    pull_requests: TableUI,
    workflow_runs: TableUI,
}

impl Tables {
    pub fn pull_requests(&self) -> TableUI { self.pull_requests.clone() }
    pub fn workflow_runs(&self) -> TableUI { self.workflow_runs.clone() }

    pub fn show_pull_requests_runs(&self) -> bool { self.workflow_runs.show_prs }
    pub fn show_successful_runs(&self) -> bool { self.workflow_runs.show_success }

    pub fn toggle_show_workflow_success(&self) -> Self {
        Tables {
            workflow_runs: self.workflow_runs.toggle_success(),
            ..self.clone()
        }
    }

    pub fn toggle_show_workflow_pulls(&self) -> Self {
        Tables {
            workflow_runs: self.workflow_runs.toggle_prs(),
            ..self.clone()
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Default, Clone)]
#[serde(default)]
pub struct TableUI {
    show_prs: bool,
    show_success: bool,
}

impl TableUI {
    pub fn is_show_pr(&self) -> bool { self.show_prs }
    pub fn is_show_success(&self) -> bool { self.show_success }

    pub fn toggle_success(&self) -> Self {
        TableUI {
            show_success: !self.show_success,
            ..self.clone()
        }
    }

    pub fn toggle_prs(&self) -> Self {
        TableUI {
            show_prs: !self.show_prs,
            ..self.clone()
        }
    }
}
