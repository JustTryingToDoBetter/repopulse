mod github;
mod models;
mod parser;
mod scoring; 

use anyhow::Result;
use clap::Parser;

use github::GithubClient;
use parser::parse_github_url;
use scoring::{calculate_health_score, classify_score};

#[derive(Parser)]
#[command(name = "repopulse")]
#[command(version = "0.1.0")]
#[command(about = "Analyse a github repository from the command line")]
struct Cli {
    #[arg(help = "Github repository url")]
    repo_url : String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    let repo_summary = parse_github_url(&args.repo_url)?;

    let github_client = GithubClient::new();

    let repo = github_client
        .fetch_repo(&repo_summary.owner, &repo_summary.repo)
        .await?;

    let score = calculate_health_score(&repo);
    let classification = classify_score(score);

    println!();
    println!("Repo: {}", repo.full_name);
    println!("Stars: {}", repo.stargazers_count);
    println!("Forks: {}", repo.forks_count);
    println!("Open Issues: {}", repo.open_issues_count);
    println!(
        "Primary Language: {}",
        repo.language.unwrap_or_else(|| "Unknown".to_string())
    );
    println!("Last Updated: {}", repo.updated_at);
    println!();
    println!("Health Score: {}/100", score);
    println!("Status: {}", classification);
    println!();

    Ok(())
}
