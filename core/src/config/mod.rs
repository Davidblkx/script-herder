pub mod config_env;
pub mod config_json;
pub mod provider;

use serde_json::Value;
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;
use log::{debug, trace};

use crate::config::config_json::ConfigJson;
use crate::config::config_env::ConfigEnv;
use crate::enum_str;
use crate::infra::error::CoreError;

pub enum Config {
    Json(ConfigJson),
    Env(ConfigEnv),
    None,
}

pub enum ConfigTarget {
    Machine,
    Repo,
    Local,
    Env,
}

impl Config {
    pub fn get<T>(&self, key: &str) -> Option<T> 
        where T: std::str::FromStr + serde::de::DeserializeOwned {
        match self {
            Config::Json(src) => src.get(key),
            Config::Env(src) => src.get(key),
            Config::None => None,
        }
    }

    pub fn get_value(&self, key: &str) -> Option<Value> {
        match self {
            Config::Json(src) => src.get_value(key)
                .map(|v| v.clone()),
            Config::Env(src) => src.get_value(key)
                .map(|v| Value::String(v.to_string())),
            Config::None => None,
        }
    }

    pub fn set<T>(&mut self, key: &str, value: T) -> bool
    where T: serde::Serialize {
        match self {
            Config::Json(json) => {
                json.set(key, value);
                true
            },
            _ => false,
        }
    }

    pub fn set_value(&mut self, key: &str, value: Value) -> bool {
        match self {
            Config::Json(json) => {
                json.set_value(key, value);
                true
            },
            _ => false,
        }
    }

    pub fn save(&mut self) -> Result<(), Box<dyn Error>> {
        match self {
            Config::Json(json) => json.save(),
            _ => Ok(()),
        }
    }

    pub fn load(&mut self) -> Result<(), Box<dyn Error>> {
        match self {
            Config::Json(json) => json.load(),
            _ => Ok(()),
        }
    }
}

impl AsMut<Config> for Config {
    fn as_mut(&mut self) -> &mut Config {
        self
    }
}

pub struct AppConfig {
    pub provider: provider::ConfigProvider,
    root: PathBuf,
}

impl AppConfig {
    pub fn new(root: PathBuf) -> AppConfig {
        AppConfig {
            provider: provider::ConfigProvider::new(),
            root,
        }
    }

    pub fn use_env(&mut self) {
        debug!("Using environment variables");
        self.provider.register_top(Config::Env(ConfigEnv::new(true, Some("SH_".to_string()))));
    }

    pub fn from_json(json_path: PathBuf) -> Result<AppConfig, CoreError> {
        debug!("Loading config file: {:?}", json_path.canonicalize());
        let mut config = AppConfig::new(json_path.canonicalize()?);

        trace!("Creating config for machine: {:?}", json_path.canonicalize());
        let machine_config = AppConfig::create_machine_config(json_path)?;
        trace!("Creating config for current dir: {:?}", std::env::current_dir());
        let cwd_config = AppConfig::create_folder_config(std::env::current_dir()?, false)?;
        let repo_config = AppConfig::create_repo_config(&machine_config, &config.root)?;

        config.provider.register_default(Config::Json(machine_config));
        config.provider.register_default(cwd_config);
        config.provider.register_default(repo_config);

        Ok(config)
    }

    fn create_machine_config(path: PathBuf) -> Result<ConfigJson, CoreError> {
        AppConfig::ensure_file(&path)?;
        match ConfigJson::from_file(path) {
            Err(e) => Err(CoreError::for_err(e)),
            Ok(cfg) => Ok(cfg),
        }
    }

    fn create_repo_config(machine_config: &ConfigJson, root: &PathBuf) -> Result<Config, CoreError> {
        trace!("Reading repository path");
        let repo_var = machine_config.get::<String>(KnownConfigs::RepoPath.to_str());
        let repo_path = AppConfig::internal_get_repo_path(repo_var, root);
        match repo_path {
            Some(path) => {
                trace!("Repository path: {:?}", &path.canonicalize());
                return AppConfig::create_folder_config(path, true)
            },
            None => Ok(Config::None),
        }
    }

    fn create_folder_config(path: PathBuf, create: bool) -> Result<Config, CoreError> {
        if !path.exists() && !create {
            return Ok(Config::None);
        }

        AppConfig::ensure_dir(&path)?;
        let folder_config_path = path.join(".config-sh.json");
        if !folder_config_path.exists() && !create {
            return Ok(Config::None);
        }

        AppConfig::ensure_file(&folder_config_path)?;
        match ConfigJson::from_file(folder_config_path) {
            Err(e) => Err(CoreError::for_err(e)),
            Ok(cfg) => Ok(Config::Json(cfg)),
        }
    }

    fn ensure_file(path: &PathBuf) -> Result<(), CoreError> {
        trace!("Ensure file exists: {:?}", path.canonicalize());
        match path.try_exists() {
            Err(e) => return Err(e.into()),
            Ok(exists) => {
                if !exists {
                    trace!("Creating config file: {:?}", path.canonicalize());
                    let _ = match File::create(path) {
                        Err(e) => return Err(e.into()),
                        Ok(file) => file,
                    };
                    
                    match std::fs::write(path, "{}") {
                        Err(e) => return Err(e.into()),
                        Ok(_) => return Ok(()),
                    }
                }
            }
        }
        Ok(())
    }

    fn ensure_dir(path: &PathBuf) -> Result<(), CoreError> {
        trace!("Ensure dir exists: {:?}", path.canonicalize());
        match path.try_exists() {
            Err(e) => return Err(e.into()),
            Ok(exists) => {
                if !exists {
                    trace!("Creating config folder: {:?}", path.canonicalize());
                    match std::fs::create_dir_all(path) {
                        Err(e) => return Err(e.into()),
                        Ok(_) => return Ok(()),
                    }
                }
            }
        }
        Ok(())
    }

    fn internal_get_repo_path(path: Option<String>, root_file: &PathBuf) -> Option<PathBuf> {
        match path.map(|p| PathBuf::from(p)) {
            Some(path) => {
                if path.is_relative() {
                    let root = match root_file.parent() {
                        Some(e) => e,
                        None => &root_file
                    };
                    Some(root.join(path))
                } else {
                    Some(path)
                }
            },
            None => None,
        }
    }

    pub fn get<T>(&self, key: KnownConfigs) -> Option<T> 
        where T: std::str::FromStr + serde::de::DeserializeOwned {
        self.provider.get(key.to_str())
    }

    pub fn get_value(&self, key: KnownConfigs) -> Option<Value> {
        self.provider.get_value(key.to_str())
    }

    pub fn set<T>(&mut self, key: KnownConfigs, value: T)
    where T: serde::Serialize {
        self.provider.set(key.to_str(), value)
    }

    pub fn set_value(&mut self, key: KnownConfigs, value: Value) {
        self.provider.set_value(key.to_str(), value)
    }

    pub fn get_repo_path(&self) -> Option<PathBuf> {
        let path = self.get::<String>(KnownConfigs::RepoPath);
        AppConfig::internal_get_repo_path(path, &self.root)
    }
}

enum_str!(KnownConfigs {
    RepoPath = "core.repo.path",
    GitUser = "core.git.user",
    GitEmail = "core.git.email",
    LogLevel = "core.log.level",
});