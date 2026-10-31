# L0 — region progress
Hash do Código: ec911e08

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/block-place-region.toml sha256:ad1c3b199f21c8a84384ff679c5c79a9b9e7cbbcc8d49e2d3f30a288fa210c47
- 00_nucleo/prompts/_nuclei/layout/block-inline-region.toml sha256:029c2929c9e294efceb796a5358a6f425f02ae91c519314080f413374f4f4403
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:3921c80230cfcc66a807429fbf8bc07ee090bae4988c351b6524023038bab84b
- 00_nucleo/prompts/_nuclei/layout/horizontal-space-paragraph.toml sha256:bbcb7c79c93e3d2da4580dfbc5586174a9698f02e3d3579ea886a1ff2608ed4b
- 00_nucleo/prompts/_nuclei/layout/installed-line-ascent.toml sha256:993c94c8c186448b35b1ba8a0d84c1aadd95e9db0a09b19394fe95b08db52c7e
- 00_nucleo/prompts/_nuclei/layout/paragraph-hanging-indent.toml sha256:12bf17ff10c443c56512c508693160e37e80f6ee925c506912718231b39ced12
- 00_nucleo/prompts/_nuclei/layout/subframe-overflow.toml sha256:06c0f9a3e9b81ea332d57174938fda46c0865535787945094cea27ee5afbd562
- 00_nucleo/prompts/_nuclei/place-flow-boundary.toml sha256:2e0595153eaf379004fd35fe6a3887a7d62532bec78700833e8b8569e672137e

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/region_progress.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

Historical separation evidence:cursor measures/selects word/chunk breaks at `cursor.rs:195–271`, emits highlighted text at `cursor.rs:149–193`, and transitions regions at `cursor.rs:314–417`. Metrics defines default width at `metrics.rs:249–262`; LineCommit requests post-line progress at `line_commit.rs:696–703`. The 63 controlled source→PDF observations and isolated pre-export measurements are archived in `lab/results/construction/diagnostics/auditoria-quebra-paginacao-2026-09-28/`. They expose limitations, but separation preserves those exact decisions. Initial code/L0/provenance are in `lab/results/construction/diagnostics/separacao-texto-linhas-regioes-2026-09-28/`.

Thirty bilateral controls in `lab/results/construction/diagnostics/estudo-correcao-falhas-texto-2026-09-28/` measure Tekt/vanilla terminal 2/1, explicit terminal break 3/2, continuation 3/2 pages. Isolated preexport emits Four/additional on page 2 baseline 101.84 with page height 100. line_commit.rs:657-696 predicts next baseline; region_progress.rs:113-128 transitions postline; cursor.rs:80-135/parbreak.rs:41-80 set baseline/spacing. `lab/results/construction/diagnostics/correcao-demanda-regional-2026-09-28/{initial.json,before/}` preserves original source/L0 hashes/state. First implementation's 36 real-font controls in first-after-pdf/first-pdf-green.log predate later rebuild; terminal-columns loses Three, terminal-footnote changes 2→1 pages with main/note overlap. Column startup must clear old block state, and terminal note fitting must keep conservative next-line clearance despite trimmed final geometry. No new footnote format/reservation/public contract is inferred.

containers-clip-2026-10-13/before-renders.json shows oversized Block Shape committing two blank pages versus one clipped region. shape.rs:119/cursor.rs:125/line_commit.rs:438/region_progress.rs:115,131 permit external overflow inside is_sub_frame, while sub_frame.rs:109-121,317-340 cannot restore a committed page. `lab/results/construction/diagnostics/subframe-isolamento-2026-10-13/before.json.gz` preserves source/L0/HEAD/dirty state/status/diffstat/UTC; test conditions before changes, without raising heights/dropping items. Helper repair alone does not complete Block/Box collection/dimensions/clip.

`lab/results/construction/diagnostics/place-fluxo-2026-10-14/before/results.json.gz`records the same HEAD, uncommitted tree/diffstat/UTC/sources/binaries/argv and historical pin. All 24 root/Box pairs after 12 prefixes differ: Linebreak/Parbreak require y22.25pt, Shape y27pt, Block/Grid/VSpace20pt y35pt, Tekt 15pt. Rotate also overlaps independently. `line_commit.rs:721–733` prepares next baseline; `shape.rs:196–206` adds below/cap; `block.rs:465` separates boundary/margin; `place.rs:91–99` recognizes still-pending line only. Pinned flow/distribute.rs:703–715 uses physical flow offset for axis-unspecified Place. Weak margins/overlap/new page/rollback/subframe leakage can refute the causal-end hypothesis.

`lab/results/construction/diagnostics/place-fluxo-2026-10-14/block-regions-before/results.json.gz` preserves same HEAD, dirty tree/diffstat/UTC/sources/binaries. Only 2 of 24 bilateral bboxes match against pin `9f2b6e8715237cb086899a42873660fe744622e8`: inset3/top Tekt(20,15) versus vanilla(20,18); bottom/right(71,37) versus(62,44); clip deletes Tekt marker. `block.rs:90` ignores inset.right;`:220` installs width as body;`:275` uses Place's zero advance as clip height;`:290` groups absolute children without rebase. `placement.rs:505` uses page/subframe without Block region. Pinned `typst-layout/src/flow/block.rs:66-89,223-234` forces inner size, adds inset then clips;`:278-280` subtracts inset from region. Geometry is language/buffers mechanics. Nesting/text/auto/clipping/sibling leakage refute local-region correction.

`lab/results/construction/diagnostics/bibliography-structure-2026-10-18/{pagination-block,block-freeze}.json.gz` preserves state/UTC/HEAD/sources/binaries/argv/boxes for candidate `c1c222ae54ab3b67487471326012a7f6c4ed86c1e6bd61b1830ec6a987c2ccea` against oracle `bd5ebec285505889e8ff1d4f68804ab147a699f3` (binary `9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`). It regains two IEEE200×90pt pages but loses continuation inset; corrected 70pt fixture reveals prefix orphaned before body admission. `region_progress.rs:75–76` resets margin and `paragraph_pagination.rs:69,87–105` can legitimately group three lines for widow/orphan. Prefix must accompany actual first line, not an invalid FIRSTWORD-first-page requirement or forced greedy/disabled protection. First prefix paint remains before body paint after admission, never changed eval/show realization order. Orphan prefix/lost inset/double emission/new effect/changed strategy/Block-column-subframe regression refutes correction.

`lab/results/construction/diagnostics/bibliography-hanging-2026-10-18/before-sources.json.gz` preserves HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, uncommitted tree, status/diffstat/UTC and 2,099 sources/L0s. before.json.gz contains 92 pairs; extra-before.json.gz 26; boundary-before.json.gz 10; repeat-before.json.gz 4. Oracle commit: `bd5ebec285505889e8ff1d4f68804ab147a699f3`; binary SHA-256: `9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`; initial Tekt: `2cc69a555e9738dc659c06ed6d933853ab981cbb73e93c388a5b43c55afad095`. At width 160pt/margin 8/font12, hanging10 leaves first-line x8 and moves continuations x8→18; negative-5 requires x3. RTL leaves first right edge 152 and moves continuations to 142. Tekt ignores the length. MLA defaults to 18pt (1.5em); ambient set 3em does not replace it, while show-set3em requires 36pt; IEEE/Plain do not acquire hanging indent through that show-set. bibliography.where(style:"mla") does not match in the vanilla probe: preserve selector choice rather than forcing override.

Inline-only Block/Box bodies ignore hanging10; explicit Par inside them and Block with Parbreak apply it. Simple Heading/Raw/List/Quote do not. Partial, zero, repeated and empty-scope declarations close boundaries: FIRST/MIDDLE/LAST at 5.168/27.464/49.76 versus all 5.168 in Tekt. The first lexical probe used literal brackets; it was refuted, corrected to #[...] and retained as extra-before-initial, not evidence of Styled scope. Extreme values produce extractor-clipped paint; missing pdftotext output does not mean Content deletion. A partial 24pt font does not change the shared length's 12pt em, as partial-font records.

Pinned source: `typst-realize/src/lib.rs:1061–1085,1190–1219` distinguishes fragment/Par and interruption; `typst-layout/src/flow/collect.rs:72–134` selects flow; `inline/mod.rs:174,216–221` and `collect.rs:141–147` resolve shape; `inline/line.rs:496–503` positions; `rules.rs:510–523` provides CSL inset/compensation. `typst-library/src/model/bibliography.rs:112–127,367` determines default/precedence. This measures language/geometry, requiring neither vanilla Rust structure nor inferred historical intent. Lost boundaries, run-relative em, double indent, page reset, imposed greedy fallback, lost copies, reexecuted callbacks or control regressions refute closure.

mixed-style-metrics-2026-10-18first-after/first-bib-after/first-regions-after/candidate1-sources preserve first candidate changing only line_commit with seven passing tests. audit-candidate1 finds matching SVG heading y in six bibliography cases and italic→roman, but forced still differs 0.156. linebreak.rs:46–58 appends ExplicitLinebreakBoundary with empty exterior-style Text: causal for bidi, not material face. Twenty regional PDF controls preserve same binaries/pin/sources as before-sources: at 39.44pt Tekt sends ITALIC to page 2 versus vanilla page 1;55.14pt forced 2 versus 1 pages;70pt wrap 3+1 versus 2+2 lines. Initial fix corrects wrapped y but not regional decisions. Invalid multipage SVG-without-template attempt stays in regions-before as a failed command, not SVG parity proof; use valid PDF and future regional script requests PDF only.

region_progress.rs:83–88 installs one-face baseline without updating assumed ascent;`:143–152` admits forecast before actual text. paragraph_pagination.rs:43–59 rejects differing edges and disables positive-cost grouping despite known selected ranges. Pinned flow/collect.rs:196–239 protects first/last actual-height frames; distribute.rs:343–364 advances group only if it fits next region. Language geometry/words/pages, not Profile shape/SVG IDs. Premature jump/recount/new callbacks/Group-based metrics/width-selection-leading changes or column/note/float/NoTypography regression refute correction.

Candidate links ensure_line_baseline in paragraph_pagination to cursor's initial/block top computations: KnownTop 6 could still test exterior 7 and evict prepared text prematurely. Pinned inline/line.rs:559–568 uses actual maximum frame. Prechange RED after real material boundary with previous baseline 7/below 6.5/page 20/italic top 6 requires baseline 19.5 on current page, zero widow/orphan isolating admission. This later refinement supersedes ordinary-continuation-only discrimination only for certified KnownTop in initial/block branches.

## Exclusive ownership and current contract

Layer L1. Sole consumer: `01_core/src/compiler/layout/region_progress.rs`. Own next_region implementation and post_line fallback. Preserve flow-suffix acceptance before any mutation; column close/next/merge order; Auto dimensions before admissible float/footnote drains; page_commit; next physical-page margin/binding resolution; next cursor/baseline/reservation/block reset; pending cell tails; first-column startup. Delegate fitting to Place and drains/geometry/assembly to their owners; no future Content scan/I/O/new checkpoint/dynamic dispatch.

After resolving current initial/block baseline, admit next material text line before emitting when finite predicted baseline exceeds effective bottom, pending line lacks material and current region has material. Never repeatedly advance empty region or acquire metrics for Generic predicate. post_line remains only uncertified overflow rejection/transition fallback; a LineCommit-certified fitting line never advances just because hypothetical next baseline overflows. Preserve paragraph/fractional owner collapse and all drains/reset timing.

Private Copy Admission distinguishes Generic/Text/KnownTop. Original outer guards still protect subframe/infinite/already-material-line/empty-region. Only after prediction exceeds bottom compute candidate: Generic uses prediction without font query; Text uses resolved current-face top; KnownTop uses supplied certificate. Text/KnownTop candidate is cursor_y+actual_top-assumed_ascent, with absent assumed ascent neutralizing difference and Some(0) valid. Advance only if candidate too exceeds bottom; do not mutate cursor/items merely to test fitting. Cursor owns initial/block protocol and must use KnownTop in those branches as later specified, not duplicate them here.

On installing a physical-page baseline save Some(top) from face actually used; if baseline remains pending save None without font query. Columns keep own startup protocol; no new persistent Layouter state.

After start_next_column clear last_flush_advance/prev_block_equation_descent/prev_block_below_pending/block_chain_active exactly like physical-page transition through shared private reset_closed_region_flow. Also clear flow_placement_end and invalidate block_place_region; never restore prior-fragment region. Clear private occurrence decoration ranges associated with Place auto-Y fixup at region transition, preserving hub/checkpoint/subframe/Block ownership of storage/restore and placement-only occurrence registration.

post_line/admit_next_text_line neither transition nor reject exterior transaction during is_sub_frame. Explicit next_region/page/column protocol and float drains stay unchanged; local oversized items remain local for owning container clip/measure.

After next same-page column, or completed physical page/first-column initialization, reinstall Some block_inline_region through Block's helper after natural region geometry and before next emitted line. None retains old flow; no change to drains/reset/float/suffix acceptance. Then reopen/reapply paragraph-indent horizontal origin against actual new region, retaining initial/continued logical line count; no extra page, duplicate inset or prior-width copy.

## Acceptance and limits

Preserve controlled corpus/current suites' text/positions/styles/pages/errors and current static signatures. Internal ADR-0127 parity corrections require L0 before RED→GREEN/restricted reseal, no new public/default/phase. Structure/prefix arithmetic is mechanics; regional word/line/page geometry is language, not PDF bytes or copied vanilla layout.

Regional-demand controls preserve terminal/explicit-break/continuation pages, fitting-line no-transition, first-line progress, no empty-region loop and preexport bounds. Recheck columns Three and footnote conservative-clearance/nonoverlap with unchanged format/reservation. Subframe tests first fail then pass with ordinary exterior pagination/core/build/fmt/lineage; never drop items or raise test capacity.

Necessary private causal state is authorized only for measured contracts, with LIFO save/restore. Certified line/shape/Block/Align/Grid/Raw/VSpace anchors do not complete Block/inset/fragmentation/late inline Box-Transform or other Heading/List/Stack/Equation/Columns. Invalid witness keeps legacy fallback/pending status; no broad Place or tiling parity. Revalidate full suite/HTML-context/PDF-SVG/build-doc-fmt-check/lineage/warning inventory.

Prefix controls require RED→GREEN same actual prefix/body page, continuation inset and OUTSIDE, Simple/Optimized and prior controls. Measure PDF90/150/250, RTL, prefix-only/formatted/color/regressions; build/suites/doctests/Clippy/Rustdoc/fmt/strict-reverse lineage/differential Tekt under MemoryMax12GiB/High8GiB/swap0/jobs1, without publication. Generic hanging/overrides and multiple bibliographies retain their own open scopes.

This internal parity correction follows ADR-0127 continuous flow: no new public field/signature, external flag/default, phase or realization order. Initial/continuation shape supersedes only the corresponding constant-width limitation. Certified Latin RTL remains subject to the nucleus conditions, without active BidiCursor/tracking or invented BidiSpan; every other prior limit remains. Off-page values/copies require frame/SVG inspection rather than equality of clipped extraction. Do not anticipate the separate R3-T1 scope.

Require RED→GREEN before implementation and source→layout/bilateral controls for relevant signs, boundaries, variants and restoration. Executing a grouped arm does not prove every alternative: closed nominal inline/flow-context classifications are intentional and need class/variant tests. Reseal only after V15/V26 are valid; run tests, build, Clippy/Rustdoc/fmt, reverse lineage and differential Tekt against pin1279707, under 12GiB/High8GiB/swap0/jobs1. No publication.

Mixed-face refinement is private continuous-flow ADR-0127 without public/product-flag/default/phase/callback-order change, preserving earlier refused/scalar paths. Regional certification expands only same-size/leading prepared text, zero baseline_offset, finite nonnegative edges, faces with different tops/descents; variable sizes/leading remain outside it. Require RED→GREEN four added regressions/prior controls,30+20 regional PDFs,132+94+21 corpus, broad suite/Clippy/Rustdoc/lineage under 12GiB/High8GiB/swap0/jobs1, recording failed attempts. No full variable-metrics parity claim.

Current-API mixed-face tests include smaller-top next line fitting despite forecast and first baseline after transition with a different face. Keep KnownTop actual-block-boundary RED unchanged; variable size/leading and opaque/future content remain excluded.


## Private deferred Note frame transport

### Measurement before correction

`lab/results/set-page-deferred-corrections-2026-10-21/red-2/tests.log` records
the actual missing Note-subtree admissions. The later root-controlled fitting
measurement and pinned oracle page-text evidence distinguish a complete Note frame
from its accepted prefix and continuing suffix. The oracle keeps NOTE TEXT/START
in the first page and emits the later Figure in the second; whole-body deferral
or moving the complete call does not establish that language behavior. Exact
before bytes/hashes/HEAD/dirty state for this owner are archived in
`lab/results/set-page-deferred-corrections-2026-10-21/agents/index/note-transport-before.json.gz`.
The root authorizes an internal layout parity correction, with no eval/layout
phase change, public contract, new identity source or attempt budget.

### Exclusive transport obligation

Reset the closed region's private footnote_reserved_height only after its
Note flush and page commit, together with existing float reservations. Preserve
actual deferred Note frames/cursors for the next region; never discard or
rebuild the continuing body merely to reset its old regional reservation. A
feature-owned hook may establish the next region's reservation from that exact
retained frame, without a Content re-evaluation or geometry-derived identity.

Validate actual split Note text/page/counter/query/identity and no-fit progress,
plus existing no-break notes, columns, subframes, rollback, floats and running
content controls. Preserve strict trace coverage/foreign authority, warnings,
canonical owner paths and forward/reverse lineage; no suppression or fixture
weakening. These transport declarations alone do not certify general pagination.

After the next physical/column region geometry is installed and its old reserve
is reset, call the feature-owned refresh_footnote_reservation hook for its exact
retained frame remainder. Do so after start_next_column as well as after a new
physical page, without feature geometry formulas in this coordinator.


## Weak pagebreak parity correction — 2026-10-23

### Evidence before decision

`lab/results/weak-pagebreak-2026-10-23/baseline.json` binds the uncommitted source/L0/config state to HEAD, dirty stat and UTC; `before/receipt.json` binds both verified executables, inputs, commands and observations. `pagebreak.rs:21-58` ignores weak and always requests transitions; `finish.rs:103-106` requires the newly opened page unless it is a PageRun boundary. `page_run.rs:126-131` unconditionally closes its body. The pinned source `lab/typst-original/crates/typst-library/src/layout/page.rs:554-557` explicitly defines skipping weak breaks on empty pages; its tests `tests/suite/layout/pagebreak.typ:15-24,61-76` document weak/material distinctions. Initial, repeated, terminal and strong/weak controls plus parity and explicit PageRun probes distinguish language intent from mechanics. Page membership, content and geometry are observables; the private successor flag is a chosen mechanism. The inference that an optional successor suffices is refuted by lost notes/placed content, changed strong/parity empties, or leaked rollback state.

### Decision and exclusive responsibility

Clear weak-successor state on an ordinary regional transition; the explicit weak-break owner may mark the newly opened successor optional after its requested transitions. Preserve overflow, columns, deferred tails, notes and floats.

This is an ADR-0127 continuous-flow language-parity correction: L0 first, reseal, RED→GREEN and complete revalidation. No public contract, default mode, pipeline phase or compatibility change is authorized. Existing nucleus obligations and unrelated limits remain binding.


## Refutation: unpainted material and required successors

### Evidence before decision

`lab/results/weak-pagebreak-2026-10-23/edge-after/receipt.json` and `unpainted-before/receipt.json` bind the first candidate executable to UTC/inputs/commands; its build and source state are in `validation.json` and `final-source.json.gz`. These controls refute painting alone as page materiality: hidden text, zero/fractional horizontal spacing and zero/weak/strong vertical spacing can require a page without paint; empty/metadata-only Hide does not. `h_space.rs:27-45` advances or queues spacing without an item, `v_space.rs:20-42` records advancement, and `hide.rs:20-28` removes only painted output. `pagebreak.rs:42-44` also overwrites an empty strong successor's requiredness during weak parity adjustment, dropping a required final page. These are language-visible page obligations, not a request to copy upstream collection mechanics. The earlier painting-only hypothesis was rejected, not qualified by changing the oracle or deleting its cases.

### Exclusive correction

Reset unpainted page material only after the physical page is committed; a column transition within that page preserves the witness.

L0 precedes code under continuous-flow ADR-0127. No public field, signature, trait, phase, default or compatibility change. Validate the refuters and retain all previous note/float/Context/column/strong/PageRun controls.

## Image flow and horizontal paragraph correction — 2026-10-27

Measurement precedes this correction: `lab/results/construction/diagnostics/image-flow-cause-2026-10-27/{baseline.json,results.json,refuters.json,completion.json}` binds HEAD212aed044, uncommitted inputs/diffstat/UTC, pinned64c5e8c0/tree483bd6f1/SHA22897f93 and received Tekt SHA2941a5ba. Direct PDF streams show image1 at y5 and images2–16 at y13.2; vanilla has eight30x20pt images per page at step46.4. Removing h gives vanilla step33.2 but retains Tekt overlap. Margin30 still resets later images to13.2; a60x40pt image does too. Source `image.rs:198,325–335` advances cursor without publishing the physical end consumed by `cursor.rs:128–169`. Vanilla `rules.rs:771`, realization `lib.rs:1061–1087` and flow `collect.rs:162–188,240–285` distinguish image blocks and h-only paragraph gaps. These are language/geometry observations, not a requirement to copy buffers, PDF bytes or upstream execution structure. No upstream intent is inferred.

Classification: private parity repair, continuous flow ADR-0127. No public fields/signatures/trait methods, new product mode, phase change, dependency or compatibility break. Keep all received footnote/R11–R13/JPEG work. New shared obligations are pinned in the horizontal-space-paragraph Nucleus; this owner authorizes only its exclusive consumer.

### Exclusive correction

Clear a consumed main-flow horizontal-paragraph cause with existing closed-region line/block reset; never carry pending previous-page paragraph spacing to the next physical region. Do not discard child source events or change ordinary admission/footnote reservation.

Require targeted RED→GREEN, bilateral PDF/SVG<=0.001pt, source order/complete image counts, all source-margin/size/spacing refuters, prior R8–R13/notes and A–H/R1–R7, full build/tests/Clippy/fmt/Rustdoc and full lint without warning/error. Preserve known tracking/paint limits; remeasure JPEG latency/RSS against received/legacy/oracle and retain R6 resource bounds. Snapshot provenance and preserve unrelated inputs/historical receipts. Validate V15/V26 before restricted resealing; no commit/push.

### Measured lexical horizontal scope refinement

The candidate2 frozen state and pilot/geometry.json in image-flow-fix-2026-10-27 show mixed-h-scopes and siblings-h-scopes differ by43.2pt at the third image, while sole-h-scope and wrapper-h-scopes match. Equal styled siblings do not establish a shared lexical paragraph wrapper. Resolve h-only paragraph spacing from the nearest actual common StyleChain ancestor of all producers. Preserve the original anchor and monotonically narrow to its ancestor; do not infer lexical scope from equal values. Existing raw_deltas supplies identity without a public contract change. The private pending state stores an Arc-backed source chain and ancestor depth; normal one-producer closure resolves directly, mixed scope may reconstruct only the ancestor suffix for resolution. Checkpoint clones, subframes save/clear/restore, region reset and real-line absorption apply to this complete state. No public API, source reevaluation, paint, dependency or phase change. Require RED→GREEN for mixed outer/sibling scope and sole/wrapped controls, then rerun full validation.
