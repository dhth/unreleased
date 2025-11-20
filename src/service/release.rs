use crate::domain::{LastRelease, Repo};
use anyhow::Context;

pub(super) async fn get_latest_release(
    repo: &Repo,
    consider_prereleases: bool,
    token: &str,
) -> anyhow::Result<Option<LastRelease>> {
    let client = reqwest::Client::builder()
        .build()
        .context("failed to build HTTP client")?;

    let url = format!(
        "https://api.github.com/repos/{}/{}/releases",
        &repo.owner, &repo.repo
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

    let releases: Vec<LastRelease> = response
        .json()
        .await
        .context("failed to parse GitHub API response")?;

    for release in releases {
        if !consider_prereleases && release.prerelease {
            continue;
        }

        return Ok(Some(release));
    }

    Ok(None)
}
