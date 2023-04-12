use ehttp::Request;
use serde::Deserialize;

pub mod github_models;

pub(crate) fn fetch_lifetime<T>(
    token: &str,
    url: &str,
    closure: impl Send + FnOnce(Result<T, String>) + 'static,
) where for<'a> T: Deserialize<'a> {
    let request = Request::github(token, url);

    ehttp::fetch(request, |response| {
        match response {
            Ok(response) => {
                closure(
                    serde_json::from_slice::<T>(&response.bytes)
                        .map_err(|e| format!("Deserializing from slice failed: {}", e))
                );
            }
            Err(e) => closure(Err(format!("ehttp fetch failed: {}", e))),
        }
    })
}

pub const HOST: &str = "https://api.github.com";

trait GitHubRequest {
    fn github(token: &str, url: &str) -> Request;
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
