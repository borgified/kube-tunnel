# kube-tunnel

`kube-tunnel` is a command-line interface (CLI) tool for private Kubernetes tunnels.
The tool establishes secure tunnels to private cluster control planes across multiple clouds.
It uses native cloud provider tools and identity services.
You do not need public IP addresses or static SSH bastion keys.

## Supported Cloud Providers

- **Amazon Web Services (AWS)**: Uses AWS Systems Manager (SSM) Session Manager.
- **Google Cloud Platform (GCP)**: Uses Cloud Identity-Aware Proxy (IAP) TCP forwarding.
- **Microsoft Azure**: Uses Azure Bastion or Azure Arc.

---

## Prerequisites

Each cloud provider requires native CLI tools in your system `PATH`:

| Provider | Required Tools |
|---|---|
| AWS | `aws` CLI, `session-manager-plugin` |
| GCP | `gcloud` SDK |
| Azure | `az` CLI |

Run the `doctor` command to verify your local environment.

---

## Installation

### Build from Source

Verify that you have Rust installed. Compile the release binary:

```bash
cargo build --release
```

Copy the compiled binary to your system `PATH`:

```bash
sudo cp target/release/kube-tunnel /usr/local/bin/
```

### Cross-Compilation

To cross-compile for macOS Apple Silicon (ARM64):

```bash
rustup target add aarch64-apple-darwin
cargo build --target aarch64-apple-darwin --release
```

---

## Usage

### 1. Check Prerequisites

Run the `doctor` command to inspect your environment:

```bash
kube-tunnel doctor
```

The tool reports available provider tools and missing dependencies.

### 2. Connect with Explicit Flags

Use the `connect` command to start a tunnel.

#### AWS Private Cluster

Connect through an EC2 instance using SSM:

```bash
kube-tunnel connect \
  --provider aws \
  --target i-0123456789abcdef0 \
  --local-port 8443 \
  --remote-port 443
```

Forward traffic to a remote host endpoint:

```bash
kube-tunnel connect \
  --provider aws \
  --target i-0123456789abcdef0 \
  --remote-host k8s-api.internal \
  --local-port 8443 \
  --remote-port 443
```

#### GCP Private GKE

Connect through a Compute Engine instance with IAP:

```bash
kube-tunnel connect \
  --provider gcp \
  --target gke-bastion \
  --zone us-central1-a \
  --local-port 8443 \
  --remote-port 443
```

#### Azure Private AKS

Connect through Azure Bastion:

```bash
kube-tunnel connect \
  --provider azure \
  --target /subscriptions/<sub-id>/resourceGroups/<rg>/providers/Microsoft.Compute/virtualMachines/<vm-name> \
  --resource-group my-resource-group \
  --bastion-name my-bastion \
  --local-port 8443 \
  --remote-port 443
```

Connect through Azure Arc:

```bash
kube-tunnel connect \
  --provider azure \
  --target my-arc-vm \
  --resource-group my-resource-group \
  --local-port 8443 \
  --remote-port 443
```

### 3. Connect with Configuration Aliases

You can define cluster aliases in `~/.config/kube-tunnel/config.toml`.

#### Configuration File Example

Create the configuration file at `~/.config/kube-tunnel/config.toml`:

```toml
[aws.clusters.prod]
target = "i-0123456789abcdef0"
local_port = 8443
remote_port = 443

[gcp.clusters.dev]
target = "gke-bastion"
zone = "us-central1-a"
local_port = 8443
remote_port = 443

[azure.clusters.stage]
target = "stage-bastion"
resource_group = "stage-rg"
bastion_name = "stage-bastion-host"
local_port = 8443
remote_port = 443
```

#### Shorthand Syntax

Run the shorthand command with provider and alias:

```bash
kube-tunnel <provider> <alias>
```

Example:

```bash
kube-tunnel gcp dev
```

---

## Connect with kubectl

Direct your local `kubectl` command to the forwarded local port:

```bash
kubectl --server=https://127.0.0.1:8443 --insecure-skip-tls-verify get nodes
```

> **Note**: Provide valid cluster CA certificates if you do not skip TLS verification.

---

## Tunnel Termination

Press `Ctrl+C` to stop an active tunnel.
The application closes child provider processes and releases the local port.

---

## Exit Codes

The application returns standard exit codes:

| Code | Meaning |
|---|---|
| `0` | Clean shutdown after interrupt signal |
| `1` | Missing native CLI tool prerequisite |
| `2` | Invalid command line arguments |
| `3` | Local port collision (port is already in use) |
| `4` | Child tunnel process failure or authentication error |

---

## License

This project is licensed under the MIT License.
