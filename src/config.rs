use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub settings: Settings,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {
    #[serde(default = "default_projects_dir")]
    pub projects_dir: PathBuf,
    #[serde(default = "default_templates")]
    pub enabled_templates: Vec<String>,
}

fn default_projects_dir() -> PathBuf {
    dirs::home_dir()
        .expect("Could not find home directory")
        .join("Dev")
}

fn default_templates() -> Vec<String> {
    vec![
        "rust".to_string(),
        "python".to_string(),
        "platformio".to_string(),
    ]
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            projects_dir: default_projects_dir(),
            enabled_templates: default_templates(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            settings: Settings::default(),
        }
    }
}

impl Config {
    pub fn load() -> io::Result<Self> {
        let config_path = get_config_path()?;
        
        if !config_path.exists() {
            let default_config = Self::default();
            let config_str = toml::to_string_pretty(&default_config)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            
            if let Some(parent) = config_path.parent() {
                fs::create_dir_all(parent)?;
            }
            
            fs::write(&config_path, config_str)?;
            return Ok(default_config);
        }

        let config_str = fs::read_to_string(config_path)?;
        toml::from_str(&config_str)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }
}

fn get_config_path() -> io::Result<PathBuf> {
    dirs::home_dir()
        .map(|p| p.join(".config").join("newnew").join("newnew.toml"))
        .ok_or_else(|| io::Error::new(
            io::ErrorKind::NotFound,
            "Could not determine config directory"
        ))
} 