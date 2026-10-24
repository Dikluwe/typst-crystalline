# L0 — introspect/outline_entries
Hash do Código: bef9c380

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/physical-page-occurrences.toml sha256:b33f2e3c6238da01aa48783aecee6a8b6153b4f36ddcdc23edfe78434bae5b42

## Ownership and evidence

Layer L1. Exclusive productive consumer: `01_core/src/compiler/introspect/outline_entries.rs`.
At clean-tree state `df72a6c4a701917ab56a7c6169764a92ea775693`, `01_core/src/compiler/introspect.rs:922-988` contains the behavior assigned to this unit. Source and state are recorded in `lab/results/construction/diagnostics/atomizacao-introspect-2026-09-30-before.json.gz`. Separation is by behavior, preserving the API and without a performance claim.

## Contract

Materialize eligible entries after traversal, preserving the heading's causal Location, the entry's own Location, depth limits and recorded prefix/fill.

Language invariants, causal order and parity limits remain defined in `00_nucleo/prompts/core/compiler/introspect.md`; this owner must not create a second interpretation. Code uses static L1 calls without vtables, I/O or mutable global state.

## Acceptance

Preserve introspection tests and the L1 suite, build, and pass `crystalline-lint .` with consistent hashes. Future semantic changes require language measurement and an L0 update before code.


## Direct entry indexing and causal synthesis order

### Evidence before decision

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working tree;
order-before.json.gz/before.json.gz preserve state, UTC, hashes and diffstat.
trace-runtime-result.json/trace-runtime.log show the direct entry in the pure,
runtime and L3 index under NativeElement. Bare outline.entry converts to
Kind(OutlineEntry), per foundations/selector.rs:113, which the current terminal
walk does not populate. measurement-order-before.json at historical pin
`9f2b6e8715237cb086899a42873660fe744622e8` measures BARE [Heading] versus
[Heading,Direct], FILTERED [Direct,Heading] versus [Heading,Direct], STATE 2
versus 1 and COUNTER ((2,),) versus ((1,),) at the synthesized entry Location.
These are language observables, not Rust/byte equality requirements. Post-walk
synthesis allocates after updates, while its causal position is inside the
preceding outline. The realization-order nucleus already prohibits treating
numeric identity as order. Order, temporal-prefix, duplication or entry-data
failures refute the correction.

### Exclusive responsibility

Keep synthesis post-walk with the same fields, filters, depth, prefix, fill,
entry Location and causal heading Location. Use outline_elements order aligned
with kind_index Outline to identify the real anchor by index, without Rust
equality/names/content hashes or truncating synthesis. Collect new Locations per
outline and register their order immediately after the outline Location in
heading order through internal CounterRegistry/StateRegistry helpers. Preserve
all Locations and existing tags' relative order; integrate only synthesized
occurrences' causal position. A missing anchor in a partial legacy snapshot
retains the previous fallback without omitting entries. Finally sort kind_index
OutlineEntry by recorded causal order so bare/where/native agree, including
multiple outlines and a direct entry between them. Do not reconstruct/reexecute
the document, emit extra nodes or change phase, cardinality or field formulas.

### Acceptance

Before change, functional tests cover the owned entry's bare index/aliases,
three-entry order (two outlines with a direct entry between them), where and no
phantom heading; State/Counter reads at the synthesized Location reflect the
outline prefix. Then write the two internal helpers' contract tests before their
implementation: unknown anchor, duplicates/repetition, nonmonotonic IDs and
dense append. Repeat nine CLI probes without relaxed results: six original plus
order/state/counter. Validate full suite/build/fmt, strict Clippy/Rustdoc,
V1/V5/V15/V26/reverse/diff and SARIF: six V16 removed, seven expected V19, three
existing messages updated and no new V20. Under 12 GiB, zero swap and one job,
heavy commands run sequentially. This ADR-0127 internal parity correction uses
crate-private helpers and nine exclusive L0/consumer pairs; no external
signature, deliberate default or phase change. Restricted resealing only;
no commit/push/release. Other V16 findings remain open.


## Approved physical-page source construction

### Evidence before decision

Clean baseline HEAD `63b7b96b99109b61655b373dc0d248beb7838f86`; exact source/owner bytes, hashes, UTC and working-tree state are archived in `lab/results/set-page-header-implementation-2026-10-21/agents/index/before-l0.json.gz`. Current-pin bilateral source/query evidence and independent source-driver anchors are in `lab/results/set-page-header-2026-10-21/baseline-parity/receipt.json` (SHA-256 `38520d31351eb01fe7a191243a6a36318aba0c60473e0a2cc54103b36d8866bf`) and `lab/results/set-page-header-2026-10-21/agents/parity/index-driver.md`. Repeated physical raw marginals contribute distinct located semantic occurrences; counter/query order is background/header/body/footer/foreground. Dormant overwritten templates remain inert. These are language semantics/refusals, not frame scanning, Rust equality or execution-count parity. The human ratified the exact additive API/phase package in the frozen `lab/results/set-page-header-2026-10-21/agents/core/proposal.md` (SHA-256 `f509e593d67ec2c2ca19f9c32f2d561a9149d4db903e5b12f11d76dbf9cbdaa2`).

### Exclusive responsibility

Materialize entries from the complete admitted ordered headings and existing source snapshots after their prerequisite construction. Preserve referenced versus newly emitted source boundaries, existing identities and semantic field limits. No running template admission or geometry inference belongs to this owner; a generated outline clone must not create duplicate source actions.

### Acceptance and preserved boundaries

Require owning L0 before productive code and actual functional RED→GREEN, complete source-packet/parent/authority proofs, fresh raw versus Located-alias controls, retained retries and no phantom dormant/measurement effects. Reuse the shared arena and existing strict read/Sink transactions. No additional public surface, default, field, phase or compatibility change beyond the frozen approved package; no global Frame introspection, copied histories, invented Location, I/O or observer weakening. Preserve old tests and immutable F/G receipts; maintain exhaustive closed decisions and validate full suites/build/strict lint/Clippy/Rustdoc/fmt/paths/forward and reverse lineage/Nucleus DAG. Unrelated bounded debts remain explicit.


## Shared-frame derived entry allocation

The legacy allocator at `01_core/src/compiler/introspect/outline_entries.rs:63` allocates an actual synthesized entry only after the complete admitted headings are known. The physical index facade therefore delegates this same synthesis through a private allocation callback supplied by its installed retained shared index frame. The existing Locator-taking helper delegates to the same implementation and remains unchanged in signature and behavior. Retain entry field snapshots, source heading identities, depth filtering, outline anchoring, causal insertion and final kind ordering; no document/body replay or placeholder element is introduced. The callback allocates each actual entry once in synthesis order, and retained retries reuse its slot.


## Compatibility entries after actual Outline realization

Measurement precedes decision: the public target-aware source-index control
`page_running_realized_outline_entry_has_no_compatibility_duplicate` in
`01_core/src/compiler/introspect/page_running.rs` executes under a real retained
arena and Engine. Its exact RED is in
`lab/results/set-page-header-implementation-2026-10-21/source-index-diagnostic-14/receipt.json`.
The source facade already admitted one actual Entry in an explicitly untitled
realized Outline, then this helper synthesized another from the source Heading.
The earlier ordinary pipeline published its index before Outline realization;
that ordering cannot justify duplicate entries in an actual-product candidate.

Skip compatibility synthesis for an Outline whose realized body is present.
Its admitted actual Entry occurrences are authoritative. Preserve the existing
loop index when aligning the remaining raw Outline records with their locations,
and preserve all depth/fill/prefix/source-element fields, temporal placement,
allocation callback and final ordering for every unrealized Outline. Do not
remove actual body traversal, merge entries by payload, filter query results or
alter ordinary pre-realization indices. The public facade must expose the one
real actual Entry without a second synthesized compatibility object.
