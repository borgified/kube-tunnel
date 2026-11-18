# Tasks: Private Kubernetes Multi-Cloud Tunneling CLI

**Input**: Design documents from `specs/001-k8s-multi-cloud-tunnel/`  
**Prerequisites**: [plan.md](file:///home/fwiffo/projects/kube-tunnel/specs/001-k8s-multi-cloud-tunnel/plan.md), [spec.md](file:///home/fwiffo/projects/kube-tunnel/specs/001-k8s-multi-cloud-tunnel/spec.md), [research.md](file:///home/fwiffo/projects/kube-tunnel/specs/001-k8s-multi-cloud-tunnel/research.md), [data-model.md](file:///home/fwiffo/projects/kube-tunnel/specs/001-k8s-multi-cloud-tunnel/data-model.md), [contracts/cli-contract.md](file:///home/fwiffo/projects/kube-tunnel/specs/001-k8s-multi-cloud-tunnel/contracts/cli-contract.md)

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: User story label (`[US1]`, `[US2]`, `[US3]`, `[US4]`)
- Exact file paths included for all tasks

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization, dependencies, and cross-compilation targets

- [X] T001 Initialize Cargo manifest with `clap`, `tokio`, `serde`, `toml`, and `which` in `Cargo.toml`
- [X] T002 Create source and integration test directory structure in `src/`, `src/providers/`, and `tests/`
- [X] T003 [P] Configure dual-target build settings for `x86_64-unknown-linux-gnu` and `aarch64-apple-darwin` in `.cargo/config.toml`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core data structures, port collision checks, and provider abstractions

- [X] T004 Implement `Provider` enum and `TunnelConfig` struct in `src/models.rs` citing [data-model.md](file:///home/fwiffo/projects/kube-tunnel/specs/001-k8s-multi-cloud-tunnel/data-model.md) `TunnelConfig.provider` and `TunnelConfig.local_port`
- [X] T005 [P] Implement local TCP port collision detector using `std::net::TcpListener` in `src/port.rs`
- [X] T006 [P] Implement configuration file reader (`~/.config/kube-tunnel/config.toml`) with missing-file fallback in `src/config.rs` citing [data-model.md](file:///home/fwiffo/projects/kube-tunnel/specs/001-k8s-multi-cloud-tunnel/data-model.md) `TunnelConfig`
- [X] T007 Define `CloudTunnelProvider` trait and command dispatcher in `src/providers/mod.rs`
- [X] T008 [P] Implement prerequisite tool validator and `doctor` command logic in `src/doctor.rs`

**Checkpoint**: Foundation ready — provider implementations and user stories can proceed.

---

## Phase 3: User Story 1 - AWS SSM Tunneling (Priority: P1) 🎯 MVP

**Goal**: Connect to private Kubernetes targets via AWS Systems Manager port forwarding.

**Independent Test**: Execute `kube-tunnel connect --provider aws --target <id> --local-port 8443` or unit test argument builder.

### Tests for User Story 1
- [X] T009 [P] [US1] Add unit test for AWS SSM `start-session` argument generation in `src/providers/aws.rs`

### Implementation for User Story 1
- [X] T010 [US1] Implement `AwsProvider` constructing `aws ssm start-session` command in `src/providers/aws.rs`
- [X] T011 [US1] Implement child process execution loop and `tokio::signal::ctrl_c` handler in `src/runner.rs`
- [X] T012 [US1] Wire AWS provider and direct command-line arguments into `src/main.rs`

**Checkpoint**: User Story 1 works as an independent MVP.

---

## Phase 4: User Story 2 - GCP IAP TCP Tunneling (Priority: P2)

**Goal**: Connect to private GKE targets via Google Cloud Identity-Aware Proxy.

**Independent Test**: Execute `kube-tunnel connect --provider gcp --target <instance> --zone <zone> --local-port 8443`.

### Tests for User Story 2
- [X] T013 [P] [US2] Add unit test for GCP IAP command generation in `src/providers/gcp.rs`

### Implementation for User Story 2
- [X] T014 [US2] Implement `GcpProvider` constructing `gcloud compute start-iap-tunnel` command in `src/providers/gcp.rs`
- [X] T015 [US2] Register GCP provider in CLI command dispatcher in `src/main.rs`

**Checkpoint**: AWS and GCP tunnels both functional independently.

---

## Phase 5: User Story 3 - Azure Arc & Bastion Tunneling (Priority: P3)

**Goal**: Connect to private AKS targets via Azure Bastion and Arc native tools.

**Independent Test**: Execute `kube-tunnel connect --provider azure --target <id> --resource-group <rg> --local-port 8443`.

### Tests for User Story 3
- [X] T016 [P] [US3] Add unit test for Azure Bastion command arguments in `src/providers/azure.rs`

### Implementation for User Story 3
- [X] T017 [US3] Implement `AzureProvider` constructing `az network bastion tunnel` command in `src/providers/azure.rs`
- [X] T018 [US3] Register Azure provider in CLI command dispatcher in `src/main.rs`

**Checkpoint**: All three cloud provider tunnels functional.

---

## Phase 6: User Story 4 - Configuration File & Shorthand Aliases (Priority: P2)

**Goal**: Support shorthand syntax `kube-tunnel <provider> <alias>` with fallback to explicit arguments.

**Independent Test**: Create sample config, run `kube-tunnel gcp na01`, and verify target resolution.

### Tests for User Story 4
- [X] T019 [P] [US4] Add unit tests for configuration file alias parsing and missing-file fallback in `tests/config_test.rs`

### Implementation for User Story 4
- [X] T020 [US4] Connect CLI top-level argument handler to resolve aliases from `config.rs` with explicit flag fallback in `src/main.rs`

**Checkpoint**: Shorthand syntax works seamlessly alongside explicit command-line flags.

---

## Phase 7: Polish & Multi-Platform Validation

**Purpose**: Integration testing, release packaging, and cross-compilation

- [X] T021 [P] Implement integration tests verifying CLI exit codes (0, 1, 2, 3) in `tests/cli.rs`
- [X] T022 Compile Linux release binary `cargo build --release` and verify executable
- [X] T023 Compile macOS Apple Silicon release binary `cargo build --target aarch64-apple-darwin --release` and verify Mach-O artifact
