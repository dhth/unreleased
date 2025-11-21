use std::{fmt::Display, path::PathBuf};

use super::repo::{RawRepo, Repo, RepoValidationError};
use anyhow::Context;
use clap::ValueEnum;
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
    let mut raw: RawConfig =
        toml::from_str(contents.as_ref()).context("couldn't deserialize TOML")?;

    if let Some(regex) = repo_filter {
        raw.repos.retain(|v| regex.is_match(&v.repo));
    }
    let config: Config = raw.try_into()?;

    Ok(config)
}

#[derive(Debug, Clone, Copy)]
pub struct StdoutConfig {
    pub plain_output: bool,
}

#[derive(Debug, Clone)]
pub struct HtmlConfig {
    pub output_path: PathBuf,
    pub title: String,
    pub template: Option<String>,
}

#[derive(Debug, Clone)]
pub enum OutputType {
    Stdout(StdoutConfig),
    Html(HtmlConfig),
}

#[derive(Debug, Clone)]
pub struct RunConfig {
    pub output_type: OutputType,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum OutputFormat {
    Stdout,
    Html,
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let format = match self {
            OutputFormat::Stdout => "stdout",
            OutputFormat::Html => "html",
        };

        write!(f, "{}", format)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::{assert_debug_snapshot, assert_snapshot, assert_yaml_snapshot};

    //-------------//
    //  SUCCESSES  //
    //-------------//

    #[test]
    fn parsing_correct_config_works() -> anyhow::Result<()> {
        // GIVEN
        let config_str = r#"
[[repos]]
repo = "owner/repo-a"

[[repos]]
repo = "owner/repo-b"
head_ref = "branch"

[[repos]]
repo = "owner/repo-c"
consider_prereleases = true
"#;
        // WHEN
        let result = parse_config(config_str, None)?;

        // THEN
        assert_yaml_snapshot!(result, @r"
        repos:
          - owner: owner
            repo: repo-a
            head_ref: main
            consider_prereleases: false
          - owner: owner
            repo: repo-b
            head_ref: branch
            consider_prereleases: false
          - owner: owner
            repo: repo-c
            head_ref: main
            consider_prereleases: true
        ");

        Ok(())
    }

    //------------//
    //  FAILURES  //
    //------------//

    #[test]
    fn parsing_invalid_toml_fails() {
        // GIVEN
        let config_str = r#"
[[repos]]
repo   "owner/repo-a"
"#;
        // WHEN
        let result = parse_config(config_str, None).expect_err("result should've been an error");

        // THEN
        assert_debug_snapshot!(result, @r#"
        Error {
            context: "couldn\'t deserialize TOML",
            source: Error {
                message: "key with no value, expected `=`",
                input: Some(
                    "\n[[repos]]\nrepo   \"owner/repo-a\"\n",
                ),
                keys: [],
                span: Some(
                    18..18,
                ),
            },
        }
        "#);
    }

    #[test]
    fn parsing_config_with_invalid_schema_fails() {
        // GIVEN
        let config_str = r#"
[[repos]]
repo = "owner/repo-a"
consider_prereleases = "yes"
"#;
        // WHEN
        let result = parse_config(config_str, None).expect_err("result should've been an error");

        // THEN
        assert_debug_snapshot!(result, @r#"
        Error {
            context: "couldn\'t deserialize TOML",
            source: Error {
                message: "invalid type: string \"yes\", expected a boolean",
                input: Some(
                    "\n[[repos]]\nrepo = \"owner/repo-a\"\nconsider_prereleases = \"yes\"\n",
                ),
                keys: [
                    "repos",
                    "consider_prereleases",
                ],
                span: Some(
                    56..61,
                ),
            },
        }
        "#);
    }

    #[test]
    fn parsing_config_with_invalid_data_fails() {
        // GIVEN
        let config_str = r#"
[[repos]]
repo = "owner/repo-a"

[[repos]]
repo = "ownerrepo-b"

[[repos]]
repo = "owner/"

[[repos]]
repo = "/repo"

[[repos]]
repo = ""
head_ref = ""
"#;
        // WHEN
        let result = parse_config(config_str, None).expect_err("result should've been an error");

        // THEN
        assert_snapshot!(result, @r#"
        config has errors:
         - repo #1 has errors:
           - repo needs to be in the format "owner/repo"
         - repo #2 has errors:
           - repo name is empty
         - repo #3 has errors:
           - owner is empty
         - repo #4 has errors:
           - repo needs to be in the format "owner/repo"
           - head_ref is empty
        "#);
    }
}
