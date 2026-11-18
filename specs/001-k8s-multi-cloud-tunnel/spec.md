# Feature Specification: Private Kubernetes Multi-Cloud Tunneling CLI

**Feature Directory**: `specs/001-k8s-multi-cloud-tunnel`

**Created**: 2026-11-17

**Status**: Ready for Planning

**Input**: User description: "create a rust binary that allows connection to private kubernetes clusters in all three CSPs. i want to use their own native tools to do so. ie. ssm, iap, arc."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - AWS SSM Tunneling for Private Clusters (Priority: P1) 🎯 MVP

As a cloud engineer, I want to establish a secure local tunnel to an AWS private Kubernetes cluster using AWS Systems Manager (SSM) so that I can access the private cluster without public IP addresses or bastion SSH keys.

**Why this priority**: Establishes the core tunnel execution model, local port binding, and process lifecycle for the CLI using AWS SSM.

**Independent Test**: Execute `kube-tunnel connect --provider aws --target <instance-id-or-cluster> --remote-port 443 --local-port 8443` and verify that traffic sent to `localhost:8443` reaches the target private endpoint.

**Acceptance Scenarios**:

1. **Given** valid AWS credentials and installed AWS CLI with session-manager-plugin, **When** the user executes `kube-tunnel connect --provider aws --target i-0123456789abcdef0 --remote-port 443 --local-port 8443`, **Then** the CLI opens local port 8443 and outputs a ready message with the local endpoint URL.
2. **Given** the local port 8443 is already in use by another process, **When** the user attempts to connect, **Then** the CLI terminates with an error message stating port conflict without spawning child processes.
3. **Given** the tunnel is active, **When** the user sends SIGINT (Ctrl+C), **Then** the CLI terminates the child AWS SSM process cleanly and frees the local port.

---

### User Story 2 - GCP IAP Tunneling for Private GKE (Priority: P2)

As a cloud engineer, I want to establish a local tunnel to a Google Cloud private GKE cluster using Identity-Aware Proxy (IAP) TCP forwarding so that I can access the cluster control plane securely.

**Why this priority**: Expands multi-cloud support to GCP environments using native `gcloud` IAP mechanisms.

**Independent Test**: Execute `kube-tunnel connect --provider gcp --target <instance-name> --zone <zone> --remote-port 443 --local-port 8443` and verify active port forwarding through IAP.

**Acceptance Scenarios**:

1. **Given** authenticated `gcloud` credentials, **When** the user executes `kube-tunnel connect --provider gcp --target gke-bastion --zone us-central1-a --remote-port 443 --local-port 8443`, **Then** the CLI starts the IAP tunnel and displays the active local endpoint.
2. **Given** an invalid or non-existent GCP instance name, **When** the user runs the command, **Then** the CLI captures the error from `gcloud` and presents a clear diagnostic message to standard error.

---

### User Story 3 - Azure Arc and Bastion Tunneling for Private AKS (Priority: P3)

As a cloud engineer, I want to establish a local tunnel to a private Azure AKS cluster or host using Azure Arc or Azure Bastion so that I can access the cluster securely without public IP addresses.

**Why this priority**: Completes the multi-cloud triumvirate across AWS, GCP, and Azure.

**Independent Test**: Execute `kube-tunnel connect --provider azure --target <resource-id> --resource-group <rg> --remote-port 443 --local-port 8443` and verify connectivity through Azure native tooling.

**Acceptance Scenarios**:

1. **Given** authenticated Azure CLI credentials, **When** the user executes `kube-tunnel connect --provider azure --target <resource-id> --resource-group my-rg --remote-port 443 --local-port 8443`, **Then** the CLI initiates the Azure native tunnel and displays the listening address.
2. **Given** Azure CLI is missing or unauthenticated, **When** the user runs the command, **Then** the CLI reports the missing prerequisite and exits with a non-zero code.

---

### User Story 4 - Tool Prerequisites Check and Tunnel Lifecycle (Priority: P4)

As a cloud engineer, I want to inspect prerequisites and view active tunnel statuses so that I know my environment is properly configured.

**Why this priority**: Provides operational visibility, self-healing, and diagnostic assistance for operators.

**Independent Test**: Execute `kube-tunnel doctor` to report status of `aws`, `session-manager-plugin`, `gcloud`, and `az`.

**Acceptance Scenarios**:

1. **Given** any system, **When** the user runs `kube-tunnel doctor`, **Then** the system outputs a checklist of detected tools, versions, and missing plugins.

---

### Edge Cases

- **Missing Native Tools**: When `aws`, `gcloud`, or `az` is not found on `$PATH`, the CLI halts immediately with an installation guide link before attempting connection.
- **Port Conflicts**: When the requested local port is unavailable, the CLI rejects execution immediately and can optionally suggest an open random port.
- **Sudden Disconnection**: When the remote cloud provider terminates the connection, the CLI detects child process termination and logs an informative disconnection message.
- **Signal Handling**: When SIGINT or SIGTERM is received, the CLI propagates termination signals to child provider processes to avoid orphaned background tunnels.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST detect the availability of native provider CLI tools (`aws`, `session-manager-plugin`, `gcloud`, `az`) on the system path.
- **FR-002**: The system MUST provide a `doctor` command that validates prerequisites for each cloud provider.
- **FR-003**: The system MUST support AWS SSM port forwarding (`AWS-StartPortForwardingSession`) to private endpoints.
- **FR-004**: The system MUST support GCP IAP TCP forwarding (`gcloud compute start-iap-tunnel`) to private endpoints.
- **FR-005**: The system MUST support Azure native tunneling (`az network bastion tunnel` or `az ssh arc`) to private endpoints.
- **FR-006**: The system MUST bind to a local TCP port on `127.0.0.1` and forward traffic to the target cluster API port.
- **FR-007**: The system MUST verify that the requested local port is available before starting any child process.
- **FR-008**: The system MUST supervise child tunnel processes and handle graceful shutdown upon receiving SIGINT or SIGTERM.
- **FR-009**: The system MUST display output connection details, including target host, local listening port, and example `kubectl` usage instructions.
- **FR-010**: The system MUST write errors and diagnostic messages to standard error and return non-zero exit codes upon failure.
- **FR-011**: The build system MUST support compiling release binaries for Linux (`x86_64-unknown-linux-gnu`) and macOS Apple Silicon (`aarch64-apple-darwin`).

### Key Entities

- **Provider**: Enumeration representing supported cloud platforms (`Aws`, `Gcp`, `Azure`).
- **TunnelConfig**: Configuration specifying the cloud provider, target resource identifier, remote port (default 443/6443), local listening port, and provider-specific parameters (region, zone, resource group).
- **TunnelSession**: Represents an active running tunnel process, encapsulating child PID, provider type, local port, and status.
- **PrerequisiteReport**: Status record for cloud CLI binaries and plugins on the host machine.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Pre-flight checks (`kube-tunnel doctor`) complete and display results in under 500 milliseconds.
- **SC-002**: Tunnel process initiates within 3 seconds of command invocation when native tools and credentials are valid.
- **SC-003**: 100% of child tunnel processes are cleanly terminated upon CLI exit, leaving zero orphaned processes.
- **SC-004**: Zero custom tunnel daemons or proprietary agents required on the target clusters beyond native cloud provider tooling.
- **SC-005**: Build tooling successfully outputs compiled standalone binaries for Linux (`x86_64-unknown-linux-gnu`) and macOS Apple Silicon (`aarch64-apple-darwin`).

## Assumptions

- Users have installed and authenticated the respective cloud provider CLI tools (`aws`, `gcloud`, `az`) for the target environments.
- Target Kubernetes clusters reside in VPCs/VNets with native tunneling support enabled (SSM agent on bastion/node, IAP enabled in GCP project, Azure Bastion or Arc agent registered).
- Client machine is a Linux, macOS, or modern POSIX environment with local loopback TCP networking.
