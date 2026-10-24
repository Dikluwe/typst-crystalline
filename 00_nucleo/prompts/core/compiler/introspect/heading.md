# L0 — introspect/heading
Hash do Código: 2fc48e32

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/introspection/content-snapshot.toml sha256:d9b2ab50748dec1246d8a55c0b702fa2ee5e23dd737bedf3689e6c9f0297ac29
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:7510f3bba27256221d4447db608839af562ac71b4eacc0c042f693b531a95184
- 00_nucleo/prompts/_nuclei/layout/physical-page-occurrences.toml sha256:b33f2e3c6238da01aa48783aecee6a8b6153b4f36ddcdc23edfe78434bae5b42
```

## Ownership and evidence

Layer: L1. Sole consumer: `01_core/src/compiler/introspect/heading.rs`.
The current snapshot_fields body at `:22` and TOC projections at `:93`,
`:122` implement the helpers below. The parent calls snapshot_fields while
the causal chain/label is available (`01_core/src/compiler/introspect.rs:2016`);
the Heading branch applies inclusion gates after walking the body.
`01_core/src/compiler/stdlib/structural/heading.rs:127` and
`01_core/src/compiler/eval/rules.rs:1620` delimit the existing producers.
Implemented snapshots must not be described as an awaiting-materialization
proposal, nor as evidence of support for every parser-accepted heading option.

## Walk-branch refinement measurement

At the uncommitted state captured in
`lab/results/construction/diagnostics/atomizacao-introspect-heading-label-2026-09-30-before.json.gz`,
`01_core/src/compiler/introspect.rs:2083-2220` owns the Heading walk branch,
while this consumer already owns snapshot and TOC computations. The `introspect`
test filter passed 263 tests before movement. This is evidence of separable
behavior, not a new timing or bilateral vanilla measurement.

## Complete modeled snapshot

Keep the pure descendant helper receiving HeadingElem, StyleChain and optional
causal Label and returning the ordered public field map:

| Field | Captured value |
| --- | --- |
| level | The modeled heading level as an integer. |
| depth | The modeled level when HEADING_SET_DEPTH is present; otherwise 1. |
| offset | 0, without inventing an unmodeled custom offset. |
| numbering | Exact string only when heading.numbering is Bool(true) and heading.numbering.pattern is Str; otherwise None. |
| supplement | Preserve an explicit custom specification during private construction; otherwise Text Content from the canonical heading-language supplement owner at the causal language. Runtime finalization publishes a newly resolved immutable snapshot before query, as specified below. |
| outlined | The stored boolean. |
| bookmarked | Explicit boolean, or Auto when absent; not is_bookmarked(). |
| hanging-indent | Auto for the currently modeled producer. |
| body | The frozen semantic body Content, not plain text. |
| label | Causal Label, appended last only when present. |

A stale ancestral pattern does not override a disabled gate.
For a synthetic active gate without a recoverable language string pattern,
capture numbering None; do not derive a pattern from a rendered prefix or
claim parity of that synthetic input. Function/internal non-string patterns
are not fabricated into strings.

Use only `compiler::lang::heading_supplement_for_lang`, owned by
`00_nucleo/prompts/core/compiler/lang/heading_supplement.md`. Missing language
uses English without mutating StyleChain or activating hyphenation.
Do not load translations at runtime, infer an environmental region or maintain
a competing table. French is Section through the canonical owner.
The pinned snapshot nucleus owns authoritative presence and causal immutability.

## TOC projections

Compute sequential auto-toc labels and location-aware heading numbers without
mutating the introspector. When numbering is active, the auto-label text and
outline prefix are the hierarchical number with a trailing dot; when inactive,
auto-label text is empty and the optional outline number is absent.
Do not add a heading supplement to the outline number.
Return the supplied frozen body and level with the auto label and optional
number. The parent walk allocates the real Heading Location and emits its real Start/End.
The Heading descendant owns its complete walk branch: increments the threaded
auto-label counter, captures numbering from the causal StyleChain, freezes the
body at the Heading Location, walks descendants once through the parent walk,
then emits derived auto-label, TOC and bookmark tags in that order at the same
Location. The descendant independently gates outlined and bookmarked records.
It calls the parent tag-store population helper before each derived Start and
keeps each paired zero-hash End. The parent match remains exhaustive and
delegates this branch by a static free-function call.
Body references' supplemented text belongs to
`00_nucleo/prompts/core/compiler/introspect/labelled.md`.

## Acceptance and modeled boundary

Cover default/markup/explicit levels, transported string numbering and set none,
causal language/label, snapshot reuse after style changes, auto-label sequencing
and numbered/un-numbered TOC projection. Preserve counter/TOC behavior.

A complete snapshot means complete fields of the modeled producer, not universal
heading parity. The later callback contract below supersedes the historical supplement
Content/Func/none exclusion only on its specified routes. Numbering Func, custom
offset or hanging indent, untransported region and all set-rule heading fields
are not made supported by this helper. Constructor outlined/bookmarked and explicit
level already exist; do not incorrectly classify those as future features.
Precedence of constructor numbering:none under an ancestral set and show-set
language effects across phases require separate evidence/owner decisions.
No new constructor, callback API, public field or phase is authorized here.


## Measured arity boundary and nominal lint exception

### Structural evidence before decision

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working tree,
captured at `2026-10-15T16:39:18.873477+00:00`. Full sources, prior L0,
signatures/hashes/UTC/status/git diff HEAD --stat are in
`lab/results/construction/diagnostics/too-many-arguments-2026-10-15/before.json.gz`.
Individual classification and owner clauses read before deciding are in
`lab/results/construction/diagnostics/too-many-arguments-2026-10-15/classification.json`.
Clippy spans were reconfirmed against inventory sources. This measures Rust
structure, not fresh parity or a functional defect: C009,
`01_core/src/compiler/introspect/heading.rs:147`, `walk_heading`, has eight
arguments against threshold seven.

### Exclusive prescription and scope

The Heading descendant free function under ADR-0109 keeps borrowed accumulators
and the causal chain, delegating recursion to walk. Its arity follows static
atomization without moving rendering into entities. Preserve its signature,
argument order and semantics. The exact machine metadata remains:
`#[expect(clippy::too_many_arguments, reason="C009: Free function descendente da feature Heading sob ADR-0109: conserva os acumuladores emprestados e a chain causal, delegando a recursão ao walk")]`.
This Portuguese reason is literal metadata, not untranslated normative prose.

The initial audit prescribed an internal maintenance target and did not
materialize it. Preserve current public signatures and this owner's language
decisions; the maintenance judgment is supported by the stated evidence, while
arity alone proves no functional defect. Confirm the warning with workspace/
all-targets Clippy and no `unfulfilled_lint_expectations` after annotation;
preserve owner calls and functional controls. Do not raise the global threshold
or silence other lints.

Only the named function may carry this exact lint expectation. Never apply it
to a module/impl/crate/global configuration. A residual exception for dead-
argument corrections depends on that correction. Remove/review the expectation
if arity falls below threshold; never suppress unfulfilled expectations.
This specific permission supersedes generic suppression prohibitions only for
the justified nominal item/lint. All other corrections remain required.

The original audit updated only L0 and its lineage seal; code attributes and
signature/body changes remained pending materialization. Corrections require
owner language/integration tests and before/after call controls, without relaxed
fixtures/tolerances. Preserve layers/phase/defaults/static dispatch and no mutable
global state. Private signature corrections also migrate callers and their L0
before code. New public/compatibility requirements need a concrete ADR-0127
proposal and confirmation; this exception grants neither.

### Materialization evidence and acceptance

The validated inventory
`lab/results/construction/diagnostics/clippy-locais-2026-10-16/clippy-inventory.json`
retains 79 warnings. Source/span/current-reason/signature/hash reconfirmation is
in `lab/results/construction/diagnostics/clippy-excecoes-2026-10-16/before.json.gz`,
with HEAD, uncommitted working tree, UTC and git diff HEAD --stat. Concrete
functions/data were read before deciding. Arity/type heuristics are Rust
mechanics, not new parity or measured cognitive benefit.

The 2026-10-16 materialization covers only C009's justified direct expectation,
reconfirmed against current spans and individual clauses. Use the nominal
function attribute, preserving signature/body/calls/cfg/unrelated attributes.
Correction entries, including conditional C047/C066, remain pending and receive
no annotation in that batch.

ADR-0127 continuous flow adds lint metadata only: no public contract, executable
body, cfg/default/phase/dependency/architecture change. Only explicit nominal
exceptions supersede generic suppression prohibitions. L0 precedes code,
restricted resealing and proof that removing added attributes plus lineage
header recovers the prior source after rustfmt normalization. Measure RED first;
then check all-targets inventory: exactly 46 arities and five test warnings
satisfied by 49 expectations, no new warning, unfulfilled expectations as errors.
The 11 arities requiring correction remain visible. Validate build/fmt/strict-
and reverse-lineage/Tekt without new errors or warnings, existing Counter tests
and coverage under 12 GiB/zero swap/one job. The earlier full suite is evidence
for the same executable code; add no tests mirroring lint attributes and require
no fresh performance comparison for metadata.

## Ratified compact carrier emission

### Evidence before decision

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working tree;
`lab/results/construction/diagnostics/clippy-enums-publicos-2026-10-16/before.json.gz`
retains UTC/sources/L0/hashes/status/git diff HEAD --stat. Existing owners were
read before proposing. probe-details.json retains rustc 1.92/argv/rlib/binary/
probe SHA-256/output; probe-result.json records 12 GiB/zero swap/one job/events.
This synthetic carrier/register-copy probe is not compile/layout/RSS benchmarking
or a real workload distribution. Candidates mirror real domain types; inline
uses productive Tag/ShowRule.

Tag is 416 bytes, boxed/Arc Start 48. For 4,096 Start/End pairs with heap-free
Heading, requested bytes/allocations are inline 3,407,872/1, Box 2,031,616/4,097,
Arc 2,097,152/4,097. Cloning boxed Tag allocates per Start; inspected productive
fixpoint.rs:86–136 and entrypoints.rs:44–66 use temporary Vec<Tag>/borrowed reads,
without cloning the vector. Walk:325–356 populates the index by reference before
moving info to Start. Synthetic clone frequency is not assumed productive.

Transformation is 328 and ShowRule 376 bytes; Box/Arc<Styles> candidates are 120
and 168. rules/show.rs:464–470 copies to_vec before every registration;
apply.rs:108–112,349–352 filters/clones rules. Across 128 registrations of simple
100% show-set without retained old snapshots, requested bytes/allocations are
inline 12,276,960/384, Box 8,194,560/8,640, Arc 5,530,624/512. With retained
snapshots, requested peaks are 3,203,856, 4,141,744 and 1,477,808. A custom-channel
control copying 4,096 rules requests 4,107 inline, 8,203 Box and 11 Arc
allocations, including Vec growth. apply/intercept/outline/table_cells borrow
styles; application still clones a delta when needed.

Mandatory counterexample: building one simple 100% show-set collection of 4,096
rules requests 1,540,096 bytes inline, 2,031,616 Box and 2,097,152 Arc. Arc may
retain excess custom-channel capacity; the rich 128-registration control without
old snapshots ends with 78,064 live bytes inline and 156,432 Arc. Claim no
universal gain or ignore atomic/indirection cost. This is ADR-0030/0107 storage
mechanics, not language parity or measured AI benefit. Real relevant memory/time
regression refutes the favorable copy-cycle inference; retain those controls.

### Ratification and exclusive responsibility

The human approved manifest
`282da3f8f79080a4831e1898879775e32a62caa0b2ffa941b8fca9198136b950`;
`lab/results/construction/diagnostics/clippy-enums-publicos-2026-10-16/approval-receipt.json`
records approval. The former ADR-0127 public-payload/compatibility proposal gate
is satisfied; this contract, including T1–T6 and no release, is in force.

Migrate only existing Tag::Start(loc, info) emission to
Tag::Start(loc, Box::new(info)). Preserve info construction/values, index
population before moving info, loc/End/hash, walk order and effects/counters.
There is one allocation per Start; do not clone info or move population, change
deduplication/reservation/snapshot/recursion. Public form belongs to
`00_nucleo/prompts/core/entities/tag.md`; this owner preserves its own behavior.

### Acceptance and owner boundaries

Ratified L0 precedes tests and real RED; reseal strictly changed owners. No
large_enum_variant suppression, threshold/dependency/flag/default/phase change.

- T1: construction Box<ElementInfo>/Arc<Styles> and relative budgets (Tag at most
  three Location; Transformation at most Content plus alignment) fail before
  and pass after without freezing a universal ABI. Those payloads stay owned by
  their entity prompts.
- T2: preserve Start/End/Location/payload/label, value equality/structural hashes,
  independent Box clone and mutation isolation; hashes are not persistence format.
- T3: existing introspection/convergence/nesting/heading/label/state/counter/query/
  metadata tests retain expectations/errors.
- T4: Transformation::Style clone shares Arc; Arc::make_mut on a copy preserves
  original styles including custom; Func/Content/Str variants remain unchanged.
  This is a dependent entity boundary, not implementation in convergence.
- T5: show-set/func, precedence/scope/labels/outline/table.cell/textual/regex
  controls preserve semantics/morphology/errors with no extra execution.
- T6: full workspace/build/all-targets Clippy with prior expectations/lints as
  errors, rustdoc -D warnings with four indexes, fmt/diff/strict/reverse lineage
  and no Tekt increase, under 12 GiB/zero swap/one job. Record HEAD/working tree/
  hashes/diffstat/UTC and distinguish requested sizes/bytes from RSS.

No release. Any signature/field/phase change beyond these ratified payloads
requires a separate proposal. This owner preserves its existing emission behavior apart from the ratified boxing.

## Heading callback evidence and responsibility

The 2026-10-17 evidence records HEAD
`b1b486f3d42431f8ea360bc5712637ee1e3d4f6c` and working-tree/UTC/hash provenance
in `lab/results/construction/diagnostics/lote-a1-callbacks-reflexao-seletores-2026-10-17/before.json.gz`.
`before-pdf-probe.json.gz` records `heading-supplement-panic` succeeding in Tekt
and producing `A1_HEADING` in vanilla
`bd5ebec285505889e8ff1d4f68804ab147a699f3`. Both accept
`heading-supplement-valid`, but only vanilla uses `Custom` in the reference.
Upstream `model/heading.rs:249–289` synthesizes supplement per occurrence;
`model/reference.rs:393–415` applies the function and display.
`Set.rs:286–320` ignores supplement and clears numbering;
`snapshot_fields` at `heading.rs:55–68` fixes translation. These are observable
callbacks/values, not required upstream mechanics.

snapshot_fields preserves an explicit custom heading.supplement specification;
without one it keeps the causally translated default. Func is still a private
specification under construction, not a published result. Runtime finalization
replaces the entry with a newly resolved immutable snapshot before query receives
it. This supersedes the historical supplement Content/Func/none exclusion only
on these routes. Add no new translation table.

This parity correction follows ADR-0127 continuous flow: no new Rust API, public
field or transfer of existing logic between eval and layout. The existing
post-walk Engine boundary applies callbacks. L0 first, RED and bilateral controls;
no callbacks in encoders/walk/layout or claim of parity for every field.


## Approved physical-page source construction

### Evidence before decision

Clean baseline HEAD `63b7b96b99109b61655b373dc0d248beb7838f86`; exact source/owner bytes, hashes, UTC and working-tree state are archived in `lab/results/set-page-header-implementation-2026-10-21/agents/index/before-l0.json.gz`. Current-pin bilateral source/query evidence and independent source-driver anchors are in `lab/results/set-page-header-2026-10-21/baseline-parity/receipt.json` (SHA-256 `38520d31351eb01fe7a191243a6a36318aba0c60473e0a2cc54103b36d8866bf`) and `lab/results/set-page-header-2026-10-21/agents/parity/index-driver.md`. Repeated physical raw marginals contribute distinct located semantic occurrences; counter/query order is background/header/body/footer/foreground. Dormant overwritten templates remain inert. These are language semantics/refusals, not frame scanning, Rust equality or execution-count parity. The human ratified the exact additive API/phase package in the frozen `lab/results/set-page-header-2026-10-21/agents/core/proposal.md` (SHA-256 `f509e593d67ec2c2ca19f9c32f2d561a9149d4db903e5b12f11d76dbf9cbdaa2`).

### Exclusive responsibility

Capture source operands and chain-dependent field snapshots before source context is lost. For a validated planned root, derive auto-label/TOC/bookmark/frozen-body projections incrementally against the candidate at the specified causal root/post-child boundaries, not a stale body-only counter snapshot. Sharing one Location among root and derived facts does not duplicate root action or parent membership. Keep the existing static helper ownership and legacy traversal behavior.

### Acceptance and preserved boundaries

Require owning L0 before productive code and actual functional RED→GREEN, complete source-packet/parent/authority proofs, fresh raw versus Located-alias controls, retained retries and no phantom dormant/measurement effects. Reuse the shared arena and existing strict read/Sink transactions. No additional public surface, default, field, phase or compatibility change beyond the frozen approved package; no global Frame introspection, copied histories, invented Location, I/O or observer weakening. Preserve old tests and immutable F/G receipts; maintain exhaustive closed decisions and validate full suites/build/strict lint/Clippy/Rustdoc/fmt/paths/forward and reverse lineage/Nucleus DAG. Unrelated bounded debts remain explicit.
