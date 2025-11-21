use crate::domain::CommitLog;
use anyhow::Context;
use chrono::{DateTime, Utc};
use serde::Serialize;
use tera::Tera;

const BUILT_IN_TEMPLATE: &str = include_str!("assets/template.html");

#[derive(Serialize)]
struct HtmlData {
    title: String,
    timestamp: String,
    commit_logs: Vec<HtmlCommitLog>,
}

#[derive(Serialize)]
struct HtmlCommitLog {
    repo: String,
    base_ref: String,
    head_ref: String,
    compare_url: Option<String>,
    commits: Vec<HtmlCommit>,
}

#[derive(Serialize)]
struct HtmlCommit {
    short_sha: String,
    html_url: String,
    message: String,
    author: String,
    date: String,
}

pub(super) fn render_output(
    commit_logs: &[CommitLog],
    custom_template: Option<&str>,
    title: &str,
    now: DateTime<Utc>,
) -> anyhow::Result<String> {
    let mut tera = Tera::default();

    match custom_template {
        Some(template) => tera
            .add_raw_template("template.html", template)
            .context("failed to parse HTML template")?,
        None => tera
            .add_raw_template("template.html", BUILT_IN_TEMPLATE)
            .context("failed to parse built-in HTML template")?,
    }

    let html_data = build_html_data(commit_logs, title, now);

    let mut context = tera::Context::new();
    context.insert("title", &html_data.title);
    context.insert("timestamp", &html_data.timestamp);
    context.insert("commit_logs", &html_data.commit_logs);

    tera.render("template.html", &context)
        .context("failed to render HTML template")
}

fn build_html_data(commit_logs: &[CommitLog], title: &str, now: DateTime<Utc>) -> HtmlData {
    let html_commit_logs: Vec<HtmlCommitLog> = commit_logs
        .iter()
        .map(|log| {
            let commits: Vec<HtmlCommit> = log
                .commits
                .iter()
                .map(|commit| {
                    let short_sha = commit.sha.chars().take(7).collect::<String>();
                    let html_url = commit.html_url.clone();
                    let message = commit
                        .commit
                        .message
                        .lines()
                        .next()
                        .unwrap_or(&commit.commit.message)
                        .to_string();
                    let author = commit.commit.author.name.clone();
                    let date = commit.commit.author.date.format("%b %e, %Y").to_string();

                    HtmlCommit {
                        short_sha,
                        html_url,
                        message,
                        author,
                        date,
                    }
                })
                .collect();

            let compare_url = if !commits.is_empty() {
                Some(log.html_url.clone())
            } else {
                None
            };

            HtmlCommitLog {
                repo: log.repo.full_name(),
                base_ref: log.base_ref.to_string(),
                head_ref: log.head_ref.to_string(),
                compare_url,
                commits,
            }
        })
        .collect();

    HtmlData {
        title: title.to_string(),
        timestamp: now.format("%Y-%m-%dT%H:%M:%SZ").to_string(),
        commit_logs: html_commit_logs,
    }
}

#[cfg(test)]
mod tests {
    use super::super::testdata::{TEST_HTML_TEMPLATE, get_test_commit_logs};
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn built_in_html_template_is_rendered_correctly() {
        // GIVEN
        let commit_logs = get_test_commit_logs();
        let now = Utc.with_ymd_and_hms(2025, 1, 16, 12, 0, 0).unwrap();

        // WHEN
        let html =
            render_output(&commit_logs, None, "versions", now).expect("result should've been Ok");

        // THEN
        insta::assert_snapshot!(html);
    }

    #[test]
    fn custom_html_template_is_rendered_correctly() {
        // GIVEN
        let commit_logs = get_test_commit_logs();
        let now = Utc.with_ymd_and_hms(2025, 1, 16, 12, 0, 0).unwrap();

        // WHEN
        let html = render_output(&commit_logs, Some(TEST_HTML_TEMPLATE), "versions", now)
            .expect("result should've been Ok");

        // THEN
        insta::assert_snapshot!(html);
    }
}
