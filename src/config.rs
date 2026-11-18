use std::collections::HashMap;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use crate::models::{Provider, TunnelConfig};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(default)]
    pub aws: ProviderSection,
    #[serde(default)]
    pub gcp: ProviderSection,
    #[serde(default)]
    pub azure: ProviderSection,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProviderSection {
    #[serde(default)]
    pub clusters: HashMap<String, ClusterEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterEntry {
    pub target: String,
    #[serde(default = "default_local_port")]
    pub local_port: u16,
    #[serde(default = "default_remote_port")]
    pub remote_port: u16,
    pub remote_host: Option<String>,
    pub zone: Option<String>,
    pub resource_group: Option<String>,
    pub bastion_name: Option<String>,
}

fn default_local_port() -> u16 {
    8443
}

fn default_remote_port() -> u16 {
    443
}

impl AppConfig {
    /// Returns default path ~/.config/kube-tunnel/config.toml
    pub fn default_path() -> Option<PathBuf> {
        let home = std::env::var_os("HOME")?;
        Some(PathBuf::from(home).join(".config").join("kube-tunnel").join("config.toml"))
    }

    /// Loads configuration from given path or default location.
    /// If the file does not exist, returns Ok(None) to allow graceful fallback.
    pub fn load_from_path(path: &Path) -> Result<Option<Self>, String> {
        if !path.exists() {
            return Ok(None);
        }
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read config file {}: {}", path.display(), e))?;
        let config: Self = toml::from_str(&content)
            .map_err(|e| format!("Failed to parse config file {}: {}", path.display(), e))?;
        Ok(Some(config))
    }

    /// Resolves an alias under a given provider into a TunnelConfig.
    pub fn resolve_alias(&self, provider: Provider, alias: &str) -> Option<TunnelConfig> {
        let section = match provider {
            Provider::Aws => &self.aws,
            Provider::Gcp => &self.gcp,
            Provider::Azure => &self.azure,
        };
        let entry = section.clusters.get(alias)?;
        Some(TunnelConfig {
            provider,
            target: entry.target.clone(),
            local_port: entry.local_port,
            remote_port: entry.remote_port,
            remote_host: entry.remote_host.clone(),
            zone: entry.zone.clone(),
            resource_group: entry.resource_group.clone(),
            bastion_name: entry.bastion_name.clone(),
        })
    }
}
