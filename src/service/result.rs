use super::log::get_commit_log;
use super::release::get_latest_release;
use crate::domain::{CommitLog, Repo};
use anyhow::Context;

pub(super) async fn get_changelog_for_repo(
    repo: &Repo,
    token: &str,
) -> anyhow::Result<Option<CommitLog>> {
    let latest_release = get_latest_release(repo, repo.consider_prereleases, token)
        .await
        .context("couldn't get the latest release")?;

    let latest_release = match latest_release {
        Some(r) => r,
        None => return Ok(None),
    };

    let commit_log = get_commit_log(repo, &latest_release.tag_name, &repo.head_ref, token)
        .await
        .context("couldn't fetch commits")?;

    Ok(Some(commit_log))
}
