use anyhow::{anyhow, Result};
use reqwest::Client;

pub struct GithubClient {
    client: Client,
    base_url: String,
}

impl GithubClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "https://api.github.com".to_string(),
        }
    }

    pub fn with_base_url(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }

    pub async fn fetch_repo(&self, owner: &str, repo: &str) -> Result<GithubRepo> {
        let url = format!("{}/repos/{}/{}", self.base_url, owner, repo);

        let response = self
            .client
            .get(url)
            .header("User-Agent", "repopulse-cli")
            .send()
            .await?;
        
        if !response.status.is_success(){
            return Err(anyhow!(
                "Github API request failed with status: {}",
                response.status()
            ));
        }

        let repo = response.json::<GithubRepo>().await();

        Ok(Repo)
    }
}

