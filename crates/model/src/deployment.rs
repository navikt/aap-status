use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Deployment {
    pub url: String,
    pub id: i64,
    pub task: String,
    pub environment: String,
    pub created_at: String,
    pub updated_at: String,
    pub statuses_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Status {
    pub url: String,
    pub id: i64,
    pub state: State,
    description: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum State {
    Error,
    Failure,
    Inactive,
    Pending,
    Success,
    Queued,
    InProgress,
}

impl ToString for State {
    fn to_string(&self) -> String {
        format!("{:?}", self.clone())
    }
}

impl Status {
    pub fn description(&self) -> String {
        match self.state {
            State::Success => String::default(), // not iteresting on success
            _=> self.description.clone()
        }
    }
}
