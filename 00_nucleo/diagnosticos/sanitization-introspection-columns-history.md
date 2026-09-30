# Introspection and columns: documentary sanitization history

## Scope, provenance and limits

This diagnostic records a documentation-only audit of
`00_nucleo/prompts/compiler/introspect.md`, its seven child prompts,
`00_nucleo/prompts/compiler/columns.md` and
`00_nucleo/prompts/compiler/footnote_overflow_columns.md`.
It is not a materializable L0, a global sanitation verdict or permission to
change productive code.

Audit HEAD: `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`.
Baseline capture: `2026-09-27T00:45:59.314559+00:00`.
The working tree was already uncommitted; the assigned prompts had no local
changes at intake. The source observations below refer to the working-tree
bytes, not necessarily HEAD. No binaries were rebuilt and no parity or runtime
tests were run for this documentation change. Source existence is implementation
evidence, not independent GREEN evidence.

Repository instructions and active ADR-0107/0108/0109/0127/0129/0130 govern
language-level acceptance, evidence before decisions, static feature ownership,
public/phase gates, 1:1 lineage and canonical L0 references. Older migration
ADRs explain history only; their obsolete intermediate forms do not supersede
current contracts. Existing shared nuclei and pins are unchanged.

The active upstream target from the repository instructions is
`9f2b6e8715237cb086899a42873660fe744622e8`, not a moving main or version
label. The ratified oracle is `lab/typst-original/target/release/typst`, with
the recorded SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`;
its recipe/provenance is the existing P1489 diagnostic. No new oracle identity
check or language comparison was performed in this documentation audit.

## Reproducible audit evidence

Reinspect the actual functions rather than comments that describe old phases:

```sh
git rev-parse HEAD
git status --short -- 01_core/src/compiler/introspect.rs 01_core/src/compiler/introspect 01_core/src/compiler/layout/columns.rs 01_core/src/compiler/layout/footnote_flush.rs
rg -n '^pub |^pub\(crate\)|^impl Introspector|Selector::Before|Selector::After|record_action|numbering_active' 01_core/src/compiler/introspect.rs 01_core/src/compiler/introspect/from_tags.rs
rg -n 'run_fixpoint\(|introspect_to_fixpoint\(' 01_core/src 03_infra/src 04_wiring/src
rg -n 'introspect_with_runtime|introspect_bundle_documents_with_runtime|introspect_with_runtime_for_pipeline' 03_infra/src
rg -n 'OutlineEntry|_ => None' 01_core/src/compiler/introspect/locatable.rs 01_core/src/compiler/introspect/extract_payload.rs
rg -n 'page_columns|footnote|column_width|horizontal\(|body_dir|TextStyle|close_current_column' 01_core/src/compiler/layout/columns.rs
rg -n 'top_safe|force_emit|layout_warnings' 01_core/src/compiler/layout/footnote_flush.rs 01_core/src/compiler/layout/mod.rs 03_infra/src/pipeline/paged.rs
```

The relevant audited source-byte identities and intake status follow. They are
provenance for this audit, not an enduring working-tree inventory in an L0.

- `01_core/src/compiler/introspect.rs`: SHA-256 `06eafdb1654ec11306224a06b5ce98e121591db7ed53d5ba36e32744d81f53a8`; intake `M 01_core/src/compiler/introspect.rs`.

- `01_core/src/compiler/introspect/convergence.rs`: SHA-256 `7ebec8616972fa232880538ae23328c09f1fd3dbbb789880e357742f32a1bb5e`; intake unchanged relative to HEAD.

- `01_core/src/compiler/introspect/extract_payload.rs`: SHA-256 `49af59772123ae6786eccbb9b20738ddb7f0a5720ac84fb5120747190213a834`; intake unchanged relative to HEAD.

- `01_core/src/compiler/introspect/fixpoint.rs`: SHA-256 `137dbf43fba5cc846418c7341c872ee63b017dfcf1c7c222534682d2470da084`; intake `M 01_core/src/compiler/introspect/fixpoint.rs`.

- `01_core/src/compiler/introspect/from_tags.rs`: SHA-256 `b2ecda795032e64911c73e91bb73ccfdbdcbaaff54b75c9f01d96e7f7e02e6fe`; intake `M 01_core/src/compiler/introspect/from_tags.rs`.

- `01_core/src/compiler/introspect/heading.rs`: SHA-256 `5cc735fa132f934f057a964dd536cb44c365d00bb2bc98abb3143eba3ec21009`; intake unchanged relative to HEAD.

- `01_core/src/compiler/introspect/labelled.rs`: SHA-256 `ccd5e262dc3b5a3e57cb28cc753af29f94ab1747e8817e547c4f992e1d5b1622`; intake unchanged relative to HEAD.

- `01_core/src/compiler/introspect/locatable.rs`: SHA-256 `2f7c68b0aed4da3352c86f9b1f804cf92bb0272bb15c75e8697d3e192283cc57`; intake `M 01_core/src/compiler/introspect/locatable.rs`.

- `01_core/src/compiler/layout/columns.rs`: SHA-256 `da35d0d6de081bc709e184502236ba894262cf7c8b3eb9ba1bf6701b67a2fcec`; intake `M 01_core/src/compiler/layout/columns.rs`.

- `01_core/src/compiler/layout/footnote_flush.rs`: SHA-256 `9d95d7e1675de60858bf1c42a6ac6b4c49f39a01903710fe4067c4bc6ea46555`; intake unchanged relative to HEAD.

- `01_core/src/compiler/stdlib/structural/heading.rs`: SHA-256 `30312d058d9d8170a14b4b71bef464ffdb9f51d794a729b8d75d1bbdcc807bd7`; intake unchanged relative to HEAD.

- `01_core/src/compiler/eval/rules.rs`: SHA-256 `bd075800e8975e24581517deae4f3d42e56fad45aec945996cad7874598e2a76`; intake `M 01_core/src/compiler/eval/rules.rs`.

- `01_core/src/entities/content/page_structure.rs`: SHA-256 `0e5fa87032667c15cc53f6fc7c26ceebc516f852afd848bf5067df248333f680`; intake `?? 01_core/src/entities/content/page_structure.rs`.

- `03_infra/src/pipeline/paged.rs`: SHA-256 `dc276cfe4db95675050bc5a7ffa78eab90191413c701581be9c1c03777b4e0a5`; intake `?? 03_infra/src/pipeline/paged.rs`.

## Migrated history and corrected claims

### Analytical state and tag construction

The former M1–M6 narrative described tags discarded beside CounterStateLegacy,
parallel writes, successive consumer migrations and an optional-Engine
from_tags constructor. Those were intermediate execution states.
The current APIs return TagIntrospector alone; the concrete walk populates its
stores directly. CounterStateLegacy and from_tags::from_tags are absent.
Historic comments and test names are not evidence that either still exists.

The durable decisions retained from that history are a pure causal walk,
location-aware stores, separate deferred function evaluation and post-recursion
derived records for state-dependent label/TOC data. Derived records reuse
the target identity; they are not new automatic counter events.
The old M5 exception table, sub-step schedules, field/variant/method totals,
parallel-write completion claims and migration instructions were removed from
normative prompts rather than turned into current requirements.

The old SetHeadingNumbering/SetEquationNumbering variants and global
numbering_active state channels no longer define production numbering.
The walk captures effective lexical chain values in each payload.
Equation numbering is active for block Str/Func specifications, and Heading
advances only when its captured gate is active. The alleged dormant equation
gate is therefore obsolete. Figure's per-kind and element-function writes
still exist; the element-function counter is not declared removable dead code
merely because an older text called a previous string-key write "legacy".

### Runtime, convergence and counter functions

The from_tags child now owns runtime postpasses and filtered replay, not
construction/index population. Function update errors propagate; display
callbacks still defensively yield Empty on errors. The old suggestion of
future diagnostic propagation cannot erase that distinction.

CounterUpdate::Func and separate positional component arity are implemented.
The historical proposal that passed one array was superseded by the measured
two-component callback behavior (P1149); display callbacks still receive one
array. Filtered Element-containing counters select the event log with language
equality, fold the whole sequence before projecting a prefix, and evaluate
functions without introspective context. Later callback failure may affect an
earlier get. These are preserved requirements, not proof that all contextual
integration/parity gates have been independently closed.

Source search finds full run_fixpoint/introspect_to_fixpoint calls in the helper
and its tests, not L3 production adoption. The production runtime processes
provided content and does not reevaluate the whole document through that helper.
The helper retains its five-iteration cap and consecutive tag-hash comparison.
DefaultHasher's present implementation is not a contractual promise of SipHash,
cross-toolchain stability or collision freedom, nor a semantic convergence
proof. The helper does not execute all of the parent's store-preparation steps.

### Snapshots, selectors and phase boundaries

The Heading snapshot carrier and helper are implemented, despite old "proposal,
gate pending" headings. This audit does not invent a missing historical approval:
implementation and authorization records are separate evidence categories.
The immutable complete-modeled-field snapshot and pinned nucleus remain.
Strong/Emph own occurrences and the action log are implemented as well.
Test-only P1341/P1342 observation hooks were superseded by the P1353 retirement;
they are not productive instrumentation obligations.

Before/After selectors, synthesized OutlineEntry queries, prefix-info maxima,
outline realization transport, bundle Locator seeds/Page resets and the HTML
position trait projection are present in current consumers. MathMid/MathStretch
and the over/under mathematical families are already terminal in the relevant
walkers, not future work merely because a historical heading said "proposed".
No new public semantics or phase transition is approved by removing those labels.

The old R5 measurement refers to crystalline HEAD
`b303f1f15b610e09872b567027e0d806387fde8c` plus uncommitted changes and
historical upstream `a51e02804`; the former heading prompt records independent
measurement SHA-256
`82b2de8863ae5cd4b706eb9d5a6b285e1ed31dce3c126c8e5383a5af59ab46c8`.
The Strong/Emph and filtered-key decision campaign used
`2f42d64253547734564513a1159ee6b584c1c4b4` and the same older upstream.
Its language rationale was own occurrences, realized default delta, semantic
body capture and whole-log resolution before prefix, not equality of Rust data
or vanilla's execution mechanism. These old observations are not rerun here
and do not prove parity at the current upstream pin.

Heading's constructor already models explicit level, outlined and bookmarked.
Set-rule handling is narrower; snapshot offset/hanging-indent remain 0/Auto and
supplement uses the canonical language owner. Thus the old debt list must not
be read as "all of these inputs are absent" or "all are now supported".
Show-set language appearing only in the rendered tree remains a phase-boundary
finding; pre-show label transport does not resolve it.

Bundle contracts retain global introspection with one causal checkpoint before
each body and a Page-only reset/replay. The historical P1485 comparison on
`bde47c744e0bffb6229005e4b5f04c2ed0e9ef08` plus uncommitted work against
`586e1bd43` motivated isolating Page without isolating heading/state counters.
It is evidence for the recorded older snapshot, not a fresh upstream parity run.
Visible versus reference Page numbering views keep their distinct callback
arguments [current, total] and [current], while preserving raw numbering;
realization remains with its existing pipeline/entity owners.

### Columns and footnote overflow

The physical operations formerly described as a proposed move from cursor now
live in layout/columns.rs. footnote_flush.rs is the unique consumer of the
overflow prompt. Old multi-target lists no longer assign ownership over
cursor, columns, layout_types or pipeline; those are integration collaborators.

The useful history is the usable-width correction, independent sibling origins,
auto-Block spacing, per-segment fr distribution and regional line-number
consolidation. Fixed historical page counts, coordinates, fonts, timings and
temporary input filenames do not define acceptance. Geometry now uses margin
sides and horizontal()/vertical(), not a scalar 2*margin assumption.
body_dir now prefers an inner visible wrapper before its outer wrapper, not
unconditionally the first outer Styled.

Page columns flush notes by segmented region; explicit segmented containers
accumulate notes below the container at the first logical column.
Continuous flow still closes/flushed notes per physical column in both forms:
the earlier scope-out/intention for explicit-container accumulation is not
silently marked implemented. Shared introspector numbering has replaced the
old local footnote_counter save/restore mechanism.

P1416's ordinal-maximum coordination remains relevant to the non-pattern path,
while string patterns use stable baseline consolidation across explicit
siblings (P1417). Page columns consolidate separately with shared text style
(P1418); the empty-first-column capture in close_current_column is implemented
(P1419). Old "Unknown" coverage statements are evidence limits, not a blanket
assertion that the current product cannot render these cases.

The overflow history (P595, commit abbreviation `9b65181bf` with pending
P594 changes) identified main-text overlap and motivated including cursor/open
line positions in top_safe, plus warnings for forced oversized emission.
Its vanilla version/pin and pending diff were not sufficiently identified;
temporary /tmp inputs are not enduring proof. Its claim that quarantined vanilla
did not apply page columns is therefore not a current parity exception.
Warnings are now existing fields forwarded to a returned warnings vector in
pipeline/paged.rs, not a newly authorized field or Sink-writing mechanism.
An oversized forced body may overlap/truncate with a warning; no-overlap for
every body and vanilla-style continuation are not established contracts.

## Remaining ambiguities and follow-ups

- Classification/extraction: OutlineEntry is true in locatable.rs but has no
  extractor arm; walk treats it as a leaf, while materialize_outline_entries
  synthesizes its query entries separately. That explains one query path but
  does not establish direct-input allocation alignment. No source fix or new
  universal invariant was inferred. Dynamic alignment also depends on its
  preexisting kind/payload agreement.
- Authorization provenance: implemented APIs/features whose old sections still
  say "gate pending" do not automatically prove every past gate was recorded.
  This change does not retroactively approve a public change or declare the
  P1339 independent gates complete.
- Ordinary non-Element state/counter functions retain existing post-walk
  insertion semantics. Full causal recomputation of their later deterministic
  snapshots is not established merely by the filtered replay or Page fix.
- Producer limits: page_structure.rs wraps SetPage groups wherever encountered,
  including after preceding content, so the old universal mid-document
  scope-out is not supported by source absence. End-to-end parity of those
  transitions was not tested. Its synthesized Columns use gutter None, and
  the inspected page producers do not establish page(gutter:) transport;
  explicit columns(gutter:) is modeled. No broader support/rejection contract
  was created.
- Coverage: nested fractional wrappers/columns, multi-region overflow,
  heterogeneous line-number callbacks, local par.line divergence, RTL marginal
  numbering and the direct-segmented empty-first-column shared-style case
  require fresh language/geometry evidence. A source path is not a parity pass.
- Placement: top_safe uses item positions, not complete ink bounds. Oversized
  warning behavior does not prove recoverable note splitting. More exact
  clipping/overlap guarantees need measurements and owner-approved changes.
- Remainder integration: footnote_flush.rs:383 leaves deferred bodies in the
  pending buffer, but columns.rs:595 restores the saved exterior buffer after
  segmented layout. Inference: a still-pending last-segment/container remainder
  could be discarded at that boundary. No runtime reproduction was performed;
  a focused overflowing segmented-note test could confirm or refute this risk.
  The flush's existing remainder contract is retained, not replaced by a claim
  that all callers already preserve it correctly.
- Hash lineage: source @prompt-hash headers and Hash do Código metadata are
  deliberately unchanged. Editing source headers is forbidden in this batch;
  a later authorized scoped reseal is needed. No global --fix-hashes,
  --update-snapshot or formatter was run.

## Documentary validation

`git diff --check` passed for the assigned tracked prompts.
Every canonical prompt reference was checked for an existing target;
Hash do Código metadata and all existing nucleus pins were preserved.
`crystalline-lint --checks v15,v26 .` reported no violations.
Read-only `crystalline-lint --checks v5 .` reported PromptDrift warnings for
the assigned consumers, as expected with unchanged source headers, and also
for other work outside this batch. Its default warning-level exit success
is not a clean V5 verdict. Those external warnings were not repaired.
No build, test suite, formatter, source-header reseal or snapshot update was run.

## Historical originals and recovery

The intake prompts were clean relative to the audit HEAD. Recover the exact
pre-sanitization text from that immutable revision, not a dead temporary path:

```sh
git show 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5:00_nucleo/prompts/compiler/introspect.md
```

Use the same command with each canonical path below and check its SHA-256.
This retains the detailed old migration/gate narratives in version history;
the English sections above carry their relevant decisions and qualifications
into this diagnostic, without republishing them as active contracts.

- `00_nucleo/prompts/compiler/introspect.md`: `a9ec81e1596e145a4b3592a7cbaf1f027c5db15a0a81ae65172ce77807c67e6e`.

- `00_nucleo/prompts/compiler/introspect/convergence.md`: `0334a735b2a632be3f9562f610958a6bcac40a13bada3757dbbee3f3d7e06a88`.

- `00_nucleo/prompts/compiler/introspect/extract_payload.md`: `c2ff13632be6ebc395634643097f4f13c134eab6c95080bdbb4a53fb69eaf301`.

- `00_nucleo/prompts/compiler/introspect/fixpoint.md`: `9a0fb90452c91a75fe1f091af514f782b2c16a3c6d3b0b95a52b137748b76b1b`.

- `00_nucleo/prompts/compiler/introspect/from_tags.md`: `ce580c45284df1348226e6da3bbe3565e66fbec0f99f8aba6a83152de78a6eed`.

- `00_nucleo/prompts/compiler/introspect/heading.md`: `b731524d15e71111a7ced67627582cde5094cbeac598387b49d0f3c8c64758e4`.

- `00_nucleo/prompts/compiler/introspect/labelled.md`: `8f2122b0adf8d5b1d6be95bfcf911d253be4a8ed535a3aa191e93862828fa1bf`.

- `00_nucleo/prompts/compiler/introspect/locatable.md`: `e3cbed3fe1c4180f384256228601c0b18a0231d39e92859ba83176103748fd29`.

- `00_nucleo/prompts/compiler/columns.md`: `441914e618e586463cf084fe309ff1779068037cf481c84f2318267d1b9815f7`.

- `00_nucleo/prompts/compiler/footnote_overflow_columns.md`: `c2b36956b2c61d0f663e2d81f4dec4c52400714236855b11b17242abf1252571`.
