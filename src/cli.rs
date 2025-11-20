use std::path::PathBuf;

use clap::{Parser, Subcommand};

const NOT_PROVIDED: &str = "<NOT PROVIDED>";

/// unreleased shows the commits to your GitHub repos since their last release
#[derive(Parser, Debug)]
pub struct Args {
    #[command(subcommand)]
    pub command: UnreleasedCommand,
    /// Output debug information without doing anything
    #[arg(long = "debug", global = true)]
    pub debug: bool,
}

#[derive(Subcommand, Debug)]
pub enum UnreleasedCommand {
    /// Show unreleased commits for repos
    #[command(name = "run")]
    Run {
        /// Path to the unreleased's file
        #[arg(long = "config-path", short = 'c', value_name = "PATH")]
        config_file_path: Option<PathBuf>,
        /// Regex to use for filtering repos
        #[arg(long = "filter", short = 'f', value_name = "REGEX")]
        repo_filter: Option<String>,
    },
}

impl std::fmt::Display for Args {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match &self.command {
            UnreleasedCommand::Run {
                config_file_path,
                repo_filter,
            } => format!(
                r#"
command:                Run
config file path:       {}
repo filter:            {:?}
"#,
                config_file_path
                    .as_ref()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or(NOT_PROVIDED.to_string()),
                repo_filter.as_deref().unwrap_or(NOT_PROVIDED),
            ),
        };

        f.write_str(&output)
    }
}
