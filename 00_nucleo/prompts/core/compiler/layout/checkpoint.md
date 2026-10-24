# L0 — compiler/layout/checkpoint
Hash do Código: c755ab36

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/block-place-region.toml sha256:ad1c3b199f21c8a84384ff679c5c79a9b9e7cbbcc8d49e2d3f30a288fa210c47
- 00_nucleo/prompts/_nuclei/layout/bibliography-plan.toml sha256:5c68aee638a285b125399ae9198883a90ae27a0c3a89f68e0765b86fa84b8197
- 00_nucleo/prompts/_nuclei/layout/bibliography-prefix-admission.toml sha256:b556a9f22e29d478ae1afac3e31427aeea3fa58f44a4aa04e46c86b90fb5779c
- 00_nucleo/prompts/_nuclei/layout/block-auto-alignment.toml sha256:4a93d5c55bf975b506b6f02ae484a91fdf7449716f9248dea2fcaa6b7b0738b6
- 00_nucleo/prompts/_nuclei/layout/block-inline-region.toml sha256:029c2929c9e294efceb796a5358a6f425f02ae91c519314080f413374f4f4403
- 00_nucleo/prompts/_nuclei/layout/flow-frame-width.toml sha256:0684f5fcab7844d2ac81dd302d9afc7c12595cf5651a9f4c7c91daea8166b3f7
- 00_nucleo/prompts/_nuclei/layout/installed-line-ascent.toml sha256:993c94c8c186448b35b1ba8a0d84c1aadd95e9db0a09b19394fe95b08db52c7e
- 00_nucleo/prompts/_nuclei/layout/link-frame-scopes.toml sha256:91cb1c097fc694c138acf2d044a3fc46dc7c41ec608ac557f086e1dc41ec0f57
- 00_nucleo/prompts/_nuclei/layout/paragraph-hanging-indent.toml sha256:12bf17ff10c443c56512c508693160e37e80f6ee925c506912718231b39ced12
- 00_nucleo/prompts/_nuclei/layout/physical-page-occurrences.toml sha256:b33f2e3c6238da01aa48783aecee6a8b6153b4f36ddcdc23edfe78434bae5b42
- 00_nucleo/prompts/_nuclei/layout/position-frame-carriers.toml sha256:1f0c020521a9654478f7ba3325005ba621fe5f26b307b56dc49e8c526fceb59f
- 00_nucleo/prompts/_nuclei/layout/subframe-inline-extents.toml sha256:ec315f38498aa37ead497fdcb72cdd7e401cd00752baf5a535c2b5fae69374ed
- 00_nucleo/prompts/_nuclei/layout/text-segment-scope.toml sha256:68203377765579bc29327c7d8fded6f4f831d11682b6b7c9abbf2dbf65b28bad
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:a4a76bdb3fa5069232468af724106217bcd1dc85231c9d2cf54b4da69e241425
- 00_nucleo/prompts/_nuclei/place-coordinate-origin.toml sha256:b9012d07e2d22b7c3984963a6d4e90eedbb9b925280f4136f8db8aaa9459bbc8
- 00_nucleo/prompts/_nuclei/place-flow-boundary.toml sha256:2e0595153eaf379004fd35fe6a3887a7d62532bec78700833e8b8569e672137e
- 00_nucleo/prompts/_nuclei/smartquote-semantics.toml sha256:d92a80d952b2cb79d90b96a75a0387cfde2da24f2fd41327de8ab0e647a97f8c

## Evidence before specification

At HEAD `f7195cdf6` (clean tree,2026-09-30), the extracted block is at `01_core/src/compiler/layout/mod.rs:829-1377`; this establishes ownership boundaries, not a new performance or bilateral-parity measurement. Full historical observations, failed candidates, source coordinates and scope qualifications are preserved in `lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/compiler/layout/checkpoint.md`; the following evidence register identifies the causal support for current obligations.

Paragraph direction: `lab/results/construction/diagnostics/direcao-alinhamento-2026-10-05/initial.json.gz` preserves HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree/UTC/sources/binaries and thirteen probes. RTL/LTR paragraphs containing opposite-direction runs expose anchoring by first run (`cursor.rs:156–162`, `line_commit.rs:210–216`); pinned `flow/collect.rs:129–136` and `inline/mod.rs:189,232` resolve shared direction/alignment. Structural-text preflight/state is mechanics; shared paragraph anchors/run direction are language. Whole-style/parbreak/wrap controls refute missing shared state. `lab/results/construction/diagnostics/bidi-contexto-2026-10-05/{probe.json.gz,initial.json.gz,inventory.json}` refutes isolated 456 789 and records the human-ratified immutable API/three optional fields (manifest 1ae7513e).

`lab/results/construction/diagnostics/block-align-2026-10-06/baseline-verification.json.gz` records 14 controls with source/commands/boxes/binaries and state in before.json. Automatic versus fixed-width Block and nested Align expose lost lexical alignment (`placement.rs:35-46,176-177`, `cursor.rs:154-162`, `block.rs:175-185`); pinned `flow/collect.rs:193`, `flow/block.rs:145-166,285` distinguish line alignment/expanded versus auto regions. Positions/form, not copied mechanics, govern acceptance.

`smartquote-2026-10-06/{before,fields-before,partial-and-html,proposal-controls}.json.gz` records historical pin `9f2b6e8715237cb086899a42873660fe744622e8`, binary hashes/HEAD/working tree/diffstat/UTC/sources/commands/environment/stdout/stderr. Lost semantic fields and premature glyph choice are measured through PDF text and HTML DOM/text/diagnostics. `markup_stream.rs:79–128`, layout smartquote.rs, `stdlib/text/smartquote.rs:112–117`, `observation/elements.rs:242–250`, and pinned markup/inline/smartquote sources support the approved per-flow context. Human confirmation on 2026-10-06 approved double bool→Option<bool>, enabled:Option<bool>, pure helper/context and eval→target-render glyph selection, only for quotes.

Returned inline extents: box-composicao-2026-10-13/core-first.log and before.json.gz preserve the failure after parent state was correctly restored; `measurement.rs:509-512` had read leaked line fields.20pt Box shifted 3pt still requires 17pt. `box-baseline-inicial-2026-10-14/before/results.json.gz` preserves twelve sources/pinned oracle/binaries/dirty tree/HEAD/UTC and before.json.gz source/L0. `boxed.rs:21-38`, `line_commit.rs:421-437`, `sub_frame.rs:292-300` show fictitious initial typography and conflation of zero/absence; actual frame contributions in pinned `inline/line.rs:559-568` support distinguishing None/Some(0). Text/transform/Raw/HTML/nested-Box regression refutes the correction.

Position/link evidence: `box-contexto-2026-10-14/probe.json.gz` and `posicoes-compostas-2026-10-14/before.json.gz` preserve source/binaries/HEAD/tree/UTC for displaced paint with local point(0,0); `mod.rs:1131-1148,1180-1200`, `finish.rs:295-302` and `html_frame.rs:30-199` locate capture/sealing/carriers. Preserve single-pass registration and per-sub-store sealing (ADR-0073/0074), never global vanilla introspection. Coordinate nucleus preserves absolute Grid and local Place/Box through page transformation without double origin. `lab/results/construction/diagnostics/links-paginacao-2026-10-14.md`, `posicoes-compostas-2026-10-14/automatic-boundary-probes` and `links-captura-2026-10-14/before.json.gz` preserve state/binaries/pin and stale-index failures (`references.rs:270`, `link.rs:22-32`, `outline.rs:28-50`). Clamping old indices proves no prefix/order/fragment/destination/geometry preservation.

Place/local-block evidence: `place-origem-local-2026-10-14/{before,extra-before}/results.json.gz` and `lab/results/construction/diagnostics/place-fluxo-2026-10-14/{before,block-regions-before}/results.json.gz` preserve HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, dirty tree/diffstat/UTC/sources/binaries/argv and historical pin. `placement.rs:463,505,533`, `place.rs:91–131`, `line_commit.rs:721–733`, `block.rs:90,220,275,290,465` expose local origin, committed-flow anchor and Block-region defects. Pinned flow/distribute.rs:703–715 and flow/block.rs:66–89,223–234,278–280 distinguish axis choice/physical region/inset/clip. Text/VAlign/scope/fixup/checkpoint, weak-margin/overlap/page/rollback and nested/text/auto/clip/sibling controls are independent refuters.

Causal width: `block-auto-width-2026-10-14/before/results.json.gz` preserves state/HEAD/UTC/status/diffstat/sources/binaries/argv; blue-geometries-corrected.json corrects the original color read without rerun. `block.rs:222,416,453`, `line_commit.rs:753`, `sub_frame.rs:354` and pinned flow/block.rs:145–166,281–290,339–363 distinguish Auto width from available width. `lab/results/construction/diagnostics/block-auto-alignment-2026-10-14/before/results.json.gz` preserves corresponding state/binary hashes and lost clipped right/center/link geometry; `placement.rs:241` prematurely finalizes finite avail_w while `block.rs:287–303` learns causal width after body. Inset/larger siblings/Link/fixed-or-nested Block/subframe/rollback refute late-width fixups.

`transformacoes-2026-10-14/after-first/results.json.gz` preserves candidate hashes: percentage Move uses sentinel height, and A/TEST/B remain inline instead of blocks. Images were inspected without realignment; pinned `typst-layout/src/rules.rs:725–739` uses BlockElem::single_layouter for four language functions. This is block semantics, not generic Transform enum convention; correction stays within approved X4. Explicit/nested-region/spacing/body/rollback regressions refute it.

`lab/results/construction/diagnostics/transformacoes-2026-10-14/prefix-nesting-before/results.json.gz` preserves same HEAD/dirty state/status/stat/UTC/sources/binaries/historical pin. `sequence/fractional_preflight.rs:151–171` refuses multiline, `word_layout.rs:49` closes under local style, `line_commit.rs:398–480` uses it for floor/leading/expected top, and `sequence.rs:239` refuses partial-segment closure. Pinned flow/collect.rs:129–145,179–214 resolves shared trunk/spacing/leading; each frame supplies its own geometry. These are language text/geometry/pagination, without copied buffers/algorithms/bytes or inferred intent.

Bibliography evidence: `lab/results/construction/diagnostics/bibliography-structure-2026-10-18/{before.json.gz,before-sources.json.gz,pagination-block.json.gz,block-freeze.json.gz}` preserves HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, dirty tree/UTC/status/diffstat/sources/argv/text/boxes/binaries and current oracle `bd5ebec285505889e8ff1d4f68804ab147a699f3` (tree `cac836c07e4dec2a89219370428292393a3216b5`, binary `9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`). Lost entry structure, continuation inset and prematurely painted prefix are distinct defects; `region_progress.rs:75–76` drops inset, `paragraph_pagination.rs:69,87–105` can legitimately admit body later for widow/orphan. Prefix follows actual first-line page, not an imposed FIRSTWORD-first-page assertion or greedy/widow-orphan change. `lab/results/construction/diagnostics/bibliography-hanging-2026-10-18/{before-sources,before,extra-before,boundary-before,repeat-before}.json.gz` preserves hanging signs/lexical-boundary/MLA-default/show-set probes, corrected initial lexical probe and clipping qualifications. Source references and exact measurements remain in the archived owner and diagnostics; historical pins alone do not certify the current target.

## Exclusive ownership and complete checkpoint contract

Layer L1. Sole productive consumer: `01_core/src/compiler/layout/checkpoint.rs`. Own representation/capture/restore of layout checkpoints with complete causal rollback. Preserve public contract/default behavior/pipeline phase/compatibility/L1 purity/static dispatch. Capture existing slots/buffers/cursors/prefixes as well as every addition below; restore must never recompute from paint/text or replay content.

- Capture/restore both private paragraph/run direction slots with styles. Never derive restored direction from items. Capture explicit_line_alignment alongside them; rollback must preserve lexical alignment without leakage.
- Preserve private bidi Cursor with Arc input, exact current span and bidi_scope_active guard. A rejected integral scope cannot let descendants recertify a partial subset. Checkpoint retains cursor/guard; subframes suspend/restore both. Full paragraph preparation, visit-order offsets, Sequence/Par scope and nested Sequence inheritance, Text word/Literal-space subspans, weak-Space consumption, Linebreak boundary and exact-untransformed Word emission remain boundary criteria under `00_nucleo/prompts/core/compiler/layout/bidi_origin.md`.
- Clone/restore the entire SmartQuoteContext, including stack/predecessor, replacing old flags. Do not recompute committed prefix. Rejected/replayed suffix returns to identical context and emits identical glyphs once; retain all other causal state.
- Capture last_sub_frame_inline_extents with last_sub_frame_width/bottom. Do not calculate baseline or change transaction policy. Capture line_assumed_ascent as Option<f64>, keeping None distinct from Some(0), with no conversion; all remaining rollback stays unchanged.
- Capture private append-only position-list length and truncate on rollback with existing frames/runtime. Carrier IDs belong to that registry: aborted capture cannot outlive its frame or win sealing. Capture/restore active link scopes and destination-registry length; truncate only aborted captures and preserve surviving registry indices.
- Capture sub_frame_origin_y with is_sub_frame, preserving committed prefix/callback ordinal and other fields. Capture flow_placement_end and block_place_region without changing other suffix data.
- Clone/restore private decoration ranges associated with auto-Place Y-fixup index. The hub stores; this owner checkpoints; subframe/Block isolate/restore; region transition clears; placement registers only this occurrence's segments; Block applies geometry's same delta before scope exit. These are canonical boundaries with `00_nucleo/prompts/core/compiler/layout/mod.md`, `00_nucleo/prompts/core/compiler/layout/sub_frame.md`, `00_nucleo/prompts/core/compiler/layout/block.md`, `00_nucleo/prompts/core/compiler/layout/placement.md` and `00_nucleo/prompts/core/compiler/layout/region_progress.md`, each owning its own operation.
- Capture flow_frame_extent, last_sub_frame_flow_width and block_auto_width together with causal flow state, including rejected speculative attempts. Capture relative_region_size with subframe context; rollback must not inherit discarded attempt dimensions.
- Clone immutable text_segment_chain and capture geometric_text_lines. Preserve length-based buffer discipline and every other field.
- Capture private bibliography plan by clone/move of the same Option<Arc> beside existing cache. Rollback/speculative measurement preserves discriminant and never rebuilds plan; other buffers keep current discipline.
- Clone/move pending_bib_prefix and Copy block_inline_region with other contexts. Restore preserves consumption/absence and exact horizontal region, without rendering or text-based recalculation.
- Capture/restore hanging State and pending_bib_escape in every path with existing fields. Never recalculate on replay or rerun body.

## Approval, acceptance and explicit limits

These private corrections follow continuous-flow ADR-0127: L0 precedes RED→GREEN and restricted resealing, without new public entity/trait/signature/flag/default/compatibility/phase or unsafe. Quote-specific public/phase changes are already approved only within their ratified scope; semantic identity/fields remain in their canonical entity/eval owners, paged glyphs in `00_nucleo/prompts/core/compiler/layout/smartquote.md`, and HTML remains semantic with its own escaping (ADR-0128). The single pure L1 quote helper has per-flow/compilation state, no I/O/globals/vtable/property registry/copied memoization, no new modes/watch API/math phase. Ratification supersedes conflicting history only for quotes.

Direction certificate covers structural Text/Empty/Space/SpaceFromNewline/Linebreak/Sequence/Styled/Strong/Emph; unknown units keep prior fallback. Bidi's initial certified subset is incremental Simple without generic cells/columns/subframes/tracking/hyphenation/justification/smallcaps, Text newline/tab/segmentation; refused regions retain None. RTL-base/RTL-run/opposite-override certified subset uses incremental without changing permitted breaks; pure LTR retains prior planning and explicit Optimized is not overridden. Do not promise full mixed bidi, explicit alignment, HTML or every paragraph form; width/tracking formulas remain. Direction slots/helpers send no metadata to L3.

Keep L1 suite/build/crystalline-lint green; extraction alone establishes no performance gain. Direction tests cover RTL paragraph/LTR line and converse, integral style, parbreak reset, checkpoints/subframes and bilateral CLI text/positions. Align tests cover nested Align, explicit-width Block, post-Align scoping, auto dimensions/ordinary text/prior corpus without redefining negatives or claiming time gain.

Inline-extents causal test P1491 stays green with unchanged 17pt expectation and checkpoint/layout suite. Initial-ascent tests cover zero, geometry and full suite, never altered regression expectations. Position tests cover coordinates/no fictitious paint/height/identity/order/visual clones/rollback, HTML/geometry/context/query/full suite. Link controls cover breaks before/within body, prefix/suffix/regions/destinations/bounds/rich nesting/columns/deferred/context/rollback and no private output boundaries; revalidate no-break controls/build/lineage.

Place origin and postcommitted-flow anchors remain distinct. Certified producers cover line/shape/Block/Align/Grid/Raw/VSpace; invalid witness retains legacy fallback and pending status. Local surviving Block region/clipping does not close Block fragmentation/dimensions/outset/metadata-decorator integration, late inline Box/Transform baseline or uncertified Heading/List/Stack/Equation/Columns. LIFO save/restore is mandatory for authorized private causal state. Revalidate full HTML/context/PDF/SVG/suite/build/doc/fmt/check/lineage/warning inventory and bilateral refuters. Historical tiling-pending notes are superseded only by separately recorded approval, never by this checkpoint work.

Causal-width and auto-alignment refinements preserve complete bilateral controls and refuters; they do not complete Auto height/backing, text/math alignment, deferred exterior decoration, horizontal Place, fractions, fragmentation, outset or PDF Parent. Relative-region correction preserves body/spacing/explicit/nested regions and rollback within approved transformation scope. Multiline extension applies only to positively certified textual prefixes, retaining prior scalar certificates; it does not certify opaque content, bidi/floats or universal pagination. Require three causal cases plus fonts/colors/margins/columns/restoration/checkpoints/subframes/full suite/lineage.

Bibliography plan acceptance covers functional relative-gutter/interentry-margin RED, public legacy Content/signatures, prefix-free CSL, Hanging/Plain discriminants, empty/order/citations/rollback and bilateral list boxes/text while preserving indent/citation-grouping divergences. Private plan transport does not certify title margins, complete CSL, per-occurrence attribution/multiple bibliographies or all hanging composition/overrides. Do not impose greedy fallback to mask initial-advance support. Prefix/body share actual admitted page and preserve continuation inset/OUTSIDE, Simple/Optimized, PDF90/150/250, RTL, prefix-only/formatted/color and prior controls, without repeated eval/show or new effects. Hanging State is limited to its pinned initial/continuation contract, including signs/lexical boundaries/restoration; Latin RTL needs nucleus conditions without active BidiCursor/tracking or invented BidiSpan. Off-page paint requires frame/SVG inspection, not clipped-extraction equality; do not anticipate separate R3-T1 scope.

For these bibliography/hanging gates, L0 first and RED→GREEN, V15/V26 before resealing, build/workspace/doctests/strict Clippy/Rustdoc/fmt/strict-reverse V1/V5/V15/V26/differential Tekt (hanging pin1279707), with SARIF no new error/warning. Grouped-arm execution never proves every class/variant; test each relevant nominal class. Heavy commands sequential under MemoryMax12GiB/High8GiB/swap0/jobs1/debug0/incremental0. No commit/push/release/publication. Preserve all unresolved scopes rather than closing the overall objective through carrier-only tests.


## Approved Figure/Grid occurrence counter correction

### Evidence before decision

Baseline HEAD `731f0ffdf4c7c152f590551e72ff63f7bd85ef85`; the reviewed current-pin measurements and exact source anchors are retained in `lab/results/v16-corrections-2026-10-19/figure-kind/review.md`, `agents/counter/design.md` and `agents/counter/counter-probes.json`. In that evidence, captionless/numbered Figures count; manual Set/Func actions make caption ordinal indexing incorrect; typed element kinds and custom names require distinct keys. These are language values/errors, not Rust or rendering-byte equality. The human explicitly approved the complete Figure/Grid contracts, phases and compatibility changes in the current implementation session. This approval satisfies ADR-0127 for this exact scope. Historical conflicting caption-only or productive string-projection clauses are superseded only as described here; unrelated contracts remain.

### Exclusive responsibility

Checkpoint copy/restore retains synthetic-only figure_progress exactly; prepared productive Figure layout never reads/increments it. Existing page/layout/measurement rollback remains intact.

### Acceptance and limits

L0 precedes functional RED and implementation. Require independent fixed expectations for complete demanded histories, captionless/hidden/None demand controls, manual actions before/after requested Locations, empty Set state, typed aliases/homonyms/custom names, actual Paged caption/reference text, both-target explicit reads, bundle metadata rejection before effects and candidate diagnostic retention. No callback count as parity oracle. Preserve the independent filtered-read/replay/Sink algorithms and HTML implicit-rendering debt. Require exhaustive real-enum extension probes at new owned decisions, all twelve established gates, full differential lint, canonical owner paths, V1/V5/V15/V26 and reverse lineage with scoped resealing. No lint suppression, contract weakening, new defaults, memoization or public changes beyond the reviewed approval. Record HEAD/dirty file list, UTC, argv and executable identity for measurements.

Checkpoint also preserves the private scoped transported_root_location used solely through a Located root's transparent source/style/recipe transport; rollback neither allocates a replacement identity nor passes the parent's identity to an ordinary child.


## Approved physical page occurrence protocol

### Evidence before decision

At baseline HEAD `63b7b96b99109b61655b373dc0d248beb7838f86`, the complete source/owner snapshot and applicable ADRs are retained in `lab/results/set-page-header-2026-10-21/agents/core/source-evidence.json.gz` and its supplement. `baseline-parity/receipt.json` there binds current-pin physical query/state/counter/page observations and inert overwritten templates. `ambient-baseline/receipt.json` refutes freezing creation10pt styles: insertion uses actual page20/30pt ambient style. These are language observations; the source plan, packet transcript and rollback representation are mechanics. The exact five facades, opaque types and Paged discovery/realization/source-construction phase were explicitly approved in `lab/results/set-page-header-2026-10-21/review.md` (SHA-256 `cb4a684b483e7f9f6ac3f7eef52277f7ada405033a24e4e66e55811e93c12039`), recorded in `lab/results/set-page-header-implementation-2026-10-21/approval.json`. Only conflicting no-new-phase clauses for this exact protocol are superseded; legacy signatures/defaults and unrelated limits remain.

### Exclusive protocol responsibility

Capture/restore the complete optional planned state with existing causal state: current source authority/cursor/typed child route, source-phase stack, selected slot bindings, page-effective environment, accepted packet events/page associations, requests and all consumed-resolution flags. Rollback must reverse mutations to existing entries as well as truncate appended tails. Preserve accepted prefixes and deferred packet ownership; restoring only a trace length is insufficient. Measurement checkpoints cannot publish events or consume physical resolutions. No change to public checkpoint/entity APIs is authorized.

### Acceptance and preserved boundaries

Require semantic RED→GREEN for repeated physical header/footer/four-slot order, zero-paint updates, source-derived parents, overwritten inert templates, aliases and physical pages, effective20/30pt ambient plus explicit inner styles, nested PageRun restoration, Context reads, and exact duplicate-label/source diagnostics. Cover speculative suffix rollback, deferred floats/notes, subframes, measurement isolation, complete body/slot coverage, stale/mixed authority and math/par.line coexistence. Do not weaken existing tests, callback budgets, per-store sealing, source morphology, HTML isolation or old public compatibility. No generic painted-frame semantic reconstruction, source re-evaluation, second arena/Context solver, direct Func marginal mode, new default/flag or extra public API is authorized. Every changed descendant retains its own current 1:1 owner and explicit typed route; finite source inventories are not blanket traversal permission. Exhaustive new decisions need actual future-variant barriers, and the complete lineage/Nucleus, build/test/lint/Rustdoc/fmt/Clippy gates remain required without suppressions.

For planned candidates, retain the complete pending footnote body queue and its parallel source cursors when later flushing can drain existing entries; a length-only rollback cannot restore a drained body. The unplanned legacy queue discipline remains unchanged.


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

Clone and restore the entire optional deferred Note frame queue, reserved
height and optional capture state together with existing numbered-body and typed
source-cursor queues. Frames and certified cuts may mutate before queue length
changes: retaining only a length is insufficient. Rollback restores exact owned
items/markers/cuts/decorations/orphan alignments and capture atomic depth/origin;
it does not recapture a Note, replay callbacks or reconstruct membership from
paint. Preserve every prior checkpoint field and boundary.

Validate actual split Note text/page/counter/query/identity and no-fit progress,
plus existing no-break notes, columns, subframes, rollback, floats and running
content controls. Preserve strict trace coverage/foreign authority, warnings,
canonical owner paths and forward/reverse lineage; no suppression or fixture
weakening. These transport declarations alone do not certify general pagination.

### Aligned unplanned Note queues after rejected emission

`footnote_flush.rs` drains and replaces the numbered-body queue together with its
source-cursor and optional captured-frame queues. The current checkpoint stores a
complete frame snapshot even for an unplanned Layouter, while its numbered-body
snapshot was conditional on a PageRunning pass. After actual flush consumes an
old body, truncating to an old length cannot reconstruct that source. This is a
source-proven queue transaction defect, not permission to fabricate a new body.

Capture a full numbered-body snapshot whenever capturing its corresponding frame
and cursor queues, including unplanned layout. Cloning an empty queue allocates no
body storage; nonempty snapshots retain only the bounded current Note source
queue and existing Content clones. Restore that exact queue with its paired
frames, never a length-only approximation after destructive prefix consumption.

Acceptance exercises an actual unplanned Note: emit its body speculatively,
restore the existing flow checkpoint, then finish. Its final visible Note text
must occur once and match a control that performed no rejected flush. The test
observes rendered content, not a field-by-field copy of the implementation.


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

Capture and restore the complete optional immutable inline allocation tree
with Arc clone, preserving absence and assigned zero exactly. A rejected suffix
must restore consumed/absent assignment state without source replay beyond the
existing transaction protocol or reconstruction from paint. This owner transports
the state; it neither prepares widths nor changes acceptance policy.

This is an internal layout parity correction under ADR-0127 continuous flow:
no public entity field, trait method, signature, flag/default, compatibility or
eval/realize/layout phase change. L0 precedes meaningful RED→GREEN and scoped
resealing. Require actual nested/weighted/zero-width/source-once/decoration,
subframe-restoration and rejected-suffix tests, all prior geometry controls,
bilateral parity, full workspace tests, Clippy/rustdoc/fmt, hashes and canonical
forward/reverse lineage, with every warning retained.


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

Capture and restore both private tri-state baseline witnesses exactly, including absent, authoritative no-baseline and present-zero. A rejected speculative suffix cannot consume or overwrite its parent or completed-child decision.

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

Capture and restore the complete optional private piece-alignment scope, including pending actual line, item ranges, decoration associations and finalized line records, by clone. Do not merely truncate a mutable prefix or retain speculative geometry.

Require genuine declared-metric RED→GREEN for nested center/right, unequal-width multiline and decorated multiline; preserve direct Shape/natural-text/inner-left controls, source effects once, rollback, complete bilateral PDF/SVG geometry at the unchanged bound, all tests and both lineage directions. No warning suppression, ignored case, threshold relaxation, fake source identity or public/phase expansion.


## Weak pagebreak parity correction — 2026-10-23

### Evidence before decision

`lab/results/weak-pagebreak-2026-10-23/baseline.json` binds the uncommitted source/L0/config state to HEAD, dirty stat and UTC; `before/receipt.json` binds both verified executables, inputs, commands and observations. `pagebreak.rs:21-58` ignores weak and always requests transitions; `finish.rs:103-106` requires the newly opened page unless it is a PageRun boundary. `page_run.rs:126-131` unconditionally closes its body. The pinned source `lab/typst-original/crates/typst-library/src/layout/page.rs:554-557` explicitly defines skipping weak breaks on empty pages; its tests `tests/suite/layout/pagebreak.typ:15-24,61-76` document weak/material distinctions. Initial, repeated, terminal and strong/weak controls plus parity and explicit PageRun probes distinguish language intent from mechanics. Page membership, content and geometry are observables; the private successor flag is a chosen mechanism. The inference that an optional successor suffices is refuted by lost notes/placed content, changed strong/parity empties, or leaked rollback state.

### Decision and exclusive responsibility

Capture and restore the private weak-successor state alongside PageRun boundary state in full flow-suffix checkpoints. Speculative weak transitions must not leak into replay or terminal page commitment.

This is an ADR-0127 continuous-flow language-parity correction: L0 first, reseal, RED→GREEN and complete revalidation. No public contract, default mode, pipeline phase or compatibility change is authorized. Existing nucleus obligations and unrelated limits remain binding.


## Refutation: unpainted material and required successors

### Evidence before decision

`lab/results/weak-pagebreak-2026-10-23/edge-after/receipt.json` and `unpainted-before/receipt.json` bind the first candidate executable to UTC/inputs/commands; its build and source state are in `validation.json` and `final-source.json.gz`. These controls refute painting alone as page materiality: hidden text, zero/fractional horizontal spacing and zero/weak/strong vertical spacing can require a page without paint; empty/metadata-only Hide does not. `h_space.rs:27-45` advances or queues spacing without an item, `v_space.rs:20-42` records advancement, and `hide.rs:20-28` removes only painted output. `pagebreak.rs:42-44` also overwrites an empty strong successor's requiredness during weak parity adjustment, dropping a required final page. These are language-visible page obligations, not a request to copy upstream collection mechanics. The earlier painting-only hypothesis was rejected, not qualified by changing the oracle or deleting its cases.

### Exclusive correction

Capture and restore page_has_unpainted_material with all existing weak-successor state, preserving speculative nonpainting admissions and their rollback.

L0 precedes code under continuous-flow ADR-0127. No public field, signature, trait, phase, default or compatibility change. Validate the refuters and retain all previous note/float/Context/column/strong/PageRun controls.


## R1 — retain the formatted entry marker (2026-10-23)

Measurement before decision: `lab/results/footnote-markers-r1-2026-10-23/baseline.json` and `before/receipt.json` bind exact HEAD, uncommitted changed-file list, source archive, UTC, executable and fixed oracle identities. Successful note and note-successor probes expose generated `[1]` instead of `1`; custom exposes decimal bracket footers despite correctly formatted inline `a` and `(ii)`. Source `footnote.rs:45–48` formats the call; `footnote_fragments.rs:286` and `footnote_flush.rs:156` independently synthesize brackets. Oracle `typst-library/src/model/footnote.rs:305–338` realizes the original note numbering as superscript in its entry. This is visible language morphology; internal queue representation is mechanics.

Classification: ADR-0127 continuous parity correction, private layout metadata only, no public contract, default mode, pipeline phase or compatibility change.

Exclusive obligation: Snapshot and restore the full private pending-note tuple including generated entry prefix, atomically with the existing body/cursor/frame state. Destructive flush rollback must retain custom marker identity.

Acceptance: tests first must demonstrate actual marker failure before implementation and pass afterward. Cover default, custom and intentionally bracketed numbering, literal body brackets, captured and uncaptured emission, and checkpoint preservation. Reuse generated Content rather than export text replacement. Preserve the existing body styles and note-area model, source admissions, diagnostic behavior and unrelated tests. Full build, tests, lineage and linter gates are required. This does not certify broader entry typography, column placement, nested-note body completeness, call/body migration or oversized-note continuation; measured prior limits remain open.


## R8 horizontal superscript compensation is paint-only

Source evidence precedes this correction: pinned inline/line.rs:458–484 applies
script compensation by Frame::translate without changing advance. The approved
metrics tuple supplies this offset; current text.rs adds it to cursor_x, which
also shifts following text. The existing real font fixtures have zero X offset,
so injected nonzero provider controls are necessary refuters, not font-name cases.
Keep logical advance, wrapping, justification and source identity unchanged;
translate only the text/highlight paint of the actual script run. This is private
formula/state correction, no new public field/API/phase (ADR-0127 continuous).
Capture and restore private text_horizontal_offset with the active style so speculative rollback cannot leak paint compensation.
Require nonzero injected offset and following-text/rollback/subframe RED→GREEN, actual-font controls and complete validation.


## R9 — source page-column admission and drain

Evidence before decision: candidate2 and candidate3 receipts under lab/results/construction/diagnostics/long-footnotes-r9-2026-10-24 preserve exact inputs, dirty stat, UTC and binaries. The first passes40/41; the wider68-case matrix refutes retaining context only at segmented exit: page-columns follows columns/flow.rs:119–126, which closes active columns before terminal draining. cols2-h80-n2 also measures Alpha atx20 versus155.2pt; cols2-h100-n3 unnecessarily defers a fitting whole-three entry. Pinned flow/compose.rs:650–777 measures admission at the source carrier and retains future column regions. These physical positions/pages are language, private state/dispatch is mechanics. This supersedes the refuted column follow-up saved in l0-column-followup.json, whose source changes were withdrawn without changing received code.

Classification: private parity/correctness, ADR-0127 continuous flow; no public field/signature/default/phase/compatibility change.

Exclusive owning obligation: Capture and restore the private page_column_notes boolean with the existing page_columns scope on rollback. No source fact, callback, queue or other snapshot field changes.

Require actual carrier-column RED→GREEN before implementation, the wider68-case PDF/SVG contract at unchanged0.001pt, original R8 and A–H/R1–R7 controls, unchanged effects, resources, full workspace validation, both lineage gates and preservation audits.


## R10 — source superscript runs and mandatory breaks

Measurement before decision: `lab/results/construction/diagnostics/superscript-r10-2026-10-24/{baseline.json,before/receipt.json,baseline-audit.json,approved-metrics.md,approval.json}` binds received R9 source/UTC/dirty stat and pinned bd5ebec2 binaries. Six of fifteen controls fail the unchanged complete physical words/pages and0.001pt gate: three literal-newline cases, Nimbus hard break, and Nimbus/Liberation mixed fallback. Pinned inline/collect.rs:191-196 includes mandatory breaks in source script text; inline/shaping.rs:970-1004,1090-1107,1167-1217 retains outer size, actual face/run and full causal coverage. This is language geometry and source preservation, not a byte/algorithm obligation. Font-specific synthetic metrics cannot be guessed from the primary face after fallback. The exact additive FontMetrics::text_superscript_runs contract was approved at SHA65d01600110f9bffea1eaff0ca349f737ab426e384fb891e88d8f6504cf911b9; every other public method/field/default/phase stays unchanged.

Classification: approved public method plus private parity integration under ADR-0127. No font I/O, callbacks, dynamic evaluation or font-name coefficients in L1. Require real module RED→GREEN before runtime code, default/override/borrowed contract tests, source partition and restoration, native/synthetic/mixed-face/warm-cold/explicit-size refuters; all original controls, R8/R9 and A–H/R1–R7 replays, PDF/SVG0.001pt and full workspace/lineage/resources validation. Preserve all historical observations and unrelated received input bytes.

### Exclusive owning obligation

Save/restore the private superscript_source_break boolean with the existing actual text paint offset and typography state. No rollback or retry may leak another lexical script's native/synthetic decision.


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

Capture/restore the additional private literal_script_break_pending flag as part of current-flow state; preserve the existing script-break and offset state.

Require module RED→GREEN, expanded54-source matrix without changing thresholds or source membership, original15/R8/R9, A–H/R1–R7, full workspace/Clippy/fmt/Rustdoc/forward-reverse lineage, resource and preservation gates. Retain every failed candidate and its actual source state.
