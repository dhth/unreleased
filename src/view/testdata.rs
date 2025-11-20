use crate::domain::{Author, Commit, CommitDetail, CommitLog, Repo};
use chrono::{TimeZone, Utc};

pub(super) fn get_result_and_commit_logs() -> Vec<CommitLog> {
    let log1 = CommitLog {
        repo: Repo {
            owner: "owner".into(),
            repo: "app-one".into(),
            head_ref: "main".into(),
            consider_prereleases: true,
        },
        base_ref: "v1.0.0".into(),
        head_ref: "main".into(),
        commits: vec![Commit {
            sha: "ae7de14".to_string(),
            commit: CommitDetail {
                message: "First commit".to_string(),
                author: Author {
                    name: "User A".to_string(),
                    date: Utc.with_ymd_and_hms(2025, 1, 15, 10, 0, 0).unwrap(),
                },
            },
            html_url: "https://github.com/owner/app-one/commit/abc1234567890".to_string(),
        }],
        html_url: "https://github.com/owner/app-one/compare/1.0.0...1.1.0".to_string(),
    };

    let log2 = CommitLog {
        repo: Repo {
            owner: "owner".into(),
            repo: "app-two".into(),
            head_ref: "main".into(),
            consider_prereleases: true,
        },
        base_ref: "v2.0.0".into(),
        head_ref: "main".into(),
        commits: vec![
            Commit {
                sha: "1443d43".to_string(),
                commit: CommitDetail {
                    message: "add cli test for when no versions match app filter".to_string(),
                    author: Author {
                        name: "User A".to_string(),
                        date: Utc.with_ymd_and_hms(2025, 1, 16, 11, 30, 0).unwrap(),
                    },
                },
                html_url: "https://github.com/owner/app-two/commit/1443d43".to_string(),
            },
            Commit {
                sha: "c536d77".to_string(),
                commit: CommitDetail {
                    message: "allow filtering apps to run for (#3) commit".to_string(),
                    author: Author {
                        name: "User B".to_string(),
                        date: Utc.with_ymd_and_hms(2025, 1, 16, 11, 0, 0).unwrap(),
                    },
                },
                html_url: "https://github.com/owner/app-two/commit/c536d77".to_string(),
            },
            Commit {
                sha: "2ff3e97".to_string(),
                commit: CommitDetail {
                    message: "allow configuring table style (#2) commit".to_string(),
                    author: Author {
                        name: "User A".to_string(),
                        date: Utc.with_ymd_and_hms(2025, 1, 15, 10, 0, 0).unwrap(),
                    },
                },
                html_url: "https://github.com/owner/app-two/commit/2ff3e97".to_string(),
            },
        ],
        html_url: "https://github.com/owner/app-two/compare/2.0.0...2.1.0".to_string(),
    };

    vec![log1, log2]
}
