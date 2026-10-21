# L0 — infra/pipeline/html
Hash do Código: 2e8c2d4b

**Layer:** L3
**Exclusive consumer:** `03_infra/src/pipeline/html.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/compiler-feature-gates.toml sha256:09b32666fb1b1fa7ba8f2ef7c979c60cbc64cc515b8d9c46818bec19d7471fb1
- 00_nucleo/prompts/_nuclei/export/format-option-precedence.toml sha256:9c4f3ce07c56762e4c588a0f223c3ea1e7e50d6c00f83ff457fad766b0cea3cf
- 00_nucleo/prompts/_nuclei/export/svg-glyph-font-context.toml sha256:fe41319c4bf27b69b635c5666e46f5b8b9b99160323926fb83889b0f09b97417
```

## Current evidence

- `03_infra/src/pipeline/html.rs:64` — `resolve_html_pretty`.
- `03_infra/src/pipeline/html.rs:71` — `compile_to_html_string_with_features_and_serialization_and_pretty_override`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/pipeline/html.rs`.

## Owned contract

- Own the semantic HTML compilation facade, HTML option resolution and intrinsic `html.frame` preparation/position collection. The external evaluation target remains Html; its feature gate precedes evaluation and is never inferred from output extension or serialization mode.
- Evaluate, collect document format options, delegate contextual preparation, resolve intrinsic frame islands and export through the selected `HtmlSerializationMode`. Do not paginate the whole HTML document or create a global PagedDocument/PageStore.
- Resolve pretty once as `pretty_override.unwrap_or(options.html.pretty)`. Wrappers without a decision pass None; the explicit boolean wrapper passes Some. Deliver the same resolved decision to external HTML and inline SVG frames; inline frames do not consult `format.svg`.
- Crystalline/Vanilla modes affect encoding only, not features, content, context or diagnostics. No CLI/env access belongs here.
- Traverse frames in occurrence order with effective styles and invoke the intrinsic L1 paged facade locally. Nested frames collapse into the outer island; distinct occurrences retain distinct causal identities. Layout/render failure aborts rather than dropping a frame or emitting a placeholder.
- For actually observed HTML `location.position()` reads, supply intrinsic positions to selective stabilization before terminal marker projection. `location.page()` stays none: an intrinsic Page is never a public HTML page.
- Missing ids, collisions, ambiguous topology or unmatched attempts fail closed. General query-derived Location identity outside an island remains outside this contract; do not associate it by ordinal.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.


## Realized snapshot and export bindings

Inspection html.rs:102 found rebuilt initial introspection already produced by
realization. Use Module::realization_introspector under
`00_nucleo/prompts/core/entities/module.md`, independent clone when available,
existing fallback otherwise. Preserve feature/target/options/context preparation/
intrinsic-frame positions. Initial snapshot is not final context/position result;
revalidate language observations/frames. Under the ratified Page-snapshot package,
prepare nonpaginated Page snapshots before discovery and select/revalidate producers
whose actually observed reads change when only Page views are replaced. No physical
HTML pages or previously forbidden HTML filtered-counter reads.

identidade-fonte-shaped-2026-10-05/probe.txt supports private resolve_export_fonts
returning bound original fonts under existing instancer gate. Instancing belongs
to render/export consumer in same phase, diagnostics/API/page selection unchanged;
old name did not promise already-instanced bytes.

## Test-module ordering

clippy-locais-2026-10-16/before.json.gz retains sources/L0/hashes/HEAD/tree/UTC/
diffstat at html.rs:147, inspected use before selected-lint RED. Move only
#[cfg(test)] p1465_tests after productive helpers, keeping name/body/attributes/
imports/all tests, no module/visibility/pipeline change. Internal ADR-0127 no
public/default/phase/lint suppression; L0 first/scoped seals/exact inventory
reduction/no new warnings/RED→GREEN/unfulfilled expectations as error/workspace
suite/build/fmt/Rustdoc/lineage under12GiB/zero swap/job1. No derive/Entry/module
syntax-mirror tests; actual bbox requirement belongs to its historical scope.

## Ratified Heading supplement before queries

lote-a1-callbacks-reflexao-seletores-2026-10-17/{final-freeze, target-controls}.json.gz
retains HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c/dirty tree/UTC/diffstat/sources/
argv at pinbd5ebec285505889e8ff1d4f68804ab147a699f3. heading-raw-constructor-func
passes baseline/vanilla but candidate Styled transport changes func; query sees
temporary causal wrapper and HTML ignores callback panic. walk.rs:930–945 and
paged.rs:419–427/html.rs:101–106 use pure pre-query walk. Language values/errors,
no upstream-mechanics requirement. Owner confirmed review
`lab/history/construction/records/00_nucleo/materialization/heading-supplement-runtime-2026-10-17/review.md`,
SHA256`66739b13596013f4343bdc3ffacbdb9081151e3d9863e5d962e8ff13fd68775c`.

In initial snapshot, including realized one, call only public ratified L1 Heading
facade via private runtime with EvalTarget::Html/effective features before
prepare_html. Propagate errors/warnings; no whole-document paged layout/encoder
callback execution. Preserve frame islands/positions/gates/pretty. H1–H8/bilateral
RED before code; gate authorizes only Heading facade, not separately scoped five
State helpers/new public fields. Existing Styled/other callback boundaries stay.
Workspace/doctests/build/Clippy/Rustdoc/fmt/V1-V5-V15-V26/reverse/no V16 growth,
sequential12GiB/High8GiB/swap0/job1. RAM gain needs measurement, not intent.
