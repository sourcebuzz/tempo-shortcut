use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub jira_url: String,
    #[serde(default)]
    pub api_token: String,
}

impl Config {
    fn path() -> PathBuf {
        let mut dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        dir.push("tempo-shortcut");
        dir.push("config.toml");
        dir
    }

    pub fn load() -> Config {
        match fs::read_to_string(Self::path()) {
            Ok(content) => toml::from_str(&content).unwrap_or_default(),
            Err(_) => Config::default(),
        }
    }

    pub fn save(&self) -> io::Result<()> {
        let path = Self::path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self).unwrap_or_default();
        fs::write(&path, content)?;

        // config file holds the API token, keep it readable only by the owner
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&path)?.permissions();
            perms.set_mode(0o600);
            fs::set_permissions(&path, perms)?;
        }

        Ok(())
    }

    pub fn is_configured(&self) -> bool {
        !self.jira_url.trim().is_empty() && !self.api_token.trim().is_empty()
    }
}
