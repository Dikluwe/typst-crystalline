# L0 — introspect/entrypoints
Hash do Código: df115401

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/physical-page-occurrences.toml sha256:b33f2e3c6238da01aa48783aecee6a8b6153b4f36ddcdc23edfe78434bae5b42
- 00_nucleo/prompts/_nuclei/realization-order.toml sha256:958649ea9ebdd471f101874ff8cf94feb995ee037c43eb6e2f1d1ad19a151ef3

## Ownership and evidence

Layer L1. Exclusive productive consumer: `01_core/src/compiler/introspect/entrypoints.rs`.
At clean-tree state `df72a6c4a701917ab56a7c6169764a92ea775693`, `01_core/src/compiler/introspect.rs:750-921` contains the behavior assigned to this unit. Source and state are recorded in `lab/results/construction/diagnostics/atomizacao-introspect-2026-09-30-before.json.gz`. Separation is by behavior, preserving the API and without a performance claim.

## Contract

Execute the pure pass and bundle/runtime variants with one shared causal Locator. Register seeds before bodies, prepare outlines/parents once and coordinate postprocessors without reevaluating the document.

Language invariants, causal order and parity limits remain defined in `00_nucleo/prompts/core/compiler/introspect.md`; this owner must not create a second interpretation. Code uses static L1 calls without vtables, I/O or mutable global state.

## Acceptance

Preserve introspection tests and the L1 suite, build, and pass `crystalline-lint .` with consistent hashes. Future semantic changes require language measurement and an L0 update before code.


## Causal identity and bundle membership

Reserve explicit Locations across the whole input before allocating ordinary/
contextual elements, preserving disjointness and tag order. Both bundle
entrypoints privately register each body's generated Tag::Start Locations in
CounterRegistry. Checkpoints remain layout aids, not exclusive semantic-identity
boundaries. Empty bodies register empty scopes.

## Heading supplement runtime coordination

### Evidence before decision

HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, working tree/UTC/hash are in
`lab/results/construction/diagnostics/lote-a1-callbacks-reflexao-seletores-2026-10-17/before.json.gz`.
before-pdf-probe.json.gz: heading-supplement-panic succeeds in Tekt and errors
A1_HEADING at vanilla `bd5ebec285505889e8ff1d4f68804ab147a699f3`.
heading-supplement-valid is accepted in both, but only vanilla uses Custom in
references. Upstream model/heading.rs:249–289 synthesizes per-occurrence
supplement; model/reference.rs:393–415 applies function/display. Set.rs:286–320
ignores supplement/clears numbering; heading.rs:55–68 snapshot_fields fixes the
translation. This is observable callback/value behavior, not upstream mechanics.

### Exclusive coordination

Both runtime entrypoints finalize Heading in their existing post-walk block
for equation/State callbacks before publishing intr, propagating errors. The
pure no-Engine entry remains pure. Do not reexecute the document or change
eval/layout phase/convergence. This ADR-0127 correction adds no Rust API/public
field or logic transfer between eval/layout; the Engine post-walk callback
boundary already exists. L0 first, RED/bilateral controls; no callback in
encoder/walk/layout or universal field-parity claim.

## Ratified object-reader State coordination

### Evidence before decision

HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, uncommitted working tree;
before.json.gz and final-freeze.json.gz under
`lab/results/construction/diagnostics/lote-a1-callbacks-reflexao-seletores-2026-10-17`
preserve UTC/status/diffstat/sources. state-counterproof.json.gz records bilateral
RED: readers initialized to 1/10 with the same key require 2/11 after +1; an
unread callback never executes; reused updates execute per occurrence. Pin:
`bd5ebec285505889e8ff1d4f68804ab147a699f3`; upstream
introspection/state.rs:226–245,419–455,477–520. The reader's init starts the
sequence; document order, not the Location integer, orders it. The measured
method treats Func as Set and eager postprocessing loses that semantics. These
are language values/errors, not Rust equality or a mandated algorithm.
The earlier State proposal was withdrawn; the owner ratified the object-reader
runtime gate in that session. Review:
`lab/history/construction/records/00_nucleo/materialization/state-reader-runtime-2026-10-17/review.md`,
SHA-256 `6d6a176b8e27cc53c38fe6650c03c217448059f4e7287fd8848fe54e80247fa4`.

### Exclusive responsibility and partial supersession

Remove only automatic apply_state_funcs calls from both runtime entrypoints.
Keep pure action recording in walk. Coordinate internal fallible State displays
to propagate demanded-update errors, preserving legacy defensive display
callbacks/coercion. Keep public entrypoint signatures/other postprocessors.
This supersedes only the historical automatic State phase.

### Acceptance and boundaries

The object-reader gate is ratified for its five signatures and read phase.
L0 first, functional RED before code; GREEN against the stated pin for language
and independent controls for Tekt extensions. Do not broaden other public
contracts. Validate build/workspace/doctests, strict Clippy/Rustdoc, fmt,
V1/V5/V15/V26 and reverse lineage; no increase in V16. Limits:
MemoryMax 12 GiB/High 8 GiB/swap0/jobs1; heavy tasks sequentially. Measure consumption
without inferring global benefit from representation alone.

## Ratified Heading supplement facade before queries

### Evidence before decision

HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, uncommitted working tree;
final-freeze.json.gz/target-controls.json.gz in the same callback diagnostic retain
UTC/diffstat/sources/argv. Pin `bd5ebec285505889e8ff1d4f68804ab147a699f3`.
heading-raw-constructor-func passes baseline/vanilla but fails candidate because
Styled transport changes func. Query sees a temporary causal wrapper and HTML
ignores callback panic. Sources: walk.rs:930–945, pipeline/paged.rs:419–427 and
pipeline/html.rs:101–106 prepare query through pure walk. These are language
values/errors, not a required upstream algorithm/structure. Owner confirmation
is in `lab/history/construction/records/00_nucleo/materialization/heading-supplement-runtime-2026-10-17/review.md`,
SHA-256 `66739b13596013f4343bdc3ffacbdb9081151e3d9863e5d962e8ff13fd68775c`.

### Exclusive approved facade

Expose only approved
`realize_heading_supplements(intr: &mut TagIntrospector, engine: &mut Engine<'_>, ctx: &mut EvalContext) -> SourceResult<()>`.
Delegate to the owning child; finalize only pending Heading, without walk,
State/Counter/equations/layout/export. Other entrypoints keep signatures; update
the internal child call to the index without requiring tags.

### Acceptance and limits

Ratified H1–H8 with bilateral RED before code. No public contract beyond this
Heading facade is authorized here; the five State helpers have their separate
ratification. No new public fields. Ordinary Styled/other callbacks keep their
boundaries. Validate workspace/doctests/build/strict Clippy/Rustdoc/fmt/
V1/V5/V15/V26/reverse and no V16 increase, under 12 GiB/High 8 GiB/swap0/jobs1 with
heavy commands sequentially. RAM benefit requires measurement.


## Approved Figure/Grid occurrence counter correction

### Evidence before decision

Baseline HEAD `731f0ffdf4c7c152f590551e72ff63f7bd85ef85`; the reviewed current-pin measurements and exact source anchors are retained in `lab/results/v16-corrections-2026-10-19/figure-kind/review.md`, `agents/counter/design.md` and `agents/counter/counter-probes.json`. In that evidence, captionless/numbered Figures count; manual Set/Func actions make caption ordinal indexing incorrect; typed element kinds and custom names require distinct keys. These are language values/errors, not Rust or rendering-byte equality. The human explicitly approved the complete Figure/Grid contracts, phases and compatibility changes in the current implementation session. This approval satisfies ADR-0127 for this exact scope. Historical conflicting caption-only or productive string-projection clauses are superseded only as described here; unrelated contracts remain.

### Exclusive responsibility

After complete walk, outline synthesis and parent/index preparation, finalize demanded Figure histories before returning the candidate. Add public introspect_bundle_documents_with_runtime_targets_and_starts(bodies: &[Content], targets: &[EvalTarget], features: Features, engine: &mut Engine, ctx: &mut EvalContext) -> SourceResult<(TagIntrospector, Vec<LocatorSeed>)>. Reject length mismatch before walk/effects with detached error `bundle body target metadata does not match body count`; empty/empty is valid. Existing runtime bundle facade is an all-Paged wrapper using ctx.features. Preserve global ordered actions/other postprocessors; gather only Paged implicit demands with supplied Features and Paged observer, across complete bodies, then publish one complete snapshot. HTML explicit reads remain independent; no implicit HTML rendering claim.

### Acceptance and limits

L0 precedes functional RED and implementation. Require independent fixed expectations for complete demanded histories, captionless/hidden/None demand controls, manual actions before/after requested Locations, empty Set state, typed aliases/homonyms/custom names, actual Paged caption/reference text, both-target explicit reads, bundle metadata rejection before effects and candidate diagnostic retention. No callback count as parity oracle. Preserve the independent filtered-read/replay/Sink algorithms and HTML implicit-rendering debt. Require exhaustive real-enum extension probes at new owned decisions, all twelve established gates, full differential lint, canonical owner paths, V1/V5/V15/V26 and reverse lineage with scoped resealing. No lint suppression, contract weakening, new defaults, memoization or public changes beyond the reviewed approval. Record HEAD/dirty file list, UTC, argv and executable identity for measurements.


## Approved physical-page source construction

### Evidence before decision

Clean baseline HEAD `63b7b96b99109b61655b373dc0d248beb7838f86`; exact source/owner bytes, hashes, UTC and working-tree state are archived in `lab/results/set-page-header-implementation-2026-10-21/agents/index/before-l0.json.gz`. Current-pin bilateral source/query evidence and independent source-driver anchors are in `lab/results/set-page-header-2026-10-21/baseline-parity/receipt.json` (SHA-256 `38520d31351eb01fe7a191243a6a36318aba0c60473e0a2cc54103b36d8866bf`) and `lab/results/set-page-header-2026-10-21/agents/parity/index-driver.md`. Repeated physical raw marginals contribute distinct located semantic occurrences; counter/query order is background/header/body/footer/foreground. Dormant overwritten templates remain inert. These are language semantics/refusals, not frame scanning, Rust equality or execution-count parity. The human ratified the exact additive API/phase package in the frozen `lab/results/set-page-header-2026-10-21/agents/core/proposal.md` (SHA-256 `f509e593d67ec2c2ca19f9c32f2d561a9149d4db903e5b12f11d76dbf9cbdaa2`).

### Exclusive responsibility

Retain every legacy pure/runtime/bundle signature and its existing order. Cooperate privately with the approved child facade so new planned bundle construction shares document starts, actual target/features, bibliography conversion, source parents and existing runtime postprocessor dependency order. Do not call a preliminary body-only runtime fold and subsequently splice running actions into already calculated state. HTML entries retain their semantic target and do not admit paged templates.

### Acceptance and preserved boundaries

Require owning L0 before productive code and actual functional RED→GREEN, complete source-packet/parent/authority proofs, fresh raw versus Located-alias controls, retained retries and no phantom dormant/measurement effects. Reuse the shared arena and existing strict read/Sink transactions. No additional public surface, default, field, phase or compatibility change beyond the frozen approved package; no global Frame introspection, copied histories, invented Location, I/O or observer weakening. Preserve old tests and immutable F/G receipts; maintain exhaustive closed decisions and validate full suites/build/strict lint/Clippy/Rustdoc/fmt/paths/forward and reverse lineage/Nucleus DAG. Unrelated bounded debts remain explicit.
