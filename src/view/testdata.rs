use crate::domain::{Author, Commit, CommitDetail, CommitLog, Repo};
use chrono::{TimeZone, Utc};

pub(super) fn get_test_commit_logs() -> Vec<CommitLog> {
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
            html_url: "https://github.com/owner/app-one/commit/ae7de14".to_string(),
        }],
        html_url: "https://github.com/owner/app-one/compare/v1.0.0...main".to_string(),
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
        html_url: "https://github.com/owner/app-two/compare/v2.0.0...main".to_string(),
    };

    vec![log1, log2]
}

pub(super) const TEST_HTML_TEMPLATE: &str = r#"<!DOCTYPE html>
<html>
<head>
  <title>{{ title }}</title>
</head>
<body>
  <h1>{{ title }}</h1>
  <p>Generated: {{ timestamp }}</p>

  {%- if commit_logs %}
  <h2>Commit Logs</h2>
  {%- for log in commit_logs %}
  <div>
    <h3>{{ log.repo }}</h3>
    <p>{{ log.base_ref }}..{{ log.head_ref }}</p>
    <p>Compare: <a href="{{ log.compare_url }}">{{ log.compare_url }}</a></p>
    <ul>
      {%- for commit in log.commits %}
      <li>
        <a href="{{ commit.html_url }}">{{ commit.short_sha }}</a>
        - {{ commit.message }}
        - {{ commit.author }}
        - {{ commit.date }}
      </li>
      {%- endfor %}
    </ul>
  </div>
  {%- endfor %}
  {%- endif %}
</body>
</html>
"#;
