# Code Signing Policy

## Signing Provider

Free code signing for Bóveda is provided by the [SignPath Foundation](https://signpath.org) through [SignPath.io](https://signpath.io).

> **Note:** The code signing certificate is issued in the name of the **SignPath Foundation**, not the individual project. Bóveda commits to complying at all times with the Foundation's [Terms of Use](https://signpath.org).

---

## Scope

This policy applies to all binary artifacts officially distributed under the name **Bóveda**, including:

- Installers for **Windows** (`.msi`, `.exe`)
- Packages for **Linux** (`.deb`, `.AppImage`, `.rpm`)

---

## Signing Process

All signed binaries are generated **exclusively** through our CI/CD pipeline on GitHub Actions (see `.github/workflows/release.yml`). The process guarantees:

1. **Reproducible builds:** The compiled source code originates directly from the tagged commit in the public repository (`https://github.com/aaronlomas/boveda`).
2. **No manual intervention:** No developer has direct access to the signing keys. Signing is performed automatically by SignPath.io upon approval from a designated *Approver*.
3. **Approval required:** Each signing request requires explicit manual approval by a team member with the **Approver** role before the certificate is applied.

---

## Team Roles


| Role | Description | Responsible |
| :--- | :--- | :--- |
| **Author** | Can write and submit code to the repository. | [@aaronlomas](https://github.com/aaronlomas) |
| **Reviewer** | Reviews and approves Pull Requests before merging. | [@aaronlomas](https://github.com/aaronlomas) |
| **Approver** | Authorizes code signing requests on SignPath. | [@aaronlomas](https://github.com/aaronlomas) |

> All team members use **Multi-Factor Authentication (MFA)** on both their GitHub and SignPath accounts.

---

## Privacy Policy

Bóveda **does not transfer user information** to any networked system unless explicitly requested by the user. See [PRIVACY.md](./PRIVACY.md) for more details.

---

## Integrity and Verification

To verify that an artifact was officially signed by Bóveda through the SignPath Foundation:

- **Windows:** Right-click on the installer → Properties → Digital Signatures. The signer must be `SignPath Foundation`.
- SHA-256 *checksums* for each release are published in the [GitHub Releases](https://github.com/aaronlomas/boveda/releases) section.

---

## Compliance Commitment

The Bóveda team commits to:

- Immediately notifying the SignPath Foundation if any misuse of the certificate is detected.
- Fully cooperating with any investigation initiated by the Foundation.
- Not signing software that does not originate directly from the public source code of this repository.
- Keeping this policy updated following any changes in the team or the release process.

---

*Last updated: 2026-06-01*
