use std::{collections::HashMap, path::Path, str::FromStr};

use hemtt_bin_error::Error;
use serde::{Deserialize, Serialize};

mod hemtt;
mod signing;
mod version;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Configuration {
    /// The name of the project
    name: String,
    /// Prefix for the project
    prefix: String,
    /// Main prefix for the project
    mainprefix: Option<String>,

    #[serde(default)]
    /// version of the project
    version: version::Options,

    #[serde(default)]
    /// Headers to be added to built PBOs
    headers: HashMap<String, String>,

    #[serde(default)]
    /// Files to be included in the root of the project, supports glob patterns
    files: Vec<String>,

    #[serde(default)]
    hemtt: hemtt::Features,

    #[serde(default)]
    signing: signing::Options,
}

impl Configuration {
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub const fn version(&self) -> &version::Options {
        &self.version
    }

    #[must_use]
    pub fn prefix(&self) -> &str {
        &self.prefix
    }

    #[must_use]
    pub const fn mainprefix(&self) -> Option<&String> {
        self.mainprefix.as_ref()
    }

    #[must_use]
    pub const fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    #[must_use]
    pub fn files(&self) -> Vec<String> {
        let mut files = self.files.clone();
        for default in [
            "mod.cpp",
            "meta.cpp",
            "LICENSE",
            "logo_ca.paa",
            "logo_co.paa",
        ]
        .iter()
        .map(std::string::ToString::to_string)
        {
            if !files.contains(&default) {
                let path = Path::new(&default);
                if path.exists() {
                    files.push(default.clone());
                }
            }
        }
        files.sort();
        files.dedup();
        files
    }

    #[must_use]
    pub const fn hemtt(&self) -> &hemtt::Features {
        &self.hemtt
    }

    #[must_use]
    pub const fn signing(&self) -> &signing::Options {
        &self.signing
    }

    /// Load a configuration from a file.
    ///
    /// # Errors
    ///
    /// If the file cannot be read, or if the file is not valid TOML, or if the
    /// file does not contain a valid configuration, an error is returned.
    pub fn from_file(path: &Path) -> Result<Self, Error> {
        let file = std::fs::read_to_string(path)?;
        let config = Self::from_str(&file)?;

        // Validate
        if config.prefix.is_empty() {
            return Err(Error::ConfigInvalid("prefix cannot be empty".to_string()));
        }

        Ok(config)
    }
}

impl FromStr for Configuration {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        toml::from_str(s).map_err(Error::from)
    }
}