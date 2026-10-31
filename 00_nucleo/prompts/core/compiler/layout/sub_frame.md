# L0 — sub frame
Hash do Código: 699a2faa

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/block-place-region.toml sha256:ad1c3b199f21c8a84384ff679c5c79a9b9e7cbbcc8d49e2d3f30a288fa210c47
- 00_nucleo/prompts/_nuclei/layout/bibliography-prefix-admission.toml sha256:b556a9f22e29d478ae1afac3e31427aeea3fa58f44a4aa04e46c86b90fb5779c
- 00_nucleo/prompts/_nuclei/layout/block-auto-alignment.toml sha256:4a93d5c55bf975b506b6f02ae484a91fdf7449716f9248dea2fcaa6b7b0738b6
- 00_nucleo/prompts/_nuclei/layout/block-inline-region.toml sha256:029c2929c9e294efceb796a5358a6f425f02ae91c519314080f413374f4f4403
- 00_nucleo/prompts/_nuclei/layout/cell-bidi-scope.toml sha256:5c733e4756935ef5e4841991852a5f60f0f00e2b76c988c6af340b52cc0ce977
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:3921c80230cfcc66a807429fbf8bc07ee090bae4988c351b6524023038bab84b
- 00_nucleo/prompts/_nuclei/layout/flow-frame-width.toml sha256:0684f5fcab7844d2ac81dd302d9afc7c12595cf5651a9f4c7c91daea8166b3f7
- 00_nucleo/prompts/_nuclei/layout/horizontal-space-paragraph.toml sha256:bbcb7c79c93e3d2da4580dfbc5586174a9698f02e3d3579ea886a1ff2608ed4b
- 00_nucleo/prompts/_nuclei/layout/installed-line-ascent.toml sha256:993c94c8c186448b35b1ba8a0d84c1aadd95e9db0a09b19394fe95b08db52c7e
- 00_nucleo/prompts/_nuclei/layout/intrinsic-piece-alignment.toml sha256:8af39497a64b3dba29c94647c57301b023766aaa012268687421cf1fe6e4ecfb
- 00_nucleo/prompts/_nuclei/layout/link-frame-scopes.toml sha256:91cb1c097fc694c138acf2d044a3fc46dc7c41ec608ac557f086e1dc41ec0f57
- 00_nucleo/prompts/_nuclei/layout/nonexpanding-math-frame.toml sha256:8e6bd0ef718cc6d7e11292996777437b7f0fdb88c3c39d9d441c721459758563
- 00_nucleo/prompts/_nuclei/layout/optical-overhang.toml sha256:8c4e3b767247e54f966c3a86be1b528b030dc4c8d05ea9b358533351720a3cf2
- 00_nucleo/prompts/_nuclei/layout/paragraph-hanging-indent.toml sha256:12bf17ff10c443c56512c508693160e37e80f6ee925c506912718231b39ced12
- 00_nucleo/prompts/_nuclei/layout/physical-page-occurrences.toml sha256:b33f2e3c6238da01aa48783aecee6a8b6153b4f36ddcdc23edfe78434bae5b42
- 00_nucleo/prompts/_nuclei/layout/raw-block-boundary.toml sha256:a3772778f9e542de87823509c8eac57904fc73fea9526cebe6052833a69ef806
- 00_nucleo/prompts/_nuclei/layout/stack-frame-composition.toml sha256:f0c684c7c0dcaf29f7aaa29b8dbc260f15baba094fa4f6a605fc8cf16e81d8da
- 00_nucleo/prompts/_nuclei/layout/subframe-inline-extents.toml sha256:ec315f38498aa37ead497fdcb72cdd7e401cd00752baf5a535c2b5fae69374ed
- 00_nucleo/prompts/_nuclei/layout/subframe-overflow.toml sha256:06c0f9a3e9b81ea332d57174938fda46c0865535787945094cea27ee5afbd562
- 00_nucleo/prompts/_nuclei/layout/text-segment-scope.toml sha256:68203377765579bc29327c7d8fded6f4f831d11682b6b7c9abbf2dbf65b28bad
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:a4a76bdb3fa5069232468af724106217bcd1dc85231c9d2cf54b4da69e241425
- 00_nucleo/prompts/_nuclei/place-coordinate-origin.toml sha256:b9012d07e2d22b7c3984963a6d4e90eedbb9b925280f4136f8db8aaa9459bbc8
- 00_nucleo/prompts/_nuclei/place-flow-boundary.toml sha256:2e0595153eaf379004fd35fe6a3887a7d62532bec78700833e8b8569e672137e

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/sub_frame.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

`01_core/src/compiler/layout/sub_frame.rs:29,65,329` was inspected. Current ownership is established by consumer inspection, not upstream intent or new bilateral certification. Language text/geometry/morphology/errors govern acceptance; state/algorithm shape is mechanics. Historical 9f2b6e871 measurements do not certify current `bd5ebec285505889e8ff1d4f68804ab147a699f3`. Complete chronological evidence and superseded wording remain in the corresponding archived owner under `lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/compiler/layout/`.

`lab/results/construction/diagnostics/correcao-falhas-2026-09-28/`, block-align-2026-10-06/baseline-verification.json.gz and green.log, place-origem-local-2026-10-14/ and place-fluxo-2026-10-14/ distinguish geometric versus baseline origin, tail alignment, body origin and flow boundary. Later Place/Align owner clauses supersede early root-only caller restrictions, without making every other caller geometric. Tracking/direction/logical/cell-bidi receipts precede private isolation changes. Raw block-spacing before/additional/refuters-second/edges/suites-third refute inner-Strong versus regional ascent and unconditional font lookup. `lab/results/construction/diagnostics/caminhos-residuais-ci-2026-10-13/before.json.gz` inventories unused line_leading_pt scan at 265.

`lab/results/construction/diagnostics/subframe-pendentes-2026-10-13/before.json.gz` inspects pending-buffer indices and missing local tail expansion; first-candidate.json.gz and repeat-controls.json separate parent noninterference from unsupported Repeat 14-versus 13 repair and upstream block-flow difference. Box-composition before/radius-legacy-before and workspace.log expose inherited extents/fictitious strut and HTML P1493 vertical_metrics versus text_edges mismatch; installed-ascent probes distinguish None from Some(0) and smaller actual text from exterior style. Local-overflow receipts require returning all child content without outer pagination.

`lab/results/construction/diagnostics/bidi-contexto-2026-10-05/{probe.json.gz,initial.json.gz,inventory.json}` refutes isolated 456 789 resolution and preserves source/state/binaries. Human ratification of immutable API/three optional fields is manifest 1ae7513e.

`lab/results/construction/diagnostics/links-paginacao-2026-10-14.md` and receipts under `posicoes-compostas-2026-10-14/automatic-boundary-probes` record source, HEAD/dirty tree/UTC, binaries and vanilla pin. On a 160×40pt page with10pt margins, ordinary text crosses the break; Link URL, Ref and Ref Page abort with a stale index after buffers are emptied. `references.rs:270` appears in the stack; `link.rs:22-32` uses the same mechanism. `outline.rs:28-50` also saves indices before the body. Clamping an index proves neither prefix/order/fragment/destination preservation nor geometry. Earlier snapshot: `lab/results/construction/diagnostics/links-captura-2026-10-14/before.json.gz`. Content, destination, regions and bounds are observables; private boundaries are mechanics.

`block-auto-width-2026-10-14/before/results.json.gz` preserves HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree, UTC/diffstat/status/sources/binaries/argv and historical pin 9f2b6e871. The initial extractor sought blue as #0000ff; blue-geometries-corrected.json reads actual #0074d9 from the same SVGs without rerun. For Shape, Empty, Place-only, Grid and Stack, Tekt backing width is 160 versus pin54,14,14,68,68 respectively. `block.rs:222,416,453` uses available width; `line_commit.rs:753` loses cursor on closure; `sub_frame.rs:354` saves terminal cursor only. Pinned `flow/block.rs:145–166,281–290,339–363` distinguishes Auto from expansion. Geometry is language; witness structure is internal mechanics. Place contaminating width, leaking scopes, lost literal advances or changed explicit-container dimensions refute the hypothesis.

`lab/results/construction/diagnostics/transformacoes-2026-10-14/prefix-nesting-before/results.json.gz` preserves same HEAD/uncommitted state/status/stat/UTC/source/binary/historical pin. Multiword21 gains page/line leading, par 40/8-fill gains page, OUTER18/INNER8 shifts BODY/upshrinks auto page. fractional_preflight.rs:151-171 refuses multiline; word_layout.rs:49 closes in local style; line_commit.rs:398-480 uses it for floor/leading/predicted top; sequence.rs:239 refuses partial closure and missing prior chain uses spacing 8. Pinned flow/collect.rs:129-145,179-214 resolves shared trunk/spacing/leading, each frame contributing geometry. These are language text/geometry/pages without buffer/algorithm/byte or historical-intent obligation.

`lab/results/construction/diagnostics/bibliography-structure-2026-10-18/{pagination-block,block-freeze}.json.gz` preserves state/UTC/HEAD/sources/binaries/argv/boxes for candidate `c1c222ae54ab3b67487471326012a7f6c4ed86c1e6bd61b1830ec6a987c2ccea` against oracle `bd5ebec285505889e8ff1d4f68804ab147a699f3` (binary `9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`). It regains two IEEE200×90pt pages but loses continuation inset; corrected 70pt fixture reveals prefix orphaned before body admission. `region_progress.rs:75–76` resets margin and `paragraph_pagination.rs:69,87–105` can legitimately group three lines for widow/orphan. Prefix must accompany actual first line, not an invalid FIRSTWORD-first-page requirement or forced greedy/disabled protection. First prefix paint remains before body paint after admission, never changed eval/show realization order. Orphan prefix/lost inset/double emission/new effect/changed strategy/Block-column-subframe regression refutes correction.

`lab/results/construction/diagnostics/bibliography-hanging-2026-10-18/before-sources.json.gz` preserves HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, uncommitted tree, status/diffstat/UTC and 2,099 sources/L0s. before.json.gz contains 92 pairs; extra-before.json.gz 26; boundary-before.json.gz 10; repeat-before.json.gz 4. Oracle commit: `bd5ebec285505889e8ff1d4f68804ab147a699f3`; binary SHA-256: `9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`; initial Tekt: `2cc69a555e9738dc659c06ed6d933853ab981cbb73e93c388a5b43c55afad095`. At width 160pt/margin 8/font12, hanging10 leaves first-line x8 and moves continuations x8→18; negative-5 requires x3. RTL leaves first right edge 152 and moves continuations to 142. Tekt ignores the length. MLA defaults to 18pt (1.5em); ambient set 3em does not replace it, while show-set3em requires 36pt; IEEE/Plain do not acquire hanging indent through that show-set. bibliography.where(style:"mla") does not match in the vanilla probe: preserve selector choice rather than forcing override.

Inline-only Block/Box bodies ignore hanging10; explicit Par inside them and Block with Parbreak apply it. Simple Heading/Raw/List/Quote do not. Partial, zero, repeated and empty-scope declarations close boundaries: FIRST/MIDDLE/LAST at 5.168/27.464/49.76 versus all 5.168 in Tekt. The first lexical probe used literal brackets; it was refuted, corrected to #[...] and retained as extra-before-initial, not evidence of Styled scope. Extreme values produce extractor-clipped paint; missing pdftotext output does not mean Content deletion. A partial 24pt font does not change the shared length's 12pt em, as partial-font records.

Pinned source: `typst-realize/src/lib.rs:1061–1085,1190–1219` distinguishes fragment/Par and interruption; `typst-layout/src/flow/collect.rs:72–134` selects flow; `inline/mod.rs:174,216–221` and `collect.rs:141–147` resolve shape; `inline/line.rs:496–503` positions; `rules.rs:510–523` provides CSL inset/compensation. `typst-library/src/model/bibliography.rs:112–127,367` determines default/precedence. This measures language/geometry, requiring neither vanilla Rust structure nor inferred historical intent. Lost boundaries, run-relative em, double indent, page reset, imposed greedy fallback, lost copies, reexecuted callbacks or control regressions refute closure.

AST spans, patterns/guards/bodies and complements were checked before code. `lab/results/construction/diagnostics/tekt-frame-items-2026-10-16/before.json.gz` preserves HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree, UTC, sources/L0/configuration, hashes, status and git diff HEAD --stat. classification.json records 70 cases/complements across 26 owners using pinned CI-parser tree. Relevant L0 clauses were read with source; this proposes neither new architecture nor inferred upstream intent. Rust patterns/exhaustiveness are mechanics; product semantics/morphology/geometry/effects/diagnostics remain unchanged.

AST sites F011 layout_sub_frame_with_origin at 293 and F012 text_of at 553 were read; scoped width/alignment/relative-region/decoration-origin receipts keep full sources, exact HEAD/dirty state, UTC/diffstat and binaries before decisions.

## Exclusive ownership and current contract

The isolated path temporarily installs SubLayoutRegion, preserving/restoring outer cursor/dimensions/line/emitted buffers/page margins/pending baseline/collapse/subframe/unconstrained flags/collectors in LIFO. Return local items/height/decor and orphan H/V alignment paths without adding parent origin twice. Save/clear validated-prefix one-shot so local flush cannot consume exterior handshake. No independent page dispatcher/checkpoint ownership. A cfg(test) entry probe may count invocations for `00_nucleo/prompts/core/compiler/layout/grid/measurement_cache.md`, absent in productive build and without behavior change.

Inline path dispatches body/drains pending tail and optionally aligns it while preserving parent prefix. It is not all-state isolation: Box owns its extra fraction/repeat protocol. Preserve origin_x=0/width=0 assertion and material-nontext height fallback. Direct private HTML markers remain ordered/returned but contribute no visual material/fallback/decor metric; Semantic/Link/Group retain classification. Extraction/removal belongs to `00_nucleo/prompts/core/compiler/layout/html_frame.md`.

Geometric path shares isolation, starts at y 0 with text baseline pending and defers font metrics until actual text; first nontext consumes origin geometrically. Measure text tail from top through real final bottom, without subtracting top twice. Baseline path retains installed baseline and Raw's explicit exception; current `00_nucleo/prompts/core/compiler/layout/placement.md` chooses geometric for all nonfloat Place and Align, superseding initial root-only restriction. Save/install sub_frame_origin_y=start_y and restore before return. Keep explicit_line_alignment inherited; placement installs/restores override, not this helper. Align isolated final tail through cursor once when override exists, even without RTL, before drain; inline keeps prior policy.

Isolated line swap also swaps pending_fr/pending_fr_natural_widths/pending_repeats/justify_opportunities into empty locals and restores LIFO. Expand local final fractions/repeats once under local width before final alignment/drain; closed lines retain normal close protocol. Parent indices never enter child and child indices never address parent. Do not change Repeat's share formula to force 14 pieces: control baseline without subframe, right-edge absolute coordinates and local no-fraction eight complete pieces establish only isolation. Repeat's block/inline and fill semantics require its owners' separate decision.

Save/zero/restore inline ascent/descent and line_assumed_ascent presence/value. Geometric starts None; legacy stores Some of actual vertical_metrics-installed baseline, with Raw regional calibration only when terminal Raw-block. Some(0) is valid, not fallback. Compare actual inline ascent and directly present Text/TextShaped/Glyph top edge; fold starts from actual text, exterior style only if no text exists. Shift only final tail/baseline by positive difference, including pure-text ascent; do not add typographic descent to Group without direct text. Use inline descent for bottom, retain closed-line protocol and publish local last_sub_frame_inline_extents before restoring parent; final outer result replaces nested child result. Do not substitute unused line_leading_pt scan for current formulas; remove only that scan, retaining last_block_descent_y save/take/restore.

For legacy terminal Raw only initialize regional ascent after saving parent; first Raw converts local top with that installed regional ascent, not inner Strong/Emph style. Measure terminal Raw's real bottom minus top, without pending leading/below; first empty local Raw uses correct legacy start minus exterior top edge. Never request font catalog for geometric/nontext origin. Raw margins/defaults/product classification remain owned by `00_nucleo/prompts/core/compiler/layout/raw.md`.

Save/clone/restore tracking State with outer line. Inline temporary prefix swap for tail alignment preserves body's producer provenance. Save/clear paragraph active/pending directions in independent region, restoring LIFO; alignment override is distinct from direction. Ratified logical transport uses private Cursor, Arc input and exact spans prepared for the whole paragraph under certified lexical base before emission. Preserve visit-order offsets, not string matching. Sequence/Par create scope and nested Sequence inherits; checkpoint retains cursor/current span and subframe isolates/restores. Text supplies word/Literal-space subspans; Space consumes its segment even when weak; Linebreak consumes boundary; Word emits span only for exact untransformed text. Initial scope is incremental Simple outside generic cells/columns/subframes, without tracking/hyphenation/justification/smallcaps and Text newline/tab/language segmentation. Refused regions have None; old global preparation remains for no-context paragraphs. Certified RTL-base/RTL-character/opposite-override subset uses incremental without changing permitted breaks; explicitly selected Optimized is untouched. `00_nucleo/prompts/core/compiler/layout/bidi_origin.md` owns preparation/validation; each consumer performs only its own role. Uncertified constructors use None and read patterns `..`, preserving pinned origin transforms without broadening API/phase. Suspend cell_bidi_allowed in isolated path only when already nested in another subframe; direct cell retains owner's capability. Inline always suspends/restores; keep parent guard/cursor protocol.

Start local segments for inherited lexical links and close before returning items while retaining parent's open-buffer boundaries; membership includes deferred frames. Inline retains body-item boundaries. Link resolution waits until geometry fixups.

Save/take flow_placement_end around body so Place cannot consume parent flow; remove/restore Block local region and private decoration ranges in LIFO, allowing descendants' own regions. Save/reset flow_frame_extent to local origin/zero width; publish last_sub_frame_flow_width before restoration while retaining legacy last_sub_frame_width/public returns. Capture final causal line after fr/repeat expansion and before RTL through line_commit's width helper, without extra vertical flush or rerender. Isolated path clears/restores block_auto_width; inline keeps existing behavior.

Save/restore relative_region_size: local width is supplied region; explicit caller height wins, otherwise finite Block height, inherited real relative region or previous physical region takes priority over Block Auto/pagination sentinel. Genuine exterior infinity remains infinite. Terminal native transform returns own physical end without below; Matrix retains legacy algorithm. Suspend/restore text_segment_chain/geometric_text_lines only in isolated path; inline preserves lexical inheritance.

Suspend both admitted bibliography-prefix and inline Block-region contexts with existing geometry/Block-place region on subframe entry, restore LIFO. Measurement/Place cannot consume exterior prefix or outer-page bounds; inner Blocks may install own context. Keep return/offset/callback/dimension/purity contracts. Isolate hanging State and pending queue from exterior; local classification follows inline fragment versus explicit Par/body boundary and does not consume parent's CSL compensation, including inline/callback isolation. Manual-tail decoration collector uses effective local paragraph left before outer restore, retaining end_x/baseline.

F011 replaces only fallback pattern with fully scoped FrameItem::TextReplacement/TextSeparator/Semantic/Line/Image/Shape/Group/Link; F012 with FrameItem::TextSeparator/TextShaped/Line/Glyph/Image/Shape. Keep original bodies/arms/guards/transforms/calls/returns/order. Preserve callbacks/effects/metadata/clipping/fonts/positions/paint/width under their current owners. This work does not expand traversals, turn wrappers into leaves or substitute text/ActualText for geometry. Test helpers preserve fixtures/expected outputs/tolerances/counts/Unknown failure; cleanup cannot accept missing observation. Nominal alternatives retain the prior shared body and visible informative V19 notes; one arm's execution does not prove every alternative.

## Acceptance and limits

Require causal RED→GREEN on both origins, local finite Shape overflow/multiline text, all returned content with preserved outer prefix/cursor/dimensions/no outer pages, and normal exterior pagination. Verify local fractions/repeats/internal flush and exterior eventual expansion versus no-subframe baseline, body/prefix producer identity, direction/cell nested scope, pending links/deferred frames, RTL/fixup paths, causal width/Stack, small text/nested Box extents, HTML P1493 and Raw Strong/plain/no-font controls. Preserve source→PDF/SVG positions and all existing negative limits; isolation is prerequisite, not complete container or Repeat certification. Link captures require boundary/callback/rollback controls and no private markers in final output. Place/Block fragmentation/late-inline/resources remain under their canonical owners.

Require occurrence/UTF8/restoration/parbreak tests and bilateral wraps for logical transport. Tracking corrections require RED→GREEN, bilateral text/position/line controls for both directions/signed values and preserved LTR/uniform/terminal controls, error-free build/lint and restricted resealing; all other recorded limits stay open.

Require RED→GREEN for breaks before/within body; preserve prefix/suffix, regions, destinations/bounds, rich body/nesting/columns, deferred content and context/rollback, with no private boundaries in output. Revalidate no-break controls, suite, build and lineage.

Prefix controls require RED→GREEN same actual prefix/body page, continuation inset and OUTSIDE, Simple/Optimized and prior controls. Measure PDF90/150/250, RTL, prefix-only/formatted/color/regressions; build/suites/doctests/Clippy/Rustdoc/fmt/strict-reverse lineage/differential Tekt under MemoryMax12GiB/High8GiB/swap0/jobs1, without publication. Generic hanging/overrides and multiple bibliographies retain their own open scopes.

This internal parity correction follows ADR-0127 continuous flow: no new public field/signature, external flag/default, phase or realization order. Initial/continuation shape supersedes only the corresponding constant-width limitation. Certified Latin RTL remains subject to the nucleus conditions, without active BidiCursor/tracking or invented BidiSpan; every other prior limit remains. Off-page values/copies require frame/SVG inspection rather than equality of clipped extraction. Do not anticipate the separate R3-T1 scope.

Require RED→GREEN before implementation and source→layout/bilateral controls for relevant signs, boundaries, variants and restoration. Executing a grouped arm does not prove every alternative: closed nominal inline/flow-context classifications are intentional and need class/variant tests. Reseal only after V15/V26 are valid; run tests, build, Clippy/Rustdoc/fmt, reverse lineage and differential Tekt against pin1279707, under 12GiB/High8GiB/swap0/jobs1. No publication.

Internal continuous-flow ADR-0127 maintenance introduces no enum/field, trait method/public signature, default, dependency, compatibility or phase. Do not materialize historical pending proposals. L0 precedes restricted bilateral resealing. Add no suppression, threshold change or match→if-let rewrite to hide V16.

Before code, run new FrameVisitor control and workspace suite with unchanged expected outputs, and establish structural RED for 70 family warnings through AST reading. Afterwards require zero covered-family V16 and no new warning, retaining external V16 as pending. For productive core matches reached by cargo check -p typst-core, use a disposable copy with a sentinel FrameItem variant to measure absent E0004 barriers before/present after. The probe covers only compiled sites, not cfg(test) helpers or L3 blocked by dependency. Across the whole package replay patterns/guards/bodies, verify unchanged enum/signatures, real build/tests and no stub bodies.

Revalidate full workspace suite, cargo build, Clippy workspace/all-targets -D warnings and -D unfulfilled_lint_expectations, fmt, Rustdoc -D warnings with four HTML indexes, strict/reverse V1/V5/V15/V26 lineage, SARIF and diff --check. Freeze sources/L0/configuration during commands. Heavy commands run sequentially with MemoryMax12GiB/MemorySwapMax 0/CARGO_BUILD_JOBS1. Structural cleanup does not establish new global parity, memory gain or measured AI benefit.


## Ratified Circle constructor origin

### Evidence and approval before implementation

At HEAD `954d13185e26a1ed90f6d7c60f6ba27760ca9889`, the source/owner audit in
`lab/results/v16-corrections-2026-10-19/circle-origin/l0-before.json` freezes
UTC, exact diffstat and hashes. The approved proposal
`lab/results/v16-corrections-2026-10-19/batch-f/agents/geometry/circle-deferred/proposal/review.md`
(SHA-256 `5b7136bc18f419004113d8d83c9c54adb816e3b70389039aeeed35e1c6f772e8`)
binds the baseline equality and rejected full-Length candidate receipts.
`stdlib/shapes/ellipse.rs:92-104` erased the em component;
`entities/elements/shape.rs:21-29` used one geometry carrier for both constructors.
Those observations concern language identity and geometry, not mandatory upstream
Rust structure. The explicit user approval is recorded in
`lab/results/v16-corrections-2026-10-19/circle-origin/approval-receipt.json`;
it satisfies ADR-0127 section 2 for exactly this public type/field and migration.

### Exclusive approved responsibility

Migrate this consumer's existing generic ShapeElem literals to explicit
`ShapeOrigin::Geometry`, preserving every other field, fixture, expectation,
argument, algorithm and output. Equal axes or Ellipse kind do not infer Circle.
This is only the required migration for the approved field, not a new geometry
behavior or permission to materialize any unrelated pending proposal. Full-target
compilation and existing tests must retain all current observations/tolerances.


## Approved physical page occurrence protocol

### Evidence before decision

At baseline HEAD `63b7b96b99109b61655b373dc0d248beb7838f86`, the complete source/owner snapshot and applicable ADRs are retained in `lab/results/set-page-header-2026-10-21/agents/core/source-evidence.json.gz` and its supplement. `baseline-parity/receipt.json` there binds current-pin physical query/state/counter/page observations and inert overwritten templates. `ambient-baseline/receipt.json` refutes freezing creation10pt styles: insertion uses actual page20/30pt ambient style. These are language observations; the source plan, packet transcript and rollback representation are mechanics. The exact five facades, opaque types and Paged discovery/realization/source-construction phase were explicitly approved in `lab/results/set-page-header-2026-10-21/review.md` (SHA-256 `cb4a684b483e7f9f6ac3f7eef52277f7ada405033a24e4e66e55811e93c12039`), recorded in `lab/results/set-page-header-implementation-2026-10-21/approval.json`. Only conflicting no-new-phase clauses for this exact protocol are superseded; legacy signatures/defaults and unrelated limits remain.

### Exclusive protocol responsibility

Provide private planned-child entrypoints while retaining every legacy public signature and existing geometric/inline isolation behavior. Source-bearing children and generated wrappers route explicit typed source edges; isolated measurement and genuinely generated-only sublayout carry explicit None. Save/restore caller plan cursor/environment/slot state in LIFO order without sharing operational Location identity. Actual emitted child packets remain attached to returned/deferred contributions until their final destination is accepted; measurement cannot append events or consume resolutions. No technical Content wrapper or fake Location is inserted.

### Acceptance and preserved boundaries

Require semantic RED→GREEN for repeated physical header/footer/four-slot order, zero-paint updates, source-derived parents, overwritten inert templates, aliases and physical pages, effective20/30pt ambient plus explicit inner styles, nested PageRun restoration, Context reads, and exact duplicate-label/source diagnostics. Cover speculative suffix rollback, deferred floats/notes, subframes, measurement isolation, complete body/slot coverage, stale/mixed authority and math/par.line coexistence. Do not weaken existing tests, callback budgets, per-store sealing, source morphology, HTML isolation or old public compatibility. No generic painted-frame semantic reconstruction, source re-evaluation, second arena/Context solver, direct Func marginal mode, new default/flag or extra public API is authorized. Every changed descendant retains its own current 1:1 owner and explicit typed route; finite source inventories are not blanket traversal permission. Exhaustive new decisions need actual future-variant barriers, and the complete lineage/Nucleus, build/test/lint/Rustdoc/fmt/Clippy gates remain required without suppressions.

A generated prefix-plus-source-body subframe keeps the original combined Content for its existing measurement/classification formulas, but dispatches the prefix explicitly unplanned and the exact retained source-node body under its cursor. It never assigns the generated Sequence a source identity. Deferred footnote bodies carry parallel queue cursors, preserving legacy tuple fixtures and original producer objects.

An explicitly unplanned subframe temporarily suspends the complete running pass, not only its current cursor. Generated/measurement traversal cannot alter source slot bindings, page environments, requests, admissions or consumption. Restore the exact outer pass afterwards; planned source subframes retain accepted contributions.


All productive inline callers now supply explicit source planning through layout_sub_frame_inline_planned, including None for an isolated unplanned body. Remove only the unused private forwarding layout_sub_frame_inline shim, retaining the complete inline implementation, None behavior, geometry and restoration. No legacy public signature is changed and no lint suppression is introduced.


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

Save/install zero/restore footnote_reserved_height around both isolated and
inline subframes. A local body never inherits the exterior page's reserved note
space. Every ordinary isolated/inline subframe takes and later restores the outer
optional FootnoteCapture in LIFO order, so nested generated subframes cannot
certify cuts in their parent's item array. A private layout_sub_frame_footnote
entrypoint alone preserves the feature-installed capture through its actual
Note subframe, including when the Note itself is inside a parent subframe. The
private isolated helper receives this explicit capability; all existing public
entrypoints/signatures remain unchanged. Ordinary source admission and actual
markers are unchanged. Preserve existing note queues and source cursor isolation; do not
rerender a captured Note or change any public return/signature.

Validate actual split Note text/page/counter/query/identity and no-fit progress,
plus existing no-break notes, columns, subframes, rollback, floats and running
content controls. Preserve strict trace coverage/foreign authority, warnings,
canonical owner paths and forward/reverse lineage; no suppression or fixture
weakening. These transport declarations alone do not certify general pagination.


## Private assigned-width transport (2026-10-21)

### Measurement before specification

`lab/results/nested-fractions-repeat-2026-10-21/measurement-before-decision.json`
pins four fresh bilateral receipts and their UTC intervals, HEAD
`cf21f47a3b63f0e9fc5308925e1d0b67925e7227`, exact uncommitted source inputs and
`git diff HEAD --stat`. Product SHA-256 `94208ba4526d7bda97d4519aa8eb12212bd13dc1f8932552f235940e73238485`
is compared with pinned vanilla `bd5ebec285505889e8ff1d4f68804ab147a699f3`,
binary SHA-256 `9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.
`boxed.rs:196–305` mixes child fraction boundaries into the exterior denominator.
Pinned `typst-layout/src/inline/line.rs:577–591` allocates the final Box width
before its actual body. `word_layout.rs:31–44` shows why translation of paint
prepared at a provisional width cannot repair general body wrapping.
These are observable language geometry/effect requirements; tree representation
is private mechanics, and source inspection alone does not certify parity.

### Exclusive transport contract

On independent subframe entry take the exterior immutable inline allocation
Option, starting the local actual body with None. Restore the exact exterior
Option in LIFO order after local composition. This is geometry scope isolation,
not a source-plan or admission replacement. Preserve all return values, baseline,
font, flow, decoration, pending-buffer and source isolation contracts.

This is an internal layout parity correction under ADR-0127 continuous flow:
no public entity field, trait method, signature, flag/default, compatibility or
eval/realize/layout phase change. L0 precedes meaningful RED→GREEN and scoped
resealing. Require actual nested/weighted/zero-width/source-once/decoration,
subframe-restoration and rejected-suffix tests, all prior geometry controls,
bilateral parity, full workspace tests, Clippy/rustdoc/fmt, hashes and canonical
forward/reverse lineage, with every warning retained.


## Affine observation of isolated Repeat frames

`lab/results/nested-fractions-repeat-2026-10-21/core-interim-red/receipt.json.gz`
and `test.log` preserve the exact current source inputs, HEAD/uncommitted stat
and UTC of the real `child_repeat_tail_is_expanded_before_return` failure.
`text_positions` observes only flat Text and consequently reports no copies
inside the correctly composed prepared Groups. The actual new Repeat tests
pass for complete copies; the fixed-width bilateral corpus retains independent
coordinates and source bytes. This refutes the flat observer, not the existing
eight-copy or 1e-9 geometry expectation.

The test-only position visitor must recurse through neutral Group, Link,
Semantic and TextReplacement wrappers, composing Group translation and matrix
from root to leaf. Observe Text and TextShaped in world coordinates, preserving
every prior text/count/coordinate expectation, assertion, input and bound. No
productive geometry, diagnostic, source admission or fallback changes follow
from this observation repair. Preserve all previous isolation and root controls.


The second actual full-core receipt
`lab/results/nested-fractions-repeat-2026-10-21/refinement-semantic-red/receipt.json.gz`
refutes including an empty logical Text leaf newly reached inside a technical
carrier in the content-position observer: A/Linebreak/B produces A, empty string,
B while the unchanged expected logical text remains A/B. The test-only visitor
records nonempty Text/TextShaped leaves; this does not change productive frames,
line-height observations, input, expected A/B positions, isolation assertions or
their 1e-9 bound. An empty string is no text token, not a lost visible marker.


## Private first-flow baseline witness

### Measurement before correction

`lab/results/nested-fractions-repeat-2026-10-21/prototype-1/zero-source-after/measurement.json.gz`
binds actual source, candidate/oracle SHA-256, fonts, HEAD, uncommitted diffstat
and UTC. Zero-copy Repeat loses its prepared baseline, moving adjacent BEFORE/TAIL
while its body height remains real. Source inspection at pinned
`lab/typst-original/crates/typst-layout/src/repeat.rs:19–29`,
`inline/box.rs:32–47,79–89`, and `flow/distribute.rs:689–699` shows that
a prepared frame retains baseline even without copies, and the FIRST actual
in-flow frame decides baseline presence. The local Box paint scan cannot see it.
Exact preamendment owners/sources and state are archived in
`lab/results/nested-fractions-repeat-2026-10-21/baseline-witness-before.json`.
These are language geometry obligations; private carrier representation is mechanics.

### Exclusive private obligation

Save/reset the active first-flow witness on entry. Register the settled actual pending paragraph baseline only after final inline ascent adjustment. Publish the completed tri-state result in local coordinates, then restore the parent witness; nested last results never become active parent decisions. Preserve all geometry, height, pending-line and assignment isolation.

Require genuine RED→GREEN for empty Repeat baseline, positive copies, earlier Text,
earlier empty frame, nested insets/shift, Shape/empty controls, LIFO and checkpoint
restoration, then the same pinned bilateral SVG/PDF observations without relaxing
1e-8 geometry or semantic membership. Unknown legacy producers retain their
previous fallback unless a measured owning frame explicitly supplies a decision.
This is an ADR-0127 continuous private layout correction, with no public contract,
new product mode/default, compatibility break or eval/layout phase change.
L0 precedes implementation and scoped resealing; full tests/Clippy/rustdoc/fmt,
zero Tekt errors/warnings, paths and forward/reverse lineage remain mandatory.


## Measured nonexpanding Repeat alignment — 2026-10-22

### Evidence before decision

`lab/results/remaining-parity-2026-10-22/repeat-refuters-before/measurement.json.gz`
retains exact source/font/binary/build inputs, UTC2026-10-22T12:02:34.973885+00:00
through12:02:51.345977+00:00, uncommitted HEAD
`cf21f47a3b63f0e9fc5308925e1d0b67925e7227` and its dirty-file stat. Product
SHA-256 `9f696f72294ea255cb5ef499e1882265c52e73b2d91dc1ca0bf76768ea803fcb`
and ratified oracle commit`bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree
`cac836c07e4dec2a89219370428292393a3216b5`, executable
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`
are pinned. The eight-source manifest and full matrix are in the adjacent
`agents/repeat/` evidence. Nested fixed-child center/right and unequal-width
multiline center/right fail; decorated multiline also loses line paint.
Direct Shape, natural text and an explicit inner-left override are GREEN.
Repeat clears inherited alignment before composing its piece; pinned upstream
`typst-layout/src/repeat.rs:19,55–59` preserves lexical styles and separately
places copies. `shape.rs:251–283` emits flow geometry without line alignment,
refuting global translation. Nested-only inheritance does not fix the measured
multiline defect. Language geometry/paint are required; data structures and
iteration mechanics are free under ADR-0107. Classification and pre-edit hashes
are preserved in `agents/repeat/classification-before-L0.json`.

### Private contract

The private nonexpanding-piece adapter starts a fresh alignment scope after saving the exterior scope; ordinary independent subframes suspend it and restore LIFO while preserving lexical alignment. Publish the completed logical width, settle only that scope, clear outgoing resolved decoration tokens, and restore every parent field. Public SubLayoutRegion and public APIs stay unchanged.

Require genuine declared-metric RED→GREEN for nested center/right, unequal-width multiline and decorated multiline; preserve direct Shape/natural-text/inner-left controls, source effects once, rollback, complete bilateral PDF/SVG geometry at the unchanged bound, all tests and both lineage directions. No warning suppression, ignored case, threshold relaxation, fake source identity or public/phase expansion.


## R8 unified note capture adapter

Evidence: footnote_flush's obsolete unnormalized legacy adapter was its only
caller of layout_sub_frame_generated_prefix (rg after R8 capture materialization).
All note callers now use the existing layout_sub_frame_footnote adapter, preserving
its generated-prefix/original-body route and Footnote capture purpose. Remove the
unused descendant-only generated-prefix Ordinary wrapper; retain shared internal
routing, ordinary/planned/repeat adapters, signatures and all source/geometry
protocols. No public contract or phase change. Full tests/Clippy/lineage required.


## R8 horizontal superscript compensation is paint-only

Source evidence precedes this correction: pinned inline/line.rs:458–484 applies
script compensation by Frame::translate without changing advance. The approved
metrics tuple supplies this offset; current text.rs adds it to cursor_x, which
also shifts following text. The existing real font fixtures have zero X offset,
so injected nonzero provider controls are necessary refuters, not font-name cases.
Keep logical advance, wrapping, justification and source identity unchanged;
translate only the text/highlight paint of the actual script run. This is private
formula/state correction, no new public field/API/phase (ADR-0127 continuous).
Save and clear private text_horizontal_offset at independent subframe entry, then restore on exit, alongside the existing style scope.
Require nonzero injected offset and following-text/rollback/subframe RED→GREEN, actual-font controls and complete validation.


## R10 — expanded signed-tracking, fallback-space and literal-tail refuters

Measurement before decision: candidate2-build/receipt.json/source.json.gz and
candidate2-matrix/receipt.json bind exact dirty inputs, UTC, sources and binary
f2a61237c15688e655ff7c62a117cb50a4d3a154e426bd31bed61fa999385e68
against the fixed bd5ebec2 oracle. Original15 script PDF/SVG controls and all
R8/R9 controls pass; the54 additional controls expose22 failures. At Nimbus11,
trailing literal LF loses a3.058pt following markup space; explicit DejaVu
fallback overmeasures that space by0.43858pt. At DejaVu11 tracking0.2pt, the
script's three intervals overadvance by0.18018pt. Nimbus synthetic digits need
0.12pt intervals and the following fallback glyph needs the same last-prefix
interval rather than zero. Pinned inline/shaping.rs:1269-1295 divides tracking
by outer ctx.size before each glyph's actual reduced size is applied; :473-510
retains outer-size line edges. Source sequence/space causes and actual selected
text face explain the remaining cases, without a font-name branch or looser
physical-word/page/0.001pt gate. This is private parity integration of the
approved method, not another public contract/default/phase change.

### Exclusive owning obligation

Save/clear/restore the private literal_script_break_pending cause at both existing independent and inline subframe entries alongside the paint offset; an isolated region must not consume or create its caller's pending separator cause. No other style/source/metric contract changes.

Require module RED→GREEN, expanded54-source matrix without changing thresholds or source membership, original15/R8/R9, A–H/R1–R7, full workspace/Clippy/fmt/Rustdoc/forward-reverse lineage, resource and preservation gates. Retain every failed candidate and its actual source state.


## R13 — approved ShapeElem literal migration

The source inventory in lab/results/construction/diagnostics/relative-lines-r13-2026-10-25/migration-owners.json identifies this consumer. Owner approved the public ShapeElem.relative_line addition (proposal SHA-256 d8c9a01413232bac7113d1148af3c1b3bcd671bb37eeca68a94d5749d671db00). Initialize that field to None in existing constructors and fixture literals, preserving all geometry, paint and behavior. No other implementation change is authorized here. Full existing tests, build and lineage must pass.

## Image flow and horizontal paragraph correction — 2026-10-27

Measurement precedes this correction: `lab/results/construction/diagnostics/image-flow-cause-2026-10-27/{baseline.json,results.json,refuters.json,completion.json}` binds HEAD212aed044, uncommitted inputs/diffstat/UTC, pinned64c5e8c0/tree483bd6f1/SHA22897f93 and received Tekt SHA2941a5ba. Direct PDF streams show image1 at y5 and images2–16 at y13.2; vanilla has eight30x20pt images per page at step46.4. Removing h gives vanilla step33.2 but retains Tekt overlap. Margin30 still resets later images to13.2; a60x40pt image does too. Source `image.rs:198,325–335` advances cursor without publishing the physical end consumed by `cursor.rs:128–169`. Vanilla `rules.rs:771`, realization `lib.rs:1061–1087` and flow `collect.rs:162–188,240–285` distinguish image blocks and h-only paragraph gaps. These are language/geometry observations, not a requirement to copy buffers, PDF bytes or upstream execution structure. No upstream intent is inferred.

Classification: private parity repair, continuous flow ADR-0127. No public fields/signatures/trait methods, new product mode, phase change, dependency or compatibility break. Keep all received footnote/R11–R13/JPEG work. New shared obligations are pinned in the horizontal-space-paragraph Nucleus; this owner authorizes only its exclusive consumer.

### Exclusive correction

At both independent and inline subframe entries save/take the parent's optional horizontal-paragraph cause; restore it LIFO at return. Local flush/measurement cannot close the parent's cause. Do not change returned metrics, origin, paint transforms or source dispatch to transport this scalar.

Require targeted RED→GREEN, bilateral PDF/SVG<=0.001pt, source order/complete image counts, all source-margin/size/spacing refuters, prior R8–R13/notes and A–H/R1–R7, full build/tests/Clippy/fmt/Rustdoc and full lint without warning/error. Preserve known tracking/paint limits; remeasure JPEG latency/RSS against received/legacy/oracle and retain R6 resource bounds. Snapshot provenance and preserve unrelated inputs/historical receipts. Validate V15/V26 before restricted resealing; no commit/push.

### Measured lexical horizontal scope refinement

The candidate2 frozen state and pilot/geometry.json in image-flow-fix-2026-10-27 show mixed-h-scopes and siblings-h-scopes differ by43.2pt at the third image, while sole-h-scope and wrapper-h-scopes match. Equal styled siblings do not establish a shared lexical paragraph wrapper. Resolve h-only paragraph spacing from the nearest actual common StyleChain ancestor of all producers. Preserve the original anchor and monotonically narrow to its ancestor; do not infer lexical scope from equal values. Existing raw_deltas supplies identity without a public contract change. The private pending state stores an Arc-backed source chain and ancestor depth; normal one-producer closure resolves directly, mixed scope may reconstruct only the ancestor suffix for resolution. Checkpoint clones, subframes save/clear/restore, region reset and real-line absorption apply to this complete state. No public API, source reevaluation, paint, dependency or phase change. Require RED→GREEN for mixed outer/sibling scope and sole/wrapped controls, then rerun full validation.

## Stack image frame correction — 2026-10-27

### Measurement before decision

`lab/results/construction/diagnostics/stack-image-flow-fix-2026-10-27/before.json` records HEAD212aed0449150dd296ceea13fc2be665cc5db01a, uncommitted diff/stat/status and every received productive/L0 input, UTC and executable identities. before/{receipt.json,geometry.json}, observe-before.log and effects-before/{receipt.json,query-absolute.json} are fresh bilateral measurements against ratified64c5e8c02fe0394d913ead36e42ef8c1334d2107/tree483bd6f1/SHA22897f93; received product17f39776 includes the completed image/h and JPEG work. Stack3 image tops14.834/22.072/29.310 versus vanilla5/25/45; Stack16 drift181.596pt; Stack20 is one page versus19+1 images on two pages. RTL/BTT source-order placement is wrong; mixed text/image, hidden and clipped children also fail. Pure horizontal text remains qualified. Negative spacing is rejected only by Tekt's P156I gate. Context metadata puts child19 on page1 versus2. Source stack.rs:115–119 returns zero descent for every child without text, discarding a computed bottom; :144 adds an outer text baseline, :181 uses the legacy-origin route, :219 advances by that zero descent plus text top. sub_frame.rs:315–344 distinguishes geometric from baseline origins, and :497–521 returns a cursor-based fallback except for previously owned cases. Pinned upstream stack.rs:274–300 accumulates completed frame sizes and :303–366 composes their geometry. These are observed language geometry/pages/accepted arguments; no upstream intent is inferred and matching Rust/PDF bytes is not acceptance.

### Classification and scope

Private parity correction, continuous ADR-0127: no public API/entity field, trait method, dependency, new product mode, phase or compatibility break. Existing signatures and source identities remain. The old ascent/descent fallback and reverse-direction limitations are superseded only for this measured frame-composition correction. Existing Ordinary/Footnote/Repeat subframe routes and all received JPEG/image/h/notes/line work remain preservation controls. No new page dispatcher, source replay, callback cache or semantic reconstruction from FrameItem is authorized. Use the existing region transition and static source-child protocol. Individual child-internal multipage text, fractional stack spacings and general alignment are not certified by these finite image controls.

### Exclusive subframe responsibility

Add only a private Stack purpose/entry using the same existing planned-child isolation machinery with geometric origin. Ordinary, Footnote and Repeat return policies are unchanged. For the Stack route, a completed block/no-pending-line uses the already recorded causal flow_placement_end relative to geometric origin, default zero for no flow; do not count a prepared below gap or inspect paint to manufacture logical height. A real pending text/inline line retains existing resolved text-edge/manual-flush height. Save/clear/restore parent width, placement/baseline, horizontal-paragraph cause, source cursor, insertion state and collectors through the existing path. No new public signature, entity field, callback or phase. This private purpose is a consumer-specific policy, not a new user option.

### Acceptance

Require functional RED→GREEN before implementation, source-current bilateral PDF/SVG<=0.001pt, all four directions, spacing/default/negative, target clipping, mixed/no-paint children, nesting and exact-fit/19+1/14+6 pagination, text adjacency and h boundaries. Preserve real Context/page/label/counter source semantics and all current positive image/h, A–H/R1–R13/notes/relative-line controls. Repeat all12 established technical gates, unfiltered lint with zero warning/error, frozen provenance and restricted lineage reseal. Recheck JPEG acceptance/complete decode/dedup/time/RSS and R6 size/memory gates with no competing builds. Bound the known invalid oversized oracle case explicitly; never hide a timeout/signal as a qualified rejection. Historical self-sealed stdout limitations remain explicit; do not rewrite old manifests. No commit/push.

### Positive frame certificate refinement

The existing subframe tri-state flow_baseline (sub_frame.rs:472,527) distinguishes an explicit non-text producer Some(None) from absent certification and real text baseline. Restrict the new Stack-purpose causal-end return override to that positive non-text certificate with no pending real line. Retain text/uncertified/manual-flush height policy, including terminal literal text, and all existing purpose routes. This adds no witness field or new public API.

## Stack child origin transport (2026-10-27)

Measurement before decision: `lab/results/construction/diagnostics/stack-image-flow-fix-2026-10-27/effects-candidate/context-binding.json` records the twentieth child with metadata location page 2, but both captured `here().page()` and the retained Here Location page 1; vanilla reports page 2 for all three. `page_running_requests.rs:990-1003` and `realized.rs:285-307` register descendant original identities only when absent. Ancestor registration can therefore prevent a local child carrier from being captured. This is observable location semantics, not a requirement to imitate vanilla mechanics.

Decision (ADR-0127 continuous flow): while entering the existing private Stack subframe, recapture the child's already-authorized original identities at the local origin, before dispatch. Reuse the selected source-plan traversal, operational/pruned/ref predicates and existing marker protocol; this anchoring mode must not admit source events a second time or execute callbacks. Without a plan, reuse the existing original-location collection, without introducing a new allocator/identity rule. Ordinary original registration remains only-if-absent. Later explicit placements remain authoritative and nested Stack frames repeat the same local transport. Geometry and semantic event ordering remain separate. No public contract, phase, global dispatcher or Context solver changes.

Acceptance: an ancestor-pre-registered realized child origin must follow that child's transported frame onto page 2; source-level stored Here Location, captured page integer and metadata Location must all match vanilla. Counter/footer source ordering and all current context stability gates remain mandatory.

The second candidate refutes eager observation of every descendant: `effects-candidate2/binding.json` in the same diagnostic records rotated Metadata/Here positions after the preliminary reanchor advanced the Locator for unvisited visual nodes. Local recapture must neither observe/allocate locations nor create an absent runtime position. It only attaches a local carrier to identities already admitted by an ancestor. Existing normal traversal remains the sole first admission/allocator authority. Preserve the failing candidate and add a RED test asserting that recapture leaves the Locator seed and unvisited membership unchanged.

## Closed block frame excludes cursor preparation (2026-10-27)

Measurement first: `full-attempt1/validation-complete/tests-domain-infra.log` under the Stack diagnostic retains the unchanged p1132b failure: auto-page height183.88811338582676 rather than the existing125.236 expectation. `math-stack-refuter/receipt.json` independently records received125.2361, candidate183.8881 and pinned vanilla125.236115. The first math baseline is unchanged, but the b/c advances each include an extra14.663pt. `equation.rs:224-230` installs the raw first-block ascent, `:595-612` flushes and stores the actual final raw baseline+descent in last_block_descent_y; line_commit retains that physical baseline/end while cursor advances to prepare the next line. Stack's no-final-line fallback selected cursor_y despite this existing physical-end certificate.

Decision: for Purpose::Stack with no material pending final line and a present local last_block_descent_y, use that causal bottom minus local start_y as height, clamped at zero, before the existing geometric nontext flow fallback. This applies to completed block math/image/Block/Stack frames without inspecting painted text or rerunning math. Preserve every other purpose, pending-line formula, actual items/baselines, source identities and exterior state. Last closed physical bottom excludes preparation for a nonexistent next line and pending lower margin. Require unchanged p1132b RED→GREEN, exact auto dimensions, mixed math/image controls and revalidation of every prior image/wrapper/direction case.

The proposed additional terminal-chain guard was refuted: tail-descender-red.log passes the corrected explicit-descender fixture on the pre-guard implementation. record_flow_baseline describes the first anchor, while line_commit.rs:997-1013 already reconciles last_block_descent_y to the closed line's baseline/default or declared descent. The initial default-edge fixture incorrectly demanded2.2pt when default allocates zero; preserve its failure as invalid test setup. Keep the explicit-edge preservation control, use the existing closed-bottom certificate without an added block_chain_active guard, and preserve all other purposes/paths. This does not close the separately observed internal paragraph-top difference after image in a compound child.

## Natural block mathematics in horizontal Stack — 2026-10-27

### Measurement before classification

`lab/results/construction/diagnostics/math-stack-width-fix-2026-10-27/before.json`
freezes HEAD212aed0449150dd296ceea13fc2be665cc5db01a, uncommitted diff/stat/status,
2167 inputs and received release SHA d141e2ee123c262b37d0bf7e7c83fd00429504782b9f8f3c030072cd834cbdf1.
The source inputs exactly match the previous Stack completion. The ratified oracle
is commit64c5e8c02fe0394d913ead36e42ef8c1334d2107/tree483bd6f1 and executable
SHA22897f93c7c2c8589cf5913b0ce735f1c01ba711c70f55c7a350f7c4b96c0b34.
Fresh bilateral `before3/{receipt.json,geometry.json}` retains sources, hashes,
UTC, argv and physical PDF/text observations. At400pt the following image starts
at210.146 rather than18.292pt; at200pt it starts110.146 rather than18.292pt.
A fixed100pt Block matches, but an Auto Block differs187.354pt, Auto Box194.354pt,
compound math/text173.0715pt and finite wrapped prose10.266pt. Fixed Box differs
2.5pt and a fixed inset Block3pt in equation position. Root/vertical/inline
controls match. Numbered LTR instead requires a full390pt child: image402 rather
than received210.146; blindly shrinking every equation is refuted.

`stack.rs:128-146` supplies the finite available region to each child;
`equation.rs:309-313` centers against that region using page margins, then
`:586-590` advances cursor through the displaced extent. `line_commit.rs:184-205`
records that right edge. `sub_frame.rs:289,357,569,614` isolates finite regions,
auto-width intent and the causal width witness. `block.rs:310,358-373` already
settles generic deferred X paths against completed allocation; `boxed.rs:496-535`
composes once and settles local paths after deriving Auto size. Pinned upstream
`stack.rs:173-179` disables child expansion on the stacking axis while retaining
finite region size; `math/mod.rs:204-209` returns raw unnumbered frames and
`:230-244` allocates numbered frames using region width. These are observed
language geometry/accepted content; Rust algorithms/bytes need not match. No
upstream intent is inferred. A following image shifted by region size, changed
finite wrapping, duplicate effects, moved fixed-width math, or changed root/
vertical controls refutes the candidate.

### Classification and preservation

ADR-0127 continuous internal parity flow. Keep public signatures/entity fields,
traits, dependencies, features, default modes, pipeline phases and compatibility.
Only existing layout-time frame policy changes. Private purpose/adapter arguments
may carry natural-width intent; no new Layouter field or independent dispatcher.
Keep received JPEG validation/streaming/dedup and the target alignment recipe byte
identical. Preserve all received notes/image/h/Stack/counter work. Invalid initial
multi-letter abc fixtures were rejected by both compilers; corrected a b c is the
usable control. `phantom` is rejected by the oracle and remains an explicit
out-of-scope binding refuter; the preexisting Repeat/math mismatch is retained,
not qualified or used to claim width parity. Probe completion, not an assumed
hang, is recorded; no process was stopped.

### Exclusive responsibility

Carry a private nonexpanding-width choice in Stack purpose and add an isolated Auto Box adapter with the same geometric-origin path and existing return tuple. Install the existing block_auto_width intent only for those requested modes, after saving it; ordinary/footnote/repeat and vertical Stack retain their existing clear policy. Restore it LIFO. Preserve finite relative_region_size/wrapping and all height/source/baseline/collector isolation. An empty closed automatic frame must not acquire an RTL terminal cursor alignment that manufactures width; actual pending lines retain their existing settlement. Return completed width and deferred local paths without consuming parent entries. No new field or public signature.

### Acceptance

Add module-level functional RED before implementation for natural width independent
of200/400pt constraint, LTR/RTL placement, different-width display lines, compound
prose and finite wrapping, Auto/fixed Box and Block allocation, numbered reservation,
exterior/root/vertical/inline preservation and state/source effects once. Compare
complete source PDF and SVG geometry/pages/text at the unchanged0.001pt gate and
independent raster; keep refuters explicit. Revalidate all previous Stack/image/h,
Here/counter/footer, math/notes/relative-line and A-H/R1-R13 controls. Require all12
technical gates, unfiltered lint zero warnings/errors, V15/V26 before restricted
resealing and frozen source/L0/configuration during execution. Repeat JPEG17922,
frozen timing/CI/Luma and nine-case RSS bounds plus direct received comparison,
R6 normalized size/RSS/repeatability/math-time limits. Use one job,12GiB,zero swap;
no competing build during resource timings. Preserve raw previous receipts and
unrelated received inputs. No commit/push or new universal container parity claim.

## Terminal optical policy and existing line ownership

optical-isolation-red.json/source.json.gz binds exact dirty HEAD212aed0449, UTC,
diffstat and source inputs. The actual parent-line test emits bb. at45pt rather
than48pt after an isolated child with overhang:false returns. This refutes
restoration of the new private policy; no break/cost/default/public/phase change
is authorized. Source swaps are documented in optical_margin.md.

Save/take the optional terminal optical policy when isolating the corresponding
parent line; begin the local line without outer policy, then restore it with the
original line at the existing LIFO exit. Preserve all cursor, source, fraction,
alignment, hidden paint and regional formulas. Do not extend all-state isolation
to the separate inline-tail helper, which contributes its actual terminal text.

## Private Figure piece composition — 2026-10-29

Measurement before decision: readiness-fixes-2026-10-28/figure-geometry-before
and figure-frame-red3 bind exact source/HEAD/dirty stat/UTC and valid functional
RED. Rect/Empty caption geometry fails; Text control passes at declared10pt.
The Figure owner specifies the gap/alignment; this owner supplies actual local
frames without callbacks, second layout or source membership from painted items.

Add only a private Figure-piece adapter through the existing isolated path,
with geometric origin, finite wrapping constraint and nonexpanding width intent.
Use the existing closed-block causal-bottom/nontext-flow height policy, preserving
all Ordinary/Footnote/Repeat/Stack policies. Keep Stack-only recapture unchanged.
Return a private typed piece containing actual width/height, tri-state first
baseline, actual flow-presence witness, items/decorations and local pending H/V
paths. Presence comes from admitted flow/line/baseline state, not ink or text.

A closed private route request may replace the two optional routing arguments:
unplanned, planned child, generated-prefix plus exact source body, and mixed
parts with parallel optional child cursors. Existing adapters select identical
old modes. Retain combined Content only for current local classification/metrics;
mixed dispatch uses the existing sequence::layout_mixed_source_parts exactly
once. No generated Sequence becomes a language source node. An all-unplanned
mixed region suspends the complete source pass; any planned part retains its
explicit authority. Exact FigureBody, selected rich supplement and caption
edges remain chosen only by the Figure owner. Save/restore every existing field
in LIFO, including optical, fractions, widths, links, footnote capture and source
state. Caller owns final frame composition, fixups, translated decorations and
block commit. No new public contract, enum variant of Content, phase or mode.

Current figure-frame-red3 precedes code; preserve first two invalid setup
attempts. Require Figure controls plus all subframe/footnote/prefix/Stack/Repeat/
source/width/baseline tests and real native PDF/SVG/state/counter measurements.
A private route enum is exhaustive, not a catchall. Full gates, lineage and
history/resources/JPEG of the final source remain mandatory.

## Readiness intrinsic line settlement, 2026-10-29

Measured figure-geometry-after1/extra-after1 against current64c5e8c0 and exact
figure-frame-build1 inputs, followed by figure-alignment-red1 (four failures,
five controls pass), establish premature page-width alignment. The owning
Figure L0 records values and scope; no general phase/API change is authorized.

Generalize the private FigurePiece carrier to IntrinsicPiece and its adapter
to layout_intrinsic_piece, retaining exact mixed source routes and closed
logical height/tri-state baseline/flow width. A private facts sink also retains
the existing PieceAlignment for the actual piece instead of settling against
the provisional region. Its line/decor facts belong to that occurrence only.
Enable line capture in this intrinsic purpose; Repeat's established purpose
still settles at its own bounded final width, unchanged. Parent collector,
line identities, explicit alignment, running source packets and state remain
LIFO-isolated. Do not recreate Content or execute it twice.

Own a private settle_intrinsic_piece helper applying the retained line scope
once to the supplied final intrinsic width and then existing X/Y local fixup
paths to the same width/height. Restore parent queues; resolved child line
tokens cannot leak into a parent scope. Figure chooses its own piece widths;
textual Align may use this only when it already has a finite positive textual
certificate under active intrinsic line capture and automatic width. All old
ordinary/stack/footnote/repeat adapters retain their existing contracts.

## Rustfmt gate repair (2026-10-29)

The frozen `physical13-validation2/fmt.log` under
`lab/results/construction/diagnostics/readiness-fixes-2026-10-28/`
records a genuine formatting RED. Apply Rust 1.92.0 rustfmt to this consumer
without changing executable tokens, fixtures, expectations or assertions.
Preserve all behavioral obligations above; reseal both directions and rerun
the unchanged gates against the final frozen source. Internal ADR-0127 flow.


## Automatic page width and divider — issue 8525 (2026-10-31)

### Measurement before decision

The independent agent-layout and agent-svg-math reports/receipts under
`lab/results/construction/diagnostics/typst-open-bugs-2026-10-30/` reproduce
infinite SVG viewport/divider geometry and PDF Letter fallback/lost second text.
`01_core/src/compiler/layout/divider.rs:35` multiplies the unbounded-region
mechanism into visible geometry; `page_geometry.rs:62-69` then includes that
shape in intrinsic page extent. `finish.rs:44-48` and intermediate page assembly
consume the contaminated result. Exact received HEAD, dirty stat, UTC and
source/owner bytes are frozen in
`lab/results/construction/diagnostics/auto-page-width-8525-2026-10-31/before.json.gz`.
This is observed geometry/content semantics; it does not establish upstream
intent. The pinned vanilla zero-width divider is a separate defect and is not
an acceptance requirement. A later wider sibling, stale index, lost content,
changed fixed-page line or queue leakage on replay refutes the repair.

### Exclusive correction

Save/take the parent pending divider queue with parent items, start the isolated child with no parent entries, and resolve child dividers before link grouping/item extraction using the finite child region width or its completed material extent plus the existing symmetric margin convention when unbounded. Restore the parent queue LIFO. Ordinary/Stack/Footnote/Repeat/intrinsic source routing and all existing return policies remain unchanged.

Internal correctness correction under ADR-0127 continuous flow: L0 first,
restricted reseal, functional RED then GREEN, build and unfiltered lineage.
Keep public contracts, modes/defaults, compatibility and pipeline phases.
Acceptance requires finite content-derived page/SVG/PDF geometry, a nonzero
divider when body width is positive, and complete text before/after the rule.
Do not copy vanilla zero width or force a paper size. This does not certify
other relative-size shapes, asymmetric margins or general auto-page parity.


## Issue 8583 — approved consumer obligation

Measurement and explicit public/phase approval are bound by before/combined
source archives, baseline.json and approval.json under
`lab/results/construction/diagnostics/issue-8583-2026-10-31/`.
The radius caster at casting.rs:31–39 erases ratios and rect.rs:44–56 drops
side dictionaries. These are language geometry/paint losses.

Migrate only this consumer's ShapeElem literal/destructure: legacy rect_radius
and rect_stroke are None (or explicitly bound/observed for observation). Preserve
all existing values, expectations and other APIs; public addition is approved.
Do not materialize unrelated square/container features.
