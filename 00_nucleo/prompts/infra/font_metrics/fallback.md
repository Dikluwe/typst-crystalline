# L0 — infra/font_metrics/fallback
Hash do Código: 9d64a46b

**Layer:** L3
**Exclusive consumer:** `03_infra/src/font_metrics/fallback.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/fallback-selection.toml sha256:faf6c20021b467fdb2a864625f4f6dd4386b5ef28c137b946e5cb4e4ce073d52
```

## Current evidence

- `03_infra/src/font_metrics/fallback.rs:23` — `face_kerning`.
- `03_infra/src/font_metrics/fallback.rs:46` — `advance`.
- `03_infra/src/font_metrics/fallback.rs:168` — `advance_shaped`.
- `03_infra/src/font_metrics/fallback.rs:184` — `vertical_metrics`.
- `03_infra/src/font_metrics/fallback.rs:226` — `cap_height`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/font_metrics/fallback.rs`.

## Owned contract

- Own FontMetrics for FallbackFontMetrics and face_kerning, using the resolution/cache children. Resolve per character and apply style variations to a cloned face, never mutate the cached base.
- The explicit `FontMetrics` size argument governs advance and shaped advance, including scaling and cache identity. For shaping, use a local style with that size when it differs from `style.size`; preserve all other style fields and the caller's style.
- Preserve legacy kern then kerx within one face, not cross-font kerning or a new GPOS path. advance_shaped delegates to the shaper with the shared face cache.
- Preserve FixedMetrics/0.6 defaults for unresolved vertical/ink/bbox metrics. Apply canonical math/ssty and TextItem predicates; never remap ink codepoints.
- Math constants come from the first applicable primary with MATH, while variants/assemblies/kern/italic-correction/attachments use the active face. Keep fallback behavior and neutral L1 return types.

## Verification and boundary

Run `cargo test -p typst-infra --lib font_metrics:: -- --test-threads=1` when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.
