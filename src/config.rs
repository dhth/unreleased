use crate::domain::{Config, parse_config};
use anyhow::Context;
use etcetera::{BaseStrategy, choose_base_strategy};
use regex::Regex;
use std::path::{Path, PathBuf};

pub fn get_default_config_path() -> anyhow::Result<PathBuf> {
    let strategy = choose_base_strategy()
        .context("couldn't determine your machine's default config directory")?;
    let config_dir = strategy.config_dir().join("unreleased");
    std::fs::create_dir_all(&config_dir).with_context(|| {
        format!(
            "couldn't create unreleased's config directory: {}",
            config_dir.to_string_lossy()
        )
    })?;

    Ok(config_dir.join("unreleased.toml"))
}

pub fn get_from_file<P>(path: P, repo_filter: Option<&Regex>) -> anyhow::Result<Config>
where
    P: AsRef<Path>,
{
    let contents = std::fs::read_to_string(&path).context("couldn't read file")?;

    let config = parse_config(&contents, repo_filter)?;

    Ok(config)
}
