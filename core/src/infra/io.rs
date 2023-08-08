use std::error::Error;
use std::path::PathBuf;
use std::fs;

pub struct ConfigFile {
    pub path: PathBuf,
}

pub struct ConfigData {
    pub data: String,
}

pub trait ConfigIO {
    fn read(&self) -> Result<String, Box<dyn Error>>;
    fn write(&mut self, data: &str) -> Result<(), Box<dyn Error>>;
}

impl ConfigFile {
    pub fn new(path: PathBuf) -> ConfigFile {
        ConfigFile { path }
    }
}

impl ConfigIO for ConfigFile {
    fn read(&self) -> Result<String, Box<dyn Error>> {
        let content = fs::read_to_string(&self.path)?;
        Ok(content)
    }

    fn write(&mut self, data: &str) -> Result<(), Box<dyn Error>> {
        fs::write(&self.path, data)?;
        Ok(())
    }
}

impl ConfigData {
    pub fn new(data: String) -> ConfigData {
        ConfigData { data }
    }
}

impl ConfigIO for ConfigData {
    fn read(&self) -> Result<String, Box<dyn Error>> {
        Ok(self.data.clone())
    }

    fn write(&mut self, data: &str) -> Result<(), Box<dyn Error>> {
        self.data = data.to_string();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_data_has_init_data() {
        let data = ConfigData::new("test".to_string());
        assert_eq!(data.data, "test");
    }

    #[test]
    fn config_data_can_read() {
        let data = ConfigData::new("test".to_string());
        let result = data.read();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test");
    }

    #[test]
    fn config_data_can_write() {
        let mut data = ConfigData::new("test".to_string());
        let result = data.write("test2");
        assert!(result.is_ok());
        assert_eq!(data.data, "test2");
    }
}
