# FG1 independent counter/client review

Status: **not ready for a user gate**. V16-055 remains open.

This bounded review covers typed counters, references, Introspector and its L3 wrapper. The companion JSON pins the current draft, source/owner graph, UTC, HEAD and uncommitted diff. It distinguishes fully read owners from integration dependencies still requiring a complete owner audit. It is not a complete realization migration manifest.

## F1 — Counter event history is not a Figure occurrence index

Set/Step are materialized in population. Filtered Func is deliberately skipped in postpass and evaluated by the demanded filtered fold. Replacing string keys with Selector::Element without a coherent read/finalization design leaves captions/references inconsistent with language counter reads. value_at_index counts all history records whereas layout idx counts Figure occurrences.

Required: Define demanded immutable histories and occurrence-to-Location lookup, including callback timing/error context/demand boundaries, or another measured coherent design. Never execute Func through the read-only trait or blindly fold unused keys.

Evidence: `01_core/src/compiler/introspect/queries.rs:341–348`, `01_core/src/compiler/layout/figure.rs:37–49`, `01_core/src/compiler/introspect/populate.rs:288–307`, `01_core/src/compiler/introspect/from_tags/counter_updates.rs:99–106`

## F2 — Supplement policy conflicts with current owner and is incomplete

Current references L0 excludes implicit supplement for filtered Element selectors; string helper treats known table/raw/image names as element supplements. Typed Name("table") cannot be passed through that helper as if it were Element(table). Draft does not specify exact custom-name error/absence behavior.

Required: Measure and specify focused typed Figure policy, explicit empty supplement precedence, numbering-none and reference behavior. Update reference/label/helper owners before code; do not silently reuse a legacy fallback.

Evidence: `01_core/src/compiler/layout/references.rs:354–407`, `01_core/src/compiler/introspect/labelled.rs:62–79`, `01_core/src/compiler/lang/figure_supplement.rs:84`

## F3 — Compatibility adapter cannot certify semantic migration

Legacy Some("table") now means custom Name rather than inferred/native table. Synthetic data without resolved_kind also needs an explicit downstream policy. Current reconstruction using Content::figure would discard prepared state.

Required: Classify actual producers and fixture intent; preserve entire lexical/prepared state in maps and temporal reconstruction; define synthetic/unprepared payload boundary.

Evidence: `01_core/src/entities/content/construction.rs:415`, `01_core/src/compiler/introspect/materialize_time.rs:113–119`, `01_core/src/entities/content/traversal.rs:873`, `01_core/src/compiler/eval/observation/elements.rs:845–852`

## F4 — Derived Rust Func equality/hash is not element identity proof

Rust native Func equality/hash compares names. Draft assumes validated canonical Element identity but does not make the validation/normalization boundary concrete. Deriving FigureKind PartialEq/Hash does not establish language acceptance for aliases/homonyms.

Required: Pin canonical element validation/normalization and ensure private progress/counter lookups use matching semantics; preserve alias and homonym negatives.

Evidence: `01_core/src/entities/func.rs:407–454`, `01_core/src/compiler/eval/operators/equality.rs:201–240`

## F5 — Occurrence preparation and demand phase not ready for approval

The draft introduces a public entity/payload/trait compatibility change and pre-show preparation. Encoder is separately reviewing preview/final show-set ordering and semantic-field traversal. This bounded counter/client review does not certify that phase or complete affected graph.

Required: Keep V16-055 open. Finish owner graph and measured phase/effect policy in a separate gate package; prior Context occurrence approval does not authorize Figure.

Evidence: `lab/results/v16-corrections-2026-10-19/batch-f/fg1-proposal/design-draft.md`

## Correction and scope

Population does materialize manual Set/Step. My earlier informal claim that it skipped all filtered manual updates was wrong and is withdrawn. Filtered Func postpass and occurrence/history ordinal mismatch are the actual issues.

Only TagIntrospector and CountingIntrospector implement the public trait in productive code. The L3 wrapper must migrate its argument while retaining instrumentation slot 15; direct integration-test callers also change. Generic CounterRegistry string-key tests must not be renamed automatically: they may remain valid user-string counter tests.

The existing Selector vocabulary and a form-B owner avoid inventing a new counter variant, but this alone does not make the phase/callback design coherent. A demanded-history cache is a candidate, not ratified design. No productive source/L0 changed and no Cargo or product execution was performed for this review.
