# Data Model & Domain Design: Private Kubernetes Multi-Cloud Tunneling

**Feature**: [spec.md](file:///home/fwiffo/projects/kube-tunnel/specs/001-k8s-multi-cloud-tunnel/spec.md)  
**Date**: 2026-11-17

## Domain Entities

### 1. `Provider`
Enum representing supported Cloud Service Providers (CSPs).

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum Provider {
    Aws,
    Gcp,
    Azure,
}
```

---

### 2. `TunnelConfig`
Configuration parameters required to establish a tunnel.

| Field | Type | Description | Constraints / Default |
|---|---|---|---|
| `provider` | `Provider` | Target CSP | Required (`aws`, `gcp`, `azure`) |
| `target` | `String` | Target identifier (Instance ID, GKE bastion name, or Azure VM ID) | Non-empty string |
| `remote_port` | `u16` | Remote Kubernetes API server port | Defaults to `443` or `6443` |
| `local_port` | `u16` | Local listening port on 127.0.0.1 | Must be between `1024` and `65535` |
| `remote_host` | `Option<String>` | Remote host/IP for AWS SSM forwarding | Optional host override |
| `zone` | `Option<String>` | GCP compute zone | Required for GCP if target is an instance |
| `resource_group` | `Option<String>` | Azure resource group | Required for Azure Bastion |
| `bastion_name` | `Option<String>` | Azure Bastion resource name | Required for Azure Bastion |

---

### 3. `PrerequisiteStatus`
Status report of external CLI tools and plugins.

| Field | Type | Description |
|---|---|---|
| `tool_name` | `String` | Executable name (`aws`, `gcloud`, `az`, `session-manager-plugin`) |
| `installed` | `bool` | Whether found on `$PATH` |
| `version` | `Option<String>` | Detected version string |
| `notes` | `Option<String>` | Guidance or installation instructions |

---

### 4. `TunnelSession`
Represents an active, monitored child process tunnel.

| Field | Type | Description |
|---|---|---|
| `id` | `String` | Unique session ID (UUID or slug) |
| `provider` | `Provider` | Associated provider |
| `local_port` | `u16` | Local listening port |
| `pid` | `u32` | OS Process ID of child provider CLI |
| `started_at` | `DateTime<Utc>` | Timestamp of session start |
