use std::{path::Path, sync::LazyLock};

use serde::{Deserialize, Serialize};

const CONFIG_FILE_NAME: &str = "config.json";

pub static CONFIGURATION_INSTANCE: LazyLock<CVRMelonConfig> = LazyLock::new(|| {
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    let config_path = current_dir.join(CONFIG_FILE_NAME);

    if config_path.exists() {
        let config = std::fs::read_to_string(config_path).expect("Failed to read config file");
        let config: CVRMelonConfig =
            serde_json::from_str(&config).expect("Failed to parse config file");

        config
    } else {
        let config_default = CVRMelonConfig::default();
        let config = serde_json::to_string_pretty(&config_default)
            .expect("Failed to serialize default config");
        std::fs::write(config_path, config).expect("Failed to write default config file");

        config_default
    }
});

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
#[serde(default, rename_all = "camelCase")]
pub struct CVRMelonConfig {
    chillout_folder: String,
}

impl CVRMelonConfig {
    pub fn save(&self) {
        let current_dir = std::env::current_dir().expect("Failed to get current directory");
        let config_path = current_dir.join(CONFIG_FILE_NAME);

        let config = serde_json::to_string_pretty(&self).expect("Failed to serialize config");
        std::fs::write(config_path, config).expect("Failed to write config file");
    }

    pub fn chillout_folder(&self) -> &str {
        &self.chillout_folder
    }

    pub fn set_chillout_folder(&mut self, folder_path: String) -> Result<(), &'static str> {
        let path = Path::new(&folder_path);
        if path.exists() {
            if path.is_dir() {
                self.chillout_folder = folder_path;
                self.save();
                Ok(())
            } else {
                Err("The path is not a directory")
            }
        } else {
            Err("The path does not exist")
        }
    }
}
