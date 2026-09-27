# L0 — infra/system-world/packages
Hash do Código: f8907a88

**Layer:** L3
**Exclusive consumer:** `03_infra/src/world/packages.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/network/custom-ca-cert.toml sha256:0b28776068ad6b8e85a028a26cfc359679770b03f76d250bfd3ad68ff72643e3
```

## Current evidence

- `03_infra/src/world/packages.rs:18` — `package_candidate_dirs`.
- `03_infra/src/world/packages.rs:40` — `default_package_cache_dir`.
- `03_infra/src/world/packages.rs:46` — `with_custom_ca`.
- `03_infra/src/world/packages.rs:60` — `package_root_of`.
- `03_infra/src/world/packages.rs:79` — `package_identity_of`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/world/packages.rs`.

## Owned contract

- Own local package roots, manifest entrypoint loading, preview download fallback and the custom-CA builder; use the existing package-downloader owner for network/TLS behavior.
- Search XDG data (or HOME/.local/share) before XDG cache (or HOME/.cache), under typst/packages. Default download cache uses the last available base; absent configurable base leaves downloader None.
- Search full namespace/name/version identity and accept the first existing directory. Invalid manifest in that directory fails, not fallback to another copy. Load typst.toml entrypoint through registered source identity/cache.
- Download only preview after exhausting local roots. Propagate downloader errors; preserve `package not found (searched for {spec})` and physical identity/prefix policy without extra canonicalization.
- with_custom_ca reconstructs the downloader but does not read the certificate eagerly. Consume the pinned custom-CA nucleus; do not implement TLS in World or wire secret bytes into L1.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.
