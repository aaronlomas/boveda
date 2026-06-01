# Contributing Guide 🤝

Thank you for your interest in improving **Bóveda**! This is a community-driven project dedicated to providing digital sovereignty and uncompromised security. As a credential manager, we maintain exceptionally high quality and security standards.

---

## Our Philosophy: "Security by Isolation"

Before proposing any changes, it is vital to understand our architecture:
- **Isolation:** Core business logic and cryptography reside in `crates/boveda-core`. The UI (Svelte) and the bridge (Tauri) are strictly consumers of the command facade.
- **Zero-Knowledge:** The design must guarantee that neither developers nor third parties can ever access user data.
- **Memory Hardening:** Every secret must be physically overwritten (`Zeroize`) and protected against system memory dumps (`mlock`).

---

## 🚀 Getting Started

### Prerequisites
- **Rust:** (Latest stable version).
- **Node.js & pnpm:** We use `pnpm` for frontend dependency management.
- **System Libraries:** Please check the [README.md](./README.md) for Tauri system dependencies required by your OS.

### Environment Setup
1. Fork the repository.
2. Clone your fork: `git clone https://github.com`
3. Install dependencies: `pnpm install`
4. Run in development mode: `pnpm tauri dev`

---

## 🛠️ Code Standards

### Backend (Rust)
We maintain a **"Zero Warnings"** policy. Your code must pass:
- **Formatting:** `cargo fmt --all`
- **Lints:** `cargo clippy --workspace --all-targets -- -D warnings`
- **Security:** `cargo audit` (whenever new dependencies are added).

**Rules:**
- **Secrets in memory:** Always use `SecretString` or `SecretBytes` wrappers for any sensitive data in memory; these wrappers enforce `Zeroize` and memory protection routines.
- **No serialization of secret wrappers:** Do not derive `Serialize`/`Deserialize` on `SecretString` or `SecretBytes`. If a type requires serialization, implement a custom routine that redacts the value (e.g., outputs `"[REDACTED]"`) to prevent accidental leaks.
- Never print secrets in logs or error messages (use `{:?}` with redacted wrappers).

### Frontend (Svelte 5)
- We use **Svelte 5 (Runes)**. Avoid legacy Svelte 4 patterns.
- Run `pnpm check` to validate types and component consistency.
- Maintain the premium look: rely on the design system driven by the CSS variables in `src/app.css`.

---

## 🧪 Testing

We do not accept PRs that lower test coverage or break the CI pipeline.
- **Unit tests:** `cargo test --package boveda-core`
- **Integration:** Verify that the unlocking and account management flows function flawlessly in `dev` mode.

---

## 📝 Commits

We suggest following the [Conventional Commits](https://www.conventionalcommits.org/) specification:
- `feat:` New feature.
- `ui:` Visual or user experience changes.
- `fix:` Bug fix.
- `refactor:` Code improvements without functional changes.
- `sec/security:` Specific security hardening or cryptographic changes.
- `chore:` Maintenance, tool, or dependency updates.
- `docs:` Documentation.
- `ci:` Continuous integration pipeline adjustments.

---

## 🔒 Reporting Vulnerabilities

If you discover a security flaw, please **DO NOT open a public Issue**. Refer to our [Security Policy](./SECURITY.md) to report it responsibly.

---

By contributing, you agree that your contributions will be licensed under the **AGPL-3.0** license.

