use crate::models::TunnelConfig;
use crate::providers::CloudTunnelProvider;
use tokio::process::Command;

pub struct AwsProvider;

impl CloudTunnelProvider for AwsProvider {
    fn name(&self) -> &'static str {
        "AWS (Systems Manager)"
    }

    fn check_installed(&self) -> Result<(), String> {
        if which::which("aws").is_err() {
            return Err("AWS CLI ('aws') is not installed or not in PATH.".to_string());
        }
        if which::which("session-manager-plugin").is_err() {
            return Err("AWS Session Manager plugin ('session-manager-plugin') is not installed or not in PATH.".to_string());
        }
        Ok(())
    }

    fn build_command(&self, config: &TunnelConfig) -> Result<Command, String> {
        let mut cmd = Command::new("aws");
        cmd.arg("ssm").arg("start-session");
        cmd.arg("--target").arg(&config.target);

        if let Some(host) = &config.remote_host {
            cmd.arg("--document-name")
                .arg("AWS-StartPortForwardingSessionToRemoteHost");
            let params = format!(
                r#"{{"host":["{}"],"portNumber":["{}"],"localPortNumber":["{}"]}}"#,
                host, config.remote_port, config.local_port
            );
            cmd.arg("--parameters").arg(params);
        } else {
            cmd.arg("--document-name")
                .arg("AWS-StartPortForwardingSession");
            let params = format!(
                r#"{{"portNumber":["{}"],"localPortNumber":["{}"]}}"#,
                config.remote_port, config.local_port
            );
            cmd.arg("--parameters").arg(params);
        }

        Ok(cmd)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Provider, TunnelConfig};

    #[test]
    fn test_aws_port_forwarding_args() {
        let provider = AwsProvider;
        let mut config = TunnelConfig::new(Provider::Aws, "i-0123456789".to_string(), 8443);
        config.remote_port = 443;

        let cmd = provider.build_command(&config).unwrap();
        let cmd_inner = cmd.as_std();
        let args: Vec<&str> = cmd_inner.get_args().map(|s| s.to_str().unwrap()).collect();

        assert_eq!(cmd_inner.get_program(), "aws");
        assert!(args.contains(&"ssm"));
        assert!(args.contains(&"start-session"));
        assert!(args.contains(&"i-0123456789"));
        assert!(args.contains(&"AWS-StartPortForwardingSession"));
    }

    #[test]
    fn test_aws_remote_host_port_forwarding_args() {
        let provider = AwsProvider;
        let mut config = TunnelConfig::new(Provider::Aws, "i-0123456789".to_string(), 8443);
        config.remote_host = Some("k8s.internal".to_string());
        config.remote_port = 6443;

        let cmd = provider.build_command(&config).unwrap();
        let cmd_inner = cmd.as_std();
        let args: Vec<&str> = cmd_inner.get_args().map(|s| s.to_str().unwrap()).collect();

        assert!(args.contains(&"AWS-StartPortForwardingSessionToRemoteHost"));
        assert!(args.iter().any(|arg| arg.contains("k8s.internal")));
    }
}
