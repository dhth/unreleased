mod auth;
mod cli;
mod config;
mod domain;
mod service;
mod view;

use anyhow::Context;
use chrono::Utc;
use clap::Parser;
use regex::Regex;

use crate::domain::{HtmlConfig, OutputFormat, OutputType, RunConfig, StdoutConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = cli::Args::parse();

    if args.debug {
        print!("DEBUG INFO\n{args}");
        return Ok(());
    }

    match args.command {
        cli::UnreleasedCommand::Run {
            config_file_path,
            repo_filter,
            output_format,
            stdout_plain_output,
            html_output_path,
            html_title,
            html_template_path,
        } => {
            let repo_filter = repo_filter
                .map(|pattern| Regex::new(&pattern))
                .transpose()
                .context("invalid regex pattern provided")?;

            let run_config = RunConfig {
                output_type: match output_format {
                    OutputFormat::Stdout => OutputType::Stdout(StdoutConfig {
                        plain_output: stdout_plain_output,
                    }),
                    OutputFormat::Html => {
                        let template = if let Some(ref template_path) = html_template_path {
                            Some(std::fs::read_to_string(template_path).with_context(|| {
                                format!("failed to read HTML template from {:?}", template_path)
                            })?)
                        } else {
                            None
                        };

                        OutputType::Html(HtmlConfig {
                            output_path: html_output_path,
                            title: html_title,
                            template,
                        })
                    }
                },
            };

            let config_path = config_file_path.unwrap_or(config::get_default_config_path()?);
            let unreleased_config = config::get_from_file(&config_path, repo_filter.as_ref())
                .with_context(|| {
                    format!(
                        "couldn't get config from file \"{}\"",
                        config_path.to_string_lossy()
                    )
                })?;

            if unreleased_config.repos.is_empty() {
                anyhow::bail!("no repos match the provided filter");
            }

            let token = auth::get_token()?;

            let changelogs = service::get_changelogs(&unreleased_config.repos, &token).await;
            if !changelogs.errors.is_empty() {
                return Err(anyhow::anyhow!(changelogs.errors));
            }

            let output = view::render_output(changelogs.logs.as_slice(), &run_config, Utc::now())?;

            match &run_config.output_type {
                OutputType::Stdout(_) => {
                    println!("{}", output);
                }
                OutputType::Html(html_config) => {
                    if let Some(parent) = html_config.output_path.parent() {
                        std::fs::create_dir_all(parent)
                            .with_context(|| format!("failed to create directory {:?}", parent))?;
                    }
                    std::fs::write(&html_config.output_path, output).with_context(|| {
                        format!("failed to write HTML to {:?}", html_config.output_path)
                    })?;
                    println!(
                        "HTML report written to: {}",
                        html_config.output_path.display()
                    );
                }
            }
        }
    }

    Ok(())
}
