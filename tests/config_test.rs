use kube_tunnel::config::AppConfig;
use kube_tunnel::models::Provider;

#[test]
fn test_config_parsing_and_alias_resolution() {
    let toml_data = r#"
[gcp.clusters.na01]
target = "gke-bastion-na01"
zone = "us-central1-a"
local_port = 8443
remote_port = 443

[aws.clusters.prod01]
target = "i-0123456789abcdef0"
remote_host = "k8s-prod.internal"
local_port = 8444
remote_port = 443

[azure.clusters.eu01]
target = "/subscriptions/123/resourceGroups/rg/providers/Microsoft.Compute/virtualMachines/vm"
resource_group = "rg-aks"
bastion_name = "bastion-eu"
local_port = 8445
remote_port = 443
"#;

    let config: AppConfig = toml::from_str(toml_data).expect("failed to parse TOML");

    // Resolve GCP na01
    let gcp_tunnel = config.resolve_alias(Provider::Gcp, "na01").expect("failed to resolve gcp na01");
    assert_eq!(gcp_tunnel.target, "gke-bastion-na01");
    assert_eq!(gcp_tunnel.zone.as_deref(), Some("us-central1-a"));
    assert_eq!(gcp_tunnel.local_port, 8443);

    // Resolve AWS prod01
    let aws_tunnel = config.resolve_alias(Provider::Aws, "prod01").expect("failed to resolve aws prod01");
    assert_eq!(aws_tunnel.target, "i-0123456789abcdef0");
    assert_eq!(aws_tunnel.remote_host.as_deref(), Some("k8s-prod.internal"));
    assert_eq!(aws_tunnel.local_port, 8444);

    // Resolve Azure eu01
    let azure_tunnel = config.resolve_alias(Provider::Azure, "eu01").expect("failed to resolve azure eu01");
    assert_eq!(azure_tunnel.resource_group.as_deref(), Some("rg-aks"));
    assert_eq!(azure_tunnel.local_port, 8445);

    // Non-existent alias
    assert!(config.resolve_alias(Provider::Gcp, "non_existent").is_none());
}

#[test]
fn test_missing_config_fallback() {
    let non_existent_path = std::path::Path::new("/tmp/non_existent_kube_tunnel_config_98234.toml");
    let result = AppConfig::load_from_path(non_existent_path).expect("must not error on missing file");
    assert!(result.is_none());
}
