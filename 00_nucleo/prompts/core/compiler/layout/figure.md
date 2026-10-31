# L0 — figure
Hash do Código: e7737e66

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/flow-frame-width.toml sha256:0684f5fcab7844d2ac81dd302d9afc7c12595cf5651a9f4c7c91daea8166b3f7
- 00_nucleo/prompts/_nuclei/layout/intrinsic-piece-alignment.toml sha256:8af39497a64b3dba29c94647c57301b023766aaa012268687421cf1fe6e4ecfb
- 00_nucleo/prompts/_nuclei/layout/physical-page-occurrences.toml sha256:b33f2e3c6238da01aa48783aecee6a8b6153b4f36ddcdc23edfe78434bae5b42

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/figure.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/figure.rs:19`, `01_core/src/compiler/layout/figure.rs:70`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. The former reference `9f2b6e8715237cb086899a42873660fe744622e8` and older vanilla pins are historical snapshots. They do not certify the current ratified reference `bd5ebec285505889e8ff1d4f68804ab147a699f3` without a new measurement.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/core/compiler/layout/mod.md`; the contract below defines this unit's operations, not ownership of that hub.

Read `figure.numbering` from the active chain: absence selects `1`, String supplies the pattern, None or another value disables numbering. For an enabled kind, consume the per-kind figure index and obtain the injected introspector number, with the existing index-plus-one fallback. Register the current page for LoF in the enabled-numbering branch, even if no caption is present. Format the number or decimal fallback and use the language owner's kind-specific supplement. Dispatch the body first; if a caption exists, emit a linebreak and dispatch the optional `supplement number: ` prefix plus rich caption. Do not own supplement tables, counter construction or outline fixpoint iteration.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.


## Approved Figure/Grid occurrence counter correction

### Evidence before decision

Baseline HEAD `731f0ffdf4c7c152f590551e72ff63f7bd85ef85`; the reviewed current-pin measurements and exact source anchors are retained in `lab/results/v16-corrections-2026-10-19/figure-kind/review.md`, `agents/counter/design.md` and `agents/counter/counter-probes.json`. In that evidence, captionless/numbered Figures count; manual Set/Func actions make caption ordinal indexing incorrect; typed element kinds and custom names require distinct keys. These are language values/errors, not Rust or rendering-byte equality. The human explicitly approved the complete Figure/Grid contracts, phases and compatibility changes in the current implementation session. This approval satisfies ADR-0127 for this exact scope. Historical conflicting caption-only or productive string-projection clauses are superseded only as described here; unrelated contracts remain.

### Exclusive responsibility

Capture current_location before children. Prepared resolved_kind Figures with present numbered visual caption use typed key and that Location prefix from prepared snapshot, never event index or figure_progress. Missing productive Location/history appends `figure counter snapshot could not be verified` at nearest Source span and returns without fabricated prefix; a present empty state is not missing and can yield an empty formatted number. Use prepared resolved supplement including explicit empty/None. Retain figure_progress/index-plus-one fallback only for unprepared synthetic resolved_kind=None, including historical numbering-enabled/no-caption increments. Func numbering counts/demands; existing missing visual formatting remains explicit debt, no fabricated decimal parity. Rendering still delegates body/caption.

### Acceptance and limits

L0 precedes functional RED and implementation. Require independent fixed expectations for complete demanded histories, captionless/hidden/None demand controls, manual actions before/after requested Locations, empty Set state, typed aliases/homonyms/custom names, actual Paged caption/reference text, both-target explicit reads, bundle metadata rejection before effects and candidate diagnostic retention. No callback count as parity oracle. Preserve the independent filtered-read/replay/Sink algorithms and HTML implicit-rendering debt. Require exhaustive real-enum extension probes at new owned decisions, all twelve established gates, full differential lint, canonical owner paths, V1/V5/V15/V26 and reverse lineage with scoped resealing. No lint suppression, contract weakening, new defaults, memoization or public changes beyond the reviewed approval. Record HEAD/dirty file list, UTC, argv and executable identity for measurements.


### Rich prepared caption supplement

The approved resolved supplement is Content. Flattening it with plain_text before emission loses language morphology (for example Strong text), violating ADR-0107. Preserve that Content in the private caption prefix and compose it with a plain numeric/separator Content node; existing synthetic String prefixes become Content::text. The private sole-caller layout_figure prefix changes Option<String> to Option<Content>, with no public gate. Require bold supplement morphology and ordinary number/caption preservation. Function number formatting remains separate debt.


### Figure block paragraph boundaries

Root's frozen target caption control reports pinned oracle `A B Figure 2` versus candidate `AB Figure 2`: consecutive Figure bodies were merged into one paragraph. Figure is a block language object independently of whether it has a caption. Add a rendered text and baseline-order RED control for two prepared captionless Figures followed by ordinary text before changing dispatch. The bounded correction must end the preceding paragraph before each body and finish the Figure's emitted paragraph before following content, preserving numbering/action/snapshot identity and existing body/caption order. No public API, new mode or eval/layout transfer; this is an internal morphology correction at the owning renderer.

Functional RED was observed by the orchestrator in `lab/results/v16-corrections-2026-10-19/figure-kind-implementation/integration-tests-6/tests.log`: `figure_blocks_keep_adjacent_bodies_and_following_text_in_separate_paragraphs` emitted ALPHA, BETA and TAIL at the same baseline. Use the existing paragraph owner before the Figure body and after its final emitted body/caption; perform no paragraph mutation before a missing productive snapshot has been rejected.


## Approved physical page occurrence protocol

### Evidence before decision

At baseline HEAD `63b7b96b99109b61655b373dc0d248beb7838f86`, the complete source/owner snapshot and applicable ADRs are retained in `lab/results/set-page-header-2026-10-21/agents/core/source-evidence.json.gz` and its supplement. `baseline-parity/receipt.json` there binds current-pin physical query/state/counter/page observations and inert overwritten templates. `ambient-baseline/receipt.json` refutes freezing creation10pt styles: insertion uses actual page20/30pt ambient style. These are language observations; the source plan, packet transcript and rollback representation are mechanics. The exact five facades, opaque types and Paged discovery/realization/source-construction phase were explicitly approved in `lab/results/set-page-header-2026-10-21/review.md` (SHA-256 `cb4a684b483e7f9f6ac3f7eef52277f7ada405033a24e4e66e55811e93c12039`), recorded in `lab/results/set-page-header-implementation-2026-10-21/approval.json`. Only conflicting no-new-phase clauses for this exact protocol are superseded; legacy signatures/defaults and unrelated limits remain.

### Exclusive protocol responsibility

Route FigureBody, FigureCaption and actually selected FigureSelectedSupplement through exact source-plan edges while retaining body/prefix/caption ordering, snapshot validation, typed numbering and paragraph boundaries. A generated numeric/separator prefix is nonadmitting; the rich selected supplement and caption keep their own source routes inside generated wrappers. Do not infer an edge by comparing Content, execute a dormant supplement or count technical caption wrappers as new language elements. Preserve the legacy synthetic helper entrypoint and all existing fixtures.

## Readiness: default Figure frame geometry — 2026-10-29

Measurement before decision: readiness-fixes-2026-10-28/figure-geometry-before/
receipt.json binds immutable physical10-release/source archive, dirty HEAD212aed0449,
UTC/diffstat and current oracle64c5e8c0. Rect/Block body captions are20.438pt too
low; Empty body adds14.388pt; Text body is already vertically correct. Leading2em
changes only the product's gap, while spacing2em changes the following paragraph
in the oracle. Native Figure default is centered, uses weak0.65em gap and an
ordinary block (pinned library/model/figure.rs:302–304,435–440 and layout/rules.rs:
311–334). Root Figure Location stays20pt/20pt independently of centered ink.
These are language geometry/positions; generated frames and buffering are mechanics.

Continuous ADR-0127 internal parity correction; no public field/signature, mode,
phase transfer, new callbacks or API. Keep prefix/counter validation before any
flow mutation. Replace the forced caption linebreak with one bounded composition
of actual body and rich caption/prefix, with0.65em between nonempty logical frames
and no spurious gap for Empty body/caption. The gap is independent of par.leading;
existing outer paragraph spacing still follows the completed Figure.

Compose each actual child once in the current layout phase. Preserve exact
FigureBody/FigureSelectedSupplement/FigureCaption source edges, emitted prefix
order and rich supplement; generated separators/envelopes have no source member.
Use actual logical dimensions/baselines, preserving zero/unknown distinctions,
not flattened text/paint bounds, guessed heights, source rescanning or duplicate
callback layout. Settle default horizontal centering against the intrinsic common
frame width while preserving explicit inner Align overrides, rich/multiline paint,
decorations/links and source positions. Keep atomic block/region behavior and
parent state restoration through existing frame admission/commit owners.

This work is initially incomplete until both named temporary fronts
figure-caption-vertical and figure-intrinsic-alignment pass. It does not authorize
complete caption synthesis, Figure gap/placement public fields, function numbering,
implicit HTML, global set-align or generic Block show semantics. Those debts and
all old negative cases remain explicit. Native-source measurement must precede
any additional claimed subset.

Before implementation add declared-metric functional REDs for Rect/Empty/Text
body, changed leading, default centering and explicit left body under a wider
caption. Independently known10pt text/cap7pt inputs fix the caption baselines,
not observations from the implementation. Require full body/caption/tail content
and no overlap. Revalidate the original12 source fixtures and figure-caption/
figures-rgb, actual source/query locations, styles/supplements/counters/headers,
PDF/SVG at original0.001pt/independent paint, prior qualified physical12 corpora
and resource/JPEG gates. Do not close unrelated root-right TAIL or other debt
by changing fixtures/observers/tolerances. Update each genuinely affected owner
before its code, maintain valid Nucleus/ownership, seal and run all12 gates.

The first figure-frame-red attempt is a compilation failure, not a functional
RED: the test imported ShapeKind through layout_types, where it is private.
Import the same public geometry::ShapeKind directly. Keep every fixture, metric,
expectation and bound unchanged, and rerun the actual test before implementation.

figure-frame-red2 executed all five tests, but its textual preservation control
exposes a fixture setup defect: emitted Text size is11pt (cap7.7), not the
declared10pt (cap7). Layouter's lexical default chain must explicitly contain
Size(10pt), followed by the normal TextStyle::from(chain) synchronization.
Correct only that setup; keep all known10pt inputs/expected baselines/widths and
1e-8 bounds. No productive geometry may be changed using this invalid setup.

figure-frame-red3 is the valid declared10pt RED: textual preservation passes;
Rect/Empty and the two horizontal geometry cases fail. Begin one-pass private
body/caption frames, preserve tri-state baseline and actual flow presence, and
settle their local alignment paths only after the common intrinsic width is
known. A root explicit Align declaration through transparent Source/Located/
Realized/Styled wrappers overrides the default piece centering; use typed
content fields, with exhaustive alternatives, not names/text or paint inference.
The full12 native cases and richer inline alignment remain required after unit
GREEN; module success alone does not complete the two named fronts.

Physical transport detail: block::commit_prepared_frame may start a new page and
replace origin.x with the new line start. Preserve intrinsic flow width by
committing at the causal line origin; place the visual centering inside a
technical child Group. Decorations receive the same displacement after the
actual committed origin is known. Auto-width parent settlement remains a
required measured control, not certified by the five fixed-region unit tests.

figure-frame-all1:231 pass, one existing captionless synthetic boundary test
fails because it visits text coordinates directly after new technical groups
were introduced (ALPHA/BETA both local7.7). Preserve the previous captionless
synthetic entrypoint's direct planned body between paragraph boundaries; these
fixtures were explicitly retained. The new logical composition is restricted
to a present caption. Productive realized captionless Figures remain ordinary
Blocks. Do not change this existing observer/fixture/assertion to pass the gate.

### Refutation of fixed-region-only horizontal composition

figure-frame-build1 source/receipt and immutable release bind dirty HEAD212aed
and UTC2026-10-29T06:11:35+00:00 onward. figure-geometry-after1 and extra-after1
preserve24 current-pin cases and independent boxes/paint. Rect/Empty/Text
caption Y and following spacing now agree, but caption X is0.968pt left;
Block BODY remains26.074pt left. Multiline caption/body short lines lose
35.6125/25.234pt centering. Caption-right loses text outside the visible page.
Auto-parent regresses to75.033pt right: the visual page-width displacement
entered a parent whose causal width is only the Figure. Root-right/global
Set-align and captionless centering remain separate old debt. Underlined
Figure changes extracted order (TAIL before caption), requiring causal geometry
investigation before acceptance; preserving a word multiset is insufficient.

Before further implementation add declared10pt REDs for unequal-width body
lines (9-character first line,1-character second), unequal-width caption lines
(7 and1 characters), actual right-aligned caption (three characters remain
visible/centered as its intrinsic frame), and an actual auto Block around
Figure (shape left equals following TAIL left; caption centered in80pt).
Keep original five and all232 existing tests unchanged. Default Center must
apply lexically to actual text lines and fixed child frames, with restoration.
Use one-pass deferred logical line alignment and its existing optical-margin
formula, never add an empirical0.968pt offset. Preserve explicit Align and
per-source dispatch. Capture Figure's line-alignment facts privately until
its own intrinsic dimensions are known; Repeat settlement stays unchanged.
Do not center against a provisional parent Auto region; final automatic
parent width and decoration integration need their actual owner controls.

figure-alignment-red1 is functional RED: all four new independent10pt cases
fail, all five original controls pass. Install/restores default Center while
composing actual body and caption. Settle each piece's captured line geometry
at its own intrinsic width before positioning in the common Figure width,
so root explicit inner alignment is not applied twice. For an automatic
parent, commit with zero provisional centering and register the actual Group
with the existing horizontal deferred queue against the committed causal
origin. A containing Block settles it at its actual final width. Root finite
regions keep their known centering inside the technical Group. External
decoration transport and underlined flow remain explicit validation blockers.

figure-alignment-green1 passes all9 fixed-metric controls with unchanged
inputs. This does not certify child fixed frames or external decoration.
Add two actual Figure controls before corrections: an80×35 fixed Block with
BODY uses its own Center (24pt text starts common center−12), and Underline
wrapping the completed Figure must preserve CAP before following TAIL with
the same baseline and paragraph separation. The24-source native evidence
retains the earlier fixed-body mismatch and underlined order regression.
Keep these as causal controls; do not patch observers or sorted-word gates.

figure-optical-red1 preserves48 passing readiness controls and exactly two
injected optical failures. The fixed Block BODY and direct Underline/Tail
controls pass: they do not prove painted Block allocation or markup whitespace.
Preserve them and add separate actual painted80×35 Block (its backing starts
center−40) and Underline followed by SpaceFromNewline before TAIL controls.
These require the same source geometry; do not infer success from BODY ink or
from an input lacking the original source newline. No Block/decor algorithm
change is authorized from a speculative failure; measure these first.

## Remaining source refuters after release2

figure-frame-build2 source/receipt and release2 bind exact dirty HEAD212aed,
UTC and inputs.52 readiness and8099 core tests pass (five old ignored). Actual
PDF/SVG paints match exactly for Rect/Text/Empty/Block, changed leading/spacing,
body-left/rich/location, multiline, explicit painted Block, most parents and
page move. This does not close all Figure geometry. Caption-right/empty-caption
prefix is1.7644pt right; fixed-block-left BODY is116.074pt right; source Underline
still places TAIL with the caption. Existing direct and newline-only controls
pass because they dispatch separately and do not exercise Sequence's material
line bookkeeping. Preserve these controls; add one joined Sequence variation
with Underline(Figure), SpaceFromNewline and TAIL. Add explicit root Align Left
around a painted80×35 Block under114pt caption: BODY and backing both start
at common center−57pt. Do not change old expectations. VSpace body and old
captionless/global Set-align/query failures remain explicit separate debt.

figure-boundary-green1 passes55/56, including all four former failures and
restored TAIL; only the new stroke assertion fails. Its setup wrongly assumed
FixedMetrics underline1pt/0.5pt at10pt. metrics.rs:135–137,628–635 explicitly
defines the existing default as2pt/0.6pt; observed output121.3661417/0.6 is
correct for that provider, not an algorithmic failure. Correct this new fixture
by injecting a local provider that delegates every FixedMetrics measurement
but explicitly supplies the declared0.1em/0.05em underline. Keep all expected
positions/thickness, input10pt size and bounds, the true sequence-flow RED and
the recorded failed run. Do not modify productive metrics or fit expected values.

physical13-validation1 is NOT qualified: six gates pass, domain/infra finds
two actual deferred-note page regressions, preserved in its immutable logs.
Do not change either original source/expectation. Audit whether actual FIRST
paint moved or only its physical binding before selecting a correction; exact
source/current full-code snapshot is archived with the failed gate. No new
API or phase proposal is approved by these diagnostics.

### Deferred-note binding regression: measured neutral carrier frontier

figure-note-diagnostic1 reproduces both unchanged integration failures with
exact inputs/UTC/diffstat. Visible pages contain FIRST/C1 on page1, while
ANCHOR/SECOND/NC are on page2; only FIRST's physical binding incorrectly follows
ANCHOR. At frame admission an existing neutral position-carrier-only current
line is not drained because admit_frame_start flushes only causal line/cursor
advance. Intrinsic body/caption save/restore that parent line; commit appends
the painted Group directly. The pending root carrier then migrates with ANCHOR.
This hypothesis is refuted if draining that exact neutral frontier leaves either
original vector wrong or changes any prior physical source/paint control.

After ordinary Figure frame admission and before its actual child composition,
when the existing parent line is nonempty and consists solely of the canonical
neutral context/position/link markers, with no pending fractional/repeat work,
close it through the existing empty-line flush. This commits already admitted
carriers without new source actions, identity, typography, line advance or
parent paint. Preserve current coordinates and ordinary admission/page policy;
this does not claim to fix the earlier moved-Figure position debt. Keep all
other lines/queues on the unchanged route. Require the two original integration
tests to turn GREEN with every expectation intact, plus all previous controls
and fresh full-source validation. No public or phase/order change is introduced.

figure-note-green1 confirms both exact original integrated note tests now pass,
with their unchanged programs, page/state vectors, visible text and warnings.
The neutral frontier repair is bounded to this existing rendering path; retain
page-move negative controls and run the complete physical-page/core/integrated
suites and original binary comparisons before any qualification claim.

## Rustfmt gate repair (2026-10-29)

The frozen `physical13-validation2/fmt.log` under
`lab/results/construction/diagnostics/readiness-fixes-2026-10-28/`
records a genuine formatting RED. Apply Rust 1.92.0 rustfmt to this consumer
without changing executable tokens, fixtures, expectations or assertions.
Preserve all behavioral obligations above; reseal both directions and rerun
the unchanged gates against the final frozen source. Internal ADR-0127 flow.

## Default caption-gap citation

The full warning-denying SARIF gate in
`lab/results/construction/diagnostics/readiness-fixes-2026-10-28/comparison-physical13/full-lint-suite.log`
records V21 for the unchanged gap formula. Pinned upstream commit
`64c5e8c02fe0394d913ead36e42ef8c1334d2107`,
`typst-library/src/model/figure.rs:302–304`, declares default Em(0.65).
Place the source citation immediately before the gap binding, where V21 reads
its provenance; change no executable token, behavior, fixture or assertion. Reseal and run the full linter
with warnings denied before further build/validation. Internal ADR-0127 flow.
