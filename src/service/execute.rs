use super::result::get_changelog_for_repo;
use crate::domain::{CommitLogFetchErrors, CommitLogResults, Repo};
use futures::stream::{FuturesUnordered, StreamExt};
use std::sync::Arc;
use tokio::sync::Semaphore;

const MAX_CONCURRENT_FETCHES: usize = 20;

pub async fn get_changelogs(repos: &[Repo], token: &str) -> CommitLogResults {
    let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT_FETCHES));
    let mut futures = FuturesUnordered::new();

    for repo in repos {
        let semaphore = Arc::clone(&semaphore);
        let repo_clone = repo.clone();
        let token_clone = token.to_string();

        futures.push(tokio::task::spawn(async move {
            let permit = semaphore.acquire().await;
            if let Err(e) = permit {
                return (
                    repo_clone,
                    Err(anyhow::anyhow!("couldn't acquire semaphore: {e}")),
                );
            }

            let result = get_changelog_for_repo(&repo_clone, &token_clone).await;

            (repo_clone, result)
        }));
    }

    let mut commit_logs = Vec::new();
    let mut errors = CommitLogFetchErrors::new();

    while let Some(task_result) = futures.next().await {
        match task_result {
            Ok((_app, Ok(Some(log)))) => commit_logs.push(log),
            Ok((app, Err(e))) => {
                errors.add_repo_error(app, e);
            }
            Err(e) => {
                errors.add_system_error(anyhow::anyhow!("task panicked: {e}"));
            }
            _ => {}
        }
    }

    commit_logs.sort_by(|a, b| a.repo.cmp(&b.repo));

    CommitLogResults {
        logs: commit_logs,
        errors,
    }
}
