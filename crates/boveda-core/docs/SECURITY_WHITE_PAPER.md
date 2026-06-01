# 🛡️ Bóveda Core: Security White Paper

This document details the security architecture, cryptographic design, and memory hardening measures implemented in **Bóveda Core**. Our goal is to provide full transparency for users and security auditors.

---

## 1. Security Philosophy: "Security by Isolation"

Bóveda Core is designed around the principle of strict process isolation. The core engine functions as a black box that handles secrets exclusively within protected memory, exposing a minimal interface (Facade) to radically reduce the attack surface. The system is entirely **Zero-Knowledge**: all data never leaves the user's device, and the developer has no technical means to recover master passwords.

## 2. Cryptographic Stack

Bóveda utilizes modern, resilient, and high-performance cryptographic primitives:



| Function | Algorithm | Rationale |
| :--- | :--- | :--- |
| **KDF (Key Derivation Function)** | Argon2id | Winner of the Password Hashing Competition; highly resistant to GPU/ASIC brute-force attacks. |
| **Symmetric Encryption** | ChaCha20-Poly1305 | Authenticated Encryption with Associated Data (AEAD); extremely fast and secure, preferred over AES on systems lacking dedicated hardware acceleration. |
| **Database Encryption** | SQLCipher (AES-256-GCM) | Industry standard for transparent SQLite database encryption featuring page-level integrity verification. |
| **Entropy Generation** | `rand::rngs::OsRng` | Leverages the operating system's cryptographically secure pseudorandom number generator (CSPRNG). |

---

## 3. Key Management and Hierarchy

### 3.1 Master Key Derivation
The master password is never stored anywhere on the system. Upon unlocking the vault:
1. The `vault.salt` is read (32 random bytes generated during initialization).
2. **Argon2id** is executed to derive a 256-bit key.
3. Default parameters are optimized for local desktop security:
   - Time: 3 iterations.
   - Memory: 64 MB.
   - Parallelism: 4 threads.

### 3.2 In-Memory Key Protection
Once derived, the **Master Key** is wrapped inside a `MasterKey` struct that strictly enforces:
- **mlock / VirtualLock:** Locks the underlying physical memory pages to prevent the operating system from moving the key into the swap file or pagefile on the disk.
- **Zeroization:** Physically overwrites the key's raw bytes with zeros the exact moment the struct goes out of scope (`Drop`).

---

## 4. Process Hardening

### 4.1 Anti-Forensics (Linux/Unix/Windows)
Upon startup, Bóveda executes preventive security measures at the OS level:
- **Linux:** Calls `prctl(PR_SET_DUMPABLE, 0)` to disable core dumps and prevent unprivileged processes from attaching via `ptrace`.
- **Windows:** Configures `SetErrorMode` to prevent system error dialogs that could inadvertently write crash dumps to disk.

### 4.2 Transparent and Double-Layer Encryption
The entire SQLite database file is encrypted by SQLCipher. Additionally, Bóveda enforces a **second layer of encryption** (Application-Layer Encryption): every sensitive field (username, password, notes, URL) is individually encrypted using ChaCha20-Poly1305 before database insertion. Even in the hypothetical event of an SQLCipher key compromise, the underlying secrets remain fully protected.

---

## 5. Multi-Factor Authentication (TOTP)

Bóveda Core supports TOTP (RFC 6238) as an optional second factor for vault decryption:
- The TOTP secret is stored encrypted using the user's Master Key.
- Single-use **recovery codes** are generated and stored under encryption to prevent permanent lockouts in case the authentication device is lost.

---

## 6. Traceability and Audit Logs (SOC2 Readiness)

Every sensitive operation is recorded in an immutable audit log stored directly within the encrypted database:
- Successful and failed unlock attempts.
- Secret exposure events (every time a user reveals a password).
- Data import and export actions.
- Changes to core security settings.

---

## 7. Export Format (.bvda.pack)

The export format is an encrypted binary container that utilizes:
- An independent export key derived using a fresh cryptographic salt and a specific export password.
- An in-memory data pipeline that decrypts and re-encrypts records on the fly, guaranteeing that secrets never touch the disk layout in plaintext during the export process.

---

## 8. Security Considerations
- **Side-Channel Resistance:** Constant-time comparison primitives (via the `subtle` crate) are used for critical authentication and TOTP verification flows.
- **Data Validation:** All inputs are strictly sanitized and validated to mitigate injection attacks or buffer overflows before reaching the cryptographic sub-layer.
