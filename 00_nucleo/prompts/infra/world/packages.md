# L0 — infra/system-world/packages
Hash do Código: b8719c73

**Layer:** L3
**Exclusive consumer:** `03_infra/src/world/packages.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/network/custom-ca-cert.toml sha256:4b9f69ff41790a55a5b2d6a7b9982da858092e4bb2f0c503d6332a14a510e3ec
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


## Fallible Source and file-registration integration

Human approval on 2026-10-22 is retained in
`lab/results/panic-elimination-2026-10-22/source-contract-approval.json`;
its review SHA-256 is `2c879e8afa66721a8952dc8b9e7cf3c92dce58bf91aae5e28da5d603f5aa4d42`.
Source::{new,new_with_parser,detached,detached_with_parser} now return the existing
SourceResult<Source>; SystemWorld::register_file returns FileResult<FileId>.
Preserve this unit's successful content/id/span/order/diagnostic behavior while
propagating construction/registration failures through its existing error channel.
A tuple-returning entry returns its error in the existing result component and
retains already accumulated warnings. A synthetic source slot stores a failed
construction as FileResult rather than inventing a Source. Test-only fixture
helpers must explicitly require their valid construction/registration result;
all previous substantive assertions remain. No productive unwrap/expect, panic
catch, default Source, identity reuse or silent error is permitted for this migration.
This clause supersedes only the old infallible return/panic expectations for the
five approved APIs; it does not authorize unrelated signatures, fields, phases
or broadening/loss of existing diagnostics or negative controls.
