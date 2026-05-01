# Bóveda — Password Manager 🔒

Bóveda is a locally-hosted, secure, and modern password manager built with **Tauri**, **Svelte**, and **Rust**. It ensures your data remains purely local and encrypted at rest using industry-standard cryptography.

![License](https://img.shields.io/badge/license-MIT-blue)
![Tauri](https://img.shields.io/badge/Tauri-2.0-orange)
![Svelte](https://img.shields.io/badge/Svelte-5.0-red)
![Rust](https://img.shields.io/badge/Rust-1.70+-black)

## ✨ Features

- **Zero-Knowledge Architecture:** Your master password is never stored anywhere.
- **Strong Encryption:** Uses `AES-256-GCM` for authenticated encryption and `Argon2id` for Key Derivation (KDF) to protect against GPU/ASIC attacks.
- **Glassmorphism UI:** A beautiful, customizable, modern interface built with TailwindCSS.
- **Auto-Lock:** Automatically locks the vault after 5 minutes of inactivity to protect your data if you walk away.
- **Offline First:** SQLite database stored locally on your device. No cloud syncing, no data leaks.
- **Secure Exports:** The `.db` file is fully encrypted. You can safely export and back it up.

## 🏗 Architecture & Security Flow

Understanding the security flow is crucial for contributing:

1. **Vault Initialization:**
   - When setting a master password, we generate a random 32-byte `salt`.
   - We derive a 32-byte `Master Key` using **Argon2id** (`password` + `salt`).
   - We encrypt a static challenge text (`"boveda_auth"`) using **AES-256-GCM** with the derived key.
   - The `salt` and `encrypted_challenge` are stored in the SQLite database (`vault_meta` table).

2. **Unlocking the Vault:**
   - The user inputs their master password.
   - We derive the key using the stored `salt`.
   - We attempt to decrypt the `encrypted_challenge`. If it results in `"boveda_auth"`, the password is correct.
   - The `Master Key` is held in memory (`Mutex<Option<MasterKey>>` in Rust) and used to encrypt/decrypt accounts on the fly.

3. **Locking & Memory Safety:**
   - The `Master Key` uses the `Zeroize` trait. When the vault is locked (either manually or via inactivity), the memory containing the key bytes is immediately wiped.

## 🛠 Project Structure

- `src-tauri/src/commands/`: Tauri IPC commands (Backend logic).
  - `vault.rs`: Unlock, lock, crypto, and account management.
  - `settings.rs`: DB imports/exports, backgrounds, and preferences.
- `src-tauri/src/crypto.rs`: Rust cryptography (Argon2, AES-GCM).
- `src-tauri/src/db.rs`: SQLite interactions using `sqlx`.
- `src/lib/components/`: Svelte UI components (Frontend).
- `src/lib/autoLock.ts`: Inactivity monitor.

## 🚀 Development Setup

**Prerequisites:**
- [Node.js](https://nodejs.org/) (v18+)
- [Rust](https://rustup.rs/) (v1.70+)
- Tauri dependencies (see [Tauri documentation](https://tauri.app/v1/guides/getting-started/prerequisites))

```bash
# 1. Install dependencies
npm install

# 2. Run in development mode
npm run tauri dev

# 3. Build for production
npm run tauri build
```

## 🤝 Contributing

We welcome contributions! Please open an issue first to discuss what you would like to change. Read our `CONTRIBUTING.md` (coming soon) for more details.

## 📜 License

This project is licensed under the MIT License.
