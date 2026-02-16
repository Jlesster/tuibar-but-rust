use crate::modules::{ModuleConfig, ModulePosition};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub modules: Vec<String>,
    pub module_configs: HashMap<String, ModuleConfig>,
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
        let mut module_configs = HashMap::new();

        module_configs.insert(
            "workspaces".to_string(),
            ModuleConfig {
                enabled: true,
                format: Some("{id}".to_string()),
                interval: None,
                position: ModulePosition::Left,
            },
        );

        module_configs.insert(
            "window".to_string(),
            ModuleConfig {
                enabled: true,
                format: Some("{title}".to_string()),
                interval: None,
                position: ModulePosition::CenterLeft,
            },
        );

        module_configs.insert(
            "clock".to_string(),
            ModuleConfig {
                enabled: true,
                format: Some("%H:%M:%S".to_string()),
                interval: Some(1000),
                position: ModulePosition::CenterRight,
            },
        );

        module_configs.insert(
            "cpu".to_string(),
            ModuleConfig {
                enabled: true,
                format: Some("{icon} {usage}%".to_string()),
                interval: Some(2000),
                position: ModulePosition::Right,
            },
        );

        module_configs.insert(
            "memory".to_string(),
            ModuleConfig {
                enabled: true,
                format: Some("{icon} {usage}%".to_string()),
                interval: Some(2000),
                position: ModulePosition::Right,
            },
        );

        module_configs.insert(
            "network".to_string(),
            ModuleConfig {
                enabled: true,
                format: Some("{icon} {ssid}".to_string()),
                interval: Some(2000),
                position: ModulePosition::Right,
            },
        );

        module_configs.insert(
            "battery".to_string(),
            ModuleConfig {
                enabled: true,
                format: Some("{icon} {level}%".to_string()),
                interval: Some(30000),
                position: ModulePosition::Right,
            },
        );

        Self {
            modules: vec![
                "workspaces".to_string(),
                "window".to_string(),
                "clock".to_string(),
                "cpu".to_string(),
                "memory".to_string(),
                "network".to_string(),
                "battery".to_string(),
            ],
            module_configs,
            colors: ColorConfig {
                primary: "#D7BAFF".to_string(),
                surface: "#16121B".to_string(),
                text: "E9DFEE".to_string(),
            },
        }
    }
}
