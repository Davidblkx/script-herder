use script_herder_core::config::AppConfig;

pub fn run_repo_info(config: AppConfig) {
    let repo = match config.get_repo() {
        Ok(repo) => repo,
        Err(e) => {
            println!("Error getting repo: {}", e);
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