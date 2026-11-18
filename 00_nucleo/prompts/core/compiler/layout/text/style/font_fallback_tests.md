# L0 — Effective font fallback regressions
Hash do Código: 87b65247

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/immutable-family-list.toml sha256:f33ff33fa820aa3914d7de26e3abe47e4c453971161347e6c71ab028ebbac468
- 00_nucleo/prompts/_nuclei/layout/raw-defaults.toml sha256:24f0b69fa1db115ff13a786121c80cb7be3abfa2028a9a473075374a6815cfd4
- 00_nucleo/prompts/_nuclei/text-size-affine.toml sha256:c8570ec5f4f32c0805587230cafc796f35e5ee7c9f8e634e13d807db2afb6e00



Layer L1. Exclusive consumer: `01_core/src/compiler/layout/text/style/font_fallback_tests.rs`.

## Evidence before specification

Before the fallback change, `01_core/src/compiler/layout/text/style.rs:127-261`
decodes custom family arrays, `:262-273` determines nearest typed/custom font
precedence, and `:296-300` selects custom or active fallback. The current owner
preserves rich family identity, resolved size, lexical direction and script
provenance. `font_list.rs:28-30` makes Covers uninhabited; only None is representable.

## Contract

Test the actual private from_parts and resolve_parts using real StyleChain and
TextStyle. Nearer typed font wins outer custom; nearer custom wins outer typed;
same-node typed wins custom. Missing, empty, invalid or nonarray custom retains
the complete active Option, including None. Preserve ordered literal/regex family
names, variants, variant, weight, style, representable covers=None, and every
family of the typed fallback. A valid decoded list succeeds even with no active
font. The output owns its complete immutable list payload without mutating active
style or chain. Clones/fallback may share storage under immutable-family-list.
Verify ownership by replacing and dropping all inputs before inspecting the
result. Keep separate-value checks for freshly decoded lists; pointers are
mechanical observations and never language parity.
Preserve other render fields, affine resolved size, absent/explicit language,
direction, variations, mathematical/script and semantic Strong/Emph provenance.
Private cfg(test) only; no instrumentation, public API, dependency or default.
Semantic expectations do not depend on eager versus lazy allocation or a fixed
allocation count. These regressions do not certify general language parity;
outline divergence against vanilla64c5e8c02fe0394d913ead36e42ef8c1334d2107 stays open.


## Laboratory ownership correction — 2026-11-13

Before this revision, full-core validation of shared-font-list failed exactly
at pointer-nonalias assertions in this consumer:165 and180. All8520 other tests
passed. The old owner explicitly required separate storage; this is an explicit
laboratory change to that mechanical requirement, not a claim it already allowed
sharing. FontList has no mutable list accessor. The new tests preserve every
precedence, metadata and input-preservation expectation and verify that results
survive replacing/dropping input owners. The decoded-custom construction test
still distinguishes independently built lists. Archive and source hashes are in
shared-font-list-validation/before-owned-fallback-contract/qualification.json.
No font resolver implementation, public signature or language rule changes.

## Productive adoption scope — 2026-11-18

The user explicitly authorizes the cumulative packages with measured gain. This
owner adopts only its finalized implementation in the preserved capture candidate
29115606, qualified together with its required dependency owners; earlier
laboratory-only exclusions are superseded only for this specific implementation.
Existing current project ADR scope clarifications and conservative fallbacks
remain in force. No additional public method, default or phase is invented here;
already specified optional capabilities keep their explicit authority/None paths.
Diagnostic census, no-retain ablations and blocking watch markers are not product
features. Current adoption uses disk-backed task targets, one Cargo job,12GiB AS,
8GiB tree RSS and no swap; prior task-specific temporary-target paths are scoped
to their original correction. Require real capture RED→GREEN, complete suites,
lineage/ownership, source/effect/invalidation controls and measured before/after
and pinned vanilla comparison. No commit is authorized by this adoption.

## Strict test qualification for adoption

Preserve every existing test assertion, reference comparison and negative case.
Remove redundant cfg/borrows, use a named private snapshot type and array, and
pass configured raw-shaping inputs as the existing named RawShapeInput value
instead of a nine-argument helper. Test FontList HashMap/HashSet contracts through
generic Eq+Hash helpers so immutable regex backend caches do not trigger a
concrete mutable-key lint; keep actual map lookup, deduplication and field coverage.
No lint suppression or productive algorithm change is permitted by this cleanup.
