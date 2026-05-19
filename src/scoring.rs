use crate::models::GithubRepo;

pub fn calculate_health_score(repo: &GithubRepo) -> u32{
    let mut score = 50;

    if repo.stargazers_count > 0{
        score += 10;
    }

    if repo.forks_count > 0{
        score += 10;
    }

    if repo.language.is_some(){
        score += 10;
    }

    if repo.open_issues_count <= 5 {
        score += 10;
    }

    if !repo.updated_at.is_empty(){
        score += 10;
    }

    score.min(100)

}

pub fn classify_score(score: u32) -> &'static str {
    match score {
        90..=100 => "Excellent portfolio project",
        75..=89 => "Strong portfolio project",
        60..=74 => "Good MVP project",
        _ => "Needs more polish",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_scores_correctly() {
        assert_eq!(classify_score(95), "Excellent portfolio project");
        assert_eq!(classify_score(82), "Strong portfolio project");
        assert_eq!(classify_score(70), "Good MVP project");
        assert_eq!(classify_score(40), "Needs more polish");
    }
}
