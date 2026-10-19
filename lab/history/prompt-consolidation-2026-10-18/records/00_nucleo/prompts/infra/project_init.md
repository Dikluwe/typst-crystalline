# L0 — Project template materialization
Hash do Código: 66ea9fcf

**Layer:** L3
**Exclusive consumer:** `03_infra/src/project_init.rs`

## Evidence and ownership

`03_infra/src/project_init.rs:22` implements initialization, manifest validation and staged copying; `:96` implements package resolution. Parsing belongs to `00_nucleo/prompts/shell/cli.md`; command publication belongs to `00_nucleo/prompts/wiring/init.md`. The directory containing this L0 does not change the consumer's L3 layer.

Package/version resolution is referenced canonically at `lab/history/construction/retired-l0/infra/package_version_resolution.md`. This prompt does not share ownership of that consumer. Observable selection, destination safety and errors are contracts; downloader internals are not a requirement to reproduce vanilla mechanics.

## Public contract

```rust
pub struct InitResult {
    pub destination: PathBuf,
    pub entrypoint: PathBuf,
    pub spec: PackageSpec,
}
pub fn initialize(
    template: &str,
    destination: Option<PathBuf>,
    custom_ca: Option<PathBuf>,
) -> Result<InitResult, String>;
```

The CLI form is `typst init TEMPLATE [DIR]`. Accept explicit `@namespace/name:version` and versionless `@namespace/name`. Explicit versions search configured local package roots before downloading a missing preview package. Versionless preview resolution queries the remote index through the configured downloader; it does not infer the latest preview version from the local cache. Versionless non-preview resolution selects the greatest parseable package version under the data root, ignoring malformed names. Missing data/cache/package state yields the established error. Apply the custom CA selected by the CLI; do not expose its path or bytes through info.

Read and parse `typst.toml`, require a valid `[package]` identity matching requested name/version, and require `[template]` with string `path` and `entrypoint`. Both paths must be nonempty relative paths without parent/root/prefix components. Canonical template root must be inside the canonical package root; canonical entrypoint must be a file inside the template root.

An omitted destination is the package name. Reject an existing destination; the no-overwrite guarantee applies to that destination, not a promise that package resolution cannot populate a download cache first. Require an existing parent directory and a valid destination leaf name.

Copy only the template tree into an exclusively created sibling staging directory. Reject symbolic links during copying, including internal links; do not weaken this to permitting some symlinks. Complete copying before rename to the destination. Cleanup staging on failure is best-effort; do not leave a deliberately published partial project or overwrite an existing destination. Return the materialized destination, entrypoint and resolved spec.

## Observable errors and command envelope

Preserve source-selected error literals and substitutions, including:

```text
template package has no readable typst.toml
template package has an invalid typst.toml
typst.toml has no [package] section
template manifest identity does not match the requested package
package does not contain a [template] section
[template].{key} is missing
[template].{key} must be a relative path inside the package
template package directory is not accessible
template path does not exist
template path escapes the package root
template entrypoint does not exist
template entrypoint escapes the template directory
destination already exists: {path}
destination parent does not exist: {path}
destination must have a directory name
failed to create staging directory: {error}
failed to finalize project: {error}
failed to read template: {error}
failed to inspect template: {error}
failed to create template directory: {error}
failed to copy template file: {error}
template contains a symbolic link; refusing unsafe materialization
package cache directory is unavailable
package data directory is unavailable
package not found (searched for {spec})
```

Package-parser/downloader errors are propagated rather than translated by this owner. L4 success exits 0 and prints these newline-terminated lines:

```text
Successfully created project in {destination}
Entrypoint: {entrypoint}
```

Initialization failure prints `error: {error}` to stderr and exits 1; argument parsing failures exit 2.

## Acceptance and exclusions

Use local template fixtures for explicit/versionless selection, default destination, matching identity, template-only copying, existing destination refusal, missing template, unsafe paths/symlinks and cleanup after a mid-copy failure. Test preview latest-version selection independently of cache contents. Preserve established messages; machine-dependent paths are substituted, not frozen globally.

No filesystem logic enters L1 or L2. CLI changes, downloader implementation and changes to public acceptance/defaults require their respective owners and ADR-0127 approval. This specification does not add a command or reopen an implementation gate.
