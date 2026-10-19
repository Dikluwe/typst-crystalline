# L0 — infra/font_metrics/fallback
Hash do Código: 24aa0b1a

**Layer:** L3
**Exclusive consumer:** `03_infra/src/font_metrics/fallback.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/fallback-selection.toml sha256:300b35e6b7c70e79742d5edc4cda0f3419b15a41662e194fd1516b33b8cae1bb
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

## Rustdoc structure and explicit fixture initialization

### Evidence

`lab/results/construction/diagnostics/infra-imports-docs-ci-2026-10-13/before.json.gz`
retains sources/owners/spans/original compiler output, HEAD
`06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/UTC/status/diffstat.
fallback.rs:638–639 has selected warnings. Reference inspection distinguishes
productive references, descendant-test-only imports and comments; this is no new
parity measurement.

`field-reassign-2026-10-15/before.json.gz` retains sources/spans/prior owners,
HEAD/tree/UTC/diffstat, and initial-diagnostics.json records fallback.rs:759,896,
1041, 1160, 1225, 1263, 1322 constructing default test styles then assigning fields.
This is Rust representation, no new parity or product-default change.

### Contracts and acceptance

Repair Rustdoc paragraph/list separation/continuation while preserving technical
content and every test assertion/case. The textual “mais” between the two helpers
becomes conjunction “e”, without starting a Markdown list. No public signature,
default, phase, formula, acceptance or lint-suppression change. Run all-target
compile, infra test inventory/execution, build/workspace Clippy/Rustdoc/lineage,
owner-scoped resealing; no import-removal/comment-format mirror tests.

Initialize contiguous preparation fields directly in literals for the seven styles:
math; font/size; font/size/lang/dir; font/size/weight. Preserve each expression/order,
style_800 clone and later weight mutation, MATH/fonts/kerning/RTL and every shaping
control. Use ..Type::default() for remaining fields; remove mut only where no mutations
remain. No new types/builders/APIs/lint suppression. L0 first and scoped resealing,
continuous ADR-0127. Selected gate must fail before and pass after; preserve test
names/inputs/expectations/tolerances. Revalidate all-target Clippy/workspace suite/
build/fmt/Rustdoc/lineage. No tests merely mirroring initializer syntax.
