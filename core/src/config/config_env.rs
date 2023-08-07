use std::env;

pub struct ConfigEnv {
    use_config: bool,
    prefix: Option<String>
}

impl ConfigEnv {
    pub fn new(use_config: bool, prefix: Option<String>) -> ConfigEnv {
        ConfigEnv {
            use_config,
            prefix
        }
    }

    pub fn get_prefix(&self) -> Option<String> {
        self.prefix.clone()
    }

    pub fn get_key(&self, key: &str) -> String {
        match &self.prefix {
            Some(prefix) => format!("{}_{}", prefix, key),
            None => key.to_string()
        }
    }

    pub fn get_value(&self, key: &str) -> Option<String> {
        let actual_key = self.get_key(key);
        if self.use_config {
            match env::var(actual_key) {
                Ok(val) => Some(val),
                Err(_) => None
            }
        } else {
            None
        }
    }

    pub fn get<T: std::str::FromStr>(&self, key: &str) -> Option<T> {
        match self.get_value(key) {
            Some(value) => {
                match value.parse::<T>() {
                    Ok(value) => Some(value),
                    Err(_) => None,
                }
            },
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_key_return_key_with_prefix() {
        let src = ConfigEnv::new(true, Some("prefix".to_string()));
        let key = src.get_key("key");
        assert_eq!(key, "prefix_key");
    }

    #[test]
    fn get_key_return_key_without_prefix() {
        let src = ConfigEnv::new(true, None);
        let key = src.get_key("key");
        assert_eq!(key, "key");
    }

    #[test]
    fn get_value_returns_none_if_use_config_is_false() {
        let src = ConfigEnv::new(false, None);
        let value = src.get_value("key");
        assert_eq!(value, None);
    }

    #[test]
    fn get_value_returns_none_if_key_is_not_found() {
        let src = ConfigEnv::new(true, None);
        let value = src.get_value("__key__");
        assert_eq!(value, None);
    }

    #[test]
    fn get_value_returns_value_if_key_is_found() {
        let src = ConfigEnv::new(true, None);
        env::set_var("__key__", "value");
        let value = src.get_value("__key__");
        assert_eq!(value, Some("value".to_string()));
        env::remove_var("__key__");
    }

    #[test]
    fn get_returns_none_if_value_is_none() {
        let src = ConfigEnv::new(true, None);
        let value = src.get::<String>("__key__");
        assert_eq!(value, None);
    }

    #[test]
    fn get_returns_none_if_value_cannot_be_parsed() {
        let src = ConfigEnv::new(true, None);
        env::set_var("__key__", "value");
        let value = src.get::<i32>("__key__");
        assert_eq!(value, None);
        env::remove_var("__key__");
    }

    #[test]
    fn get_returns_value_if_value_can_be_parsed() {
        let src = ConfigEnv::new(true, None);
        env::set_var("__key__", "1");
        let value = src.get::<i32>("__key__");
        assert_eq!(value, Some(1));
        env::remove_var("__key__");
    }
}