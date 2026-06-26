# Privacy Policy

**Bóveda** is a local credential manager designed with the principle of **digital sovereignty**.

## Data Collection

Bóveda **does not collect, transmit, or store data outside of your device**.

- All information you enter (credentials, PINs, notes, documents) is stored **exclusively on your local system**, encrypted at rest with AES-256-CBC + ChaCha20-Poly1305.
- The application **does not make network connections** on its own initiative. There is no telemetry, no analytics, and no cloud synchronization servers.
- This program will not transfer any information to other networked systems unless specifically requested by the user or the person installing or operating it (for example, exporting a backup file).

## Data at Rest

Data is protected using:

- **SQLCipher** with AES-256-CBC for full database encryption (schema, indexes, and content).
- **ChaCha20-Poly1305** (AEAD) for individual encryption of each secret.
- **Argon2id** for key derivation from the master PIN.

## Third Parties

Bóveda does not integrate third-party SDKs that collect data. Project dependencies are open-source and auditable in `Cargo.toml` and `package.json`.

## Code Signing

Free code signing for Bóveda is provided by the [SignPath Foundation](https://signpath.org). Please review our [Code Signing Policy](./CODE_SIGNING_POLICY.md).

