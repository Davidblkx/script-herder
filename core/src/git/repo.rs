use std::path::PathBuf;
use git2;

use crate::{git::repo_info::RepoInfo, infra::error::CoreError};

pub struct Repo {
    pub path: PathBuf,
    repo: Option<git2::Repository>,
    remote: String,
}

impl Repo {
    pub fn new(path: PathBuf) -> Self {
        Repo {
            path,
            remote: String::from("origin"),
            repo: None,
        }
    }
    
    pub fn from_path(path: PathBuf) -> Result<Self, CoreError> {
        let mut repo = Repo::new(path);
        match repo.open() {
            Ok(_) => Ok(repo),
            Err(e) => Err(CoreError::for_app(e.message().to_string()))
        }
    }

    pub fn open(&mut self) -> Result<(), git2::Error> {
        self.repo = Some(git2::Repository::open(&self.path)?);
        Ok(())
    }

    pub fn get_repo(&self) -> Result<&git2::Repository, CoreError> {
        match &self.repo {
            Some(e) => Ok(e),
            None => Err(CoreError::for_app("Repository not open".to_string()))
        }
    }

    pub fn get_remote_url(&self) -> Result<String, git2::Error> {
        let repo = match &self.repo {
            Some(repo) => repo,
            None => return Err(git2::Error::from_str("Repository not open")),
        };

        let remote = repo.find_remote(&self.remote)?;
        
        match remote.url() {
            Some(url) => Ok(url.to_string()),
            None => Err(git2::Error::from_str("Remote URL not found")),
        }
    }

    pub fn get_info(&self) -> Result<RepoInfo, git2::Error> {
        let repo = match &self.repo {
            Some(repo) => repo,
            None => return Err(git2::Error::from_str("Repository not open")),
        };

        let remote_url = &self.get_remote_url()?;
        let user = repo.config()?.get_string("user.name")?;
        let email = repo.config()?.get_string("user.email")?;

        Ok(RepoInfo {
            path: self.path.clone(),
            remote: self.remote.clone(),
            remote_url: remote_url.clone(),
            user,
            email,
        })
    }

    pub fn fetch(&self, refs: &str) -> Result<(), CoreError> {
        let repo = self.get_repo()?;
        let mut remote = repo.find_remote(&self.remote)?;

        Ok(remote.fetch(&[refs], None, None)?)
    }
}