use crate::models::TunnelConfig;
use crate::providers::CloudTunnelProvider;
use tokio::process::Command;

pub struct AzureProvider;

impl CloudTunnelProvider for AzureProvider {
    fn name(&self) -> &'static str {
        "Azure (Bastion / Arc)"
    }

    fn check_installed(&self) -> Result<(), String> {
        if which::which("az").is_err() {
            return Err("Azure CLI ('az') is not installed or not in PATH.".to_string());
        }
        Ok(())
    }

    fn build_command(&self, config: &TunnelConfig) -> Result<Command, String> {
        let mut cmd = Command::new("az");

        if let (Some(bastion), Some(rg)) = (&config.bastion_name, &config.resource_group) {
            cmd.arg("network").arg("bastion").arg("tunnel");
            cmd.arg("--name").arg(bastion);
            cmd.arg("--resource-group").arg(rg);
            cmd.arg("--target-resource-id").arg(&config.target);
            cmd.arg("--resource-port").arg(config.remote_port.to_string());
            cmd.arg("--port").arg(config.local_port.to_string());
        } else if let Some(rg) = &config.resource_group {
            // Azure Arc SSH tunnel fallback
            cmd.arg("ssh").arg("arc");
            cmd.arg("--resource-group").arg(rg);
            cmd.arg("--vm-name").arg(&config.target);
            let port_map = format!("{}:127.0.0.1:{}", config.local_port, config.remote_port);
            cmd.arg("-L").arg(port_map);
        } else {
            return Err("Azure provider requires --resource-group (and optionally --bastion-name) to be specified.".to_string());
        }

        Ok(cmd)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Provider, TunnelConfig};

    #[test]
    fn test_azure_bastion_tunnel_args() {
        let provider = AzureProvider;
        let mut config = TunnelConfig::new(Provider::Azure, "/sub/123/vm/bastion".to_string(), 8443);
        config.resource_group = Some("rg-prod".to_string());
        config.bastion_name = Some("bastion-prod".to_string());
        config.remote_port = 443;

        let cmd = provider.build_command(&config).unwrap();
        let cmd_inner = cmd.as_std();
        let args: Vec<&str> = cmd_inner.get_args().map(|s| s.to_str().unwrap()).collect();

        assert_eq!(cmd_inner.get_program(), "az");
        assert!(args.contains(&"network"));
        assert!(args.contains(&"bastion"));
        assert!(args.contains(&"tunnel"));
        assert!(args.contains(&"bastion-prod"));
        assert!(args.contains(&"rg-prod"));
        assert!(args.contains(&"--port"));
        assert!(args.contains(&"8443"));
    }
}
