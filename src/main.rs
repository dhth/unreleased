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
        } => {
            let repo_filter = repo_filter
                .map(|pattern| Regex::new(&pattern))
                .transpose()
                .context("invalid regex pattern provided")?;

            let config_path = config_file_path.unwrap_or(config::get_default_config_path()?);
            let cfg = config::get_from_file(&config_path, repo_filter.as_ref())
                .context("couldn't get config")?;

            let token = auth::get_token()?;

            let changelogs = service::get_changelogs(&cfg.repos, &token).await;
            if !changelogs.errors.is_empty() {
                return Err(anyhow::anyhow!(changelogs.errors));
            }

            let output = view::render_commit_logs(changelogs.logs.as_slice(), Utc::now(), false);
            println!("{output}");
        }
    }

    Ok(())
}
