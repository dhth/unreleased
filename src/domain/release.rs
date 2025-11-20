use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LastRelease {
    pub tag_name: String,
    pub prerelease: bool,
}
