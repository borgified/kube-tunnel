# Project TODOs and Future Research

## IBM Cloud Support Evaluation

**Date Evaluated**: 2026-11-17  
**Status**: Deferred / Not Implemented

### Context
IBM Cloud does not offer a native, agent-based CLI tunneling mechanism equivalent to:
- AWS Systems Manager (SSM) Session Manager
- Google Cloud Identity-Aware Proxy (IAP) TCP forwarding
- Azure Bastion Native Client or Azure Arc

### Current State
Connecting to private Kubernetes endpoints in IBM Cloud currently requires:
1. VPC Client-to-Site VPN connections (OpenVPN)
2. Traditional SSH bastion servers (jump boxes) with static keys

Because `kube-tunnel` focuses on zero-bastion native cloud provider CLI tooling, IBM Cloud is not implemented at this time.

### Future Outlook
Re-evaluate IBM Cloud support if IBM introduces a native, identity-aware CLI port-forwarding tool in the future.
