# Code-signing policy

## v0.1.0

The first release is intentionally unsigned. Windows SmartScreen may warn because reputation and Authenticode signing are separate from source availability. Users should download from the canonical GitHub Release, compare the SHA-256 checksum, and verify the GitHub build-provenance attestation.

Checksums detect changed bytes and attestations tie an artifact to this repository's GitHub Actions workflow. Neither suppresses SmartScreen.

## Later releases

After the first release is stable, the project intends to apply for SignPath's open-source signing program. If approved:

1. GitHub Actions will produce the locked release binary.
2. The portable executable will be signed before the direct EXE and ZIP are packaged.
3. The installer will be built from that signed executable and then signed separately.
4. Publisher identity and certificate details will be documented here before use.

Private signing keys must never be exported into the repository or ordinary Actions secrets. A signing service must authorize immutable workflow output and preserve provenance.
