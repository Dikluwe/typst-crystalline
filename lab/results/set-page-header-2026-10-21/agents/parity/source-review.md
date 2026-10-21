# Running-content source and contract review

State: clean HEAD `63b7b96b99109b61655b373dc0d248beb7838f86`. Oracle commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree `cac836c07e4dec2a89219370428292393a3216b5`. Exact current source/L0 hashes and UTC are in `source-review.json`. No product run is claimed by this review.

## semantic-order

Body layout runs first, but final frame semantic order explicitly affects introspection/counters: background, header, body, footer, foreground. Each physical body fragment gets its own marginal layout with locator.next. Calling order alone is not the language acceptance oracle.

Evidence: `lab/typst-original/crates/typst-layout/src/pages/run.rs:187-201`, `lab/typst-original/crates/typst-layout/src/pages/run.rs:204-239`, `lab/typst-original/crates/typst-layout/src/pages/finalize.rs:53-80`.

Refuter: Pinned bilateral query/counter observations contradict the apparent frame-order relation; investigate rather than copy mechanics.

## context-admission

Marginals accept static rich Content including Context, not a Func marginal API. Fragment realization supplies locatable Context blocks with actual location and known styles; test physical repeated metadata, here/page and final counter before choosing a Tekt phase design.

Evidence: `lab/typst-original/crates/typst-library/src/layout/page.rs:349-418`, `lab/typst-original/crates/typst-layout/src/flow/mod.rs:139-162`, `lab/typst-original/crates/typst-library/src/foundations/context.rs:63-86`.

Refuter: Oracle rejects a supposedly accepted matrix source; classify source validity separately and preserve exact diagnostic.

## tekt-boundary

SetPage deltas install PageConfig; pure layout later renders running layers. Current static PageRun walk indexes one ordered template visit, not a measured per-physical-page event stream; full SetPage physical admission was explicitly outside the Figure closure. Context at ordinary pure layout is not a general evaluator.

Evidence: `01_core/src/compiler/eval/rules/set/page.rs:288-309`, `01_core/src/compiler/layout/set_page.rs:67-80`, `01_core/src/compiler/layout/page_running.rs:78-103`, `01_core/src/compiler/layout/page_commit.rs:43-44`, `01_core/src/compiler/introspect/walk.rs:1047-1060`, `00_nucleo/prompts/core/compiler/introspect/figure_demands.md:26-31`, `00_nucleo/prompts/core/compiler/introspect/walk.md:656-668`.

Refuter: A bounded existing-phase repair demonstrably satisfies all repeated-page/context/ref/override language controls without new capability or phase; then classify that exact repair continuous.

## identity-accessibility

Decorative repeated accessible content policy does not establish query/counter occurrence identity. Preserve stable retained Context frames, actual located aliases and fresh raw insertion distinctions; do not deduplicate by Content equality/hash or manufacture counter histories/locations.

Evidence: `00_nucleo/prompts/_nuclei/layout/page-running.toml`, `00_nucleo/prompts/_nuclei/page-counter-snapshot.toml`, `00_nucleo/prompts/infra/pipeline/context_stabilization.md`.

## approval-boundary

Internal mapping/parity correction in the same authorized boundary can be continuous. New public facade/entity field, actual per-physical-page realization phase, or eval/layout phase transfer requires a concrete public/phase approval. Prior Figure/Ref approval does not authorize that by analogy. Keep HTML paged-header nonadmission and pure layout responsibilities explicit.

Evidence: `00_nucleo/adr/adr-0127-l0-gate-stop-vs-continuous-flow.md`.

## Frozen measurements

The fifteen exact sources and their language observation properties are in `matrix.json`; `run_matrix.py` executes each oracle/product pair serially. It records refusal/success independently, exact query JSON, page count, raw per-page text and positioned words. PDF extraction order is not treated as semantic query order, and output bytes are provenance only. Invalid JSON never compares equal. Baseline product failures remain measurements, not harness acceptance. Oracle source validity must be reviewed before using any row as candidate acceptance.

No implementation or phase design is authorized by these lab artifacts. A once-only SetPage template visitor is insufficient evidence for repeated physical pages; a new Func marginal API is neither required nor proposed.
