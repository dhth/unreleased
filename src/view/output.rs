use crate::domain::{CommitLog, OutputType, RunConfig};
use chrono::{DateTime, Utc};

pub fn render_output(
    commit_logs: &[CommitLog],
    config: &RunConfig,
    reference_time: DateTime<Utc>,
) -> anyhow::Result<String> {
    let output = match &config.output_type {
        OutputType::Stdout(config) => {
            super::stdout::render_output(commit_logs, config.plain_output, reference_time)
        }
        OutputType::Html(config) => super::html::render_output(
            commit_logs,
            config.template.as_deref(),
            &config.title,
            reference_time,
        )?,
    };

    Ok(output)
}
