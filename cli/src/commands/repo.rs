use std::path::PathBuf;

use script_herder_core::{config::AppConfig, git::repo::Repo};

pub fn run_repo_info(config: AppConfig) {
    let repo_path = match config.get_repo_path() {
        Some(e) => e,
        None => {
            print!("Repository not found");
            return;
        }
    };

    let repo = match Repo::from_path(PathBuf::from(repo_path)) {
        Ok(repo) => repo,
        Err(e) => {
            println!("Error opening repo: {}", e);
            return;
        }
    };

    let info = match repo.get_info() {
        Ok(info) => info,
        Err(e) => {
            println!("Error getting repo info: {}", e);
            return;
        }
    };

    println!("Repo path: {}", info.path.to_str().unwrap());
    println!("Remote: {}", info.remote);
    println!("Remote URL: {}", info.remote_url);
    println!("User: {}", info.user);
    println!("Email: {}", info.email);
}