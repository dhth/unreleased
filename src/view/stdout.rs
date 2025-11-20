use chrono::{DateTime, Utc};

use super::date::get_humanized_date;
use crate::domain::CommitLog;
use comfy_table::{Cell, Color as TableColor, Table, presets};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

const AUTHOR_COLOR_POOL: [TableColor; 6] = [
    TableColor::Blue,
    TableColor::Cyan,
    TableColor::DarkBlue,
    TableColor::DarkCyan,
    TableColor::Green,
    TableColor::Magenta,
];

const COMMIT_MESSAGE_MAX_LENGTH: usize = 80;

pub fn render_commit_logs(
    logs: &[CommitLog],
    reference_time: DateTime<Utc>,
    plain_output: bool,
) -> String {
    let mut output = String::new();

    for (i, log) in logs.iter().enumerate() {
        output.push_str(&format!(
            "{}/{} {}..{}\n\n",
            log.repo.owner, log.repo.repo, log.base_ref, log.head_ref
        ));

        if log.commits.is_empty() {
            output.push_str(" no commits\n\n");
            continue;
        }

        let mut table = Table::new();
        table.load_preset(presets::NOTHING);

        for commit in &log.commits {
            let short_sha = &commit.sha[..7.min(commit.sha.len())];
            let first_line = commit
                .commit
                .message
                .lines()
                .next()
                .unwrap_or(&commit.commit.message);

            let truncated_message = truncate_message(first_line, COMMIT_MESSAGE_MAX_LENGTH);
            let relative_time = get_humanized_date(&commit.commit.author.date, &reference_time);

            if plain_output {
                table.add_row(vec![
                    short_sha,
                    &truncated_message,
                    &commit.commit.author.name,
                    &relative_time,
                ]);
            } else {
                let author_color = get_author_color(&commit.commit.author.name);
                table.add_row(vec![
                    Cell::new(short_sha).fg(TableColor::Grey),
                    Cell::new(&truncated_message),
                    Cell::new(&commit.commit.author.name).fg(author_color),
                    Cell::new(&relative_time).fg(TableColor::Yellow),
                ]);
            }
        }

        output.push_str(&table.to_string());
        output.push('\n');

        if i < logs.len() - 1 {
            output.push('\n');
        }
    }

    output
}

fn get_author_color(author_name: &str) -> TableColor {
    let mut hasher = DefaultHasher::new();
    author_name.hash(&mut hasher);
    let hash = hasher.finish();

    let index = (hash % AUTHOR_COLOR_POOL.len() as u64) as usize;
    AUTHOR_COLOR_POOL[index]
}

fn truncate_message(message: &str, max_len: usize) -> String {
    if message.len() <= max_len {
        message.to_string()
    } else {
        format!("{}...", &message[..max_len - 3])
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::super::testdata::get_result_and_commit_logs;
    use super::*;
    use crate::domain::{Author, Commit, CommitDetail, Repo};
    use chrono::TimeZone;

    #[test]
    fn rendering_plain_commit_logs_works() {
        // GIVEN
        let reference = Utc.with_ymd_and_hms(2025, 1, 16, 12, 0, 0).unwrap();

        // WHEN
        let result = render_commit_logs(&get_result_and_commit_logs(), reference, true);

        // THEN
        insta::assert_snapshot!(result, @r"
        owner/app-one v1.0.0..main

         ae7de14  First commit  User A  1d ago 

        owner/app-two v2.0.0..main

         1443d43  add cli test for when no versions match app filter  User A  30m ago 
         c536d77  allow filtering apps to run for (#3) commit         User B  1h ago  
         2ff3e97  allow configuring table style (#2) commit           User A  1d ago
        ");
    }

    #[test]
    fn long_commit_messages_are_trimmed() {
        // GIVEN
        let reference = Utc.with_ymd_and_hms(2025, 1, 16, 12, 0, 0).unwrap();

        let log = CommitLog {
            repo: Repo {
            owner: "owner".into(),
            repo: "app-one".into(),
                head_ref: "main".into(),
                consider_prereleases: true,
        },
            base_ref: "v2.0.0".into(),
            head_ref: "main".into(),
            commits: vec![
                Commit {
                    sha: "1443d43".to_string(),
                    commit: CommitDetail {
                        message: "add cli test for when no application versions match app filter (this commit is very long for some reason)"
                            .to_string(),
                        author: Author {
                            name: "User A".to_string(),
                            date: Utc.with_ymd_and_hms(2025, 1, 16, 11, 30, 0).unwrap(),
                        },
                    },
                    html_url: "https://github.com/org/app-two/commit/1443d43".to_string(),
                },
            ],
            html_url: "https://github.com/org/app-two/compare/v2.0.0...main".to_string(),
        };

        // WHEN
        let result = render_commit_logs(&[log], reference, true);

        // THEN
        insta::assert_snapshot!(result, @r"
        owner/app-one v2.0.0..main

         1443d43  add cli test for when no application versions match app filter (this commit i...  User A  30m ago
        ");
    }

    #[test]
    fn get_author_color_returns_consistent_color_for_same_author() {
        // GIVEN
        let author = "Alan Turing";

        // WHEN
        let mut set = HashSet::new();
        for _ in 1..=100 {
            set.insert(get_author_color(author));
        }

        // THEN
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn get_author_color_returns_valid_color_from_pool() {
        // GIVEN
        let test_authors = [
            "Alan Turing",
            "Grace Hopper",
            "Donald Knuth",
            "Ada Lovelace",
            "Dennis Ritchie",
            "Ken Thompson",
            "Linus Torvalds",
            "",
        ];

        // WHEN
        // THEN
        for author in &test_authors {
            let color = get_author_color(author);
            assert!(
                AUTHOR_COLOR_POOL.contains(&color),
                "Color for author '{}' should be in the valid color pool",
                author
            );
        }
    }
}
