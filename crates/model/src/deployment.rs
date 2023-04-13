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
            _ => self.description.clone()
        }
    }
}

#[cfg(test)]
mod deployment {
    use crate::deployment::Deployment;

    #[test]
    fn deserialize() {
        let json = r#"{ "url": "https://api.github.com/repos/octocat/example/deployments/1", "id": 1, "node_id": "MDEwOkRlcGxveW1lbnQx", "sha": "a84d88e7554fc1fa21bcbc4efae3c782a70d2b9d", "ref": "topic-branch", "task": "deploy", "payload": {}, "original_environment": "staging", "environment": "production", "description": "Deploy request from hubot", "creator": { "login": "octocat", "id": 1, "node_id": "MDQ6VXNlcjE=", "avatar_url": "https://github.com/images/error/octocat_happy.gif", "gravatar_id": "", "url": "https://api.github.com/users/octocat", "html_url": "https://github.com/octocat", "followers_url": "https://api.github.com/users/octocat/followers", "following_url": "https://api.github.com/users/octocat/following{/other_user}", "gists_url": "https://api.github.com/users/octocat/gists{/gist_id}", "starred_url": "https://api.github.com/users/octocat/starred{/owner}{/repo}", "subscriptions_url": "https://api.github.com/users/octocat/subscriptions", "organizations_url": "https://api.github.com/users/octocat/orgs", "repos_url": "https://api.github.com/users/octocat/repos", "events_url": "https://api.github.com/users/octocat/events{/privacy}", "received_events_url": "https://api.github.com/users/octocat/received_events", "type": "User", "site_admin": false }, "created_at": "2012-07-20T01:19:13Z", "updated_at": "2012-07-20T01:19:13Z", "statuses_url": "https://api.github.com/repos/octocat/example/deployments/1/statuses", "repository_url": "https://api.github.com/repos/octocat/example", "transient_environment": false, "production_environment": true}"#;
        let deployment = serde_json::from_slice::<Deployment>(json.as_bytes()).unwrap();
        assert_eq!(deployment.url, "https://api.github.com/repos/octocat/example/deployments/1");
        assert_eq!(deployment.id, 1);
        assert_eq!(deployment.task, "deploy");
        assert_eq!(deployment.environment, "production");
        assert_eq!(deployment.created_at, "2012-07-20T01:19:13Z");
        assert_eq!(deployment.updated_at, "2012-07-20T01:19:13Z");
        assert_eq!(deployment.statuses_url, "https://api.github.com/repos/octocat/example/deployments/1/statuses");
    }

    #[test]
    fn serialize() {
        let deployment = Deployment {
            url: "https://api.github.com/repos/octocat/example/deployments/1".to_string(),
            id: 1,
            task: "deploy".to_string(),
            environment: "production".to_string(),
            created_at: "2012-07-20T01:19:13Z".to_string(),
            updated_at: "2012-07-20T01:19:13Z".to_string(),
            statuses_url: "https://api.github.com/repos/octocat/example/deployments/1/statuses".to_string(),
        };

        let json = serde_json::to_string(&deployment).unwrap();
        let expected = r#"{"url":"https://api.github.com/repos/octocat/example/deployments/1","id":1,"task":"deploy","environment":"production","created_at":"2012-07-20T01:19:13Z","updated_at":"2012-07-20T01:19:13Z","statuses_url":"https://api.github.com/repos/octocat/example/deployments/1/statuses"}"#;
        assert_eq!(json, expected.to_string())
    }
}