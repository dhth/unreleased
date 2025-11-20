use crate::domain::{Commit, CommitLog, Repo};
use anyhow::Context;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct CompareResponse {
    commits: Vec<Commit>,
    html_url: String,
}

pub(super) async fn get_commit_log(
    repo: &Repo,
    base_ref: &str,
    head_ref: &str,
    token: &str,
) -> anyhow::Result<CommitLog> {
    let client = reqwest::Client::builder()
        .build()
        .context("failed to build HTTP client")?;

    let url = format!(
        "https://api.github.com/repos/{}/{}/compare/{}...{}",
        &repo.owner, &repo.repo, base_ref, head_ref
    );

    let response = client
        .get(&url)
        .header("Accept", "application/vnd.github+json")
        .header("Authorization", format!("Bearer {}", token))
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "unreleased@v0.1.0")
        .send()
        .await
        .context("failed to send request to GitHub API")?;

    let status = response.status();
    if !status.is_success() {
        match response.text().await {
            Ok(body) => anyhow::bail!("GitHub API request failed with status {}: {}", status, body),
            Err(_) => anyhow::bail!("GitHub API request failed with status {}", status,),
        }
    }

    let mut commits: CompareResponse = response
        .json()
        .await
        .context("failed to parse GitHub API response")?;

    commits.commits.reverse();

    Ok(CommitLog {
        repo: repo.clone(),
        base_ref: base_ref.to_string(),
        head_ref: head_ref.to_string(),
        commits: commits.commits,
        html_url: commits.html_url,
    })
}
