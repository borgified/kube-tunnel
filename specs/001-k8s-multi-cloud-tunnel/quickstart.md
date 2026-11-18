# Quickstart Validation Guide: Private Kubernetes Multi-Cloud Tunneling

This guide describes how to build, cross-compile, and validate `kube-tunnel`.

## 1. Local Build (Host: Linux x86_64)

```bash
cargo build --release
./target/release/kube-tunnel --version
```

---

## 2. Cross-Compilation for macOS Apple Silicon (ARM64)

Add the target and build using `cargo-zigbuild` or standard cargo cross-compilation:

```bash
rustup target add aarch64-apple-darwin
cargo build --target aarch64-apple-darwin --release
```

**Verification**:
```bash
file target/aarch64-apple-darwin/release/kube-tunnel
# Expected output: Mach-O 64-bit arm64 executable
```

---

## 3. Validation Scenarios

### Scenario A: Run Environment Diagnostics
```bash
cargo run -- doctor
```
**Expected**: Lists installed provider CLIs (`aws`, `session-manager-plugin`, `gcloud`, `az`) with status ticks/crosses.

### Scenario B: Port Collision Detection
Run an existing service on port 8443, then test:
```bash
cargo run -- connect --provider aws --target i-0123456789 --local-port 8443
```
**Expected**: Rejects immediately with code 3 stating local port 8443 is in use.

### Scenario C: Clean Signal Interruption
Establish a tunnel and press `Ctrl+C`:
**Expected**: Emits shutdown notice, kills child provider process, exits with code 0.
