use anyhow::{anyhow, Result};

use crate::models::RepoSummary;

pub fn parse_github_url(url: &str) -> Result<RepoSummary> {
    let cleaned = url.trim().trim_end_matches('/');

    let parts: Vec<&str> = cleaned.split('/').collect();

    if parts.len() < 2 {
        return Err(anyhow!("Invalid Github URL"));
    }

    let repo = parts
        .last()
        .ok_or_else(|| anyhow!("Missing Repo Name"))?
        .to_string();
    
    let owner = parts
        .get(parts.len() - 2)
        .ok_or_else(|| anyhow!("Missing Repo Owner"))?
        .to_string();

    if owner.is_empty() || repo.is_empty() {
        return Err(anyhow!("Invalid Github URL"));
    }

    Ok(RepoSummary {owner, repo})
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_github_url(){
        let result = parse_github_url("https://github.com/rust-lang/rust").unwrap();

        assert_eq!(result.owner, "rust-lang");
        assert_eq!(result.repo, "rust");
    }

    #[test]
    fn parses_url_with_trailing_slash(){
        let result = parse_github_url("https://github.com/rust-lang/rust").unwrap();

        assert_eq!(result.owner, "rust-lang");
        assert_eq!(result.repo, "rust");
    }
}