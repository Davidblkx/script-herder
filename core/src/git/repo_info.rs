use std::path::PathBuf;

pub struct RepoInfo {
    pub path: PathBuf,
    pub remote: String,
    pub remote_url: String,
    pub user: String,
    pub email: String,
}