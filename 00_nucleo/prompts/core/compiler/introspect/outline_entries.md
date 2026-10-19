# L0 — introspect/outline_entries
Hash do Código: 9bb45cb9

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
