use std::path::PathBuf;

use crate::domain::OutputFormat;
use clap::{Parser, Subcommand};

const NOT_PROVIDED: &str = "<NOT PROVIDED>";

/// View the commits to your GitHub repos since their last release
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
    #[command(name = "report")]
    Report {
        /// Path to the unreleased's file (defaults to <YOUR_CONFIG_DIR>/unreleased/unreleased.toml)
        #[arg(long = "config-path", short = 'c', value_name = "PATH")]
        config_file_path: Option<PathBuf>,
        /// Regex to use for filtering repos
        #[arg(long = "filter", short = 'f', value_name = "REGEX")]
        repo_filter: Option<String>,
        /// Output format
        #[arg(long = "output-format", short = 'o', default_value_t = OutputFormat::Stdout, value_name = "FORMAT")]
        output_format: OutputFormat,
        /// Whether to output text to stdout without color
        #[arg(long = "stdout-plain")]
        stdout_plain_output: bool,
        /// Path for the HTML output file
        #[arg(
            long = "html-output",
            value_name = "PATH",
            default_value = "unreleased.html"
        )]
        html_output_path: PathBuf,
        /// Title for HTML report
        #[arg(
            long = "html-title",
            value_name = "STRING",
            default_value = "unreleased"
        )]
        html_title: String,
        /// Path to custom HTML template file
        #[arg(long = "html-template", value_name = "PATH")]
        html_template_path: Option<PathBuf>,
    },
}

impl std::fmt::Display for Args {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match &self.command {
            UnreleasedCommand::Report {
                config_file_path,
                repo_filter,
                output_format,
                stdout_plain_output,
                html_output_path,
                html_title,
                html_template_path,
            } => {
                let flags_based_on_output = match output_format {
                    OutputFormat::Stdout => format!(
                        r#"
plain output:           {}
"#,
                        stdout_plain_output
                    ),
                    OutputFormat::Html => {
                        format!(
                            r#"
output path:            {}
title:                  {}
template path:          {}
"#,
                            html_output_path.to_string_lossy(),
                            html_title,
                            html_template_path
                                .as_ref()
                                .map(|p| p.to_string_lossy().to_string())
                                .unwrap_or(NOT_PROVIDED.to_string())
                        )
                    }
                };

                format!(
                    r#"
command:                report
config file path:       {}
repo filter:            {}
output format:          {}{}
"#,
                    config_file_path
                        .as_ref()
                        .map(|p| p.to_string_lossy().to_string())
                        .unwrap_or(NOT_PROVIDED.to_string()),
                    repo_filter.as_deref().unwrap_or(NOT_PROVIDED),
                    output_format,
                    flags_based_on_output
                )
            }
        };

        f.write_str(&output)
    }
}
