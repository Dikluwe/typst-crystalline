# L0 — Package download boundary
Hash do Código: 18ddb1fc

**Layer:** L1
**Exclusive consumer:** `01_core/src/contracts/package_downloader.rs`

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/packages/downloader-contract.toml sha256:46d1fc7a1a1c99a876e6cfcadf2e6ff1793eb840b944da823ba717d5a1a7adf2

## Contract

`PackageDownloader: Send + Sync` exposes
`download(&self, spec: &PackageSpec) -> Result<PathBuf, PackageDownloadError>`.
Preserve the requested namespace, name and version. The result identifies the
local extracted package root. `PackageDownloadError` distinguishes
`NotFound(PackageSpec)`, `VersionNotFound { spec, latest }`,
`NetworkError { url, cause }` and `IoError { path, cause }`.
Its observable `Display` messages must not be translated by this documentary
change. No HTTP/runtime error type crosses the boundary.
Network, filesystem operations, caching, extraction and TLS belong to L3;
this owner declares their pure interface, not their implementation.

## Unused local helper boundary

### Evidence before decision

`01_core/src/contracts/package_downloader.rs:93` contains the items flagged in the historical Clippy inventory. At HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, the working tree was uncommitted; sources, owners, references, UTC, status and diffstat are recorded in `lab/results/construction/diagnostics/helpers-testes-ci-2026-10-13/before.json.gz`. Call-site research and productive paths precede this classification; this is neither a new parity measurement nor performance evidence.

### Contract and acceptance

Remove only DummyDownloader and its never-instantiated local implementation in the test module. Preserve the public trait, bounds, signature, errors and Display tests; do not create a test that merely confirms its own mock.

Preserve productive paths, existing test names and bodies, external APIs, defaults and phase. This internal correction follows ADR-0127 continuous flow. Revalidate build and check for all workspace targets, the core suite, test inventory, Clippy and lineage; do not remove expectations to pass the gate.
