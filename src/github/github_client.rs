use ehttp::Request;

const HOST: &str = "https://api.github.com";

#[derive(Default)]
pub struct GitHubApi;

pub trait Fetch {
    fn fetch_path(&self, token: &str, path: &str, callback: impl 'static + Send + FnOnce(Vec<u8>));
    fn fetch_url(&self, token: &str, url: &str, callback: impl 'static + Send + FnOnce(Vec<u8>));
}

impl Fetch for GitHubApi {
    fn fetch_path(&self, token: &str, path: &str, callback: impl 'static + Send + FnOnce(Vec<u8>)) {
        self.fetch_url(token, &format!("{HOST}{path}"), callback);
    }

    fn fetch_url(&self, token: &str, url: &str, callback: impl 'static + Send + FnOnce(Vec<u8>)) {
        let request = Request {
            headers: ehttp::headers(&[
                ("Accept", "application/vnd.github+json"),
                ("User-Agent", "rust web-api-client demo"),
                ("Authorization", format!("Bearer {}", token.trim()).as_str()),
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
