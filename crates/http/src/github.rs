use ehttp::Request;
use serde::{Deserialize, Serialize};

trait GitHubRequest {
    fn github(token: &str, url: &str) -> Request;
}

#[derive(Deserialize, Serialize, Clone)] 
pub struct Client {
    rate_limit: usize,
    rate_limit_reset_in_hours: u64,
}

impl Default for Client {
    fn default() -> Self {
        Client {
            rate_limit: 5000,
            rate_limit_reset_in_hours: 24,
        }
    }
}

impl Client {
    pub fn set_rate_limit(&mut self, remaining: usize) {
        self.rate_limit = remaining;
    }

    pub fn set_reset_epoch(&mut self, reset_epoch: u64) {
        let remaining = std::time::UNIX_EPOCH - std::time::Duration::from_secs(reset_epoch);
        self.rate_limit_reset_in_hours = remaining.elapsed().unwrap().as_secs() / 3600;
    }

    pub fn get_rate_limit(&self) -> usize {
        self.rate_limit
    }

    pub fn get_rate_reset(&self) -> u64 {
        self.rate_limit_reset_in_hours
    }

    pub fn get_path(
        &mut self,
        token: &str,
        path: &str,
        closure: impl Send + FnOnce(Result<ehttp::Response, String>) + 'static,
    ) {
        self.get(token, &format!("https://api.github.com{path}"), closure)
    }

    pub fn get(
        &mut self,
        token: &str,
        url: &str,
        closure: impl Send + FnOnce(Result<ehttp::Response, String>) + 'static,
    ) {
        println!("Fetching {}", &url);

        ehttp::fetch(Request::github(token, url), |response| {
            match response {
                Err(e) => closure(Err(format!("ehttp fetch failed: {}", e))),
                Ok(response) => {
                    closure(
                        Ok(response)
                        /*
                        serde_json::from_slice::<T>(&response.bytes).map_err(|e| format!(
                            "Deserializing from slice failed:{}. Status:{}, Status Text:{}",
                            e,
                            &response.status,
                            &response.status_text,
                        ))
                        */
                    )
                }
            }
        })
    }
}

impl GitHubRequest for Request {
    fn github(token: &str, url: &str) -> Request {
        Request {
            headers: ehttp::headers(&[
                ("Accept", "application/vnd.github+json"),
                ("User-Agent", "rust, ehttp::fetch"),
                ("Authorization", format!("Bearer {}", token.trim()).as_str()),
            ]),
            ..Request::get(url)
        }
    }
}


#[cfg(test)]
mod request {
    use ehttp::Request;
    use crate::github::GitHubRequest;

    #[test]
    fn method() {
        let request = Request::github("secret.token", "some.url");
        assert_eq!(request.method, "GET");
    }

    #[test]
    fn url() {
        let request = Request::github("secret.token", "some.url");
        assert_eq!(request.url, "some.url");
    }

    #[test]
    fn authentication_header() {
        let request = Request::github("secret.token", "some.url");
        assert_eq!(request.headers.get("Authorization").unwrap(), "Bearer secret.token");
    }

    #[test]
    fn user_agent_header() {
        let request = Request::github("secret.token", "some.url");
        assert_eq!(request.headers.get("User-Agent").unwrap(), "rust, ehttp::fetch");
    }

    #[test]
    fn accept_header() {
        let request = Request::github("secret.token", "some.url");
        assert_eq!(request.headers.get("Accept").unwrap(), "application/vnd.github+json");
    }

    #[test]
    fn body() {
        let request = Request::github("secret.token", "some.url");
        assert_eq!(request.body, Vec::<u8>::new());
    }
}
