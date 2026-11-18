use clap::ValueEnum;
use serde::{Deserialize, Serialize};

/// Supported Cloud Service Providers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum Provider {
    Aws,
    Gcp,
    Azure,
}

impl std::fmt::Display for Provider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Provider::Aws => write!(f, "aws"),
            Provider::Gcp => write!(f, "gcp"),
            Provider::Azure => write!(f, "azure"),
        }
    }
}

/// Parameters required to establish a tunnel session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TunnelConfig {
    pub provider: Provider,
    pub target: String,
    pub local_port: u16,
    pub remote_port: u16,
    pub remote_host: Option<String>,
    pub zone: Option<String>,
    pub resource_group: Option<String>,
    pub bastion_name: Option<String>,
}

impl TunnelConfig {
    pub fn new(provider: Provider, target: String, local_port: u16) -> Self {
        Self {
            provider,
            target,
            local_port,
            remote_port: 443,
            remote_host: None,
            zone: None,
            resource_group: None,
            bastion_name: None,
        }
    }
}
