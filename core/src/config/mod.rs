pub mod config_env;
pub mod config_json;
pub mod provider;

use serde_json::Value;
use std::error::Error;

use crate::config::config_json::ConfigJson;
use crate::config::config_env::ConfigEnv;

pub enum Config {
    Json(ConfigJson),
    Env(ConfigEnv),
    None,
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
