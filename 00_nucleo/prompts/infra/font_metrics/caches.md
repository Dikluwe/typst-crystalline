# L0 — infra/font_metrics/caches
Hash do Código: f7b8cb95

**Layer:** L3
**Exclusive consumer:** `03_infra/src/font_metrics/caches.rs`

## Current evidence

- `03_infra/src/font_metrics/caches.rs:20` — `CachedFace`.
- `03_infra/src/font_metrics/caches.rs:29` — `new`.
- `03_infra/src/font_metrics/caches.rs:41` — `face`.
- `03_infra/src/font_metrics/caches.rs:58` — `ShapedWidthKey`.
- `03_infra/src/font_metrics/caches.rs:75` — `AdvanceWidthKey`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/font_metrics/caches.rs`.

## Owned contract

- Own CachedFace, ShapedWidthKey/AdvanceWidthKey and cache helpers. Font bytes own the face lifetime; preserve the existing encapsulated unsafe and Arc sharing.
- Load faces lazily by slot and share successful parses across clones. Failed reads/parses are not cached. Never replace per-instance state with a global cache.
- Width keys preserve text, effective measurement size bits, family, bold/italic/weight, direction, language, variation hash and explicit math mode (which changes the primary/fallback chain); advance keys additionally distinguish TextItem provenance and math size.
- Advance cache helpers receive the explicit `FontMetrics::advance` size; never substitute `style.size` for that argument. Shaped keys use the effective style supplied by the fallback owner.
- Local regressions compare shared-cache and fresh-instance widths in both request orders, using real embedded fonts with discriminating math fallback and explicit-size inputs; they characterize correctness rather than performance or new upstream parity.
- Preserve tracking bypass in shaped-width caching; None results are not inserted. Read under the original lock scope, compute outside it and insert afterward; do not alter lock order or failure retry behavior.

## Verification and boundary

Run `cargo test -p typst-infra --lib font_metrics:: -- --test-threads=1` when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.


## Local bindings and fixture initialization

### Evidence

`lab/results/construction/diagnostics/ci-helpers-testes-2026-10-14/before.json.gz`
retains source/owner/HEAD/uncommitted tree/UTC/diffstat for caches.rs:129,210.
references-before.json.gz records calls/imports/modules/references; previous suite/
inventory state was hash-checked and test list captured before editing. Private
calls/compilation were audited; dead_code alone proves no functional uselessness.
`field-reassign-2026-10-15/before.json.gz` retains source/spans/prior L0/HEAD/tree/
UTC/diffstat, and initial-diagnostics.json records default-then-assignment test
styles at caches.rs:282,285. This is Rust representation, no new parity or
product-default change.

### Contracts and acceptance

Remove only two unused pure local variant assignments and their exclusive import.
axis_variations_for_text_style already calls text_style_to_font_variant internally
and remains the axis-hash source with unchanged explicit variations/precedence.
Preserve all fields/keys/hash order/sizes/math/tracking bypass/locks/cache policy;
no underscore-local substitution. Continuous ADR-0127 adds no public field/API,
default, phase, dependency or suppression. Selected diagnostic gate RED→GREEN is
not a functional test/parity proof; keep test inventory/expectations, full suite,
lib/all-target compilation/build/Rustdoc/fmt/lineage/full diagnostic inventory;
no cfg/binding-removal mirror tests. Reseal only this owner.

Construct style_a/style_b with weights 400/700 in TextStyle literals. Preserve
axis_hash/key comparisons and repeated same-weight control; do not change caches,
locks or variation derivation. Use ..Type::default() for remaining fields; remove mut only where no mutations
remain. No new types/builders/APIs/lint suppression. L0 first and scoped resealing,
continuous ADR-0127. Selected gate must fail before and pass after; preserve test
names/inputs/expectations/tolerances. Revalidate all-target Clippy/workspace suite/
build/fmt/Rustdoc/lineage. No tests merely mirroring initializer syntax.


## R8 — approved font-backed superscript integration

Evidence before decision: the source-bound `footnote-layout-r8-2026-10-24/before/receipt.json`,
`font-tables.json` and `red-geometry.json` under `lab/results/construction/diagnostics/`
record wrong 0.65 scaling/positive baseline and font-dependent pinned geometry at
bd5ebec285505889e8ff1d4f68804ab147a699f3. `approval.json` binds the human-approved
FontMetrics::text_superscript_metrics addition. OS/2/whole-run GSUB are text metrics;
MATH constants and fixture/font-name coefficients are not substitutes. This private
integration follows ADR-0127 continuous parity flow, superseding only superscript
scaling/feature assumptions below. Keep subscript behavior and unrelated folds.
Include superscript identity in both shaped-width and scalar-advance cache keys;
otherwise equal-size native sups and ordinary text collide. Preserve every existing
key component and explicit language/axis identity. Test distinct keys.

Require actual RED→GREEN tests, fixed-pin PDF/SVG controls, unchanged geometry tolerance and complete workspace/lineage validation.


## R8 approved resolved native-script choice

Measurement before decision: R8 numbering-candidate5/receipt.json records
Nimbus11 numbering1 Z with1.4718pt error; the full marker chose synthesis but its
shortened digit was shaped with sups again. The root is
lab/results/construction/diagnostics/footnote-layout-r8-2026-10-24/.
The exact public TextStyle::superscript_native: Option<bool>, default None, was
approved from owner draft SHA4b14829d1eb38475d1985a9707a6e6ce40166196dfe524afe512d99b9cabf4d0
and is recorded in native-carrier-approval.json. Its data/default contract belongs
only to core/entities/layout_types.md; this owner retains its existing consumer.
No MATH flag, baseline sign, font-name constant, private string or source identity
may substitute for the explicit choice. No extra public field/signature/phase.
Include superscript_native in both scalar-advance and shaped-width identity
where its value changes requested features; equal text/font/size with None and
Some(false) must not share a native width. Keep all existing key components.

Require new field/default/forced-false RED→GREEN, all twelve numbering refuters at0.001pt PDF/SVG, original ten notes, six font controls, warm/cold cache and full preservation/workspace/lineage gates.
