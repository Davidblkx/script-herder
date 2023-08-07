use std::path::PathBuf;
use crate::infra::io::{ConfigIO, ConfigFile, ConfigData};
use serde_json::Value;

use std::error::Error;


pub struct ConfigJson {
    io: Box<dyn ConfigIO>,
    data: Option<Value>,
    can_write: bool,
    is_synced: bool,
}

impl ConfigJson {
    pub fn new(io: Box<dyn ConfigIO>, can_write: bool) -> ConfigJson {
        ConfigJson { io, data: None, can_write, is_synced: false }
    }

    pub fn from_file(path: PathBuf) -> Result<ConfigJson, Box<dyn Error>> {
        let io = Box::new(ConfigFile::new(path));
        let mut cfg = ConfigJson::new(io, true);
        cfg.load()?;
        Ok(cfg)
    }

    pub fn from_data(data: String) -> Result<ConfigJson, Box<dyn Error>> {
        let io = Box::new(ConfigData::new(data));
        let mut cfg = ConfigJson::new(io, false);
        cfg.load()?;
        cfg.is_synced = true;
        Ok(cfg)
    }

    pub fn is_loaded(&self) -> bool {
        self.data.is_some()
    }

    pub fn is_synced(&self) -> bool {
        self.is_synced
    }

    pub fn can_write(&self) -> bool {
        self.can_write
    }

    pub fn load(&mut self) -> Result<(), Box<dyn Error>> {
        let content = self.io.read()?;
        let data: Value = serde_json::from_str(&content)?;
        self.data = Some(data);
        Ok(())
    }

    pub fn get_value(&self, key: &str) -> Option<&Value> {
        match &self.data {
            Some(data) => match data.get(key) {
                Some(serde_json::Value::Null) => None,
                Some(e) => Some(e),
                None => None
            }
            None => None,
        }
    }

    pub fn set_value(&mut self, key: &str, value: Value) {
        match &mut self.data {
            Some(data) => {
                data[key] = value;
                self.is_synced = false;
            },
            None => {
                let mut data = serde_json::Map::new();
                data.insert(key.to_string(), value.clone());
                self.data = Some(Value::Object(data));
                self.is_synced = false;
            },
        }
    }

    pub fn save(&mut self) -> Result<(), Box<dyn Error>> {
        if !self.can_write {
            return Err("Cannot write to this source".into());
        }

        if self.data.is_none() {
            return Ok(())
        }

        let data = self.data.as_ref().unwrap();
        let content = serde_json::to_string_pretty(data)?;
        self.io.write(&content)?;
        self.is_synced = true;
        Ok(())
    }

    pub fn get<T: serde::de::DeserializeOwned>(&self, key: &str) -> Option<T> {
        match self.get_value(key) {
            Some(value) => {
                match serde_json::from_value(value.clone()) {
                    Ok(value) => Some(value),
                    Err(_) => None,
                }
            },
            None => None,
        }
    }

    pub fn set<T: serde::Serialize>(&mut self, key: &str, value: T) {
        let value = serde_json::to_value(value).unwrap();
        self.set_value(key, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_json_has_init_data() {
        let src = ConfigJson::from_data(r#"{ "key": "value" }"#.to_string()).unwrap();
        assert_eq!(src.is_loaded(), true);
        assert_eq!(src.get_value("key").unwrap(), "value");
    }

    #[test]
    fn load_fails_if_io_read_fails() {
        let io = Box::new(ConfigFile::new(PathBuf::from("")));
        let mut src = ConfigJson::new(io, true);
        assert!(src.load().is_err());
    }

    #[test]
    fn load_fails_if_json_parse_fails() {
        let io = Box::new(ConfigData::new("".to_string()));
        let mut src = ConfigJson::new(io, true);
        assert!(src.load().is_err());
    }

    #[test]
    fn get_value_returns_none_if_data_is_none() {
        let src = ConfigJson::from_data(r#"{}"#.to_string()).unwrap();
        assert_eq!(src.get_value("key"), None);
    }

    #[test]
    fn set_value_creates_data_if_none() {
        let mut src = ConfigJson::from_data(r#"{}"#.to_string()).unwrap();
        src.set_value("key", Value::String("value".to_string()));
        assert_eq!(src.get_value("key").unwrap(), "value");
    }

    #[test]
    fn set_value_updates_data_if_exists() {
        let mut src = ConfigJson::from_data(r#"{ "key": "value" }"#.to_string()).unwrap();
        src.set_value("key", Value::String("new_value".to_string()));
        assert_eq!(src.get_value("key").unwrap(), "new_value");
    }

    #[test]
    fn save_fails_if_cannot_write() {
        let mut src = ConfigJson::from_data(r#"{ "key": "value" }"#.to_string()).unwrap();
        src.can_write = false;
        assert!(src.save().is_err());
    }

    #[test]
    fn save_does_nothing_if_data_is_none() {
        let io = Box::new(ConfigData::new("".to_string()));
        let mut src = ConfigJson::new(io, true);
        assert!(src.save().is_ok());
    }

    #[test]
    fn save_writes_data_to_io() {
        let io = Box::new(ConfigData::new("".to_string()));
        let mut src = ConfigJson::new(io, true);
        src.set_value("key", Value::String("value".to_string()));
        assert!(src.save().is_ok());
        assert_eq!(src.io.read().unwrap(), "{\n  \"key\": \"value\"\n}");
    }

    #[test]
    fn get_return_none_if_null() {
        let src = ConfigJson::from_data(r#"{ "key": null }"#.to_string()).unwrap();
        assert_eq!(src.get_value("key"), None);
    }

    #[test]
    fn get_return_none_if_invalid_type() {
        let src = ConfigJson::from_data(r#"{ "key": "value" }"#.to_string()).unwrap();
        let v: Option<i32> = src.get("key");
        assert_eq!(v, None);
    }

    #[test]
    fn get_return_value() {
        let src = ConfigJson::from_data(r#"{ "key": 10 }"#.to_string()).unwrap();
        let v: i32 = src.get("key").unwrap();
        assert_eq!(v, 10);
    }
}