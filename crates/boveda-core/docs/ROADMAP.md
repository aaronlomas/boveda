# Bóveda-Core Roadmap

This document outlines the strategic vision and technical goals for the development of **Bóveda Core**. The objective is to become the open-source standard for local and cross-platform secret management.

---

## 🎯 Phase 1: Stabilization and Trust (Q3 2026)
*Focus: Code quality, internal auditing, and robustness.*

- [ ] **Test Automation (CI/CD):**
    - [x] Integrate `cargo test`, `cargo clippy`, and `cargo audit` into GitHub Actions to block unsafe code.
    - Code coverage reports exceeding 90%.
- [ ] **Fuzz Testing:** Implement `cargo-fuzz` on the decryption engine and database processing to detect rare memory errors.
- [x] **Security Whitepaper:** Comprehensively document the cryptographic design, key hierarchy, and memory hardening measures.
- [x] **Error Refactoring:** Complete the migration to structured and localized errors across all core modules.

## 🚀 Phase 2: Ecosystem Expansion (Q4 2026 - Q1 2027)
*Focus: Portability and access from multiple interfaces.*
- [x] **Armored Export Container:** Implement a strictly encrypted export format (.bvda.pack) with a zeroized memory pipeline, rejecting plaintext formats (CSV/JSON) to ensure forensic integrity.

## 🛡️ Phase 3: Advanced Hardening and Pro Features (Q2 2027+)
*Focus: Hardware integration and enterprise-level features.*

- [ ] **Hardware Security Integration (HSM):** Native support for hardware keys (YubiKey) and Secure Enclave / TPM for vault unlocking.
- [x] **Software Security (U2F):** Implement support for TOTP (Google/Microsoft Authenticator) as an additional unlocking step, featuring a setup interface with QR generation and a warning regarding dependency on external ecosystems.
- [ ] **Native Biometrics:** Windows Hello, TouchID, and FaceID unlocking integrated directly into the engine logic.
- [ ] **Optional E2EE Synchronization:** Cross-device synchronization protocol with end-to-end encryption, ensuring the server never has access to the data.
- [ ] **External Audit:** Submit the codebase to a professional security audit by an independent firm.

---

> **Note:** This roadmap is a living document and may change based on community needs and advancements in the cybersecurity field.
