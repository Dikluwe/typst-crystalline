# L0 — infra/pipeline/html
Hash do Código: 28231f75

**Layer:** L3
**Exclusive consumer:** `03_infra/src/pipeline/html.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
```

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/compiler-feature-gates.toml sha256:09b32666fb1b1fa7ba8f2ef7c979c60cbc64cc515b8d9c46818bec19d7471fb1
- 00_nucleo/prompts/_nuclei/content/symbol-identity.toml sha256:5060f0989415e92d54eab2a82afd84ba11472aef635d15830f9fb6ee3469aea9
- 00_nucleo/prompts/_nuclei/export/format-option-precedence.toml sha256:9c4f3ce07c56762e4c588a0f223c3ea1e7e50d6c00f83ff457fad766b0cea3cf
- 00_nucleo/prompts/_nuclei/export/svg-glyph-font-context.toml sha256:fe41319c4bf27b69b635c5666e46f5b8b9b99160323926fb83889b0f09b97417
- 00_nucleo/prompts/_nuclei/layout/page-rule-container-boundary.toml sha256:54dec4a08272a6ddbb91b63d388af8c25aaaa8afc7864b72226e1d125026682e


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


## Frame observer accounts for suppressed callback roots

### Evidence before decision

At baseline4382891, `03_infra/src/pipeline/html.rs:158–200` uses map_content for
frame_count and collection. `01_core/src/entities/content/traversal.rs:436`
suppresses a Located immediate-body callback. Located(HtmlFrame) loses its Frame
event; Located(Styled(Frame)) loses the Styled event carrying effective styles.
The same root suppression affects expected Context IDs within an island. Exact
source/hash evidence and local regressions are retained in
`lab/results/v16-corrections-2026-10-19/batch-g/agents/html/audit.json` and
`lab/results/v16-corrections-2026-10-19/batch-g/focused-red-infra-3/receipt.json`.
These helper cases do not establish external language reachability of every
transport shape.

### Contract and acceptance

Observe suppressed roots locally by peeling only Source/Located chains before
the Frame/Styled or expected Context-ID decision. Do not peel ShowRecipe/Realized
in this callback: generic traversal already observes their body/visual roots.
Other nonframe callback events remain explicitly inert while generic traversal
visits their descendants. Frame counting and collection use identical root-event
recognition. Preserve bottom-up nested order, exactly-once occurrences, captured
style precedence and reverse ancestor attachment.

Verify direct/Located/doubleLocated/Source/Styled combinations, siblings/nested
Frame event order and effective font sizes. Counts agree with collection;
ShowRecipe/Realized visual frames appear once and original frames never appear.
Expected Context ID discovery uses the same root observation and retains exact
matching against detailed intrinsic positions. Duplicate IDs, missing mappings,
collisions and unequal ID sets fail closed; never accept a placeholder or relax
these guards. Intrinsic positions introduce no public HTML page or PageStore.

Preserve layout/resolver/pretty behavior. This owner-local ADR-0127 correction
adds no global traversal transparency, public visitor/API, container catalogue,
phase, target or default and makes no complete frame-language parity claim.

## Measured refutation and final causal HTML boundary

`lab/results/remaining-parity-2026-10-22/agents/html-length/empty-warning-before/receipt.json`
SHA-256 `8bd8aac399399978418fd9a5c82cdbfa07bf93c6fe9779401901b55cc489a4d6` records UTC
`2026-10-22T12:42:21.170883+00:00`–`2026-10-22T12:42:21.559949+00:00`, exact HEAD/dirty stat, sources, argv,
stdout/stderr and the cached baseline's own verified source archive/build, oracle
and font pins. The cached binary is not claimed as current uncommitted source.
Empty page/empty nested page do not warn or error. Returned ContentBlock content
and nested initial Bundle content warn rather than error; the construction-marker
proposal is refuted and must not be materialized. Direct standalone Html ContentBlock
insertion and semantic HtmlFrame/Block reject page configuration. Both Heading/Page
warning orders are measured; missing standalone Heading warnings cannot justify
late sorting. This supersedes only the refuted construction/Document-consumption
clauses, preserved as diagnostic history in that task's LAB records.

After existing contextual preparation and before existing export, delegate to the
same private emitted HTML semantic projection already used for Bundle documents.
Its single causal traversal emits reached Page and deep-Heading warnings with
nearest Source origins and validates marked/semantic containers. It must not
paginate HTML or convert actual HtmlFrame body content; such bodies are only
validated for page-setting legality and passed unchanged to their frame resolver.

Meaningful before-code RED and final bilateral exact source/message/hint/order/body
checks remain required. Internal ADR-0127 continuous flow; no new public contract,
default or phase. Preserve full unsuppressed lint/test/path/hash/lineage gates.

## Approved batch E integration

Measurement: pre-E inputs and bilateral reflection in
`lab/results/symbol-identity-2026-10-23/{baseline.json,before/receipt.json,additional/receipt.json}`.
The owner approved the native Symbol declaration on 2026-10-23.
Handle Symbol explicitly as a native childless textual leaf in exhaustive complements and existing projections. Preserve the complete Unicode payload and reconstruct the same variant when mapping. Do not silently classify Symbol as a bare equation outside math.
Preserve the existing unit responsibility and all other variants/effects; no public contract beyond the approved Content addition or phase move.
