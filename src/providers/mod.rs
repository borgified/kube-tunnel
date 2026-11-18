pub mod aws;
pub mod gcp;
pub mod azure;

use crate::models::{Provider, TunnelConfig};
use tokio::process::Command;

/// Trait implemented by each native cloud provider tunnel generator.
pub trait CloudTunnelProvider: Send + Sync {
    /// Friendly provider name
    fn name(&self) -> &'static str;

    /// Checks if required CLI binaries and plugins exist on PATH
    fn check_installed(&self) -> Result<(), String>;

    /// Builds the provider's native command line process
    fn build_command(&self, config: &TunnelConfig) -> Result<Command, String>;
}

/// Returns the appropriate provider implementation.
pub fn get_provider(provider: Provider) -> Box<dyn CloudTunnelProvider> {
    match provider {
        Provider::Aws => Box::new(aws::AwsProvider),
        Provider::Gcp => Box::new(gcp::GcpProvider),
        Provider::Azure => Box::new(azure::AzureProvider),
    }
}
