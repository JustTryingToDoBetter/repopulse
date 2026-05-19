use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GithubRepo{
    pub full_name: String, 
    pub stargazers_count: u32,
    pub forks_count: u32, 
    pub open_issues_count: u32,
    pub language: Options<String>,
    pub updated_at: String, 
}

#[derive(Debug)]
pub struct RepoSummary{
    pub owner: String, 
    pub repo: String, 
}