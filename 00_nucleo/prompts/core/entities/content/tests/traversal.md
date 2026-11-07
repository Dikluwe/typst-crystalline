# L0 — entities/content/tests/traversal
Hash do Código: f9568837

**Layer:** L1 test-only
**Exclusive consumer:** `01_core/src/entities/content/tests/traversal.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/symbol-identity.toml sha256:5060f0989415e92d54eab2a82afd84ba11472aef635d15830f9fb6ee3469aea9


## Contract

Own regression coverage for bottom-up `map_content`, recursive `map_text`, terminal preservation, and payload reconstruction. Tests observe public or crate-visible
behavior and must not prescribe private implementation structure, dynamic
dispatch, or reverse layer dependencies.

## Acceptance

Cases retain independent assertions and fixtures, execute as
part of the `Content` test suite, and introduce no productive dependency.

## Closed bottom-up callback decisions

### Evidence before decision

Source inspection at HEAD `059d1a0ff` on 2026-10-19:
`01_core/src/entities/content/tests/traversal.rs:71–94` uppercases Text and
asserts that Strong observes its already-transformed body; its wildcard returns
no replacement. `01_core/src/entities/content/traversal.rs:670–674` retains
processed descendants when a callback returns None. Thus no replacement does not
mean skipping a subtree. This is a local traversal-mechanics qualification; it
does not independently certify language parity or upstream intent.

### Decision and acceptance

Retain Text replacement and the independent Strong assertion. Enumerate every
other current nominal Content alternative in the callback with the same explicit
no-replacement policy. Keep the decision here, not behind a helper or predicate.
Extend the actual callback fixture with a Space terminal, an Emph container with
a Text descendant, and siblings in a Sequence. Assert transformed text, retained
container kinds, space preservation, child order and the Strong callback count.
All existing traversal tests and assertions remain. Descendants lost after None,
reprocessed replacement text, or changed sibling order refute the repair.

Before changing the callback, run the expanded behavior test against the old
implementation. Structural RED is the old decision accepting enum growth; a
Content sentinel in a disposable real workspace must make the repaired callback
fail with E0004 at its own site. Preserve the source state, UTC and compiler
diagnostics. This is ADR-0127 continuous flow after owner sealing, with no public
contract, product default, phase or compatibility change.


## Approved Figure/Grid occurrence amendment (2026-10-19)

### Evidence before specification

Baseline `731f0ffdf4c7c152f590551e72ff63f7bd85ef85` preserves the measured proposal in `lab/results/v16-corrections-2026-10-19/figure-kind/review.md`, its `agents/contracts/design.md`, semantic partition, independent audit and counter receipts. The fixed oracle is commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree `cac836c07e4dec2a89219370428292393a3216b5`. Constructor string kind loses function identity; constructor inference precedes occurrence styles; separated Grid fields lose source order. Typed identity, lexical presence, field traversal and counter results are language semantics/morphology. The chosen Rust carriers are implementation mechanics. The owner explicitly approved the linked public fields, contracts, phases and compatibility changes on 2026-10-19 under ADR-0127; this amendment materializes that bounded decision.

### Exclusive contract

Migrate Grid fixtures with source_order:None unless source order itself is the tested input. Verify descriptors survive maps while each actual child transforms exactly once in the established map order. Add Figure map tests distinguishing lexical versus prepared supplement aliases and retaining all six fields without callback execution. Existing meaningful traversal assertions remain.

### Acceptance and limits

Preserve all unrelated prior obligations and nucleus pins. This amendment supersedes only conflicting Figure/Grid clauses stated above. Require independent semantic RED→GREEN, exhaustive decision extension builds, existing workspace/build/tests/all-targets/Clippy/Rustdoc/fmt, canonical paths, strict forward/reverse lineage and valid V15/V26 before scoped resealing. No suppression, severity downgrade, weakened assertion or global hash repair. The bounded package does not certify full Figure/Grid parity: standalone HTML rendering, complete caption synthesis, supplement-dependent selector preview, external Dynamic enumeration, Grid footer phase and complex Table supplemental target order remain visible debt.


## Approved batch E integration

Measurement: pre-E inputs and bilateral reflection in
`lab/results/symbol-identity-2026-10-23/{baseline.json,before/receipt.json,additional/receipt.json}`.
The owner approved the native Symbol declaration on 2026-10-23.
Handle Symbol explicitly as a native childless textual leaf in exhaustive complements and existing projections. Preserve the complete Unicode payload and reconstruct the same variant when mapping. Do not silently classify Symbol as a bare equation outside math.
Preserve the existing unit responsibility and all other variants/effects; no public contract beyond the approved Content addition or phase move.


## Issue 8894 — approved Stroke migration

Measurement: the source inventory and previous owner bytes in
`lab/results/construction/diagnostics/issue-8894-2026-10-31/` bind exact
HEAD/dirty diffstat/UTC and each affected source/owner before migration.
`stdlib/layout/stroke.rs:430–434` erases em; geometry.rs:81 carries only f64.
The explicit user approval in approval.json ratifies the public Length field
and occurrence-layout resolution, as specified by
`00_nucleo/prompts/core/entities/geometry.md`. This consumer retains its
exclusive responsibility and all other ratified clauses and controls.

Migrate this consumer's declarative Stroke literal thickness from scalar
points to Length::pt without changing their value or other fields. FrameStroke
remains scalar. Keep all existing semantic assertions; update only their
representation as necessary. Producers preserve full Length; layout consumers
resolve against effective font size before scalar geometry. Repr preserves
absolute/em components. RED→GREEN, build/tests and scoped lineage are required.


## Approved issue8890 Figure placement — 2026-10-31

### Measurement before decision

`lab/results/construction/diagnostics/issue-8890-2026-10-31/{analysis.json,svg-geometry.json,report.md}` preserves the exact HEAD3c6b0ce7, UTC, clean/dirty state, pinned vanilla64c5e8c0 executable/source/inputs and PDF/SVG measurements. Original Figure placement is rejected in Tekt at stdlib/figure_image.rs:342-360; vanilla loses D after deferring the float. The supported Place control preserves A–E but puts its float on page3 after continuation text, so it does not certify the original trigger. Current source/owner bytes and concurrent dirty state before this amendment are preserved under `lab/results/construction/diagnostics/issue-8890-placement-2026-10-31/before/` and before.json. Geometry/presence/positions are language; queue/carrier mechanics need not copy vanilla.

The user explicitly approved the concrete public-contract proposal with “Aprovo” in this chat; approval.json binds its exact scope. This satisfies ADR-0127 for FigurePlacement and the two public fields only, with existing layout phase/default in-flow behavior retained. The prior placement scope-out is superseded only for this approved subset.

### Exclusive responsibility

Migrate only existing Figure fixture literals/destructures for the approved two-field addition: absent placement/resolved_placement for all legacy in-flow inputs. Preserve every prior expected language result and all unrelated implementation. No behavior redesign is authorized in this owner.

### Gates and limits

L0 and scoped reseal precede real RED, code and GREEN; valid V15/V26 precede repairs. Preserve all other ownership, previous clauses/ratifications and concurrent changes. Use only /dev/shm/typst-crystalline-8890-materialization-target for this amendment's compilation/test temporaries, one job, 12GiB virtual ceiling and zero swap. This target supersedes older task-specific target paths only for the present amendment. Record state/hash/UTC around every measurement. Revalidate original and link PDF/SVG, controls and finite progress with all A–E once; page count itself is not an oracle for a known vanilla defect. No new phase/default/compatibility change beyond the approved fields. Do not suppress lint, weaken tests, copy upstream defects, commit or claim global qualification from scoped checks.
