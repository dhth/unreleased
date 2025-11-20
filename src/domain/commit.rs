use chrono::{DateTime, Utc};
use serde::Deserialize;

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct Commit {
    pub sha: String,
    pub commit: CommitDetail,
    pub html_url: String,
}

#[derive(Debug, Deserialize)]
pub struct CommitDetail {
    pub message: String,
    pub author: Author,
}

#[derive(Debug, Deserialize)]
pub struct Author {
    pub name: String,
    pub date: DateTime<Utc>,
}
