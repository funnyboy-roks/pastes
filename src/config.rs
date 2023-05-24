use std::{collections::HashMap, fs, path::PathBuf};

use anyhow::Context;
use serde::Deserialize;

const DEFAULT_USER_AGENT: fn() -> String = || crate::DEFAULT_USERAGENT.into();

// TODO: Add a way to specify other apis to interract with (probably only those hosting bytebin/paste)
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub content_type: Option<String>,
    #[serde(default = "DEFAULT_USER_AGENT")]
    pub user_agent: String,
    pub headers: Option<HashMap<String, String>>,
}

impl Config {
    pub fn get_default_config_file() -> anyhow::Result<PathBuf> {
        let mut path: PathBuf = directories::ProjectDirs::from("com", "funnyboyroks", "pastes")
            .context("Unable to open config file")?
            .config_dir()
            .into();

        path.push("config.toml");

        Ok(path)
    }

    /// Creates the default config (from `extra/config.toml`)
    ///
    /// This _does not_ overwrite a file that already exists.
    ///
    /// Returns a result with a bool.  If bool is true, then it wrote the file, false if it already
    /// existed.
    pub fn init_default_config() -> anyhow::Result<bool> {
        let path = Self::get_default_config_file()?;
        if path.exists() {
            Ok(false)
        } else {
            // mkdir recursive for the desired path
            fs::create_dir_all(path.parent().expect("path should not be /"))
                .context("Error when creating path to config file")?;

            // Write the config itself
            fs::write(&path, include_str!("../extra/config.toml"))
                .with_context(|| format!("Error when writing config to {}", path.display()))?;

            Ok(true)
        }
    }

    pub fn load_config(path: Option<&PathBuf>) -> anyhow::Result<Self> {
        let contents = match path {
            // Path has been specified, so let's use that one
            Some(path) => fs::read_to_string(path),

            // Path was not specified, so let's get the default
            None => {
                let made_config = Self::init_default_config()?;
                let config_path = Self::get_default_config_file()?;
                if made_config {
                    // Log that the path has been created if we can.
                    println!("Created config at {}", config_path.display());
                }
                fs::read_to_string(config_path)
            }
        }
        .context("unable read from config file")?;

        toml::from_str(&contents).context("unable to parse config")
    }
}
