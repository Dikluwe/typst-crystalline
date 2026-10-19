# L0 — Read-only runtime information presentation
Hash do Código: b2e57126

**Layer:** L2
**Exclusive consumer:** `02_shell/src/info.rs`

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/shell/info-projection.toml sha256:38cf0e0cd41983e89af19f163e9203e2e97cbea25b12a3a7fe6c68c915294557

## Evidence and ownership

`02_shell/src/info.rs:12` defines the DTO; `:56` and `:64` implement JSON and human presentation. Parsing belongs to `00_nucleo/prompts/shell/cli.md`, runtime snapshot collection to `00_nucleo/prompts/infra/runtime_info.md`, and composition to `00_nucleo/prompts/wiring/info.md`. Shared projection/security requirements come from the pinned nucleus.

Schema, categories and output labels are public contracts. Machine-dependent values may vary. The observation of current output does not establish full parity or authorize reducing the nucleus's category requirements.

## Public API and schema

`format_json(info: &InfoData, pretty: bool) -> Result<Vec<u8>, String>` emits UTF-8 JSON with a final newline. Compact versus pretty changes presentation only; build commit remains complete or null.

`format_human(info: &InfoData) -> Vec<u8>` emits the human projection without I/O.

Preserve the public DTOs and these serialized fields/types:

- `version`: string.
- `build.commit`: optional string; `build.platform.os` and `.arch`: strings.
- `features.html`, `features.a11y_extras`, `features.bundle`: booleans. The underscore in `a11y_extras` is the actual JSON spelling.
- `fonts.system`: boolean; `fonts.font-paths`: string array.
- `packages.data-path` and `.cache-path`: optional strings; `.custom-ca-configured`: boolean.
- `env`: ordered map from allowlisted variable names to optional strings.

The structs are `InfoData`, `BuildInfo`, `PlatformInfo`, `FeatureInfo`, `FontInfo` and `PackageInfo`; fields retain their current Rust names and types. Serde uses kebab-case for font/package fields, not feature fields.

## Human literals

Preserve labels, indentation, order and blank sections:

```text
Version: {version}
Commit: {short_commit}
Platform: {os} {arch}

Features:
  HTML: {html}
  A11y extras: {a11y_extras}
  Bundle: {bundle}

Fonts:
  System: {system}
  Path: {path}

Packages:
  Data path: {data_path}
  Cache path: {cache_path}
  Custom CA: {custom_ca_configured}
```

Repeat `  Path: {path}` for supplied font paths; omit it for an empty list. Booleans use `true`/`false`. Commit is its first eight bytes, or the whole shorter value; an absent commit displays `unknown`. Absent package paths display `unavailable`. Output ends with newline.

The current human formatter does not print `env`. The prior same-categories acceptance and pinned nucleus must not be silently weakened to approve that omission; the required reconciliation is recorded in `lab/results/construction/diagnostics/sanitization-shell-history.md`.

## Command and collection boundary

`typst info` selects human output. `typst info -f json` or `--format json` selects JSON; `--pretty` requires that format. Other formats are not advertised or accepted.

Collection is read-only: do not download packages or trigger a complete font scan. L3 resolves Typst package paths from XDG locations, falling back to HOME-based locations, and supplies configured font paths. The environment allowlist is `TYPST_FEATURES`, `TYPST_FONT_PATHS`, `TYPST_ROOT`, `XDG_CACHE_HOME`, `XDG_DATA_HOME`; absent or non-UTF-8 values are null. Do not expose proxy credentials, certificate paths/bytes or arbitrary environment values. Custom CA is represented only by a boolean.

The current command has no feature-selection option, so effective feature booleans are false and system-font configuration is true. `TYPST_FEATURES` is informational here, not an activation source.

Success exits 0. JSON serialization failure and stdout write failure exit 2, with respectively `error: failed to serialize info: {error}` and `error: failed to write info: {error}`.

## Acceptance

Verify schema/spelling/types, compact/pretty value equivalence, complete JSON commit, human fallback literals, supplied XDG paths, allowlist stability and secret exclusion. Preserve projection-category obligations pending the recorded decision; do not claim all vanilla info formats are covered.
