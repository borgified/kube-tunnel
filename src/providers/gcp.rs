use crate::models::TunnelConfig;
use crate::providers::CloudTunnelProvider;
use tokio::process::Command;

pub struct GcpProvider;

impl CloudTunnelProvider for GcpProvider {
    fn name(&self) -> &'static str {
        "Google Cloud (IAP)"
    }

    fn check_installed(&self) -> Result<(), String> {
        if which::which("gcloud").is_err() {
            return Err("Google Cloud SDK ('gcloud') is not installed or not in PATH.".to_string());
        }
        Ok(())
    }

    fn build_command(&self, config: &TunnelConfig) -> Result<Command, String> {
        let mut cmd = Command::new("gcloud");
        cmd.arg("compute").arg("start-iap-tunnel");
        cmd.arg(&config.target);
        cmd.arg(config.remote_port.to_string());

        let local_arg = format!("--local-host-port=localhost:{}", config.local_port);
        cmd.arg(local_arg);

        if let Some(zone) = &config.zone {
            cmd.arg(format!("--zone={}", zone));
        }

        Ok(cmd)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Provider, TunnelConfig};

    #[test]
    fn test_gcp_iap_command_args() {
        let provider = GcpProvider;
        let mut config = TunnelConfig::new(Provider::Gcp, "gke-bastion".to_string(), 8443);
        config.zone = Some("us-central1-a".to_string());
        config.remote_port = 443;

        let cmd = provider.build_command(&config).unwrap();
        let cmd_inner = cmd.as_std();
        let args: Vec<&str> = cmd_inner.get_args().map(|s| s.to_str().unwrap()).collect();

        assert_eq!(cmd_inner.get_program(), "gcloud");
        assert!(args.contains(&"compute"));
        assert!(args.contains(&"start-iap-tunnel"));
        assert!(args.contains(&"gke-bastion"));
        assert!(args.contains(&"443"));
        assert!(args.contains(&"--local-host-port=localhost:8443"));
        assert!(args.contains(&"--zone=us-central1-a"));
    }
}
