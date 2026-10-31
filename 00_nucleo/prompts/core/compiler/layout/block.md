# L0 — block
Hash do Código: dfde15ae

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/block-place-region.toml sha256:ad1c3b199f21c8a84384ff679c5c79a9b9e7cbbcc8d49e2d3f30a288fa210c47
- 00_nucleo/prompts/_nuclei/layout/block-auto-alignment.toml sha256:4a93d5c55bf975b506b6f02ae484a91fdf7449716f9248dea2fcaa6b7b0738b6
- 00_nucleo/prompts/_nuclei/layout/block-inline-region.toml sha256:029c2929c9e294efceb796a5358a6f425f02ae91c519314080f413374f4f4403
- 00_nucleo/prompts/_nuclei/layout/block-margin-priority.toml sha256:67e2de824eb08850828b20d39da52b8d33c59ced2138ddb4e8a2a9113aaa3cf1
- 00_nucleo/prompts/_nuclei/layout/block-spacing-resolution.toml sha256:0177da7d5af56bbce26198ac775e603cec4529be91fb3567373c243543fa85e3
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:7510f3bba27256221d4447db608839af562ac71b4eacc0c042f693b531a95184
- 00_nucleo/prompts/_nuclei/layout/flow-frame-width.toml sha256:0684f5fcab7844d2ac81dd302d9afc7c12595cf5651a9f4c7c91daea8166b3f7
- 00_nucleo/prompts/_nuclei/layout/intrinsic-literal-width.toml sha256:49b2cc837799c63414a2f68dfd25375d2238543b9bd1d502adadfc78a8df6ab1
- 00_nucleo/prompts/_nuclei/layout/intrinsic-piece-alignment.toml sha256:8af39497a64b3dba29c94647c57301b023766aaa012268687421cf1fe6e4ecfb
- 00_nucleo/prompts/_nuclei/layout/paragraph-hanging-indent.toml sha256:12bf17ff10c443c56512c508693160e37e80f6ee925c506912718231b39ced12
- 00_nucleo/prompts/_nuclei/layout/physical-page-occurrences.toml sha256:b33f2e3c6238da01aa48783aecee6a8b6153b4f36ddcdc23edfe78434bae5b42
- 00_nucleo/prompts/_nuclei/layout/single-region-stack.toml sha256:cf78ebaac0d5f3c4adbdc6ca2e6853ccd6b9ca565ec814e9f81b75a546089024
- 00_nucleo/prompts/_nuclei/place-flow-boundary.toml sha256:2e0595153eaf379004fd35fe6a3887a7d62532bec78700833e8b8569e672137e
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:96c5b3ef9be3d171d3ca4c57f30fe826068c568fc17437f83b2b9b5d7372aeff

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/block.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

`01_core/src/compiler/layout/block.rs:21,32,43` was inspected for ownership and behavior, not upstream intent or fresh parity. Language geometry/text/morphology/diagnostics determine acceptance; buffers and state shape are mechanics. Historical `9f2b6e8715237cb086899a42873660fe744622e8` measurements do not certify current `bd5ebec285505889e8ff1d4f68804ab147a699f3`. Full original measurements and partial supersessions remain in `lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/compiler/layout/block.md`.

The bounded Block/Align 14-control matrix in `lab/results/construction/diagnostics/block-align-2026-10-06/baseline-verification.json.gz` separates exterior RTL anchoring from body direction; the intrinsic-literal controls distinguish Literal advance from markup/Newline. `lab/results/construction/diagnostics/correcoes-producao-2026-10-13/math-bilateral.json` refutes restoring a float-reduced height after a body changed page. `lab/results/construction/diagnostics/place-fluxo-2026-10-14/before/results.json.gz` and block-regions-before/results.json.gz isolate causal committed-flow boundaries from Block inset/clip; extra-first/results.json.gz in block-regiao-2026-10-14 refutes resolving nested percentages against mutated global dimensions. Receipts retain sources, exact HEAD/uncommitted tree, UTC/diffstat, commands and binaries.

`block-auto-width-2026-10-14/before/results.json.gz` preserves HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree, UTC/diffstat/status/sources/binaries/argv and historical pin 9f2b6e871. The initial extractor sought blue as #0000ff; blue-geometries-corrected.json reads actual #0074d9 from the same SVGs without rerun. For Shape, Empty, Place-only, Grid and Stack, Tekt backing width is 160 versus pin54,14,14,68,68 respectively. `block.rs:222,416,453` uses available width; `line_commit.rs:753` loses cursor on closure; `sub_frame.rs:354` saves terminal cursor only. Pinned `flow/block.rs:145–166,281–290,339–363` distinguishes Auto from expansion. Geometry is language; witness structure is internal mechanics. Place contaminating width, leaking scopes, lost literal advances or changed explicit-container dimensions refute the hypothesis.

The human approved manifest`d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d` for historical T1–T3; receipt: `lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`. Bilateral Parent evidence and Solid controls remain in the approved contract; structural inspection does not replace them. `lab/results/construction/diagnostics/paint-root-2026-10-14/before/results.json.gz` preserves sources, dirty state, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, UTC, diffstat and binary hashes; it covers containers, Place and Rotate with Solid and Gradient Parent. The before-source/syntax-nodes inventory records this consumer's Group constructions/destructurings before editing: [332]. `layout_types.rs:531–539` does not distinguish roots from technical groups; `helpers.rs:253–269` rebuilds Group and must preserve the field. Box groups background/body; Block leaves body flat except for clipping. Preserving coordinates is a language requirement; envelope form is mechanics. Approval does not close previously measured general Rotate/Box geometry divergences.

paint-root-2026-10-14/block-pages-before/results.json.gz uses the same pre-migration binary as before/results.json.gz and refutes subtracting page margin twice from the local body limit. layout-first.log isolates flat test observers. first-render-delta.json and after-first/results.json.gz preserve a PDF Parent-gradient regression: the legacy collector's per-Group bbox override is not an affine parent reference. This is an open failure, not an accepted baseline; preserve its refutator, the other 35 unchanged renders, and the vanilla quarter-PDF divergence. Prepared Shape transport evidence is in the approved tiling-native/transport package; carrier approval does not close resource integration.

`lab/results/construction/diagnostics/block-auto-alignment-2026-10-14/before/results.json.gz` distinguishes available from final causal inner width and geometric right/center clipping with wider siblings. `lab/results/construction/diagnostics/transformacoes-2026-10-14/` block-height-before/results.json.gz separates explicit/Auto allocation, body end, inset and visual outset; block-boundaries-before/results.json.gz distinguishes physical origin from prepared baseline under prefix/column/page transitions and refutes old P248 outset pagination; block-boundaries-after/results.json.gz and implementation/block-boundary-layout-v2.log refute removing top-edge in caller-established local subframes (P1292 lost markers/float reservation). Fractional-boundary, automatic-spacing and prefix-nesting-controls receipts separate prepared fractional prefixes, lexical par.spacing, and weak/explicit margin strength. All preserve dirty HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, sources/binaries, UTC/status/diffstat and historical pin. Align positive-frame height has its own later correction in `00_nucleo/prompts/core/compiler/layout/placement.md`; this block correction alone did not certify it or larger-font prefixes.

`lab/results/construction/diagnostics/bibliography-structure-2026-10-18/{pagination-block,block-freeze}.json.gz` preserves state/UTC/HEAD/sources/binaries/argv/boxes for candidate `c1c222ae54ab3b67487471326012a7f6c4ed86c1e6bd61b1830ec6a987c2ccea` against oracle `bd5ebec285505889e8ff1d4f68804ab147a699f3` (binary `9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`). It regains two IEEE200×90pt pages but loses continuation inset; corrected 70pt fixture reveals prefix orphaned before body admission. `region_progress.rs:75–76` resets margin and `paragraph_pagination.rs:69,87–105` can legitimately group three lines for widow/orphan. Prefix must accompany actual first line, not an invalid FIRSTWORD-first-page requirement or forced greedy/disabled protection. First prefix paint remains before body paint after admission, never changed eval/show realization order. Orphan prefix/lost inset/double emission/new effect/changed strategy/Block-column-subframe regression refutes correction.

`lab/results/construction/diagnostics/bibliography-structure-2026-10-18/complete-audit.log`, attempt1-artifacts.tar.gz and admission-sources.json.gz preserve HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, dirty state, sources/L0, UTC/argv/binaries and current oracle. IEEE18 places an adjacent punctuation token alone after 2020; the public cache shows no causal space. The whole Block body is Latin-preparation eligible, but the exterior Bibliography refusal leaked suppression into it. Margin-priority measurements compare causal absence/explicit zero and actual positions, rather than coincident numeric values.

`lab/results/construction/diagnostics/bibliography-hanging-2026-10-18/before-sources.json.gz` preserves HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, uncommitted tree, status/diffstat/UTC and 2,099 sources/L0s. before.json.gz contains 92 pairs; extra-before.json.gz 26; boundary-before.json.gz 10; repeat-before.json.gz 4. Oracle commit: `bd5ebec285505889e8ff1d4f68804ab147a699f3`; binary SHA-256: `9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`; initial Tekt: `2cc69a555e9738dc659c06ed6d933853ab981cbb73e93c388a5b43c55afad095`. At width 160pt/margin 8/font12, hanging10 leaves first-line x8 and moves continuations x8→18; negative-5 requires x3. RTL leaves first right edge 152 and moves continuations to 142. Tekt ignores the length. MLA defaults to 18pt (1.5em); ambient set 3em does not replace it, while show-set3em requires 36pt; IEEE/Plain do not acquire hanging indent through that show-set. bibliography.where(style:"mla") does not match in the vanilla probe: preserve selector choice rather than forcing override.

Inline-only Block/Box bodies ignore hanging10; explicit Par inside them and Block with Parbreak apply it. Simple Heading/Raw/List/Quote do not. Partial, zero, repeated and empty-scope declarations close boundaries: FIRST/MIDDLE/LAST at 5.168/27.464/49.76 versus all 5.168 in Tekt. The first lexical probe used literal brackets; it was refuted, corrected to #[...] and retained as extra-before-initial, not evidence of Styled scope. Extreme values produce extractor-clipped paint; missing pdftotext output does not mean Content deletion. A partial 24pt font does not change the shared length's 12pt em, as partial-font records.

Pinned source: `typst-realize/src/lib.rs:1061–1085,1190–1219` distinguishes fragment/Par and interruption; `typst-layout/src/flow/collect.rs:72–134` selects flow; `inline/mod.rs:174,216–221` and `collect.rs:141–147` resolve shape; `inline/line.rs:496–503` positions; `rules.rs:510–523` provides CSL inset/compensation. `typst-library/src/model/bibliography.rs:112–127,367` determines default/precedence. This measures language/geometry, requiring neither vanilla Rust structure nor inferred historical intent. Lost boundaries, run-relative em, double indent, page reset, imposed greedy fallback, lost copies, reexecuted callbacks or control regressions refute closure.

## Exclusive ownership and current contract

Own only this consumer; hub storage/coordination remains in `00_nucleo/prompts/core/compiler/layout/mod.md`, fractional distribution in `00_nucleo/prompts/core/compiler/layout/sequence.md` and `00_nucleo/prompts/core/compiler/layout/sequence/fractional_preflight.md`. Resolve relative dimensions once against the original outer available dimensions/current font, finite nonnegative only. Use private parent inner width/finite height when present; height percentages refer to the real outer region, not remaining cursor space. Fractional height requires the sequence's private allocation: absence is not zero or permission to substitute intrinsic fallback.

Close pending text and resolve above/spacing/below via the pinned block-spacing-resolution nucleus, capturing lexical par.spacing from the exterior chain before body entry. Explicit fields take precedence; vanilla_defaults is only final fallback. Offer pure descendant `collapse_margin(previous, previous_is_auto, incoming, incoming_is_auto)`: equal classes use max, different classes choose explicit. Active-chain entry passes previous strength and causal absence of above/spacing; Heading reuses this helper. Keep had_line/initial-origin branches, fractional reservation, causal end/restoration and outgoing publication. At exit prev_margin_is_parbreak is true only if both below and spacing are absent; explicit zero remains strong. Terminal margins are pending until later content, never extra physical height. Preserve external-chain collapse, while body begins its own baseline/chain at inner origin without inheriting parent's margins/bottom.

Install constrained dimensions/origin/parent bbox temporarily, retaining direction, inset/outset/radius/clip/fill/stroke/overhang. Declared width is total including inset; subtract both inset edges for inner region. Set relative_region_size to resolved or real inherited/physical dimensions minus insets, in LIFO; finite Block then inherited relative then physical region is the reference priority, with genuine infinity preserved. This percentage base is separate from Auto Place's infinite-height marker.

Dispatch/close the body once. Snapshot and isolate prior flow boundary, width and private inline/hanging/planning scopes. Restore width/bbox/line start and parent contexts; restore temporary outer height/region only when the original physical page and column still survive. A transition's new height must not inherit an old float-reduced limit. Local constraints remain active during body emission. On the temporary body height limit, add margin.bottom before capping by saved exterior height so downstream subtraction exposes the exact local allocated end, without exceeding physical bounds.

Single-region vertical allocation is shared by paint_root, backing, cursor/causal end and terminal bottom. Explicit height wins; Auto uses causal body end plus inset, without prepared leading/below. Publish own end in prev_line_baseline/last_block_descent_y, clear equation descent, keep below separate and register the same Place boundary. The next external text installs its own top-edge once. This supersedes cursor-based minimum and below-minus-leading only here. Do not infer size from ink or rerun effects. Outset expands paint/clip only: it neither pads body nor moves root/region nor adds flow advance. Expand backing/mask by edges with origin -left/-top; compensate body rebase to preserve global position. Unbreakable preflight uses declared or measured-plus-inset size without outset, measured under inner width without duplicate effects; request ordinary progress only if whole size fits a region but not the remaining one.

After closed prior text use causal end+above; without line/chain distinguish physical initial origin from prepared baseline. A still-valid witness uses causal end including strong space. An invalid prefix/region witness removes prepared top once only in exterior region or certified fractional boundary; initial physical origin and local subframe without witness retain caller base. A private fractional allocation after its prepared prefix removes prepared top even if zero block_boundary coincides numerically with flush witness. Never combine prior-page start_y with a new-region bottom. Cursor does not compensate this Block end with a generic tail translation.

Explicit-width exterior position uses explicit_line_alignment or chain direction, without forcing an RTL body run. The bounded Auto text certificate requires finite region, no clip/paint/outset and no page transition: if no explicit override, shift the emitted frame and its new decorations together to the exterior RTL edge, preserving Y/body direction/order/margins and avoiding rerender. Intrinsic extent is max(existing visual extent, Literal carrier pos.x+advance); exclude Markup/Newline. Unknown/paginated/geometric bodies remain uncertified.

Auto causal width is emitted flow plus inset; explicit width wins. Isolate width origin/extent around body, restore exterior extent and publish Block allocation through existing record_flow_width_at accumulation. Use that same final width for clip/root/backing, no Place ink or second measurement. Install/restore block_auto_width from resolved_width; in the same region resolve only registered X-fixup paths against final inner bound before clipping/root, without changing causal width. The certified RTL shift moves root origin with content.

Resolve auto-Y Place fixups against causal body height before grouping, including matching decoration-range deltas. Clip at external origin with allocated/causal height and rebase children/paths once. In the same physical region wrap already-composed body in a total-allocation paint root at external origin after RTL adjustment; clip remains technical inside, backing remains earlier sibling outside. Rebase each envelope's children/pending X paths once. Prepare backing stroke in restored external paint context before borrowing insertion buffer; Color fill becomes FramePaint::Solid without widening declarative Block.fill. Preserve overhang/order/positions and existing decoration/metadata translations.

Install private Copy InlineRegion containing horizontal offset relative to exterior page/column start and inner width. Descendant helper reapplies it to current-region geometry, or natural outer geometry if absent. Same region retains saved_line_start/saved_width. After transitions restore parent context and its current-region horizontal geometry, never old origin/limits/height. This context carries no vertical inset/paint/other-phase dimensions.

Save paragraph_planning_suppressed and set false only during this independent Block body, restoring after emission/closure in LIFO. An exterior refusal does not reject the inner paragraph. Preserve direction/style/tracking/backend/configuration/CSL chars and no partial child/Styled preparation within the same refused flow, forced greedy/optimized choice or API/phase/flag. After closing exterior text, end previous hanging context and install local fragment classification (pure inline versus Par/Parbreak/structure), restore after internal closure. Existing inset/continuation/margins remain; ambient hanging does not enter an inline body; private CSL compensation may supply initial shape.

Test observers use recursive affine coordinates with unchanged content/geometry/tolerances, internal-only helper visibility; local tiling cells are paint roots, external technical masks remain soft. No expected-image rewrite may hide a regression.

## Acceptance and explicit incompleteness

Require meaningful RED→GREEN plus full bilateral SVG/PNG/PDF controls for declared/Auto sizes, body types, no/explicit-zero/weak margins, consecutive/nested/fractional cases, prefix/suffix and larger fonts, inset/outset/clipping, local percentages, origins, known/invalid witnesses, page/column changes, floats/P1292 markers and no duplicate emissions. Margin tests use real bodies/causal positions in addition to arithmetic. Independent body preparation tests include adjacent tokens under refused exterior sequence, styles/multiline/restoration, PDF18 and bibliography corpus. Revalidate workspace/core tests, all targets/build, Clippy/Rustdoc/fmt/check, strict/reverse lineage and differential warning inventories under 12GiB/zero swap.

Fragmented Blocks retain their legacy route pending dedicated measurement; per-fragment roots/height/paint/backing/outset, complete metadata/decor integration, fractional sizing, textual/mathematical Auto alignment, Place horizontal and PDF Parent resources retain separate criteria. Do not restore ended-region geometry or mix old start with new bottom; do not certify fragmentation from same-region success. Paint-root metadata alone leaves prepared paint/resources/integrated render gates open; preserve Parent-gradient refutation without tolerance changes. Universal Place requires remaining late inline Box/Transform settlement and Heading/List/Stack/Equation/Columns audits. Public Block dimension-shape proposals require their own approval provenance; internal corrections do not ratify them.

Prefix controls require RED→GREEN same actual prefix/body page, continuation inset and OUTSIDE, Simple/Optimized and prior controls. Measure PDF90/150/250, RTL, prefix-only/formatted/color/regressions; build/suites/doctests/Clippy/Rustdoc/fmt/strict-reverse lineage/differential Tekt under MemoryMax12GiB/High8GiB/swap0/jobs1, without publication. Generic hanging/overrides and multiple bibliographies retain their own open scopes.

This internal parity correction follows ADR-0127 continuous flow: no new public field/signature, external flag/default, phase or realization order. Initial/continuation shape supersedes only the corresponding constant-width limitation. Certified Latin RTL remains subject to the nucleus conditions, without active BidiCursor/tracking or invented BidiSpan; every other prior limit remains. Off-page values/copies require frame/SVG inspection rather than equality of clipped extraction. Do not anticipate the separate R3-T1 scope.

Require RED→GREEN before implementation and source→layout/bilateral controls for relevant signs, boundaries, variants and restoration. Executing a grouped arm does not prove every alternative: closed nominal inline/flow-context classifications are intentional and need class/variant tests. Reseal only after V15/V26 are valid; run tests, build, Clippy/Rustdoc/fmt, reverse lineage and differential Tekt against pin1279707, under 12GiB/High8GiB/swap0/jobs1. No publication.

This internal parity correction under ADR-0127 adds no public contract/default/phase. L0 precedes RED→GREEN/restricted resealing. Causal width is only part of the ongoing block-region/width/transport correction: Auto height/backing, late alignment, fractions, fragmentation, outset and PDF Parent retain their own criteria. Testing a carrier alone does not approve render. Preserve full bilateral controls/refuters, suite/build/doc/fmt/lineage/inventories.


## Approved physical page occurrence protocol

### Evidence before decision

At baseline HEAD `63b7b96b99109b61655b373dc0d248beb7838f86`, the complete source/owner snapshot and applicable ADRs are retained in `lab/results/set-page-header-2026-10-21/agents/core/source-evidence.json.gz` and its supplement. `baseline-parity/receipt.json` there binds current-pin physical query/state/counter/page observations and inert overwritten templates. `ambient-baseline/receipt.json` refutes freezing creation10pt styles: insertion uses actual page20/30pt ambient style. These are language observations; the source plan, packet transcript and rollback representation are mechanics. The exact five facades, opaque types and Paged discovery/realization/source-construction phase were explicitly approved in `lab/results/set-page-header-2026-10-21/review.md` (SHA-256 `cb4a684b483e7f9f6ac3f7eef52277f7ada405033a24e4e66e55811e93c12039`), recorded in `lab/results/set-page-header-implementation-2026-10-21/approval.json`. Only conflicting no-new-phase clauses for this exact protocol are superseded; legacy signatures/defaults and unrelated limits remain.

### Exclusive protocol responsibility

Route the actual body through `BlockBody` under its source plan, preserving canonical root and derived post-child facts on their accepted physical pages. Generated numbering/decorations and all preflight measurement remain unplanned. Keep every existing style/geometry/margin/strategy/restore formula and static delegation; do not use source equality or a wrapper Location to recover a missing route.


## Private prepared full-region flow frames (2026-10-21)

### Measurement before specification

`lab/results/nested-fractions-repeat-2026-10-21/measurement-before-decision.json`
pins exact baseline source/stat/HEAD/UTC and current oracle receipts. The original
repeat-with-parent/child source produces duplicate INNER in Tekt; vanilla emits
INNER once on its own following flow line. `repeat-before` independently confirms
root Repeat interrupts both prefix and suffix paragraphs. Pinned
`typst-layout/src/rules.rs:743–745` lowers Repeat to a Block single layouter;
`flow/collect.rs:240–288` supplies normal collapsed paragraph margins. Existing
`block.rs:148–201,642–691` owns those admission/exit rules. Copying inline glyphs
or forcing an ordinary soft wrap fails the observable flow/height contract.

### Exclusive helper obligations

Provide private descendant helpers `admit_prepared_frame` and
`commit_prepared_frame` for an already once-composed actual local frame. Admission
closes the causal pending text and resolves default Block above/below spacing
using the existing strength-aware collapse rule and geometric-origin distinction.
Return actual flow origin and full usable local width; no body dispatch or
measurement occurs in this helper. Preserve initial/subframe origins, local
Block region and previous causal end. Commit performs ordinary finite-region
admission for the atomic frame, without reexecuting its source, places the local
items through one neutral Group, publishes actual allocated flow width/end,
clears stale inline baseline/extent state and records the pending default weak
below margin. Return its final actual origin so the feature translates its own
local decorations by the same offset. Zero height is real; terminal pending
margin is not part of physical frame height. Nonfinite Repeat bounds are diagnosed
by the Repeat owner before requesting commit; no invented zero or fallback size.

These are private layout helpers, not a new public Block/Repeat entity, synthetic
source wrapper, source phase or default mode. Typed RepeatBody admission remains
owned by Repeat, never BlockBody. Require causal first/prefix/suffix/nested/empty/
finite-overflow/actual-height tests and source/counter/location controls, unchanged
Block margin/region corpus, full tests/build/Clippy/rustdoc/fmt/hash/lineage gates.
This continuous ADR-0127 correction supersedes no unrelated Block debt.


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

Own private record_flow_baseline(Option<f64>): first certified decision wins, preserving authoritative absence and real zero. Isolate the Block body witness, then propagate its actual baseline at the existing frame commit without letting nested children choose the exterior first frame. The private prepared-frame adapter accepts an optional real baseline and translates it by the final atomic origin before publication, even when the frame paints nothing. Preserve heights, margins, finite admission and source-body count.

Require genuine RED→GREEN for empty Repeat baseline, positive copies, earlier Text,
earlier empty frame, nested insets/shift, Shape/empty controls, LIFO and checkpoint
restoration, then the same pinned bilateral SVG/PDF observations without relaxing
1e-8 geometry or semantic membership. Unknown legacy producers retain their
previous fallback unless a measured owning frame explicitly supplies a decision.
This is an ADR-0127 continuous private layout correction, with no public contract,
new product mode/default, compatibility break or eval/layout phase change.
L0 precedes implementation and scoped resealing; full tests/Clippy/rustdoc/fmt,
zero Tekt errors/warnings, paths and forward/reverse lineage remain mandatory.

### Preserve uncertified versus absent baseline

The read-only audit at `lab/results/nested-fractions-repeat-2026-10-21/agents/acceptance/baseline-barrier-audit.json`
and Repeat audit identifies uninstrumented Align/Stack/Transform frame producers.
Their unknown result is not proof that a first frame has no baseline. Preserve
the full Option<Option<f64>> through the private prepared-frame baseline argument,
recording a decision only for outer Some and translating only its actual inner y.
Generic Block likewise propagates only a supplied body decision; outer None remains
uncertified. Never flatten unknown into authoritative absence. The Box owner's
whole-body certificate retains its existing affine-paint fallback on uncertified
producer paths; this is no universal first-frame/flow-baseline claim.

## Independent Block inside a captured intrinsic piece — 2026-10-29

figure-optical-second1 source archive/receipt binds dirty HEAD212aed, exact
UTC/inputs and functional RED for painted80×35 Block inside Figure. Its backing
starts285.637795pt instead of257.637795pt, BODY351.023622pt instead of285.637795pt;
the earlier no-backing BODY control passes. The retained current-pin native
figure controls and explicit geometry define acceptance, not Rust buffers.
block.rs:215–241 positions before final parent width; :318–330 uses the outer
piece scope for child text; :480–510 subsequently groups/inserts backing,
invalidating child item indices. These are measured causal-domain conflicts.

When a real Block enters an active private piece-alignment scope, defer its
exterior horizontal placement and use the causal saved line start. Isolate
its body: explicit inner width uses ordinary child alignment; Auto inner width
creates a fresh private piece scope, settled against actual allocated inner
width before clip/root/backing changes. Resolve child decoration associations
in that same domain, restore the parent scope in LIFO, then publish exactly
one completed typed frame operation for the actual root/backing item range
in a surviving region. Use existing resolved exterior alignment/direction and
original available free width; final parent settlement moves body/backing and
new decoration segments together. Preserve causal width, source plans, margins,
height and all ordinary callers without capture. Fragmentation remains open.


## Issue 8894 — approved Stroke migration

Measurement: the source inventory and previous owner bytes in
`lab/results/construction/diagnostics/issue-8894-2026-10-31/` bind exact
HEAD/dirty diffstat/UTC and each affected source/owner before migration.
`stdlib/layout/stroke.rs:430–434` erases em; geometry.rs:81 carries only f64.
The explicit user approval in approval.json ratifies the public Length field
and occurrence-layout resolution, as specified by
`00_nucleo/prompts/core/entities/geometry.md`. This consumer retains its
exclusive responsibility and all other ratified clauses and controls.

Preserve full Length thickness through eval construction, field access and repr.
Resolve thickness with the effective occurrence font at existing layout consumers
and prepared FrameStroke boundary before scalar geometry/overhang is calculated.
Keep all other values, field-presence, validation/error priority, behavior and
previous approvals. Point-only fixtures retain exactly their numeric values.
The approved public/phase contract is bound in approval.json. RED→GREEN and
full proportional validation plus forward/reverse lineage remain required.


## Resolve region-owned dividers before Block composition — issue 8525

Measurement before decision: candidate1-outputs.json under
lab/results/construction/diagnostics/auto-page-width-8525-2026-10-31 binds the
actual compiled binary, source hashes, dirty stat, UTC and source-level Block
refuter. The root case is finite, but block[FIRST divider LONGER SECOND] panics
at divider.rs:68 because block.rs:510 drains the body into a paint root while
its exact divider index remains in the parent queue. block-before.json freezes
this source/L0 and provenance before amendment. This is a rejected candidate,
not permission to clamp stale indices or infer source identity from paint.

For a surviving single-region Block, after allocated_w is causally known and
before any alignment/clip/root/backing changes item indices, delegate resolution
of only certified pending divider indices at or after items_before. Supply the
same completed inner right boundary plus the existing right margin convention.
Leave earlier parent requests untouched for their final region width; nested
Blocks settle their own requests before enclosing composition. Preserve body
execution count, geometry, source routing, existing fragment behavior, spacing,
styles, allocation and all previous approvals. No public API/phase/default.

The divider owner provides a private index-range resolver; this owner only calls
it at its own composition boundary. Require this actual PDF/SVG RED→GREEN,
parent/child/nested/clip/fixed controls and proportional tests/build/full lint.
Internal ADR-0127 continuous flow with L0/reseal before code.


## Issue 8790 — single-region Block/Stack composition (2026-10-31)

### Measurement before decision

`lab/results/construction/diagnostics/issue-8790-fix-2026-10-31/before.json`
freezes dirty HEAD, UTC, exact diffstat/status and source hashes. The freshly
built combined-tree binary and each PDF/SVG source/result/argv/hash/UTC are bound
in `before/receipt.json`; the independent pinned64c5e8c0 oracle is in
`vanilla/receipt.json`. Literal, unclipped and TTB each produce two Tekt pages;
adding TAIL produces three. Underfill remains one page. Vanilla literal is one
page but overlaps children after exhausting the region; do not reproduce that bug.
`01_core/src/compiler/layout/block.rs:171-191` performs only an initial
unbreakable admission. Its existing private InlineRegion is saved/restored through
Block, checkpoint and subframe as an opaque Copy context. `stack.rs:173-201`
checks only is_sub_frame to select fragmentation; :220-226 anchors reverse
placement to total logical main extent. A Block body retains main-flow semantics,
so classifying it as a generic subframe would also change Shape margin policy.
These are source-measured mechanisms; geometry/page/content/effect preservation
are language obligations, not a requirement to copy vanilla mechanics or defects.
The issue's proposed algorithm and expected positions are hypotheses. Refute them
through co-located tests: five logical22pt frames must remain present, with local
BTT y48,26,4,-18,-40 under fixed70pt; unclipped/TTB must keep22pt separation.

### Exclusive internal correction

Extend only the existing private Copy InlineRegion with inherited unbreakable
intent. Provide a private layout-descendant allows_fragmentation query combining
that context with the existing isolated-subframe constraint. Block preflight
requests a new region only when that outer context permits it. Install own
unbreakable intent OR inherited intent while the body runs, then restore the
previous opaque context through existing LIFO routes. Keep BlockBody source
routing, outer admission, style/margin policy, dimensions, clip/backing/decorations,
flow publication and checkpoint/subframe representation unchanged. No new Layouter
field, public field/signature/trait, recipe replay, phase or product mode.
The Stack owner consumes the query for its actual fragment decision. This bounded
repair does not redesign all other feature-specific pagination or explicit
pagebreak/colbreak/SetPage semantics; those owners retain their existing policy.

### Gates and boundaries

ADR-0127 continuous internal layout/parity correction: L0 and seals before code,
genuine RED→GREEN, all-item/page/clip/TTB/BTT/underfill/prefix/nested/state
controls, actual source PDF/SVG/raster, full build/tests and crystalline-lint.
Preserve previous approvals and every unrelated working-tree change. Add no
ignored test, tolerance relaxation, normalization, hidden callback or wholesale
fallback. Exact provenance and effects-once controls must distinguish this Tekt
fragmentation defect from vanilla overlap. This does not certify arbitrary
multipage Stack children, general table/footnote pagination or explicit breaks
inside unbreakable containers. No universal parity/performance claim follows.


### Auto-nested finite-anchor refuter (before refinement)

`lab/results/construction/diagnostics/issue-8790-fix-2026-10-31/candidate1-rejection.json`
and `candidate1/receipt.json` retain the rejected candidate, exact source/owner
bytes, binary/inputs/hash/UTC/dirty state. The source nested control uses an Auto
inner Block under fixed70 outer Block. It keeps one page/five22pt children but
positions them at global98,76,54,32,10 instead of58,36,14,-8,-30. The inner
Auto block_place_region has infinite height; relying only on that immediate
region loses the nearest actual declared ancestor. This refutes full closure.
Keep an optional Stack placement bottom in the same opaque private Copy
InlineRegion: own declared inner bottom wins, otherwise inherit the outer
context through Auto Blocks. A feature-owned query exposes only that optional
bottom to Stack. Checkpoints copy and subframes suspend/restore the existing
opaque value unchanged. Do not change Block Auto allocation, actual Place
region, percentages, clipping or child logical height to carry this reference.
A nearer explicit120pt inner Block must override outer70; an outer Auto with
no declared ancestor retains natural placement. Add actual Auto-nested RED and
explicit-inner precedence control before refinement; no new public/phase gate.

The optional placement bottom is qualified by the producing physical page and
column. The feature-owned query refuses it after either changes; never transport
a finished region's vertical origin into its successor. Checkpoints still copy
and subframes still suspend/restore the opaque existing InlineRegion. This
qualification adds no page dispatcher or change to explicit-break semantics.
