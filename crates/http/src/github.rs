use ehttp::Request;
use serde::Deserialize;

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

pub fn get_path<T>(
    token: &str,
    path: &str,
    closure: impl Send + FnOnce(Result<T, String>) + 'static,
) where for<'a> T: Deserialize<'a> {
    get(token, &format!("https://api.github.com{path}"), closure)
}

pub fn get<T>(
    token: &str,
    url: &str,
    closure: impl Send + FnOnce(Result<T, String>) + 'static,
) where for<'a> T: Deserialize<'a> {
    println!("Fetching {}", &url);

    ehttp::fetch(Request::github(token, url), |response| {
        match response {
            Err(e) => closure(Err(format!("ehttp fetch failed: {}", e))),
            Ok(response) => closure(
                serde_json::from_slice::<T>(&response.bytes).map_err(|e| format!(
                    "Deserializing from slice failed:{}. Status:{}, Status Text:{}",
                    e,
                    &response.status,
                    &response.status_text,
                ))
            )
        }
    })
}
