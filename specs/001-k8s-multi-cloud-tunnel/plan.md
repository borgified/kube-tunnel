# Implementation Plan: Private Kubernetes Multi-Cloud Tunneling

**Branch**: `001-k8s-multi-cloud-tunnel` | **Date**: 2026-11-17 | **Spec**: [spec.md](file:///home/fwiffo/projects/kube-tunnel/specs/001-k8s-multi-cloud-tunnel/spec.md)

**Input**: Feature specification from `specs/001-k8s-multi-cloud-tunnel/spec.md`

## Summary

Build a unified, robust multi-cloud CLI tool in Rust (`kube-tunnel`) that establishes secure local port-forwarding tunnels to private Kubernetes clusters using each provider's native tooling (AWS SSM, GCP IAP, Azure Bastion/Arc). The tool enforces pre-flight checks, verifies local port availability before execution, cleanly traps OS signals to terminate child processes, and compiles for both Linux (`x86_64`) and macOS Apple Silicon (`aarch64`).

## Technical Context

**Language/Version**: Rust 2021 edition (Rust 1.99.1 installed)

**Primary Dependencies**:
- `clap` v4 (with `derive`, `cargo`) for argument parsing
- `tokio` (features `process`, `signal`, `rt-multi-thread`, `macros`) for async child process supervision and signal interception
- `which` for detecting native cloud CLI binaries on `$PATH`

**Target Platforms**:
- Linux: `x86_64-unknown-linux-gnu`
- macOS: `aarch64-apple-darwin` (Apple Silicon ARM64)

**Project Type**: CLI binary with internal library crate

**Performance Goals**:
- `doctor` command execution: < 500ms
- Local port collision check: < 10ms
- Clean child process termination: < 1s upon SIGINT/SIGTERM

**Constraints**:
- Use native provider tools directly (`aws`, `gcloud`, `az`)
- No proprietary agents on target clusters
- Build artifacts must support both Linux and macOS ARM64

## Constitution Check

| Principle / Gate | Status | Evidence / Notes |
|---|---|---|
| **I. Library-First** | PASSED | Provider trait, port checks, and doctor logic isolated in `src/lib.rs` |
| **II. CLI Interface** | PASSED | Standard CLI protocols with stdout output, stderr errors, and POSIX exit codes |
| **III. Test-First** | PASSED | Test scenarios defined for port collision, tool detection, and argument parsing |
| **IV. Integration Testing** | PASSED | Subprocess integration tests validating exit codes and CLI contracts |
| **V. Simplicity / YAGNI** | PASSED | Direct execution of native CLIs avoids complex cloud SDK dependencies |

## Project Structure

### Documentation (this feature)

```text
specs/001-k8s-multi-cloud-tunnel/
├── plan.md              # Implementation plan (this file)
├── research.md          # Technical research & provider command options
├── data-model.md        # Domain entities (Provider, TunnelConfig, PrerequisiteStatus)
├── quickstart.md        # Build, cross-compilation, and verification guide
├── contracts/
│   └── cli-contract.md  # CLI command contract and exit codes
└── tasks.md             # Actionable task breakdown (Phase 2 output)
```

### Source Code Layout

```text
Cargo.toml               # Crate configuration and dependencies
src/
├── lib.rs               # Library root: domain models, port checking, prerequisite checking
├── main.rs              # CLI entry point, argument parsing, command execution
└── providers/
    ├── mod.rs           # CloudTunnelProvider trait and factory
    ├── aws.rs           # AWS SSM Session Manager tunnel builder
    ├── gcp.rs           # GCP IAP start-iap-tunnel command builder
    └── azure.rs         # Azure Bastion and Arc tunnel command builder
tests/
├── cli.rs               # Subprocess CLI integration tests
└── port_checker.rs      # Port conflict unit tests
```

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|---|---|---|
| *None* | N/A | Architecture is clean, focused, and minimal |
