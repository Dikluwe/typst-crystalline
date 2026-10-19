# L0 — Runtime configuration snapshot
Hash do Código: 9f0e9425

**Layer:** L3
**Exclusive consumer:** `03_infra/src/runtime_info.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/shell/info-projection.toml sha256:f7f63d22ab1b490c71f5e93f5fdf22da8e068dc15dfe1f9aa9cdb5fd7f95ab4d
```

## Current evidence

- `03_infra/src/runtime_info.rs` — `RuntimeInfoSnapshot` and `snapshot`.

These are current local source anchors, not a new upstream parity measurement.

## Contract

Capture configuration only. Resolve package-data paths from XDG_DATA_HOME, otherwise HOME/.local/share; resolve package-cache paths from XDG_CACHE_HOME, otherwise HOME/.cache; append `typst/packages` when a base exists. Split `TYPST_FONT_PATHS` with the platform path-list convention.

Keep the ordered environment allowlist `TYPST_FEATURES`, `TYPST_FONT_PATHS`, `TYPST_ROOT`, `XDG_CACHE_HOME`, `XDG_DATA_HOME`, retaining absent values. Record only whether `TYPST_CERT` exists; never expose certificate values/bytes or proxy settings. Do not contact the network, load certificates, scan fonts or create directories. L4/L2 own composition and presentation under `00_nucleo/prompts/wiring/info.md`.
