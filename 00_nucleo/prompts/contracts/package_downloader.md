# L0 — Package download boundary
Hash do Código: 57ae4d5a

**Layer:** L1
**Exclusive consumer:** `01_core/src/contracts/package_downloader.rs`

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/packages/downloader-contract.toml sha256:1fe7c4c5a8dff2081396620797bf0f94b051128c75fcaf306d8a691a50d4d91a

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
