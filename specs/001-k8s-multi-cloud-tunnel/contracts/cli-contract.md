# CLI Contract: Private Kubernetes Multi-Cloud Tunneling

**Binary Name**: `kube-tunnel`

## Commands Overview

```bash
kube-tunnel [COMMAND] [OPTIONS]
```

### 1. `kube-tunnel doctor`
Checks and reports availability of native cloud CLI tools.

#### Output Example:
```text
Checking native cloud provider tools...
  [✓] aws (AWS CLI 2.15.0)
  [✓] session-manager-plugin (1.2.553.0)
  [✓] gcloud (Google Cloud SDK 460.0.0)
  [✗] az (Azure CLI NOT FOUND - please install azure-cli)
Status: 3/4 tools ready.
```

---

### 2. `kube-tunnel connect`
Establishes a local port-forwarding tunnel to the target private Kubernetes endpoint.

#### Arguments & Options:
| Flag | Short | Value Type | Required | Description |
|---|---|---|---|---|
| `--provider` | `-p` | `aws \| gcp \| azure` | Yes | Target cloud provider |
| `--target` | `-t` | `String` | Yes | Target instance, bastion, or resource ID |
| `--local-port` | `-l` | `u16` | Yes | Local TCP port to bind (e.g., 8443) |
| `--remote-port` | `-r` | `u16` | No (default: 443) | Remote Kubernetes API port |
| `--remote-host` | | `String` | No | Remote host/IP for AWS SSM forwarding |
| `--zone` | `-z` | `String` | Conditional | GCP zone (required for GCP) |
| `--resource-group` | | `String` | Conditional | Azure resource group |
| `--bastion-name` | | `String` | Conditional | Azure Bastion resource name |

#### Exit Codes:
| Code | Meaning |
|---|---|
| `0` | Clean shutdown upon receiving interrupt signal |
| `1` | Missing prerequisite native CLI tool |
| `2` | Invalid CLI arguments |
| `3` | Local port collision (port already occupied) |
| `4` | Child tunnel process failure or authentication error |
