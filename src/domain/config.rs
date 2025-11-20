use super::repo::{RawRepo, Repo, RepoValidationError};
use regex::Regex;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(serde::Serialize))]
struct RawConfig {
    pub repos: Vec<RawRepo>,
}

#[derive(Debug)]
#[cfg_attr(test, derive(serde::Serialize))]
pub struct Config {
    pub repos: Vec<Repo>,
}

#[derive(Debug)]
pub struct ConfigValidationErrors {
    repo_errors: Vec<(usize, RepoValidationError)>,
}

impl ConfigValidationErrors {
    pub fn new() -> Self {
        Self {
            repo_errors: Vec::new(),
        }
    }

    fn add_repo_error(&mut self, version_index: usize, error: RepoValidationError) {
        self.repo_errors.push((version_index, error));
    }

    fn is_empty(&self) -> bool {
        self.repo_errors.is_empty()
    }
}

impl std::error::Error for ConfigValidationErrors {}

impl TryFrom<RawConfig> for Config {
    type Error = ConfigValidationErrors;

    fn try_from(value: RawConfig) -> Result<Self, Self::Error> {
        let mut repos = vec![];
        let mut errors = ConfigValidationErrors::new();

        for (i, raw_repo) in value.repos.into_iter().enumerate() {
            match Repo::try_from(raw_repo) {
                Ok(r) => repos.push(r),
                Err(e) => errors.add_repo_error(i, e),
            }
        }

        if errors.is_empty() {
            Ok(Self { repos })
        } else {
            Err(errors)
        }
    }
}

impl std::fmt::Display for ConfigValidationErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "config has errors:")?;

        for (index, error) in &self.repo_errors {
            writeln!(f, " - repo #{} has errors:", index)?;
            write!(f, "{}", error)?;
        }

        Ok(())
    }
}

pub fn parse_config<S>(contents: S, repo_filter: Option<&Regex>) -> anyhow::Result<Config>
where
    S: AsRef<str>,
{
    let mut raw: RawConfig = toml::from_str(contents.as_ref())?;

    if let Some(regex) = repo_filter {
        raw.repos.retain(|v| regex.is_match(&v.repo));

        if raw.repos.is_empty() {
            anyhow::bail!("no repos match the provided filter");
        }
    }
    let config: Config = raw.try_into()?;

    Ok(config)
}
