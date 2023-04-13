use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Environments {
    pub total_count: i32,
    pub environments: Vec<Environment>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Environment {
    pub url: String,
    pub id: i64,
    pub name: String,
    pub html_url: String,
}
