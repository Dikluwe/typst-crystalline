# L0 — infra/font_metrics/resolution
Hash do Código: a3cf993c

**Layer:** L3
**Exclusive consumer:** `03_infra/src/font_metrics/resolution.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/fallback-selection.toml sha256:300b35e6b7c70e79742d5edc4cda0f3419b15a41662e194fd1516b33b8cae1bb
- 00_nucleo/prompts/_nuclei/fonts/primary-resolution-cache.toml sha256:0ac6e8504aee9a06e2ae9ffc717bb4f878a3b71d37d1851fda9b93a93aacd39c
```

## Current evidence

- `03_infra/src/font_metrics/resolution.rs:17` — `FontCandidate`.
- `03_infra/src/font_metrics/resolution.rs:27` — `resolve_primary`.
- `03_infra/src/font_metrics/resolution.rs:86` — `resolve_primary_with_math_fallback`.
- `03_infra/src/font_metrics/resolution.rs:118` — `covering`.
- `03_infra/src/font_metrics/resolution.rs:193` — `resolve_font_combo`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/font_metrics/resolution.rs`.

## Owned contract

- Own FontCandidate and primary/MATH/covering/FontKey resolution. Preserve declared-family order, class fallback when primary lookup fails and first-resolved-family similarity reference.
- Append explicit math fallback faces when style.math is true, with slot deduplication; preserve the existing readable-face check without adding a MATH-table filter at insertion. Covering lookup first tries MATH-covering candidates, then primary coverage; preserve None and ordering.
- Global fallback uses World::candidates_for_char and FontBook::select_fallback. Coverage is authoritative for global filtering; load the winning eligible face only, not intervening slots or a second full catalog scan.
- Resolved identity includes family, variant and variations. Do not infer math semantics from family spelling; consume the pinned shared fallback-selection nucleus.

## Verification and boundary

Run `cargo test -p typst-infra --lib font_metrics:: -- --test-threads=1` when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

## Common fallback and bounded primary reuse

### Evidence

resolution.rs:27–65 previously used only a resolved primary, while historical
vanilla text/mod.rs:1083–1101 appends the common chain before global search.
The Hebrew correcao-rtl-restante-2026-09-29/after-leading probe shows a different
emitted face. Inference: independent metrics may diverge without the common chain,
refuted by an already identical measured face without alteration.

`content-cost-profile-2026-10-10/{resolution-calls,cpu-style-current,checked-phases}.json.gz`
retains HEAD/tree/UTC/scripts/sources/commands/release
`9ae0a3d9285b8d0b4bae0da1db058483e7f5543003fabff46bb7d93b17e474f2`.
1, 600-cell tables call select_pattern 48, 019, resolve_primary 6, 401,
text_edges/vertical_metrics 3, 200/3, 201; selection occupies 25–29% inclusive CPU
samples. resolution.rs:27–85 uses FontList/effective variant/face readability, not
size/variations; :105–124 adds MATH separately; facade :49–86 shares clone-local
caches. Reusing the last resolution may amortize scans without growth; altered
identity/chain or lock/retention costs eliminating gain refute the hypothesis.

### Contracts and acceptance

Append the ordered/deduplicated common chain after readable declared families in
text mode; math retains math fallback before common. Coverage measurement and
emission cannot choose different families merely from catalog order.

Define private entry/key and prove exact identity before base-chain lookup. Hit
clones existing candidates; miss runs the unchanged algorithm, publishing only
nonempty results with every selected face readable. Replace the prior entry;
release lock before resolving/loading. Slots/units use original face; variations/
final FontKey resolve on the existing path and MATH additions stay outside this
cache. Consume the pinned nucleus without signature changes.

Continuous ADR-0127: repeated-query RED across clones; family/regex/order/variant,
independent instances/transient failures/math/variation controls against fresh
instances, quantitative allocations/paired-time gate, preceding-release-equivalent
documents/many-identity refuters/core/infra/build/lint. Observe language selection/
metrics/origin/effects/geometry, never copied vanilla mechanics or hash-inferred
hits. Remove on divergence or robust regression.

## Internal casts and fixture initialization

### Evidence

`ci-simplificacoes-2026-10-14/before.json.gz` retains source/owner/spans/HEAD/
uncommitted tree/UTC/diffstat at resolution.rs:66,85,107,154,220; red.log records
selected-lint errors before editing. Uses/types were checked under pinned 1.92.0.
`field-reassign-2026-10-15/before.json.gz` retains source/spans/prior L0/HEAD/tree/
UTC/diffstat; initial-diagnostics.json records default-then-assignment test fields
at :302, 426, 555, 598. Both are Rust-representation maintenance, no new parity or
product-default change.

### Contracts and acceptance

units_per_em already returns u16: retain max(1), remove only u16→u16 casts in
primary/common/MATH/covering chains. Keep order/deduplication/face identity/cache
eligibility. No public/default/phase/unsafe/dependency/suppression changes. L0
first, owner-only resealing; existing functional controls with exact fixtures/
expectations/tolerances, selected-lint RED→GREEN as errors, full suite/build/
all-targets/Rustdoc/fmt/lineage/full inventory; no simplification-mirror tests.

Initialize font, plus math in the two math controls, directly in four TextStyle
literals. Keep families/mocks/query counters/MATH-face selection/fallback checks
and productive algorithm. Use ..Type::default() for other fields, remove mut only
when no mutations remain; no new types/builders/APIs/suppression. Preserve names/
inputs/expectations/tolerances; selected RED→GREEN/all-target Clippy/workspace
suite/build/fmt/Rustdoc/lineage/scoped resealing, no initializer-syntax tests.
