use ehttp::Request;

const HOST: &str = "https://api.github.com";

#[derive(serde::Deserialize, serde::Serialize, Default, Clone)]
pub struct GitHubApi {
    pub token: String,
    pub token_visible: bool,
}

impl GitHubApi {
    pub fn toggle_token_visibility(&mut self) {
        self.token_visible = !self.token_visible
    }

    pub fn update_token(&mut self, token: String) {
        self.token = token
    }
}

pub trait Fetch {
    fn fetch_path(&self, path: &str, callback: impl 'static + Send + FnOnce(Vec<u8>));
    fn fetch_url(&self, url: &str, callback: impl 'static + Send + FnOnce(Vec<u8>));
}

impl Fetch for GitHubApi {
    fn fetch_path(&self, path: &str, callback: impl 'static + Send + FnOnce(Vec<u8>)) {
        self.fetch_url(&format!("{HOST}{path}"), callback);
    }

    fn fetch_url(&self, url: &str, callback: impl 'static + Send + FnOnce(Vec<u8>)) {
        let token = self.token.trim();
        println!("Fetch {} with token {}", &url, &token);
        let request = Request {
            headers: ehttp::headers(&[
                ("Accept", "application/vnd.github+json"),
                ("User-Agent", "rust, ehttp::fetch"),
                ("Authorization", format!("Bearer {}", token).as_str()),
            ]),
            ..Request::get(url)
        };

        ehttp::fetch(request, |response| {
            match response {
                Ok(response) => callback(response.bytes),
                Err(e) => eprintln!("error fetching: {}", e),
            }
        });
    }
}
