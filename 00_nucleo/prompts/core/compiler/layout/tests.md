# L0 — tests
Hash do Código: e4051bcb

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/block-place-region.toml sha256:ad1c3b199f21c8a84384ff679c5c79a9b9e7cbbcc8d49e2d3f30a288fa210c47
- 00_nucleo/prompts/_nuclei/layout/block-auto-alignment.toml sha256:4a93d5c55bf975b506b6f02ae484a91fdf7449716f9248dea2fcaa6b7b0738b6
- 00_nucleo/prompts/_nuclei/layout/block-spacing-resolution.toml sha256:0177da7d5af56bbce26198ac775e603cec4529be91fb3567373c243543fa85e3
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:3921c80230cfcc66a807429fbf8bc07ee090bae4988c351b6524023038bab84b
- 00_nucleo/prompts/_nuclei/layout/flow-frame-width.toml sha256:0684f5fcab7844d2ac81dd302d9afc7c12595cf5651a9f4c7c91daea8166b3f7
- 00_nucleo/prompts/_nuclei/layout/outline-inline-budget.toml sha256:7995dc5e028bf26ac3bd89a9dd751f8522530af98c627211825f9b55e551a9c1
- 00_nucleo/prompts/_nuclei/layout/page-number-baseline.toml sha256:0c492a09d2660de765df65c22e703cc8c3e63cc663ee4d9ac2dd612afc80dfa3
- 00_nucleo/prompts/_nuclei/layout/raw-block-boundary.toml sha256:a3772778f9e542de87823509c8eac57904fc73fea9526cebe6052833a69ef806
- 00_nucleo/prompts/_nuclei/layout/raw-defaults.toml sha256:24f0b69fa1db115ff13a786121c80cb7be3abfa2028a9a473075374a6815cfd4
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:395a968c2fe49429b03b369cd5047df513132bfef6511ec21af466670ec2e147
- 00_nucleo/prompts/_nuclei/layout/text-segment-scope.toml sha256:68203377765579bc29327c7d8fded6f4f831d11682b6b7c9abbf2dbf65b28bad
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:88391e330334a2221170109c82b5dbf1fb60c4d2f6e2660e732df24b67554d9f
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:a4a76bdb3fa5069232468af724106217bcd1dc85231c9d2cf54b4da69e241425
- 00_nucleo/prompts/_nuclei/paragraph-composition.toml sha256:802ff8f08c2cc2e7a81987f63f980ddc1254d4ff4f8c972804d6bb3e84b4c484
- 00_nucleo/prompts/_nuclei/place-coordinate-origin.toml sha256:b9012d07e2d22b7c3984963a6d4e90eedbb9b925280f4136f8db8aaa9459bbc8
- 00_nucleo/prompts/_nuclei/place-flow-boundary.toml sha256:2e0595153eaf379004fd35fe6a3887a7d62532bec78700833e8b8569e672137e
- 00_nucleo/prompts/_nuclei/smartquote-semantics.toml sha256:d92a80d952b2cb79d90b96a75a0387cfde2da24f2fd41327de8ab0e647a97f8c
- 00_nucleo/prompts/_nuclei/table/cell-occurrence.toml sha256:bba33b336e651ccd6d41f88b0e16449932d0a986de1ee2cc3c1fdf5c8fb8c6dc
- 00_nucleo/prompts/_nuclei/table/cell-presence.toml sha256:f7d3050154813e1ea46c26e235383ae337781a4b28d10ef3248f6eaec96ce221
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:2b9e828f5e0d958f4328d98a35320ba2ea321997e541e6ab90520d2e8811e7bf
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:96c5b3ef9be3d171d3ca4c57f30fe826068c568fc17437f83b2b9b5d7372aeff


**Layer:** L1, test-only
**Exclusive consumer:** `01_core/src/compiler/layout/tests.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

`01_core/src/compiler/layout/tests.rs:31,38,48` and each affected observer were inspected. This consumer owns test-only metrics/worlds/affine observers/regressions, not productive behavior. Language text/morphology/geometry/styles/pages/diagnostics are acceptance; wrapper/buffer shape is mechanics. Historical probes against 9f2b6e871 do not certify current `bd5ebec285505889e8ff1d4f68804ab147a699f3`. Full case-level measurements, exact expected corrections and chronology remain in `lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/compiler/layout/tests.md`.

correcao-falhas-2026-09-28/core-full.log and empty-vanilla.pdf refute zero-page empty document and ascender-cancelled baseline. Regional-demand/deferred-note/column controls isolate causal fit from next-baseline prediction. identidade-hifen-discricionario initial/before/red records lost SHY. Tracking green-core/tracking-residual receipts refute an old no-interword-tracking fixture mixed 11/12pt. grid-vanilla-phase-2026-10-03.md isolates trailing-space Auto width. Prepared Realized-origin backtrace and cell-product cache probes distinguish work counts from language observables. Pinned grid/layouter.rs:779-791 distinguishes colspan track boundary 100 from painted stroke extent 102.

SmartQuote ratification 2026-10-06 covers field/context and eval→target-render changes; test phase movement preserves coverage. Table presence human receipt table-cell-presenca-aprovacao-2026-10-07.json covers lexical omitted/Auto/none; resolved-table and standalone Cell formerly proposed gates are ratified separately: `lab/history/construction/records/00_nucleo/materialization/table-cell-realizacao-d08b3-2026-10-07.md:3` confirms manifest `5fec42835e38e9a28ee2ea2714b30135f069f111c140bb5e918a0902a06cc82b` (resolved table only); `lab/history/construction/records/00_nucleo/materialization/table-cell-isolada-d08b3b-2026-10-07.md:3` confirms `3375b0a09a4d39d02c2f0bafbcaf3016c6afde27acdf632331da7a98de31e1df` (bounded standalone follow-up). Their approvals do not merge partial scopes. C2 table-local regression/samples record actual inherited/local font and Auto-column overlap.

Production-correction Clippy receipts first justified test-only deprecated expectation while Text remained permanent preshaping output; pre-shaping-ci-2026-10-13/layout-tests-before.json.gz/before.json.gz and subsequent unfulfilled warning then refuted that now-unused expectation. Box dimensions/composition/stroke/radius/core-first and Place-local layout-attempt3/baseline-expanded/candidate-expanded receipts isolate flat-observer defects, total dimensions and top-plus-baseline from true production regressions. Place-flow, Block-region, width/alignment, paint-root, prepared Shape and transform corpora record full HEAD/dirty-state/UTC/status/diffstat/sources/argv/binaries before corrections. libm affine-observer lint and observer-cleanup V16 are mechanical evidence, not new language parity.

Scope-tail-third and prefix-nesting receipts distinguish injected nominal width from real-font wrap, single versus multiline accepted scope and explicit/automatic nested margins. TS1 approved manifest `771f5fd3d36545eb4b4fb7f0c0f2c91061470d1f010fc9a887b001a636d8feba` includes this owner's fixtures; after-extended/analysis refute Heading affine fold. Typed transforms approved manifest `960f20a8593874da3f3588281dbefa535e4e0ade8109ae117acbd4f5a038281d` includes Matrix fixture adaptation, not publication.

Source→PDF RED probes in `lab/results/construction/diagnostics/fatia-funcional-tekt-2026-09-28/` and inspection of this consumer's FrameItem handlers precede textual-separator changes. Public-contract approval in `00_nucleo/prompts/core/entities/layout_types.md` was received on 2026-09-28 and recorded in materialization provenance.

Bilateral probes/exact state in `lab/results/construction/diagnostics/identidade-hifen-discricionario-2026-09-28/initial.json` and before/ measure U+002D versus SHY with coincident geometry in eight controls. This consumer's FrameItem traversal is archived in approved-before/. The owner confirmed `00_nucleo/prompts/core/entities/layout_types.md`.

The human approved manifest`d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d` for historical T1–T3; receipt: `lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`. Bilateral Parent evidence and Solid controls remain in the approved contract; structural inspection does not replace them. `lab/results/construction/diagnostics/paint-root-2026-10-14/before/results.json.gz` preserves sources, dirty state, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, UTC, diffstat and binary hashes; it covers containers, Place and Rotate with Solid and Gradient Parent. The before-source/syntax-nodes inventory records this consumer's Group constructions/destructurings before editing: [173,181,23206]. `layout_types.rs:531–539` does not distinguish roots from technical groups; `helpers.rs:253–269` rebuilds Group and must preserve the field. Box groups background/body; Block leaves body flat except for clipping. Preserving coordinates is a language requirement; envelope form is mechanics. Approval does not close previously measured general Rotate/Box geometry divergences.

AST spans, patterns/guards/bodies and complements were checked before code. `lab/results/construction/diagnostics/tekt-frame-items-2026-10-16/before.json.gz` preserves HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree, UTC, sources/L0/configuration, hashes, status and git diff HEAD --stat. classification.json records 70 cases/complements across 26 owners using pinned CI-parser tree. Relevant L0 clauses were read with source; this proposes neither new architecture nor inferred upstream intent. Rust patterns/exhaustiveness are mechanics; product semantics/morphology/geometry/effects/diagnostics remain unchanged.

## Exclusive test contract

Preserve existing test worlds/metrics/fixtures/inputs/evaluation counts/assertions/values/tolerances and failing independent controls. Do not declare absent tests implemented or certify language from aggregate status/private flags. New quantitative claims need exact source/binary/time provenance. Observe geometry by composing ancestor affine transforms/Group pos+matrix once; Semantic transparent, Link follows own convention; variant-only tests need no geometry. TextSeparator is positioned neutral leaf, TextReplacement geometry descends children while text extractor uses text once. Preserve SHY identity and U+002D controls, obligatory Noto and missing-font shaper cases, glyph/advance/cache neutrality.

Keep Block relative-carrier fixtures' absolute values, full-region-followed-text/no-terminal-empty/relative-fractional controls, independent float clearance-anchor versus fitting/reservation, atomic suffix including Place/later floats, no-marker/no-float/explicit-spacing controls. Preserve mixed/no-wrap/one-wrap prefix pairs both orders across outer size, equal/unequal and compound/multiword negative controls except explicitly superseded measured cases below. Keep alignment markers reaching layout, no panic suppression/deletion. Structural tests demand thin static exhaustive form-B delegation/private registration, no cursor inline hyphenation/measurement/page commit; canonicalize only whitespace and trailing comma immediately before ')' without changing full arm/callee/argument expressions. Structural tests claim architecture, not speed.

Empty document has one configured 160×180 page/no material. Bottom/dy text baseline is frame top+dy+text top with existing 0.01pt FixedMetrics tolerance; pending-line Place witness stays. Preserve real-font/regional controls: fitting terminal text, deferred ordinary/wrap/chunk admission, viewport continuation, explicit repeated breaks, finite/infinite/empty roots, fitting speculative suffix, column two words after paragraph and terminal-note clearance, verifying pages/text/positions alongside any private state.

Only refuted tracking fixture changes to same-size 12pt A B with exact 2*tracking B displacement for positive/negative tracking; rename/comment current semantics and preserve other tolerances. Constrained Auto measure tests cover single word/no trailing space, two words/one separator and wrap by additional word, asserting width/lines with bilateral PDF independent. Realized prepared Heading Located probe clears existing thread-local introspection counter, preserves anchor/no reindex; include prepared descendants and unprepared synthesis fallback. Cell-cache test-only per-thread counter requires two Auto-token cells→two subframes with text/last_sub_frame_width/bottom; Fixed/alignment/explicit placement retain counts/observables. Align wrapper requires six total (two measures+four emission), Fixed two, placement four; incomplete wrapper counting must not demand four.

Colspan two tracks 60+40/stroke 2 painted path is 102, retaining both-column coverage/stroke 2. SmartQuote tests cover default/en/de/fr, reuse/style/disabled/custom, primes/apostrophes/nesting/mixed markup/function, inserted word/object, parbreak/linebreak/wrap/narrow width and speculative rollback; inspect glyph/text/geometry, never toggle flags as gate, change only receipt-refuted expectations and retain eval→render coverage. Equation-block realized Text replacement between Before/After has distinct y, no duplicate origin; block:false remains inline.

Cell-presence fixtures reflect explicit arguments (x/y/fill/align/inset/breakable permit Auto; colspan/rowspan/stroke reject Auto; only fill/stroke permit none), preserving body/presence/method/JSON/clone/map/rejection spans. Resolved-table fixtures adapt raw fields/defaults and test order/effects/alias/hide/HTML/geometry within ratified scope; nested-show survival requires two CELL/one a same page/inner line lower, bilateral glyphs decide gap. Standalone literals use standalone=None with visual/clone/map/representation/HTML/geometry controls, no extra production authorization. Local Styled-fill Cell inherits external 10pt, explicit local 11pt survives and two Auto columns do not overlap; real contributions/styles/coordinates plus full real-metric bilateral suite.

Keep preshaping Text observations. Remove temporary deprecated expect and 24 old local allow(deprecated), as later unfulfilled-lint evidence supersedes only that expectation obligation; add no suppression. Remove only unused imports/private obsolete helpers, underscore unused outputs but retain evaluation, single registration per test; equivalent assertion/membership/borrow/initialization syntax cannot change scenarios/conditions/evaluation counts/observables. Never delete/ignore a witness or normalize failed behavior. Run full layout/core/Clippy afterward.

Correct Box observers to recursive affine leaves, retaining styles/baselines/tolerances; body-fitting clip may exist but must show content. Explicit width 40/inset3 total is 40, not 46. height-cabe retains visible text; Auto-height clip=true requires natural mask/content instead of no assertion; clip=false no mask anywhere. Preserve fractional tests and P1491 baseline witness (real production regression). Place tests cover fixed/Auto/inset/nested Box/outer Place, Bottom/Horizon/source-size/pending Shape-text/explicit Top, Align/Grid/Table with affine coordinates and P1292 unchanged. Five measured baseline corrections add text top to frame top for Align bottom/horizon and nested/double/Transform Place while preserving final height/finitude/dy. Rust-only nonfloating Parent fixture does not authorize language construction rejected by its owner; residual mixed/Align/Rotate geometry is not certified. Test origin restoration across subframe/checkpoint.

Committed-flow Place tests include root/Box after Linebreak/Parbreak/Shape/empty-text-fixed-Auto Block/Grid/Align/VSpace, explicit Top/successive overlays/new page/nesting. LIFO witness restores after subframe, invalidates at uncertified cursor move, returns by checkpoint. Block-region tests symmetric/asymmetric inset/Top/Bottom/Horizon/nesting/text/Auto/clip, actual mask dimensions/child coordinates/restored parent. Paint-root tests nonfractional Box/single-region Block total inset/root versus backing-parent, nested Place/Transform, translate/offset/slice true/false, no arbitrary wrapper count. Recursive observer uses deterministic pinned libm, exhaustive FrameItem with individual Semantic/TextReplacement/Link recursion and solid Shape collection; split nested fill pattern without altered values. Rerun four affected tests/checks after helper edits, never attribute earlier full suite to later code.

Auto-width tests Shape/Empty/Place-only/mixed/nested-explicit Block/Box/Grid/Stack require backing=root width, Literal versus markup and rollback; retain track 20+30/gap4 expected 68 (Grid must honor gap, never reduce 64). Auto geometric-align tests clip/left/center/right/wider sibling/nested-fixed/Link/restoration, retain separate text/decor scope. Prepared Shape fixtures require FramePaint::Solid explicitly for colors, no to_color/fallback; keep coords/stroke/tolerances. Generic page observers do not enter paint cell; tiling-specific observes prepared cell/implicit root, not Groups of repeated copies.

Generic TransformElem fixtures use operation:Matrix, preserving expected language and compatibility. Adjusted-region tests reflow true word wrapping versus false, inherited styles/alias, global affine coords; large-margin local Box/overlong first word has no empty first line, next word wraps. Use libm instead of prohibited f64 sin/cos; explicit word-set membership leaves values/tolerances intact. Reflection tests overflow word/Shape/Box use capped logical frame without clipping paint, absolute Scale weak-versus-Literal terminal spaces, normal-wrap versus mandatory terminal Literal, explicit h distance and h 1fr/h 2fr full allocation/h 0fr zero; no private accumulator/matrix representation as oracle. Relative shapes test Ratio/Relative/em active-style, explicit/Auto/nested Box, Block inset/height, nested Transform/restoration/genuine infinity; defaults/casts stay open. Shape terminal/two/order Shape→Text/Text→Shape/Shape-Rotate both orders, explicit pagebreak and strong/weak VSpace verify preserved content/positions.

Block vertical tests share allocated height among backing/root/flow with explicit/Auto/empty/text/inset/nesting and outgoing Text/Shape; outset changes paint/clip only and does not alter height 80 fits versus 90 needs next page, both outset0/30. Prepared boundaries include pagebreak text/Shape backing/inset, existing column One−Two/tails, closed line/strong space no extra top. Align Top/Horizon/Bottom after Block/text includes Auto/fixed/declared-no-ink/empty/pagebreak/continuation; P898 includes measured paragraph gap, no weakened horizontal controls. tail21/25/29 compares X/POST positions across alignments, not final height only; initial strong-space/page-specific-bottom refutators remain.

Automatic margins test par.spacing0/5/2em around Blocks, explicit above/below0, two 1fr Blocks B baseline 60/62.5/70 in page 100/margin 10 with top10/bottom0. Shared-scope tests one wrapper18 versus two, Strong, Block/Align after text and TAIL/Styled18/Linebreak after 1fr with 0/5/2em/edges, actual baselines/pages/content/heights; P1381 requires one page and measured Lead/One/After/Two/Tail, not merely count. P1383 no-wrap injected fixture uses page 240/column 105 while multiline opaque controls keep page 200; this follows measured metrics rather than rewritten bad wrap. Later multiline evidence supersedes only named p 1392 compound/p 1389 multiword negatives with positive one-page After/Tail230pt/common leading/color/text, adds nested OUTER18/INNER8 BODY49.8 and state/subframe restoration. Preserve certified legacy expectations. Nested automatic versus explicit below0/5 baselines98.4/76.8/81.8 retain BODY and margin strengths.

TS1 fixtures adapt Style::Size(Pt)→Size(absolute Length), StyleDelta.size→Option<Length>, preserving observables. Heading measured show 2em+3pt gives 36.6pt on 12/default1.4; absolute 18→18; relative 2em→33.6; successive 2em and 0.5em+3pt→19.8; Z restores 12, global 18 leaves Heading25.2. Tests fail before local fold correction.

## Exhaustive observer inventory and acceptance

Replace only wildcard pattern by the listed in-scope nominal complement, preserving complete old body/earlier arms/guards/transforms/calls/returns/order. Restricted fields/guards retain their failed cases in complement; do not expand traversal or replace geometry by ActualText. These F IDs identify AST cases, not execution steps.

| Site | Existing function | Nominal complement | Rust prefix | Restricted case retained |
|---|---|---|---|---|
| F013 | `walk` | `TextSeparator`, `Text`, `TextShaped`, `Line`, `Glyph`, `Image`, `Shape` | `FrameItem` |  |
| F014 | `has_shape` | `TextReplacement`, `TextSeparator`, `Semantic`, `Text`, `TextShaped`, `Line`, `Glyph`, `Image`, `Shape`, `Link` | `FrameItem` | Shape: kind: ShapeKind::Line { .. }. |
| F015 | `has_text` | `TextReplacement`, `TextSeparator`, `Semantic`, `Line`, `Glyph`, `Image`, `Shape`, `Link` | `FrameItem` |  |
| F016 | `collect_text` | `TextReplacement`, `TextSeparator`, `Semantic`, `Line`, `Glyph`, `Image`, `Shape`, `Link` | `FrameItem` |  |
| F017 | `sum_block_limites_empilhados_verticalmente` | `TextReplacement`, `Semantic`, `TextShaped`, `Line`, `Image`, `Shape`, `Group`, `Link` | `FrameItem` |  |
| F018 | `sum_inline_usa_right_scripts` | `TextReplacement`, `Semantic`, `TextShaped`, `Line`, `Image`, `Shape`, `Group`, `Link` | `FrameItem` |  |
| F019 | `align_duas_linhas_tem_ys_distintos` | `TextReplacement`, `Semantic`, `TextShaped`, `Line`, `Image`, `Shape`, `Group`, `Link` | `crate::entities::layout_types::FrameItem` |  |
| F020 | `layout_stack_ltr_empilha_horizontalmente` | `TextReplacement`, `TextSeparator`, `Semantic`, `Line`, `Glyph`, `Image`, `Shape`, `Group`, `Link` | `FrameItem` |  |
| F021 | `layout_stack_spacing_avanca_cursor_entre_children` | `TextReplacement`, `TextSeparator`, `Semantic`, `Text`, `TextShaped`, `Line`, `Glyph`, `Image`, `Shape`, `Group`, `Link` | `FrameItem` | guard; guard. |
| F022 | `layout_stack_spacing_avanca_cursor_entre_children` | `TextReplacement`, `TextSeparator`, `Semantic`, `Text`, `TextShaped`, `Line`, `Glyph`, `Image`, `Shape`, `Group`, `Link` | `FrameItem` | guard; guard. |
| F023 | `layout_block_height_forca_minimo_vertical` | `TextReplacement`, `TextSeparator`, `Semantic`, `Text`, `TextShaped`, `Line`, `Glyph`, `Image`, `Shape`, `Group`, `Link` | `FrameItem` | guard; guard. |
| F024 | `layout_block_height_forca_minimo_vertical` | `TextReplacement`, `TextSeparator`, `Semantic`, `Text`, `TextShaped`, `Line`, `Glyph`, `Image`, `Shape`, `Group`, `Link` | `FrameItem` | guard; guard. |
| F025 | `extract_texts` | `TextReplacement`, `TextSeparator`, `Semantic`, `TextShaped`, `Line`, `Glyph`, `Image`, `Shape`, `Link` | `FrameItem` |  |
| F026 | `p248_table_cell_overflow_preserva_fill_stroke_externos` | `TextReplacement`, `TextSeparator`, `Semantic`, `Text`, `TextShaped`, `Line`, `Glyph`, `Image`, `Shape`, `Group`, `Link` | `FrameItem` | guard; Group: clip_mask: Some(ShapeKind::Rect). |
| F027 | `p248_boxed_height_overflow_cross_radius_clip` | `TextReplacement`, `TextSeparator`, `Semantic`, `Text`, `TextShaped`, `Line`, `Glyph`, `Image`, `Shape`, `Group`, `Link` | `FrameItem` | guard; guard. |
| F028 | `extract_texts_rec` | `TextReplacement`, `TextSeparator`, `Semantic`, `TextShaped`, `Line`, `Glyph`, `Image`, `Shape`, `Link` | `FrameItem` |  |
| F029 | `text_fragments` | `TextSeparator`, `Line`, `Glyph`, `Image`, `Shape` | `FrameItem` |  |
| F030 | `walk` | `TextReplacement`, `TextSeparator`, `Semantic`, `Text`, `TextShaped`, `Line`, `Glyph`, `Image`, `Link` | `FrameItem` |  |
| F031 | `walk` | `TextReplacement`, `TextSeparator`, `Semantic`, `Text`, `TextShaped`, `Line`, `Glyph`, `Image`, `Link` | `FrameItem` |  |
| F032 | `find_abs_y` | `TextReplacement`, `TextSeparator`, `Semantic`, `Text`, `TextShaped`, `Line`, `Glyph`, `Image`, `Shape`, `Link` | `FrameItem` | guard. |
| F033 | `collect_texts` | `TextSeparator`, `Line`, `Glyph`, `Image`, `Shape` | `FrameItem` |  |
| F034 | `p994_box_em_math_produz_caixa` | `TextReplacement`, `TextSeparator`, `Semantic`, `Text`, `TextShaped`, `Line`, `Glyph`, `Image`, `Shape`, `Link` | `FrameItem` | Shape: stroke: Some(_). |
| F035 | `smartquote_rendered_text` | `TextReplacement`, `Semantic`, `Line`, `Glyph`, `Image`, `Shape`, `Group`, `Link` | `FrameItem` |  |

Preserve callbacks/effects/metadata/clipping/fonts/positions/paint/width under their current owners. This work does not expand traversals, turn wrappers into leaves or substitute text/ActualText for geometry. Test helpers preserve fixtures/expected outputs/tolerances/counts/Unknown failure; cleanup cannot accept missing observation. Nominal alternatives retain the prior shared body and visible informative V19 notes; one arm's execution does not prove every alternative.

The test owner introduces no productive API/default/phase. Require RED→GREEN and unchanged language observables for every bounded case, including negative limits, source→PDF/SVG/HTML where owned, real-metric bilateral controls and all broader suite/build/doc/check/fmt/strict Clippy/lineage/differential inventories. Fractional Box composition, fragmented Block per-root/paint/height, general tiling/Parent PDF integration, Place late inline/uncertified producers, shape defaults/casts, nested Par/opaque/mixed RTL and unrelated geometry retain canonical owners' explicit gaps. Root/carrier/build tests alone never close production/render parity. No tolerance rewrite, erased observation, missing test claim or benchmark claim is allowed.

L0 comes first, with RED→GREEN and restricted resealing. Preserve geometry/text in source→PDF/SVG controls and revalidate suite/build/doc/check/fmt/lineage. This metadata migration neither replaces Shape.fill/stroke with FramePaint nor implements PDF/SVG/raster patterns. Prepared paint transport, native exporter integration and verification (historical T1c/T2/T3) remain explicitly open and already authorized. Build success does not establish tiling parity or production readiness.

Internal continuous-flow ADR-0127 maintenance introduces no enum/field, trait method/public signature, default, dependency, compatibility or phase. Do not materialize historical pending proposals. L0 precedes restricted bilateral resealing. Add no suppression, threshold change or match→if-let rewrite to hide V16.

Before code, run new FrameVisitor control and workspace suite with unchanged expected outputs, and establish structural RED for 70 family warnings through AST reading. Afterwards require zero covered-family V16 and no new warning, retaining external V16 as pending. For productive core matches reached by cargo check -p typst-core, use a disposable copy with a sentinel FrameItem variant to measure absent E0004 barriers before/present after. The probe covers only compiled sites, not cfg(test) helpers or L3 blocked by dependency. Across the whole package replay patterns/guards/bodies, verify unchanged enum/signatures, real build/tests and no stub bodies.

Revalidate full workspace suite, cargo build, Clippy workspace/all-targets -D warnings and -D unfulfilled_lint_expectations, fmt, Rustdoc -D warnings with four HTML indexes, strict/reverse V1/V5/V15/V26 lineage, SARIF and diff --check. Freeze sources/L0/configuration during commands. Heavy commands run sequentially with MemoryMax 12GiB/MemorySwapMax0/CARGO_BUILD_JOBS1. Structural cleanup does not establish new global parity, memory gain or measured AI benefit.


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

## Approved Figure/Grid cause regressions

Evidence: the frozen Figure proposal at
`lab/results/v16-corrections-2026-10-19/figure-kind/review.md` and its paired
current-pin probes reproduce lost function-kind identity, lexical presence,
Metadata/Array traversal, ordered Grid children and occurrence show-set behavior.
The owner explicitly approved its contracts, phases and compatibility changes
before this amendment (approval recorded under figure-kind-implementation).

Add language-level regressions using the existing evaluator and layout test
world. Show callbacks must observe a function-valued table kind, a distinct
string kind, inherited omitted kind versus explicit auto, final kind after a
filtered show-set, Metadata Content/Array descent, Dictionary and Context opacity,
first Grid header/body order, and actual supplement callback display. Keep simple
body replacement and hidden-parent controls. Observe real rendered text rather
than Rust carrier equality. These tests must compile against the old API and
fail semantically before implementation. New carrier-specific tests separately
cover raw presence, clones/maps, identity and malformed order metadata. Existing
layout assertions remain except exact approved kind/count compatibility migrations;
changing a source literal alone does not authorize a weaker expected result.


### Five legacy Figure numeric fixture migrations

`figure-kind-implementation/integration-tests-2/tests.log` records five failures solely at removed automatic figure:<kind> string projections in p184e_figure_per_kind::{pipeline_completo_figure_kind_image_via_introspector,kinds_distintos_isolados_image_e_table,kind_none_default_image}, p189b_walk_puro_m5::walk_excepcao_e3_figure_via_intr and p195d_walk_labelled::labelled_figure_target_popula_figure_label_numbers. Migrate exactly these tests to actual preparation/runtime, explicit intended image/table Element identities and typed query Locations before reading materialized prefixes. Preserve all per-kind sequence/out-of-range/independence/rendered text/caption assertions; explicitly check absent automatic legacy strings. For labelled Figure, retain the existing scalar label-number assertion and replace obsolete pure resolved_labels placeholder with actual prepared supplement plus rendered reference number/text. Keep synthetic fallback and generic legacy tests unchanged. A shared test-only fixture helper may supply Engine/World and migration, never productive shadow state.

## Existing reference fixture preparation migration

The complete workspace run after GREEN9 identifies the existing p462 explicit
Figure-reference supplement fixture using a pure synthetic introspector while
requiring a productive numbered caption. Use the existing prepared_fixture runtime
helper for that test, preserving both exact default caption text and explicit
reference override. Strengthen its sibling default Figure reference to require
both caption and reference occurrences plus empty layout diagnostics, so a caption
alone cannot make reference assertions pass. Synthetic fallback-only tests stay
unchanged. This changes no productive code, signature, phase, or accepted outcome;
it aligns the old fixture with the approved prepared typed counter contract.

## Static reference consumers after the approved contextual Ref phase

reference-tests-2-core records ref_resolves_figure_number's strengthened two-use
assertion failing with only the caption present. The existing L1 prepared_fixture
now captures source Ref as a deferred contextual producer but does not execute
L3 Session. These low-level layout tests must prepare only their labelled Figure,
then append the existing unchanged static synthetic Ref before direct layout.
Use the same pattern for the explicit reference override. Preserve exactly two
caption/reference Figure 1 occurrences, original caption+override expectations,
and empty layout_errors. This tests the pure numeric consumer using an admitted
static fixture, without pretending its helper executes the new source producer.
No generic helper expansion, duplicated producer algorithm or assertion reduction.
The independent real L3 source Ref regressions and CLI matrix remain required.

## Numeric reference extracted whitespace normalization

reference-tests-4-core's actual default-Ref fixture renders both caption and
reference, but direct text extraction inserts multiple spacing characters between
reference supplement and number (`Figure   1`) while caption reads `Figure 1`.
This tests language text, not exporter spacing mechanics. Collapse extracted
Unicode whitespace runs for that assertion, preserving word/number order and
requiring exactly two complete `Figure 1` occurrences. Keep both body/caption
and explicit override obligations and empty diagnostics; never reduce the count,
remove a visible reference, or infer success from its caption alone.

For that same normalized default-reference fixture, require the exact caption
`Figure 1: legenda` once and the final visible text ending `Figure 1`, in addition
to the two occurrences. Substring `Figure 1` alone must not accept `Figure 10`.
All nonwhitespace punctuation and exact numeric values remain asserted.

The independent full p462 migration-tests-5-numeric-ref run records the same
spacing distinction in the explicit override fixture: default caption is correct,
while `Figura` plus NBSP/multiple extracted spaces plus `1` fails the old one/two
literal-space alternative. Normalize that fixture's Unicode whitespace runs too;
require exactly one full `Figure 1: legenda` caption and the exact visible suffix
`Figura 1`. This preserves the explicit override and both exact numbers, rejects
`Figura 10`, and retains empty layout errors. No source fixture or production
behavior changes; nonwhitespace characters and their order remain untouched.


## Measured Repeat test correction — 2026-10-22

### Evidence before decision

`lab/results/nested-fractions-repeat-2026-10-21/core-interim-red/receipt.json.gz`
preserves UTC `2026-10-22T01:40:02.778850+00:00`–`2026-10-22T01:42:27.009317+00:00`,
uncommitted HEAD `cf21f47a3b63f0e9fc5308925e1d0b67925e7227`, exact changed-file
stat, all frozen input hashes, full Cargo command/log and `changed_inputs=[]`.
The actual full-core run has 7810 passes, four assertion failures and five
preexisting ignored tests. These two old Repeat expectations are among its
explicit failures; no test deletion, ignore or tolerance change follows.

Bilateral `repeat-before/measurement.json.gz` in that result root (SHA-256
`e6d3c085c28062ad22fdb3b1575108d3a778842a50bf0e5c7fa8381b445e88e0`)
records UTC `2026-10-22T01:15:47.337768+00:00`–`2026-10-22T01:16:29.201630+00:00`,
the same HEAD, exact dirty state and inputs, baseline product SHA-256
`94208ba4526d7bda97d4519aa8eb12212bd13dc1f8932552f235940e73238485`,
and ratified vanilla commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree
`cac836c07e4dec2a89219370428292393a3216b5`, binary SHA-256
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.
Observable body content, typed payload, defaults and accepted arguments are
language requirements; the candidate's wrapper depth is mechanics under
ADR-0107. This correction follows ADR-0108 and ADR-0127 continuous flow.

### Scoped correction and acceptance

The old `layout_repeat_renderiza_body_no_contexto_actual` test at
`01_core/src/compiler/layout/tests.rs:8688–8695` searches only page-root Text
items and reports zero after the corrected Repeat places complete prepared
frames. The new independent Repeat tests pass in the same full-core receipt,
and bilateral root-prefix/multiline witnesses establish repeated visible
content rather than single-inline mechanics. Existing
`frame_items_recursive` at this consumer's lines678–743 already composes each
ancestor Group position/matrix and descends transparent semantic wrappers;
`document_frame_items` at lines973–978 applies it to every body page.

Use that established observer for the same `repeat[X]` fixture, preserving
the exact existing `count_x >= 1` Text assertion. Rename/comment the test for
its current complete-frame language obligation; do not demand flat output or
change source, metrics, page configuration, count threshold or tolerance.
No observer helper, productive behavior or unrelated test is authorized by
this correction. Run the owned test and the full core gate; do not call the
prior flat-observer failure a new productive language regression.


## Fallible Source and file-registration integration

Human approval on 2026-10-22 is retained in
`lab/results/panic-elimination-2026-10-22/source-contract-approval.json`;
its review SHA-256 is `2c879e8afa66721a8952dc8b9e7cf3c92dce58bf91aae5e28da5d603f5aa4d42`.
Source::{new,new_with_parser,detached,detached_with_parser} now return the existing
SourceResult<Source>; SystemWorld::register_file returns FileResult<FileId>.
Preserve this unit's successful content/id/span/order/diagnostic behavior while
propagating construction/registration failures through its existing error channel.
A tuple-returning entry returns its error in the existing result component and
retains already accumulated warnings. A synthetic source slot stores a failed
construction as FileResult rather than inventing a Source. Test-only fixture
helpers must explicitly require their valid construction/registration result;
all previous substantive assertions remain. No productive unwrap/expect, panic
catch, default Source, identity reuse or silent error is permitted for this migration.
This clause supersedes only the old infallible return/panic expectations for the
five approved APIs; it does not authorize unrelated signatures, fields, phases
or broadening/loss of existing diagnostics or negative controls.


## Weak pagebreak parity correction — 2026-10-23

### Evidence before decision

`lab/results/weak-pagebreak-2026-10-23/baseline.json` binds the uncommitted source/L0/config state to HEAD, dirty stat and UTC; `before/receipt.json` binds both verified executables, inputs, commands and observations. `pagebreak.rs:21-58` ignores weak and always requests transitions; `finish.rs:103-106` requires the newly opened page unless it is a PageRun boundary. `page_run.rs:126-131` unconditionally closes its body. The pinned source `lab/typst-original/crates/typst-library/src/layout/page.rs:554-557` explicitly defines skipping weak breaks on empty pages; its tests `tests/suite/layout/pagebreak.typ:15-24,61-76` document weak/material distinctions. Initial, repeated, terminal and strong/weak controls plus parity and explicit PageRun probes distinguish language intent from mechanics. Page membership, content and geometry are observables; the private successor flag is a chosen mechanism. The inference that an optional successor suffices is refuted by lost notes/placed content, changed strong/parity empties, or leaked rollback state.

### Decision and exclusive responsibility

Add semantic regression tests for weak pagebreak page membership: terminal/initial/repeated breaks, strong combinations, parity-directed breaks, explicit PageRun boundaries including empty bodies, placed content, notes, running decorations, and flow-suffix checkpoint restoration. Tests assert language-visible page/text membership rather than implementation call counts.

This is an ADR-0127 continuous-flow language-parity correction: L0 first, reseal, RED→GREEN and complete revalidation. No public contract, default mode, pipeline phase or compatibility change is authorized. Existing nucleus obligations and unrelated limits remain binding.


## Refutation: unpainted material and required successors

### Evidence before decision

`lab/results/weak-pagebreak-2026-10-23/edge-after/receipt.json` and `unpainted-before/receipt.json` bind the first candidate executable to UTC/inputs/commands; its build and source state are in `validation.json` and `final-source.json.gz`. These controls refute painting alone as page materiality: hidden text, zero/fractional horizontal spacing and zero/weak/strong vertical spacing can require a page without paint; empty/metadata-only Hide does not. `h_space.rs:27-45` advances or queues spacing without an item, `v_space.rs:20-42` records advancement, and `hide.rs:20-28` removes only painted output. `pagebreak.rs:42-44` also overwrites an empty strong successor's requiredness during weak parity adjustment, dropping a required final page. These are language-visible page obligations, not a request to copy upstream collection mechanics. The earlier painting-only hypothesis was rejected, not qualified by changing the oracle or deleting its cases.

### Exclusive correction

Require RED→GREEN on hidden/empty/metadata-only bodies, zero/absolute/fractional horizontal spacing and zero/weak/strong vertical spacing, both terminal and between weak breaks, plus initially empty and strong successor pages under weak parity adjustment.

L0 precedes code under continuous-flow ADR-0127. No public field, signature, trait, phase, default or compatibility change. Validate the refuters and retain all previous note/float/Context/column/strong/PageRun controls.


## R1 — retain the formatted entry marker (2026-10-23)

Measurement before decision: `lab/results/footnote-markers-r1-2026-10-23/baseline.json` and `before/receipt.json` bind exact HEAD, uncommitted changed-file list, source archive, UTC, executable and fixed oracle identities. Successful note and note-successor probes expose generated `[1]` instead of `1`; custom exposes decimal bracket footers despite correctly formatted inline `a` and `(ii)`. Source `footnote.rs:45–48` formats the call; `footnote_fragments.rs:286` and `footnote_flush.rs:156` independently synthesize brackets. Oracle `typst-library/src/model/footnote.rs:305–338` realizes the original note numbering as superscript in its entry. This is visible language morphology; internal queue representation is mechanics.

Classification: ADR-0127 continuous parity correction, private layout metadata only, no public contract, default mode, pipeline phase or compatibility change.

Exclusive obligation: Adapt the direct pending-note fixture to the private tuple including the generated prefix, retaining its original demand/clearance assertions. Marker regressions belong to footnote and footnote_fragments owners.

Acceptance: tests first must demonstrate actual marker failure before implementation and pass afterward. Cover default, custom and intentionally bracketed numbering, literal body brackets, captured and uncaptured emission, and checkpoint preservation. Reuse generated Content rather than export text replacement. Preserve the existing body styles and note-area model, source admissions, diagnostic behavior and unrelated tests. Full build, tests, lineage and linter gates are required. This does not certify broader entry typography, column placement, nested-note body completeness, call/body migration or oversized-note continuation; measured prior limits remain open.


### R1 legacy test migration after full validation

Measurement: the source-bound first full run in `lab/results/footnote-markers-r1-2026-10-23/validation-first/` reports three obsolete positive `[1]` assertions and a column counter test whose `y > 400` filter actually counted footer-body digits instead of inline calls. This precedes the correction. Update only those marker assertions to require two native superscript `1` carriers (call and entry), excluding numeric body text; retain all body, page, no-loop, overlap and warning assertions. Partition column call markers above the midpoint and entry markers below it, and require the exact causal sequence 1,2,3 separately in each region. This strengthens the intended semantic obligation; it does not preserve the defective bracket morphology or historical accidental filter.


### R1 test exhaustivity and guard gate

Measurement before decision: `lab/results/footnote-markers-r1-2026-10-23/full-lint-complete.log` records V16 for the new test collector and V17 for the column test guard after all functional gates and replays passed. This amendment owns a test-form correction, not a runtime layout change. Keep the column marker predicate inside the selected text branch rather than a compound decision guard; retain the same superscript, number and midpoint conditions. Revalidate final lineage, full lint and the established suite without suppressions or weakened observations.


## R5 — physical painting observer migration (2026-10-24)

### Measurement before classification

`lab/results/construction/diagnostics/painting-r5-2026-10-24/test-observer-migration-before.json`
preserves exact source/L0, HEAD cf21f47a3b63f0e9fc5308925e1d0b67925e7227,
uncommitted tree, full diffstat and UTC. `validation-attempt3/validation-state.json.gz`
freezes the tested productive state; `tests-domain-infra.log:7947-7977,9517-9528`
records four failures. Three root Grid observers at tests.rs:9422,9487,9701
require seven/twenty-six segments, explicitly including duplicate shared borders.
Their actual corrected six/seventeen are not by themselves parity evidence.
The independent original Table raster now has zero changed pixels in PDF/SVG
at72/600dpi (`candidate2/receipt.json`), and the actual preimplementation alpha
regression is preserved in `red-core-tests/test.log`.
Integration tests.rs:1714-1737 reads raw m/l operands and ignores cm: local(0,0)
→(100,50) under Y reflection is wrongly judged to ascend. The independent
coordinate interpreter and bilateral signed-line experiment preserve physical
downward direction. Counts/operator representation are mechanics under ADR-0107;
coverage, dimensions, alpha and transformed endpoints are language observables.

Continuous ADR-0127 test-only observer correction introduces no public contract,
product default, phase or compatibility change. This narrowly supersedes blanket
assertion/count preservation only for the four named mechanical expectations;
all input fixtures, font sizes, dimensions, actual geometry/diagnostic assertions
and all other tests/thresholds remain unchanged. L0 first, restricted reciprocal
resealing, focused GREEN and full final-source validation are required. The prior
four functional RED→GREEN regressions remain required, not replaced by this
migration. Neither aborted/failed validation nor edited counts certify success.

### Exclusive owner obligation

Retain all three existing Grid fixtures. Replace aggregate segment totals in
p227/p888 with physical border coverage: expected column boundaries from fixed
track widths, expected row boundaries from fixed heights or the equal Auto rows,
single stroke contribution at every boundary/cell midpoint, full vertical and
horizontal extents including half-thickness endpoint extensions, no gaps or
duplicate contributions. Keep strict width==abs(dx), height==abs(dy), nonzero
line assertions in p887; call the coverage observer instead of expecting seven
records. Use a test-only private helper in this owner with tight coordinate
checks, no FrameStroke PartialEq or production frame rewrite. No general frame
visitor/schema mutation or normalization of actual paint is authorized.

## R8: workspace superscript and oversized-note acceptance
Measurement before decision: validation-complete3/tests-domain-infra.log in footnote-layout-r8-2026-10-24 records p448 expecting upward movement as a larger frame y and p595 expecting an oversized warning for a now-certified splittable text note. Actual R8 paired PDFs/SVGs establish downward frame coordinates and upward superscript; change p448 to smaller y, retaining reduced size. large-note-controls/receipt.json distinguishes splittable lorem200 (successful, no warning) from an indivisible block(height:500pt,breakable:false) on a 200pt page (existing warning). Preserve the defensive-warning test by making its oversized body explicitly indivisible; add a separate certified multiline continuation test with START/END once and both call/entry markers, no oversized warning. Do not suppress warnings for uncertified or indivisible oversized frames, weaken content assertions, or alter unrelated tests. No new public/default/pipeline contract.


## R13 — approved ShapeElem literal migration

The source inventory in lab/results/construction/diagnostics/relative-lines-r13-2026-10-25/migration-owners.json identifies this consumer. Owner approved the public ShapeElem.relative_line addition (proposal SHA-256 d8c9a01413232bac7113d1148af3c1b3bcd671bb37eeca68a94d5749d671db00). Initialize that field to None in existing constructors and fixture literals, preserving all geometry, paint and behavior. No other implementation change is authorized here. Full existing tests, build and lineage must pass.


## Readiness: generated heading separation

Measurement precedes this update: readiness-fixes-2026-10-28/unicode-validation/
tests-domain-infra.log records only the three p451 numbering controls failing:
text-item concatenation yields 1.Intro, 1.Cap1.1Sec, I.AI.IBII.C. The current
heading owner specifies a 0.3em geometric separator, proved by the actual pinned
control-heading-numbering PDF/SVG and the independent numeric-gap RED→GREEN.
A literal U+0020 in a Text item is not the spacing contract. Keep all three
numbering patterns, hierarchical sequence and Roman reset expectations, and
strengthen their observation with exact prefix/body run order, same baseline,
increasing heading baselines and the measured0.3em gap using FixedMetrics.
Do not restore a space-glyph-dependent gap, normalize extracted source words,
remove these controls, or reduce their numbering assertions.


## Readiness: list group geometry and body isolation

Own the three public-layout controls for list/enum signed spacing and nested
item-body scope, transferred intact from Sequence's inline test module. Their
source evidence and required behavior are in the sequence owner's readiness
clauses: direct/inverted groups, blank boundary, spacing40/leading20-spacing5,
continuous groups, and Sequence bodies with inline math. Preserve every input,
coordinate assertion and failure; do not count a helper compilation failure
as functional RED. This dedicated test-only layout unit avoids mixing test
maintenance with the productive sibling coordinator. No productive public
contract, default or phase changes.

## Readiness: inset fractional allocation investigation

Measurement: `readiness-fixes-2026-10-28/list-body-extras/comparison.json`
and the original paired `outline-fr-box-inset` source retain the pinned
64c5e8c0 oracle, frozen input hashes, dirty HEAD and UTC. A 13.2pt left
Block inset leaves the page number near Beta instead of the right margin;
the zero-inset control reaches that margin. The standalone Rust arithmetic
receipt is a hypothesis only, not proof of the productive cause.

Add a source-shaped functional control with declared Beta19.888pt,
space2.75pt, digit5.115pt and zero-width U+2060, actual Block insets0/13.2,
and the actual fractional Box dispatch. Require source words once, the
same baseline and the digit's right edge at280pt within1e-8. Keep unknown
width, overflow, signed/zero fractions and historical rounding controls
unchanged. This clause authorizes investigation tests only; it does not
supersede the allocation owner's strict arithmetic restrictions or authorize
a productive rounding change before a functional RED and new decision.


The first inset test attempt (fraction-inset-red.json/log) incorrectly set the
region's absolute width to260 while separately installing20pt margins; its
zero-inset control ended at240pt. Preserve that failed setup as evidence but
do not count it as the productive RED. Set the absolute region width to300,
retaining20pt margins and the original expected right edge280pt. This corrects
the adapter, not the acceptance criterion.


The read-only outline-eval-probe/ and outline-metric-probe/ receipts retain
exact worker/rlib/input hashes, dirty HEAD, UTC and evaluated original trees.
The first probe omitted the font catalog, so its fallback metrics are not
productive font measurements. The second uses the CLI catalog; explicit
Libertinus Serif measures Beta19.888, space2.75, digit5.115 and shaped U+2060
zero, while the evaluated body contains an actual Repeat inside the fractional
Box and one combined Text(U+2060+1). Preserve the passing plain-Box test.
Add this actual Repeat/combined-text body with gap0.15em and both insets;
require both words once on the same baseline and the same280pt right edge.
These tests distinguish composition from a guessed numeric correction; they
still authorize no productive allocation rewrite on their own.


## Readiness: automatic number baseline and per-page style (2026-10-28)

### Measurement before decision

readiness-fixes-2026-10-28/{article-fields-before,article-fields-extra-before,
page-number-total-before}/receipt.json preserve exact source/font/compiler/oracle
hashes, current source-bound wj2 build/archive, HEAD212aed0449150dd296ceea13fc2be665cc5db01a,
dirty stat, UTC, commands and frozen productive/L0/fixture inputs. Oracle commit
64c5e8c02fe0394d913ead36e42ef8c1334d2107, tree
483bd6f1d8ac8810db47b65336d402647955fd17, executable
22897f93c7c2c8589cf5913b0ce735f1c01ba711c70f55c7a350f7c4b96c0b34.
The default bottom number is7.238pt too high at11pt; at17pt its first-page
offset is11.186pt. Bold/DejaVu produce different measured top edges, refuting
a universal constant. The top unary/compound control already matches. A final
page loses the17pt/bold/DejaVu style, and compound patterns borrow11pt even for
the first17pt page. Explicit CUSTOM footer is a DIFFERENT measured negative;
it is not fixed or waived here. Body text and None-number controls are retained.

Source page_commit.rs:62-111 and finish.rs:123-174 emit Text directly at the
number-position anchor. finish.rs:247-275 later builds compound text using the
then-current chain. The retained page-effective chain already exists and is
used for running content; this correction consumes it rather than introducing
another source/style authority. Positions, styles, words and pages are language
observables; tuple/record representation and queue mechanics are not parity.

### Exclusive correction and acceptance

Own functional composed-page REDs using declared unequal top/vertical metrics, two physical pages with11/17pt styles, unary and total-dependent patterns, top/bottom alignment and em footer offset. Require one unchanged formatted number per page, exact style and x/y from the actual page and injected metric, and nonzero top-edge distinction from ascent. Keep Top and explicit None/Content suppression controls; do not qualify the separately negative explicit-footer geometry or erase prior failures. These tests exercise commit and finish through genuine page transitions, not just the new helper's implementation.

ADR-0127 continuous internal parity correction. L0/pins first, actual RED before
code, V15/V26 before resealing, exact source words/pages/0.001pt and independent
PDF/SVG paint. Preserve all previous running/source/callback/rollback/notes,
Unicode, layout, technical/lineage and historical/JPEG resource gates without
new tolerances, ignored tests or suppression. No overall readiness claim.

## Readiness: bounded numbered Outline inline budget

### Measurement before decision

readiness-fixes-2026-10-28/outline-prefix-source-evidence.json binds exact
source/owner hashes, HEAD212aed0449150dd296ceea13fc2be665cc5db01a dirty stat/UTC,
number2 build/source archive and actual paired numbered-outline cases. The
executable is92c6346f8562438618e2cd7ba57166423316ee26fc08aa597ac357c482859fd7;
oracle64c5e8c02fe0394d913ead36e42ef8c1334d2107/tree483bd6f1d8ac8810db47b65336d402647955fd17,
executable22897f93c7c2c8589cf5913b0ce735f1c01ba711c70f55c7a350f7c4b96c0b34.
The non-Ref numbered controls retain different fill/word positions. Source
stdlib structural outline.rs:319-391 emits Empty prefix-info, a nonweak negative
hanging compensation, the original marked textual prefix and gap inside an inset
Block. style_scope.rs:133-167 preserves Label semantics through the parent-line
inline subframe; sub_frame.rs:724-748 retains its actual horizontal advance.
inline_allocation.rs:283-294,488-530 rejects the two known custom markers;
:61-64,341-346 refuses every negative absolute H. These explicit refusals are
source evidence, not proof of every causal/rendering obligation. Actual local
certificate and composed-line REDs must fail before productive implementation.
Source counts/styles/positions and preserved semantic envelopes are language
obligations; the certificate's representation and fallback route are mechanics.
No upstream intent is inferred or old floating-point hypothesis resurrected.

### Exclusive correction and acceptance

Own a functional composed-line RED with the existing standard numbered-outline body: Empty prefix-info where present, first nonweak negative hanging H, original Text prefix in Label marker, gap, body, actual fractional Repeat Box, weak spaces and original U+2060+page suffix. Use declared injected widths and the real inset Block path; cover auto metadata present/absent and different base inset. Require one prefix/body/suffix at their source positions and common baseline, suffix at the actual regional right edge, real dot copies and one preserved Semantic Label. Preserve metadata's source-side prefix-width observation. Do not substitute a helper-level allocation assertion for actual layout.

ADR-0127 continuous internal parity/certificate correction, separate from the
pending physical-phase approval. L0/pins first, meaningful RED→GREEN, V15/V26
before resealing, full source-language/PDF/SVG and technical gates. Keep all
historical/JPEG controls and remaining readiness negatives; do not relax any
metric, suppress diagnostics or claim general rich-prefix/RTL/wrapping parity.

## Multiword numbered outline budget observer

The physical4 article paired receipt observes overlapping first-entry fill;
inline_allocation currently refuses whitespace Text. Add a separate regression
with the native nested shape (prefix-info, negative H, marked1., compensating H,
inner Sequence Text Beta Beta/Space/fractional Repeat/Space/WJ/page). Use existing
DeclaredWidths, real layout and inset bounds. Require both Beta occurrences,
one page and the suffix at the existing280pt right edge. Preserve the old
single-word test unchanged. This is a position test, not a new allocator formula.
Original paired PDF/SVG fill/body geometry remains required for acceptance.

Functional RED receipt: entry-text-weight-red.json/log and source archive retain
exact inputs/HEAD/dirty list/UTC and --no-fail-fast execution. Core records one
actual suffix at81.391pt instead of the regional right edge; four actual Entry
query tests and one real static-font emitter fail in infra. The hidden Entry
control and all prior readiness tests remain green. Preserve those logs rather
than treating later tests as historical RED.

## List-to-enum paragraph boundary expectation

physical9-validation/tests-domain-infra.log preserves actual old-test failure:
20.9pt versus14.85pt, fixed metrics, exact source archived by physical9 build.
list-enum-fresh/receipt.json binds immutable product2fc7b30d… and current pinned
oracle22897f93…, sources/hashes/UTC/dirty HEAD/diffstat/build archive. Tight and
explicit Parbreak source both measure20.438pt at11pt/spacing1.2em and29.238pt at
spacing2em in both binaries. Changing paragraph spacing shifts the gap8.8pt.
The previous expectation of leading between different List/Enum groups conflicts
with this earlier parity correction. Preserve source words/Parbreak/two markers
and use supplied FixedMetrics top+bottom plus the default paragraph spacing
1.2em, not the0.65em leading. Name the test for that actual contract. No product
formula, rendering tolerance, source fixture or unrelated Terms expectation is
changed by this test correction. Require the full suite green.


## Approved TableAlignment consumer migration — 2026-10-29

The table-column-alignment-approval.json receipt ratifies review SHA256
1e73466f33e416cb14541575124b16244272079ac336e04159ac89ce1bdef66e.
Actual table-alignment-green1 compilation identifies this owner as a productive
consumer of the field type changed by that gate; the failed attempt is retained.
This migration adds no other public field, trait, signature or phase. L0 first:
Migrate the existing synthetic centered-Table cache-refusal fixture from
Some(Align2D) to Some(TableAlignment::Uniform(same Align2D)). Preserve body,
metrics, expected call count and geometry/content assertions exactly; no cache
control is relaxed. Add raw synthetic declaration/column/span geometry controls
using this same physical engine, keeping them distinct from source/prepared tests.
Require original tests plus focused RED→GREEN, valid graph/reseal, full gates
and unchanged native geometry/JPEG criteria. Function alignment, general Table
reflection, Grid column arrays, rich paint/inset, multipage spans and PDF/UA
remain outside certification.


The new raw declaration comparison in green2/core-before-repair used exact f64
equality accidentally: BBBB x differs by 2.8e-14pt between equivalent paths.
Use the already-approved physical criterion 0.001pt for that new test, retaining
text/count/axes assertions and immutable failed receipts. Its explicit-cell
reference must request the newly represented Auto raw placement as well, so the
span comparison tests the same supported coordinate contract. Do not change any
legacy expected geometry, fixture or subframe/cache call count.


## Issue 8894 — approved Stroke migration

Measurement: the source inventory and previous owner bytes in
`lab/results/construction/diagnostics/issue-8894-2026-10-31/` bind exact
HEAD/dirty diffstat/UTC and each affected source/owner before migration.
`stdlib/layout/stroke.rs:430–434` erases em; geometry.rs:81 carries only f64.
The explicit user approval in approval.json ratifies the public Length field
and occurrence-layout resolution, as specified by
`00_nucleo/prompts/core/entities/geometry.md`. This consumer retains its
exclusive responsibility and all other ratified clauses and controls.

Migrate this consumer's declarative Stroke literal thickness from scalar
points to Length::pt without changing their value or other fields. FrameStroke
remains scalar. Keep all existing semantic assertions; update only their
representation as necessary. Producers preserve full Length; layout consumers
resolve against effective font size before scalar geometry. Repr preserves
absolute/em components. RED→GREEN, build/tests and scoped lineage are required.


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

Migrate only the approved literal additions: raw Table items=None, legacy plans header_groups empty, Header level 1 unless cloning an existing header (preserve its level). Preserve every old semantic assertion and all other data; no new APIs/default/phase. Representation-only test assertions that freeze the superseded duplicate-header rejection must be replaced by positive source-order/range safety acceptance, without weakening unrelated controls.

L0 and restricted reseal precede RED/code. Preserve prior ratifications, purity and
static dispatch. Validate original source order, adjacent/replaced/nonrepeating
groups, effects and no hidden admission, spans/conflicts/padding, finite region
progress, legacy controls, native PDF/SVG/HTML, proportional suites/build and full
forward/reverse lineage/lint. All compilation/tests use only the exclusive RAM
target /dev/shm/typst-crystalline-8954-target; no fallback to disk. Rich interactions
remain bounded; no global parity or PDF/UA claim. Additional public/phase changes
require a new gate; the approved fields are not inferred from diagnostics.


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
