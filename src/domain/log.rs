use super::{Commit, Repo};

#[derive(Debug)]
pub struct CommitLog {
    pub repo: Repo,
    pub base_ref: String,
    pub head_ref: String,
    pub commits: Vec<Commit>,
    pub html_url: String,
}

#[derive(Debug)]
pub struct CommitLogResults {
    pub logs: Vec<CommitLog>,
    pub errors: CommitLogFetchErrors,
}

#[derive(Debug)]
pub enum CommitLogFetchError {
    Repo { app: Repo, error: anyhow::Error },
    System { error: anyhow::Error },
}

#[derive(Debug)]
pub struct CommitLogFetchErrors {
    errors: Vec<CommitLogFetchError>,
}

impl CommitLogFetchErrors {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn add_repo_error(&mut self, app: Repo, error: anyhow::Error) {
        self.errors.push(CommitLogFetchError::Repo { app, error });
    }

    pub fn add_system_error(&mut self, error: anyhow::Error) {
        self.errors.push(CommitLogFetchError::System { error });
    }

    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }
}

impl std::fmt::Display for CommitLogFetchErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "couldn't fetch commit logs for some repos:")?;
        for error in &self.errors {
            match error {
                CommitLogFetchError::Repo { app, error } => {
                    writeln!(f, " - {}: {}", app, error)?;
                }
                CommitLogFetchError::System { error } => {
                    writeln!(f, " - system error: {}", error)?;
                }
            }
        }
        Ok(())
    }
}

impl std::error::Error for CommitLogFetchErrors {}
