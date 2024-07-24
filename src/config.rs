use std::path::Path;

use emojis::SkinTone;
use serde::{Deserialize, Serialize};

use super::*;

pub fn user_data_dir() -> PathBuf {
    dirs::data_dir().unwrap().join(APP_ID)
}

pub fn config_file_path() -> PathBuf {
    user_data_dir().join("config.toml")
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub preferred_skin_tone: SkinTone,
    pub recent_emojis: Vec<&'static Emoji>,
}

impl Config {
    pub fn load_or_create() -> Self {
        let config_path = config_file_path();

        if !config_path.exists() {
            let config = Self::default();
            config.save();
            config
        } else {
            Self::load(&config_path).unwrap()
        }
    }

    pub fn load(path: &Path) -> Option<Self> {
        let config_toml = fs::read_to_string(path).ok()?;
        toml::from_str(&config_toml).ok()
    }

    pub fn save(&self) {
        let config_toml = toml::to_string(self).unwrap();
        fs::write(config_file_path(), config_toml).unwrap();
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            preferred_skin_tone: SkinTone::Default,
            recent_emojis: vec![],
        }
    }
}
