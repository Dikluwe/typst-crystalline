# L0 — TableHeaderElem
Hash do Código: b23fe53f

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/table_header.rs`

## Contract


`TableHeaderElem` stores public body: Content and repeat: bool, with
Debug/Clone/PartialEq and the specified hash obligation. Plain text and
emptiness delegate to body. Both maps recurse into body, preserving repeat
and Content::TableHeader identity. Fields/kind/payload default to None:
non-locatable. Structural equality includes body and repeat. Construction
and repetition during layout belong to their own owners.


The trait API/defaults belong to
`00_nucleo/prompts/core/entities/elements/mod.md`. This owner specifies only
this element consumer; language constructors and layout algorithms remain
with their respective owners. Historical inline-arm evidence is retained in
`lab/results/construction/diagnostics/sanitization-entities-contracts-history.md`.


## Issue 8954 — public contract proposal, NOT APPROVED

### Measurement before proposal

`lab/results/construction/diagnostics/issue-8954-2026-10-31/before.json.gz`
preserves HEAD, exact dirty diffstat/status, UTC and original owner/source bytes.
`measurements.json.gz` preserves the fresh RAM binary, ratified vanilla identity,
exact native inputs/commands/UTC/results and glyph positions. The official input
returns a vanilla panic but a controlled Tekt rejection. The auto-height control
prints 0,A,B,C,1 in vanilla. The accepted distinct-level control prints 0,A,B,1
in vanilla versus A,B,0,1 in Tekt, with actual glyph baselines confirming order.
Pinned model/table.rs:503-512 specifies ascending concurrent levels and replacement
of equal/higher levels. Repetition controls confirm A remains active when C
replaces B at level 2; nonrepeating C removes B without repeating C later.
`structural/table_grid.rs:134-161` rejects duplicate inferred Cell levels and
merges distinct groups globally; `table_header.rs:17-20` lacks group level;
`table.rs:39,57-64` has only one raw header and one prefix row count. These
carriers cannot retain repeated adjacent equal-level group boundaries independently
of explicit Cell Header/Data semantics. Header group level is not Cell kind.

### Gate

This section is a draft for ADR-0127 section 2.1 approval, not current code
authority. Existing approved contracts remain in force until an explicit human
receipt approves this exact package. Do not implement or reseal consumers for
this proposal before that receipt. The proposal changes public entity contracts
and requires migration of public Rust struct literals; parity corrections alone
do not waive that gate. No new product mode/flag or pipeline phase is proposed.

### Proposed exclusive entity amendment

Add `pub level: u32` to TableHeaderElem, retaining body and repeat. Native casting
requires 1..=u32::MAX and defaults to 1; the existing Content::table_header(body,
repeat) signature remains and initializes level 1. Clones, Hash, PartialEq and
both maps retain the level. Do not derive it from a Cell kind or semantic level.
This field represents the row-group's repetition/replacement level only.

After approval, update/reseal every actual affected consumer's owner before
its RED/code; retain all prior ratifications. This proposal authorizes no new
public helper signature, trait method, Cell-kind change, reflection feature or
PDF/UA conformance claim.


## Issue 8954 ratification

The human approved this exact public package with `autorizo`; receipt and review
manifest are in `lab/results/construction/diagnostics/issue-8954-2026-10-31/approval.json`.
This ratification supersedes the proposal-only gate above for that package.
All prior approvals and all other boundaries remain.


## Issue 8954 — approved integration

### Evidence before implementation

`lab/results/construction/diagnostics/issue-8954-2026-10-31/implementation-before.json.gz`
retains this exact owner/source, UTC, HEAD and dirty diffstat/status; measurements.json.gz
contains the fresh combined native probes and actual glyph baselines. The original
case is rejected, while distinct levels reorder 0,A,B,1 into A,B,0,1. Ordered
replacement/repeat controls establish group-level language behavior independently
of Cell kind or upstream panic mechanics. Human ratification is approval.json
in that directory, patch SHA 18e5019618e3c08e71c3a9d2f51a4f50d9b4a2e624fc70ab6d8de6c4aac6ee20.

### Exclusive amendment

Store approved level and preserve it through both maps, clone/hash/equality; existing constructor remains default level 1.

L0 and restricted reseal precede RED/code. Preserve prior ratifications, purity and
static dispatch. Validate original source order, adjacent/replaced/nonrepeating
groups, effects and no hidden admission, spans/conflicts/padding, finite region
progress, legacy controls, native PDF/SVG/HTML, proportional suites/build and full
forward/reverse lineage/lint. All compilation/tests use only the exclusive RAM
target /dev/shm/typst-crystalline-8954-target; no fallback to disk. Rich interactions
remain bounded; no global parity or PDF/UA claim. Additional public/phase changes
require a new gate; the approved fields are not inferred from diagnostics.
