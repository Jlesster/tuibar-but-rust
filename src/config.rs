use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub refresh_interval_ms: u64,
    pub modules: Vec<String>,
    pub colors: ColorConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ColorConfig {
    pub primary: String,
    pub surface: String,
    pub text: String,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn Error>> {
        let config_path = Self::config_path()?;

        if !config_path.exists() {
            let default_config = Self::default();
            default_config.save()?;
            return Ok(default_config);
        }

        let contents = fs::read_to_string(config_path)?;
        let config: Config = serde_json::from_str(&contents)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let config_path = Self::config_path()?;

        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string_pretty(self)?;
        fs::write(config_path, json)?;
        Ok(())
    }

    fn config_path() -> Result<PathBuf, Box<dyn Error>> {
        let home = std::env::var("HOME")?;
        Ok(PathBuf::from(home)
            .join(".config")
            .join("JlessBar")
            .join("config.json"))
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            refresh_interval_ms: 100,
            modules: vec![
                "workspaces".to_string(),
                "clock".to_string(),
                "cpu".to_string(),
                "memory".to_string(),
                "battery".to_string(),
            ],
            colors: ColorConfig {
                primary: "#D7BAFF".to_string(),
                surface: "#16121B".to_string(),
                text: "E9DFEE".to_string(),
            },
        }
    }
}
