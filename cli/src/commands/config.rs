use script_herder_core::config::AppConfig;

pub fn run_config(mut config: AppConfig, key: String, value: Option<String>, list_known: bool) {
    if list_known {
        return;
    }

    match value {
        Some(val) => {
            config.provider.set(&key, val);
            let result = config.provider.sync();
            for r in result {
                match r {
                    Ok(_) => continue,
                    Err(e) => println!("Error: {}", e)
                }
            }
        },
        None => {
            match config.provider.get_value(&key) {
                Some(val) => println!("{}", val),
                None => println!("No value found for key: {}", key)
            }
        }
    }
}
