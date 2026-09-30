# L0 — Versionless package resolution boundary
Hash do Código: caeadbbd

**Layer:** L3 reference; ownership unresolved
**Ownership boundary:** reference document; no exclusive Rust consumer declares this prompt.

## Current evidence

- `03_infra/src/project_init.rs:96` and `:133` — template package and omitted-version resolution; its declared owner is `00_nucleo/prompts/shell/init.md`.
- `03_infra/src/package_downloader.rs:132` — remote index resolution, owned by `00_nucleo/prompts/infra/package_downloader.md`.
- `01_core/src/entities/package_spec.rs` — versioned/versionless pure package types.

These are current local source anchors, not a new upstream parity measurement.

## Contract

This reference does not independently authorize another consumer. The existing code is owned by the two canonical prompts above. Preserve the existing ownership gap for investigation in `00_nucleo/diagnosticos/sanitization-infra-wiring-history.md`; resolving it requires work outside this documentary scope, not a fictitious consumer declaration.

For `typst init` with an omitted version, the initialization owner calls the downloader for `preview`, using the remote registry index rather than choosing the latest cached local preview version. A failed index request fails resolution. For another namespace, initialization enumerates only the package data directory, ignores entry names that do not parse as PackageVersion, and chooses the maximum numerical `(major, minor, patch)` tuple. Cache roots are not a local latest-version source.

PackageVersion contains three unsigned components; prerelease strings are not valid versions. Malformed local entries are skipped without additional diagnostics. Explicit versions retain the initialization owner's local-root search before download.

`#import` package syntax still requires a version. Do not widen the import parser, change PackageSpec or introduce a new resolution trait. Remote index fetching and CA configuration belong to the downloader; template filesystem work belongs to initialization. A per-SystemWorld latest-version/index cache is not part of the current consumer; the unresolved cache proposal is recorded in the diagnostic above.

## Verification boundary

The current initialization suite controls local maximum-version selection. Network-error/index-absence behavior follows the downloader. This reference is not proof of one-to-one lineage or fresh upstream parity.
