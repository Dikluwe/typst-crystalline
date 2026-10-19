# L0 — introspect/entrypoints
Hash do Código: abbab208

Núcleos Tekt:
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
