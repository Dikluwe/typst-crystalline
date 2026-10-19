# L0 — infra/pipeline/resources
Hash do Código: 96969b85

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/shaped-face-identity.toml sha256:06ae067763e4c77876f7d999903f2e5e4144df34eb799eb62f2c52a31d8a030b
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:395a968c2fe49429b03b369cd5047df513132bfef6511ec21af466670ec2e147
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:88391e330334a2221170109c82b5dbf1fb60c4d2f6e2660e732df24b67554d9f
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:96c5b3ef9be3d171d3ca4c57f30fe826068c568fc17437f83b2b9b5d7372aeff

**Layer:** L3
**Exclusive consumer:** `03_infra/src/pipeline/resources.rs`

## Current evidence

- `03_infra/src/pipeline/resources.rs:33` — `resolve_and_instantiate_fonts`.
- `03_infra/src/pipeline/resources.rs:93` — `collect_fonts_from_doc`.
- `03_infra/src/pipeline/resources.rs:123` — `collect_fonts_in_items`.
- `03_infra/src/pipeline/resources.rs:172` — `resolve_fonts`.
- `03_infra/src/pipeline/resources.rs:195` — `first_font_from_doc`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/pipeline/resources.rs`.

## Owned contract

- Own ordered document font collection, fallback resolution, complete FontKey identity and variable-font instantiation. Keep first-occurrence deduplication and declared-family order.
- Traverse background, items, foreground and Group/Link/Semantic transparently. Preserve semantic kind/placement/alt; never flatten the wrapper or turn alt into visible text.
- Include Text, TextShaped and math Glyph resources. Resolve a Glyph's actual font via FallbackFontMetrics::resolve_font_combo, not just body style.font, so a companion MATH glyph cannot be omitted from export selection.
- Keys include FontList, FontVariant and explicit FontVariations. Derived axes plus explicit per-tag overrides determine variable-instancing need and separate resources, even if only an explicit axis differs.
- Require the Python/fontTools availability probe only when a merged request contains an axis actually present in the resolved variable face. Requests containing only unknown axes follow the instancing owner's unchanged-bytes contract; preserve the missing-tool error for matching axes. A private local predicate owns this resource guard without exposing a new API.
- An empty collected font set resolves to an empty resource set without requesting World::book or World::font. All PDF, bundle PDF and raster/SVG callers use this same private resolution boundary.
- A family wins only after selection and source loading both succeed; stale/missing sources continue to later families. Preserve unresolved-entry filtering and variable-instancing diagnostics. Keep test-only first-font helpers internal; production selection uses the complete resolved font set.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

## Separator and transparent resource traversal

fatia-funcional-tekt-2026-09-28 RED/handler audit precedes2026-09-28 approved
layout_types carrier. Collect active TextSeparator font with same family/variant/
variation dedup. Approved identidade-hifen-discricionario-2026-09-28 initial/before
state/eightU+002D-SHY equal-bbox controls/approved-before traversal requires visually
transparent TextReplacement descent preserving author text/order/coordinates/
existing calculation without added advance/algorithm. Pinned shared invariants,
text/bbox/causal/recursive/literal-hyphen tests, no PDF-byte/new-selection claim.

## Shared instancer diagnostic

### Evidence and contract

With same Ubuntu Sans variable face and missing TYPST_CRYSTALLINE_PYTHON executable,
pdf.rs:444–467 emits
`fonte variável 'ubuntu sans' requer instanciação, mas Python/fontTools não está disponível`;
bundle.rs:501–518 emits divergent English. resources.rs:39–63 already has Portuguese
for SVG/PNG. Static/no-font export succeeds, real-axis request rejects. Diagnostic
is language-observable, export branch count/PDF bytes mechanics; shared-constructor
hypothesis refuted by measured three distinct call sites.

One private helper decides resolved-font dependency and diagnostic, probes only
if merged request contains actual face axis. Preserve first matching face,
Portuguese literal/detached span/no hint. Standalone PDF/bundle PDF/SVG-PNG share
helper without new public API/font-selection/export dispatch change. Tests may
inject probe result only for deterministic unavailability.

## Borrowed textual key construction

font-keys-prioridade7-2026-10-08/before.json.gz retains HEAD/tree/UTC/released7692649.
resources.rs:127–134 clones Vec-backed FontList/axes before seen.contains for each
Text/TextShaped/TextSeparator. Discarded copies are mechanics; selection order/
full FontList-variant-axes identity stay observable, gain needs measurement.
Compare whole textual key by borrow with exact existing equality; construct owned
only on first distinct occurrence. None/empty axes equivalent, preserve f32 equality
including signed zero/NaN, no HashSet/extra normalization. Private static FnOnce
thunk may build on miss only, mechanical copy not language callback. No API/public
model/phase/target-default/dedup-order-resolution change. Glyph still uses actual
resolve_font_combo, never style.font guess. Preserve layer/container traversal.

RED→GREEN:1, 000same-key items build one owned key; distinct keys preserve order/
variant-axis separation/old equivalences. Count construction/time apart from CLI,
no percentage inference. Keep only with repeated measured-path benefit and equal
selection/output/errors. Frozen-release CLI/PDF/fonts/multi-family-axes-container/
math controls/build/relevant suite/lint/owner-only seals, reversible internal
continuous ADR-0127.

## Actual shaped-face boundary

identidade-fonte-shaped-2026-10-05/{initial.json.gz, probe.txt, bilateral.json}
distinguishes regular GID/Oblique mismatch; owner ratified TextShaped.font with
manifest f4a17206500baf5952f898135944d55f9fca905285eafc995315ead1121dd409,
implementation inventory retained. Legacy constructors None/transforms keep Font;
join distinguishes original face+axes. Cached hits retain actual Font without
style forcing; collectors/subset/CMap/PDF use original binding+style coordinates.
SVG/PNG local original/prepared arrays instance once per face/request, same resource
index; None legacy fallback. Export-local association by bytes+axes, not name/slot;
retain supplied order/append missing, partition multifont once before collection,
no cross-face GID/CMap. No fake fonts/unsafe/globals/public phase/API/FontKey change;
private explicit Font/bindings allowed. Shared nucleus only; RED→GREEN/bilateral
miss-hit-VF controls, no promised speed gain.

## Prepared Shape fixture transport

### Evidence and approval

`lab/results/construction/diagnostics/tiling-nativo-2026-10-14/before.json.gz`
retains HEAD/tree/diffstat/UTC/sources/owners before migration. layout_types.rs:510–511
carried Paint/Stroke; shape.rs:151–191 materialized only group fills; tiling.rs:85
limited/cloned its grid. prepare_cell already returns TilingFrame once per
occurrence. `tiling-preparacao-2026-10-14/{before,after}-renders/results.json.gz`
shows dense-open/stroke-open divergences across all three targets. Manifesto
`d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d` was approved;
receipt `lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`.
Preparation/object counts are transport mechanics; rendering/outlines/text/geometry/
diagnostics are language observables.

### Contract and acceptance

Recursively resolve text/Glyph fonts inside prepared fill/stroke cells, preserving
existing variation/fallback decisions.
Consume the pinned nucleus. Approval supersedes only old approval-pending or
Shape-declarative-Paint/Stroke clauses within this package; other behavior survives.
L0 first, RED→GREEN, explicit fixture/consumer migration, no total From<Paint>,
carrier PartialEq or solid fallback merely to compile. Do not raise the old limit
or move layout to L3. Validate fill/stroke/resource/clipping/affine Parent/native-
target/Solid/Gradient/page semantics and full suite/build/doc/fmt/lineage. The
prepared transport, native-target integration and parity slices recorded as
T1c/T2/T3 are complete only after integrated verification; broken/partial states
are no production certificate. T4 and publication remain out of scope.

## Internal imports, Copy variant and test literals

infra-imports-docs-ci-2026-10-13/before.json.gz retains source/owner/spans/original
compiler output/HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b/dirty tree/UTC/status/
diffstat at :136. Remove only blank physical line between collect_fonts_in_items
doc and function. Preserve technical content/assertions/cases; no public/default/
phase/formula/acceptance/suppression. All-target compile/infra names-suite/build/
workspace Clippy/Rustdoc/lineage/scoped seals, no import/doc mirror tests.

avisos-clippy-tekt-2026-10-15/before.json.gz/selected-red.json retains sources/
owners/HEAD/tree/UTC/diffstat/errors at :213 cloning Copy FontVariant. Dereference
variant into resolved key, still clone families/axes, retaining order/identity/
filtering. No public/default/phase/suppression/new syntax tests; selected GREEN/
existing suite/build/all-target Clippy/fmt/doc/lineage/scoped seals.

field-reassign-2026-10-15/before.json.gz/initial-diagnostics.json retains source/
spans/prior L0/HEAD/tree/UTC/diffstat at :397. Repeated-item helper initializes
font in TextStyle literal, keeps mut and later conditional if axes variations
assignment, no eager-branch evaluation. Preserve collection size/families/axes
values-order/dedup counts; remaining ..Type::default(), no new type/builder/API/
suppression. L0 first/scoped seals/selected RED→GREEN/same names-input-expected-
tolerances/all-target Clippy/workspace suite/build/fmt/Rustdoc/lineage, no initializer
syntax tests. These Rust maintenance measurements prove no new parity/default change.
