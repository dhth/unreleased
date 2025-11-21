use serde::Deserialize;
use std::fmt::{self, Display};

const DEFAULT_HEAD_REF: &str = "main";

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(serde::Serialize))]
pub struct RawRepo {
    pub repo: String,
    pub head_ref: Option<String>,
    pub consider_prereleases: Option<bool>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
#[cfg_attr(test, derive(serde::Serialize))]
pub struct Repo {
    pub owner: String,
    pub repo: String,
    pub head_ref: String,
    pub consider_prereleases: bool,
}

impl Ord for Repo {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.owner.as_str(), self.repo.as_str()).cmp(&(other.owner.as_str(), other.repo.as_str()))
    }
}

impl PartialOrd for Repo {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for Repo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.owner, self.repo)
    }
}

#[derive(Debug)]
pub struct RepoValidationError {
    errors: Vec<&'static str>,
}

impl RepoValidationError {
    fn new() -> Self {
        Self { errors: Vec::new() }
    }

    fn add_error(&mut self, message: &'static str) {
        self.errors.push(message);
    }
}

impl std::fmt::Display for RepoValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for error in &self.errors {
            writeln!(f, "   - {}", error)?;
        }
        Ok(())
    }
}

impl TryFrom<RawRepo> for Repo {
    type Error = RepoValidationError;

    fn try_from(value: RawRepo) -> Result<Self, Self::Error> {
        let mut errors = RepoValidationError::new();

        let (maybe_owner, maybe_repo) = match value.repo.trim().split_once("/") {
            Some(("", "")) => {
                errors.add_error("repo is empty");
                (None, None)
            }
            Some((_, "")) => {
                errors.add_error("repo name is empty");
                (None, None)
            }
            Some(("", _)) => {
                errors.add_error("owner is empty");
                (None, None)
            }
            Some((owner, repo)) => (Some(owner), Some(repo)),
            None => {
                errors.add_error(r#"repo needs to be in the format "owner/repo""#);
                (None, None)
            }
        };

        let maybe_head_ref = match value.head_ref.as_deref().map(|r| r.trim()) {
            Some("") => {
                errors.add_error("head_ref is empty");
                None
            }
            Some(r) => Some(r),
            None => Some(DEFAULT_HEAD_REF),
        };

        match (maybe_owner, maybe_repo, maybe_head_ref) {
            (Some(owner), Some(repo), Some(head_ref)) => Ok(Repo {
                owner: owner.to_string(),
                repo: repo.to_string(),
                head_ref: head_ref.to_string(),
                consider_prereleases: value.consider_prereleases.unwrap_or(false),
            }),
            _ => Err(errors),
        }
    }
}
