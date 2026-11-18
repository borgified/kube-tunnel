# Technical Research & Decisions: Private Kubernetes Multi-Cloud Tunneling

**Feature**: [spec.md](file:///home/fwiffo/projects/kube-tunnel/specs/001-k8s-multi-cloud-tunnel/spec.md)  
**Date**: 2026-11-17  
**Status**: Completed

## Research Topics & Decisions

### 1. Native Cloud Tool Invocation vs. Embedded Cloud SDKs
- **Decision**: Execute native cloud provider binaries (`aws`, `gcloud`, `az`) via `tokio::process::Command`.
- **Rationale**:
  - The specification mandates using native cloud tools directly.
  - Reuses existing user authentication sessions (e.g., AWS SSO, `gcloud auth login`, Azure Entra ID) without re-implementing OAuth2 flows.
  - Avoids reverse-engineering proprietary WebSocket relay protocols used by GCP IAP and Azure Arc.
  - Significantly reduces compiled binary size and compilation time.
- **Alternatives Considered**:
  - Cloud SDKs (`aws-sdk-ssm`, Azure SDK for Rust): Rejected because user explicitly requested native tools, and full protocol replication is unnecessarily complex.

### 2. Dual-Platform Compilation Strategy (Linux & macOS ARM)
- **Decision**: Configure Cargo targets for `x86_64-unknown-linux-gnu` and `aarch64-apple-darwin` using `rustup target add` and `cargo-zigbuild` / `cross`.
- **Rationale**:
  - Fulfills the explicit user requirement to produce standalone binaries for both Linux and macOS Apple Silicon.
  - Rust native cross-compilation with Zig as the linker allows painless macOS Mach-O binary emission directly from Linux environments without requiring an Apple device.
- **Alternatives Considered**:
  - Requiring a separate macOS build machine: Inflexible for local developer workflows.

### 3. Async Runtime & Child Process Supervision
- **Decision**: Use `tokio` runtime with `process` and `signal` features.
- **Rationale**:
  - Manages long-running child tunnel processes asynchronously.
  - Intercepts POSIX signals (`SIGINT`, `SIGTERM`) via `tokio::signal::ctrl_c` and kills child processes immediately to prevent zombie/orphan background tunnels.
- **Alternatives Considered**:
  - Synchronous `std::process::Command`: Lacks clean non-blocking signal handling; interrupts often leave child tunnels running in background.

### 4. Local Port Collision Detection
- **Decision**: Attempt a non-blocking `std::net::TcpListener::bind` on `127.0.0.1:<local_port>` before launching child commands.
- **Rationale**:
  - Provides instantaneous, human-readable feedback if the desired port is already occupied.
  - Prevents provider-specific obscure connection failures.
- **Alternatives Considered**:
  - Letting the child process fail: Fails late, and different provider CLIs produce disparate error messages.

### 5. Provider Command Specifications

#### AWS SSM:
```bash
aws ssm start-session \
  --target <INSTANCE_ID> \
  --document-name AWS-StartPortForwardingSessionToRemoteHost \
  --parameters '{"host":["<K8S_ENDPOINT>"],"portNumber":["<REMOTE_PORT>"],"localPortNumber":["<LOCAL_PORT>"]}'
```
*Prerequisite*: `aws` CLI + `session-manager-plugin`.

#### GCP IAP:
```bash
gcloud compute start-iap-tunnel \
  <INSTANCE_NAME> <REMOTE_PORT> \
  --local-host-port=localhost:<LOCAL_PORT> \
  --zone=<ZONE>
```
*Prerequisite*: `gcloud` CLI.

#### Azure Arc / Bastion:
```bash
az network bastion tunnel \
  --name <BASTION_NAME> \
  --resource-group <RESOURCE_GROUP> \
  --target-resource-id <TARGET_VM_ID> \
  --resource-port <REMOTE_PORT> \
  --port <LOCAL_PORT>
```
*Prerequisite*: `az` CLI.
