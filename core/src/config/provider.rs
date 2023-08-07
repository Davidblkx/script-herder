use std::error::Error;

use crate::infra::priority_provider::{PriorityProvider, PriorityProviderIterator};
use crate::config::Config;

pub struct ConfigProvider {
    providers: PriorityProvider<Config>,
}

impl ConfigProvider {
    pub fn new() -> ConfigProvider {
        ConfigProvider {
            providers: PriorityProvider::new(),
        }
    }

    pub fn register_top(&mut self, config: Config) {
        self.providers.add_top(config);
    }

    pub fn register_default(&mut self, config: Config) {
        self.providers.add(config);
    }

    pub fn iter(&self) -> PriorityProviderIterator<Config> {
        self.providers.iter()
    }

    pub fn get<T>(&self, key: &str) -> Option<T> 
    where T: std::str::FromStr + serde::de::DeserializeOwned {
        self.providers.map_first(&|p| p.get(key))
    }

    pub fn get_value(&self, key: &str) -> Option<serde_json::Value> {
        self.providers.map_first(&|p| p.get_value(key))
    }
    
    pub fn set<T>(&mut self, key: &str, value: T) 
    where T: serde::Serialize {
        self.providers.first_mut(&|p| match p {
            Config::Json(_) => true,
            _ => false,
        }).map(|p| match p {
            Config::Json(src) => src.set(key, value),
            _ => (),
        });
    }

    pub fn set_value(&mut self, key: &str, value: serde_json::Value) {
        self.providers.first_mut(&|p| match p {
            Config::Json(_) => true,
            _ => false,
        }).map(|p| match p {
            Config::Json(src) => src.set_value(key, value),
            _ => (),
        });
    }

    pub fn sync(&mut self) -> Vec<Result<(), Box<dyn Error>>> {
        self.providers.map_mut(&|p| match p {
            Config::Json(src) => {
                if !src.is_synced() && src.can_write() {
                    Some(src.save())
                } else {
                    None
                }
            }
            _ => None,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::config::config_json::ConfigJson;

    use super::*;

    #[test]
    fn get_return_first_match() {
        let config1 = ConfigJson::from_data(r#"{ "key": "value_1" }"#.to_string()).unwrap();
        let config2 = ConfigJson::from_data(r#"{ "key": "value_2" }"#.to_string()).unwrap();
        let config3 = ConfigJson::from_data(r#"{ "key": "value_3" }"#.to_string()).unwrap();

        let mut provider = ConfigProvider::new();
        provider.register_default(Config::Json(config1));
        provider.register_top(Config::Json(config3));
        provider.register_default(Config::Json(config2));

        let value = provider.get::<String>("key");
        assert_eq!(value, Some("value_3".to_string()));
    }

    #[test]
    fn get_skip_invalid_match() {
        let config1 = ConfigJson::from_data(r#"{ "key": "value_1" }"#.to_string()).unwrap();

        let mut provider = ConfigProvider::new();
        provider.register_default(Config::None);
        provider.register_default(Config::Json(config1));
        provider.register_default(Config::None);
        provider.register_top(Config::None);

        let value = provider.get::<String>("key");
        assert_eq!(value, Some("value_1".to_string()));
    }

    #[test]
    fn get_return_none_if_no_match() {
        let mut provider = ConfigProvider::new();
        provider.register_default(Config::None);
        provider.register_top(Config::None);

        let value = provider.get::<String>("key");
        assert_eq!(value, None);
    }

    #[test]
    fn set_save_to_first_match() {
        let config1 = ConfigJson::from_data(r#"{ "key": "value_1" }"#.to_string()).unwrap();
        let config2 = ConfigJson::from_data(r#"{ "key": "value_2" }"#.to_string()).unwrap();
        let config3 = ConfigJson::from_data(r#"{ "key": "value_3" }"#.to_string()).unwrap();

        let mut provider = ConfigProvider::new();
        provider.register_default(Config::Json(config1));
        provider.register_top(Config::Json(config3));
        provider.register_default(Config::Json(config2));

        provider.set("key", "value_4".to_string());
        let value = provider.get::<String>("key");
        assert_eq!(value, Some("value_4".to_string()));
    }
}