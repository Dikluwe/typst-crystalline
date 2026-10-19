# L0 — Compilation boundary module
Hash do Código: f7fc8bd9

**Layer:** L1
**Exclusive consumer:** `01_core/src/contracts/mod.rs`

## Evidence and classification

`01_core/src/contracts/mod.rs:7-9` declares package_downloader, plugin_host and
world. It contains no World/TrackedWorld algorithm. The archived original
specified an initial World design and tracking experiment; the current pure
World owner is `00_nucleo/prompts/core/contracts/world.md`. Inspection establishes
ownership, not new upstream parity or a newly ratified tracking design.

## Contract and owner boundaries

Own only the public contract-module declarations and their existing paths.
PackageDownloader belongs to
`00_nucleo/prompts/core/contracts/package_downloader.md`; plugin handles, errors
and the PluginHost interface belong to
`00_nucleo/prompts/core/contracts/plugin_host.md`; World, SysInputs and its mocks
belong to `00_nucleo/prompts/core/contracts/world.md`. Preserve those boundaries
without copying implementations or declaring another World interface here.

World is the dependency inversion boundary: L1 declares required environment
capabilities and L3 supplies filesystem, fonts, standard library and date data.
Pure World remains independently mockable; any tracking adaptation must not
force comemo into the pure trait. comemo usage requires the existing whitelist;
all types in a trait signature must already exist in L1.

## Historical constraints and unresolved tracking claim

The archived initial design required empirical verification that a
`#[comemo::track]` TrackedWorld adapter with blanket implementation actually
tracks calls before pipeline use. Compile-time blanket conformance alone is not
runtime tracking proof. If the experiment fails, report the failure and review
the adapter before proceeding. Preserve simple mock verification of library
references, FileError::NotFound, comemo-free World tests and automatic adapter
conformance if that adapter is retained. This hub does not silently ratify or
implement a `TrackedWorld: World` supertrait; the current World owner explicitly
does not prescribe it. The unresolved historical tracking proposal and its
sample signatures remain in the archived original for owner review.

The initial `Option<i64>` date offset was a temporary substitution before
Duration existed; the current World owner's `Option<Duration>` interface governs.
The temporary V11 orphan exception was conditional on absent L3 implementations
and must be removed when SystemWorld or an equivalent implementation exists;
this prompt does not authorize recreating an obsolete exception.

## Acceptance

Preserve module paths, pure mocks and all cross-owner capability contracts.
No duplicate productive owner, runtime implementation, I/O or mutable global
state belongs in this hub. Build and lineage validation must pass. Historical
construction sequence, sample mocks, temporary stubs and experiment details are
preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/contracts/mod.md`.
