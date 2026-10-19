# Layout L0 sanitation — historical evidence and unresolved discrepancies

## Scope and status

This diagnostic archives history removed from the assigned layout L0 family. It is not a materializable prompt, a new architecture decision, a Rust implementation plan, or a statement that repository-wide sanitation is complete.

The assigned scope is `00_nucleo/prompts/compiler/layout.md`, every prompt under `00_nucleo/prompts/compiler/layout/`, and the directly contained compiler prompts matching `layout_*.md` or `layout-*.md`. Only those specifications and this diagnostic are edited. Existing dirty changes are preserved. Product code, tests, ADRs, shared nuclei, lineage headers and nucleus pins are not changed; no formatting, test execution or hash repair is part of this operation.

## Provenance and interpretation

The pre-edit snapshot was taken at 2026-09-27T00:49:43.138Z, on HEAD `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`, with an uncommitted working tree. The prompt/consumer fingerprints below identify the actual inspected text, rather than pretending HEAD alone identifies this state. The working-tree status and stat captured during this operation are appended as provenance, not as architectural or performance measurements. The aggregate pre-edit fingerprint of files outside this assigned scope was `765b40e64bdc430f87d23e208dbff31d05596395325cb3737dd08572e7bc1ddf`.

The current parity reference remains vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`. This sanitation inspects source ownership and behavior; it does not produce new bilateral parity measurements or new benchmark results. Historical results below retain their original step associations and qualifications. Results from older pins, incomplete build provenance, or a previous uncommitted state cannot close parity against the current pin. Byte identity, Rust equality, traversal shape and test counts are not language parity under ADR-0107. Historical intent is not inferred from current behavior.

The source references collected from the former prompts in the archive index are historical anchors, not automatically valid positions in today's files. Fresh anchors retained in the rewritten L0s and the current discrepancies section establish the inspected consumer boundaries. Their interpretations can be refuted by the corresponding source or a separately proven language-level measurement.

## Architectural migration and ownership history

The early extraction and form-B atomization stages moved element implementations from a large dispatcher into descendant render modules. The surviving architectural decision is ADR-0109: native dispatch stays static and exhaustive; entities remain data owners, not render owners. Historical extraction status, pending lists, code-size comparisons and test totals have no normative role.

P1503 separated physical line commitment from cursor progression. P1504 separated speculative suffix acceptance/replay, cell-tail emission, intermediate page assembly and script segmentation; Place retained final float emission and column composition retained its existing feature owner. Subsequent hub-arm extraction removed implementation bodies without removing causal gating or the exhaustive match. The sanitation removes duplicated formulas from the hub rather than replacing the dispatcher.

The current hub still owns Layouter state, checkpoint representation and capture/restore, the single occurrence Location gate, public façade signatures, bounded candidate coordination, local compatibility helpers, and terminal document assembly. Intermediate Page commitment is a distinct child operation; describing terminal composition in the hub is not permission to reabsorb that child. Child specs own their consumer's behavior and delegate only at named boundaries. Shared coordinate and callback invariants remain in the unchanged pinned nuclei.

The old counter-era narrative described locally updated counters and helpers. Those implementations no longer belong to the compatibility `counters.rs` consumer, which is empty apart from comments. Present display/callback adapters and introspector-backed reads have their own owners. The old P1031 counter scope is therefore historical, not a request to reintroduce mutable local counter state.

The Dynamic adapter historically introduced an explicit extension boundary. Retaining that already-existing boundary is different from converting native Content dispatch into dynamic dispatch. No vtable, string registry, PropMap or entity-side layout replacement is proposed.

## Fractional flow and typography investigations

P1373 recorded a full-region fractional Block losing a following line and a terminal Block producing an empty successor page. P1374 separated fixed reservation, finite fractional shares and terminal-tail handling. P1375 followed Styled/single-substantive wrappers; P1376 and P1379 investigated continuous and segmented column bases. These records explain the present bounded preflight; they do not prove arbitrary multi-region fraction distribution.

P1380–P1382 distinguished cosmetic wrappers, reduced singleton text and mixed-size baselines. P1383–P1387 tested above-outer size combinations, paragraph-to-block boundaries and overlong first words. P1388 rejected treating terminal whitespace/newline closing as an independent general geometry rule. P1389–P1392 isolated the certified single-wrap pairs, per-line extents and one-shot boundary/close hooks. Earlier hypotheses that reserved the global maximum twice, lost the terminal boundary, or treated every wrapper as a fixed line were superseded by those probes.

The current bounded certification matrix is preserved in Sequence. Runtime line closure belongs to line_commit; baseline/wrap/region progress belongs to Cursor; SubFrame isolates private arms. Neither the old step matrix nor its completion statuses authorizes flattening arbitrary content or broadening accepted style deltas.

The P1056/P1057 default investigations led to canonical font-size multipliers, not benchmark measurements: paragraph spacing and block spacing 1.2, paragraph leading 0.65. The fresh source anchors remain in vanilla_defaults. Equal numeric defaults do not imply every feature shares one spacing algorithm.

## Floats, flush markers and atomic suffix replay

P1292's successive flush investigations distinguished physical anchoring from clearance reservation and future-flow fitting. An earlier attempt could match page totals while leaving positions wrong; another kept a Place behind after moving only a line. Controls without a marker or float exposed missing Block spacing and ordinary Place anchoring, not a transaction defect.

The archived probes included a bottom float occupying 80pt on a 100pt page followed by 10pt flow: constructor-default 1.5em clearance required a later region compared with zero clearance. Other controls showed that changing clearance affected capacity rather than directly shifting an otherwise identical bottom anchor. These are imported probe dimensions and observations from the old P1292 record, not fresh results or proof against the current upstream pin.

The resulting architectural distinction remains important: Place measures/queues floats and owns ordered fitting, reservation and final positioning; Flush captures an occurrence prefix; Cursor distributes that prefix and marks a suffix boundary; flow_suffix accepts or rejects the whole following transaction; the hub captures/restores causal state and all append-only tails. A rejected suffix includes already-emitted Place material, callbacks, warnings, counters and positional maps, not only current_line.

Historical version labels, repeated RED/GREEN reports, completion claims and comparisons of internal buffer sizes are removed from L0. Existing observable regression controls remain in the test owner. No speculative future Content inspection, generic y-shift of accepted output or per-element rollback is introduced.

## Geometry, isolated frames and HTML embedding

Earlier geometry investigations distinguished local inner coordinates from composed page-global coordinates. P1493 identified semantic horizontal extent errors and traversal that discarded Group translations or affine matrices. The surviving measurement rule is to compose each supported ancestor transform once when observing geometry; a raw local coordinate is not a page-global parity observation.

P1404/P1408, P1119/P1120 and P1491–P1502 discussed boxed-frame baseline/extent and callback coordinates. The current empirical boxed calibration is an implementation compatibility constraint, not an independently renewed font-general parity claim. Baseline or clipping repairs require new evidence rather than an archived success status.

P1424 introduced intrinsic paged layout for HTML-frame content, separate from the semantic HTML target. P1486 investigated physical versus logical page context. P1493 investigated marker extraction through placement/transform wrappers and causal positions after actual geometry. Source origins, sentinel transport and final normalization are distinct operations: direct private markers must not create flow occupancy, but wrappers are not universally non-material.

Old byte/viewBox comparisons and illustrative marker coordinates are retained only as snapshot-specific motivations. They are not a requirement that L1 produce particular serialized SVG bytes, nor a proof that every affine wrapper now has bilateral parity.

Helpers retain general supported-item observation/translation, placement owns local composition, measurement owns constrained/intrinsic observation, page geometry owns finite auto dimensions, and HTML frame owns its private marker extraction path. The shared coordinate nucleus is unchanged.

## Equations and semantic carriers

P800 established inline math's text-baseline relationship rather than a math-axis displacement. P944/P945 refined math font weight/size, and P896/P987 investigated auto-width centering/numbering fixups. Later extent probes separated logical math advance, negative/prescript geometry and inline paragraph ascent/descent from Rust glyph advance sums.

P1292 investigated inline extent slack against paragraph leading. P1286/P1293 introduced or refined semantic Formula, Artifact and attachment carriers. The surviving contract uses the already-measured math body for composition, preserves the logical extent, and does not render twice to reconstruct bounds. Formula does not itself execute callbacks.

Historical “callbacks later” statuses are obsolete: current attempt coordination has fresh math and par.line passes and rejects tentative transcripts. Rich equation numbering still uses the current plain-text width projection and is not automatically proven by those callback milestones.

The old attachment signature claimed an Arc input/O(1) clone; the current consumer accepts a borrowed PdfAttachElem and creates a fresh Arc around its clone. That mechanical claim is removed without changing a public API.

## Images, shape paint, raw and localized model content

P768/P769 investigated image spacing and main-flow participation; P770 DPI handling; P771 contain/cover target clipping; P773 size metadata; P776 orientation; P1481 semantic alt/text carriers. These associations explain existing behavior and preserved restrictions, not arbitrary future image-format support.

Shape records P745/P748/P750/P762/P763/P767/P1227/P1228 described progressive geometry/paint support. Paint-family support is not equivalent to sampling everything into a solid color. A historical list of completed shape variants is replaced by each consumer's actual supported mapping.

P785a recorded raw highlighting through embedded syntax/theme data. Its earlier wholesale whitespace-preservation promise conflicts with the current token emission path and remains an open discrepancy, not a completed migration fact.

P1031/P1034 and later localization probes corrected obsolete local defaults such as a shortened figure prefix or an earlier outline title. Caption/reference/outline adapters now use their corresponding localization owners where the consumer does. The table-numbering extension still has its current English prefix and is not silently promoted into Figure semantics.

P1406/P1407/P1411 separated realized outline entry content from page-vector synthesis and repeat leaders. The old “no legacy synthesis” closure is not true for the current source: its fallback remains. Repeat owns leader spacing; Outline owns its actual realized/fallback branches.

P1458/P1459 investigated canonical figure/heading reference localization and causal target information. P1157 described a logical-page pattern promise that is not uniform in the current Page reference branch. Unknown ordinary reference labels now have a diagnostic, whereas unresolved page references retain their progressive fallback.

## Citations, bibliography and document format options

P418/P420/P1031 and P1456/P1457 documented supplied CSL rendering, escaping, numeric-range composition and fallback/localization investigations. P468/P472 and related fallback records are historical stages, not duplicate bibliography or citation specifications. Current cache interpretation belongs to bib_csl; visible bibliography/citation rendering belongs to the corresponding adapters; hub cache preparation and its compatibility body formatter remain where implemented.

Format-option stages P1462–P1469 described scalar target options, tagged/pretty policy, page ranges and PDF standards. Their “planned”/“gate pending” execution labels are obsolete: the current pure collector and validators exist. Root cascade, typed diagnostics and target independence remain normative, but exporter serialization, CLI policy and I/O are outside this layout family's ownership.

Tests retain observable content/geometry/error and static-delegation controls. Archived test totals, execution durations and benchmark notes are not renewed performance evidence. No new microbenchmark claim is made by this sanitation.

## Current contract discrepancies requiring separate decisions

The following are source-backed discrepancies or bounded-support questions, not authorization to alter behavior. The updated owners describe actual boundaries and explicitly leave missing former promises unresolved. Any correction that changes a public contract, default behavior, compatibility or pipeline phase needs the ADR-0127 gate.

| Area | Current evidence and unresolved issue | Preservation in this operation |
| --- | --- | --- |
| Counter compatibility | `01_core/src/compiler/layout/counters.rs` has no counter-update implementation. Old local counter helper ownership is obsolete. | Keep the compatibility consumer empty; display and runtime owners remain separate. |
| Outline | `01_core/src/compiler/layout/outline.rs:78` retains both realized entries and legacy fallback. | Do not document fail-closed absence as already implemented or delete fallback. |
| Table semantics | `01_core/src/compiler/layout/table.rs:174`, `:229`, `:243` build sequential rows, even page partitions and parallel visually empty carriers rather than wrapping visual rows. Footer rows/arbitrary header-level relations and measured-break correspondence remain incomplete. | Do not declare PDF/UA, assistive-technology or reflow parity. |
| Raw whitespace | `01_core/src/compiler/layout/raw.rs:103` emits through split_whitespace. | Keep the former language-level whitespace obligation open; do not bless normalization as parity. |
| Page references | `01_core/src/compiler/layout/references.rs:54` formats pattern references from the physical index, while the callback branch reads PageStore reference numbering. | Do not replace one branch or claim universal logical-page behavior. |
| Floating Place | `01_core/src/compiler/layout/place.rs:22` does not retain/apply dx/dy/scope in DeferredFloat, unlike the non-float path. | Keep missing support explicit; do not change private storage or defaults. |
| Curve paint | `01_core/src/compiler/layout/curve.rs:126` uses its current unfilled black stroke/default fill rule. | Do not promise unimplemented fill/stroke/fill-rule propagation. |
| Weak breaks | `01_core/src/compiler/layout/pagebreak.rs` and `colbreak.rs` do not inspect the weak flag in their adapters. | Preserve existing transition behavior, with no new weak-break semantics. |
| Link suffix safety | `01_core/src/compiler/layout/link.rs:18` drains the recorded pending-line suffix after rendering its body. A body that changes line storage needs separate safety evidence. | Do not claim unrestricted multipage/rich-body safety from the old prose. |
| Image clipping during translation | `01_core/src/compiler/layout/helpers.rs:144` and `01_core/src/compiler/layout/slicing.rs:84` have supported Image translation paths that reset clip_rect. | No blanket claim that all clipping metadata survives every helper. |
| Running content width | `01_core/src/compiler/layout/page_running.rs` keeps rendered numbering Content while measuring its plain-text projection. | Do not infer intrinsic rich-content width parity. |
| Other bounded geometry | Stack direction branches, Divider's symmetric margin expression and cell-tail's bounded continuation retain their current limits. | No general reverse-direction, asymmetric-margin or arbitrary-overflow promise. |

An existing defect or conflicting older requirement is not resolved merely by translating the description of current code. These questions remain open for a separately scoped L0/measurement/implementation cycle.

## Validation record

Read-only `crystalline-lint --checks v15,v26 .` reported no ownership or nucleus violations. A separate V5 SARIF inspection associated drift warnings with the assigned consumer paths, as expected because their unchanged lineage headers still declare the prior prompt hashes. V5 is configured as a warning: its zero exit status is not proof that drift is absent. Unrelated pre-existing V5 warnings also exist and are outside this task.

The byte fingerprints of every assigned consumer match the pre-edit snapshot. Each prompt's existing `Hash do Código` metadata and nucleus pin bullets were preserved exactly; canonical prompt references resolve. Scoped `git diff --check` passed. No step IDs or obsolete execution headings remain in the assigned L0 prose. The archive index is also the complete list of modified assigned prompts; this diagnostic is the only newly created document.

No Rust build/tests or global formatting is run. V5 drift is expected when L0 text changes while source lineage headers stay untouched. No global or scoped reseal is attempted because the authorized scope excludes Rust headers.

## Pre-edit prompt and consumer archive

Each entry identifies the pre-edit working-tree prompt text and the inspected consumer bytes. Historical identifiers and file:line references are retained as archival lookup keys, not reintroduced as normative step dependencies.

### 00_nucleo/prompts/compiler/layout-image.md

Pre-edit prompt SHA-256: `0447d7fa71df54ab9607a8731529df651e5e398d9da8f2e8eea0a836aeac08d8`.

Exclusive consumer: `01_core/src/compiler/layout/image.rs`; inspected SHA-256: `9f12c4b3a8ad6201a4392bf0f6e87034c549d18874d34d7b478e2bd603a6b1d4`.

Historical lookup identifiers: `P768`, `P770`, `P769`, `P771`, `P773`, `P776`, `P1481`.

### 00_nucleo/prompts/compiler/layout.md

Pre-edit prompt SHA-256: `f3644c4c856457cf5a881bccf72d71d353fbbc0bfe6ab083c92c53974a7073c1`.

Exclusive consumer: `01_core/src/compiler/layout/mod.rs`; inspected SHA-256: `17d980d72b39b64c5b36b43e10188164d0ba2d4d3b75123a806ba878187940e7`.

Historical lookup identifiers: `P1504`, `P1503`, `P1292`, `P1494`, `P1506`, `P1470`, `P1424`, `P1374`, `P1339`, `P1291`, `P1286`, `P240`, `P241`, `P1159`, `P1373`, `P1391`, `P1389`, `P1392`, `P1402`, `P1405`, `P1406`, `P1407`, `P1408`, `P1414`, `P1415`, `P1416`, `P1417`, `P1418`, `P1436`, `P1437`, `P1439`, `P1451`, `P1450`, `P1440`, `P1441`, `P1442`, `P1443`, `P1444`, `P1446`, `P1448`, `P1449`, `P1462`, `P1481`, `P1485`, `P1486`, `P1491`, `P1493`.

Historical source anchors: `cursor.rs:361-372`, `cursor.rs:1192-1303`, `cursor.rs:1087-1189`, `compiler/layout/cursor.rs:396-572`, `compiler/layout/mod.rs:151`, `cursor.rs:406-1038`, `01_core/src/compiler/layout/mod.rs:1865-1881`, `typst-html/src/rules.rs:88-91`, `typst-html/src/convert.rs:140-154`, `compiler/layout/mod.rs:1484-1501`, `01_core/src/compiler/layout/equation.rs:107-120`, `layout/mod.rs:2588-2618`, `01_core/src/compiler/layout/mod.rs:3327-3450`, `01_core/src/compiler/layout/mod.rs:2650-2665`, `01_core/src/compiler/layout/mod.rs:2516-2584`.

### 00_nucleo/prompts/compiler/layout/align.md

Pre-edit prompt SHA-256: `10391d6a684738f834ed77db6adb4876e630839e13364f0c1d79246de4d6bf23`.

Exclusive consumer: `01_core/src/compiler/layout/align.rs`; inspected SHA-256: `5c1b30f715559889eca2b2174c3c4e5200584281b5ef5e92d996f42ec1311085`.

### 00_nucleo/prompts/compiler/layout/bib_csl.md

Pre-edit prompt SHA-256: `f938a45fb6557998d02106d98df2879f8a092783f86aa2da1be702a4d78a4a16`.

Exclusive consumer: `01_core/src/compiler/layout/bib_csl.rs`; inspected SHA-256: `741325e68722499c1bc6b7722b276e2dc5aaaa20452dce8a7cbc77dab7df4590`.

Historical lookup identifiers: `P418`, `P1031`, `P420`, `P1457`, `P1456`.

Historical source anchors: `crates/typst-library/src/model/cite.rs:133-147`, `crates/typst-library/src/model/bibliography.rs:159-163`, `bibliography.rs:79-95`.

### 00_nucleo/prompts/compiler/layout/bibliography.md

Pre-edit prompt SHA-256: `58d07e1c4245cafc9fc68bc6170b033cf5b78dc8d28d0e27219a1a816400ec66`.

Exclusive consumer: `01_core/src/compiler/layout/bibliography.rs`; inspected SHA-256: `b0ebfffdacb92ee119197a9483e65e2e60ff34f03579b76c078fc6a4c987aea5`.

Historical lookup identifiers: `P468`, `P1031`, `P472`, `P159G`, `P420`.

Historical source anchors: `crates/typst-library/src/model/bibliography.rs:159-163`, `bibliography.rs:79-95`, `bibliography.rs:159-163`.

### 00_nucleo/prompts/compiler/layout/block.md

Pre-edit prompt SHA-256: `815048276cab1853ee92fde3c36760e54b3dacedcad1ee3963592c4d4b32c1e8`.

Exclusive consumer: `01_core/src/compiler/layout/block.rs`; inspected SHA-256: `2c4c9386ced2e95918f5a60a747c9ef2d4807970ca15919e28d99f6a1346061c`.

Historical lookup identifiers: `P1373`, `P1372`, `P1374`, `P1292`.

Historical source anchors: `flow/block.rs:323-331`, `layout/container.rs:347-360`, `flow/collect.rs:232-278`.

### 00_nucleo/prompts/compiler/layout/boxed.md

Pre-edit prompt SHA-256: `6eb0732864f39b0752d36ff8dadc596d565a9f8f7f6b9de11edb21d46616fd15`.

Exclusive consumer: `01_core/src/compiler/layout/boxed.rs`; inspected SHA-256: `b9b867bcf149bd8a2e113b99f31348824212cd2ab353160cd3d9b998dfb652c7`.

Historical lookup identifiers: `P1408`, `P1404`, `P1119`, `P1120`, `P1491`, `P1493`, `P1502`, `P1407`, `P1498`.

Historical source anchors: `03_infra/src/pipeline.rs:4012-4180`, `boxed.rs:145-181`, `repeat.rs:32-51`, `cursor.rs:489-519`.

### 00_nucleo/prompts/compiler/layout/cell_tail_commit.md

Pre-edit prompt SHA-256: `4020cf15bd76237d54646c501c18426f1c1563881ff067f297d9b6dd7361c447`.

Exclusive consumer: `01_core/src/compiler/layout/cell_tail_commit.rs`; inspected SHA-256: `7ad281e27536253b9db904977bfd279cd648d1bb14d2dabe4c99049e0139adb3`.

Historical lookup identifiers: `P1504`, `P1503`.

Historical source anchors: `compiler/layout/cursor.rs:1087-1189`.

### 00_nucleo/prompts/compiler/layout/cite.md

Pre-edit prompt SHA-256: `def39ac9cb9b9e0ade736dd42a3242b628d236dfad9cf11c50ca4136cdd15ddf`.

Exclusive consumer: `01_core/src/compiler/layout/cite.rs`; inspected SHA-256: `f38efbdda395a8498a529f44f9cdeedfc43ad784be962bd3409c07efafaa533c`.

### 00_nucleo/prompts/compiler/layout/colbreak.md

Pre-edit prompt SHA-256: `456b2f8d69d6cbad204048a1e1ab0771a3d392cbfd2ba6ad32f987f8e4702192`.

Exclusive consumer: `01_core/src/compiler/layout/colbreak.rs`; inspected SHA-256: `5513f25be1248570c9bb8c3e1835b5f69a9dc6793b521fa1040478e0a71e4b95`.

### 00_nucleo/prompts/compiler/layout/context_block.md

Pre-edit prompt SHA-256: `7df92d59f8d106e3dc89ebcdc04cfa4f6895a2d15dea0c2afbd6a1fcd19585d0`.

Exclusive consumer: `01_core/src/compiler/layout/context_block.rs`; inspected SHA-256: `78b4ed581afe3caf339c66cc92401ccd6ecfd0819121f6f6fee2bdfada8eb5a0`.

### 00_nucleo/prompts/compiler/layout/counter_display.md

Pre-edit prompt SHA-256: `982824e0a9d6bf426449b85f18f26142a5c3027cacfe57376d746718a4cc9d87`.

Exclusive consumer: `01_core/src/compiler/layout/counter_display.rs`; inspected SHA-256: `567b98840cf12caebc79c87c010c9d6da9167dea135005dbf719434754141d69`.

### 00_nucleo/prompts/compiler/layout/counter_display_callback.md

Pre-edit prompt SHA-256: `5091915a33f4399bc24126d31ee011492154a0030453ce256b8e16cb7d520ca8`.

Exclusive consumer: `01_core/src/compiler/layout/counter_display_callback.rs`; inspected SHA-256: `6bb2148cd8722d92de290689d36fc7f400dd82cdebcedceef0293f53b7f10cfe`.

### 00_nucleo/prompts/compiler/layout/cursor.md

Pre-edit prompt SHA-256: `b87a746c56e5127ec55a3f8b9c86827bebfc44a6acfdbf47f39b43a60d6fc7ee`.

Exclusive consumer: `01_core/src/compiler/layout/cursor.rs`; inspected SHA-256: `030948b7287f41d4b0ff157aeedbacfd6eb8ca6405ab6ac51c05b6c3c9a1d35b`.

Historical lookup identifiers: `P1503`, `P1504`, `P1292`, `P1382`, `P762`, `P1120`, `P1381`, `P1374`, `P1373`, `P1386`, `P1385`, `P1384`, `P1387`, `P1389`, `P1383`, `P1388`, `P1391`, `P1390`, `P1392`, `P1407`, `P1408`, `P1415`, `P1414`, `P1416`, `P1417`, `P1418`, `P1419`, `P1486`, `P1493`.

Historical source anchors: `cursor.rs:361-372`, `cursor.rs:35-65`, `cursor.rs:1192-1303`, `cursor.rs:1087-1189`, `cursor.rs:624-737`, `cursor.rs:361-618`, `cursor.rs:406-1038`, `cursor.rs:1039-1764`, `inline/line.rs:562-632`, `flow/distribute.rs:514-521`, `01_core/src/compiler/layout/cursor.rs:1229-1245`, `01_core/src/compiler/layout/cursor.rs:1100-1170`.

### 00_nucleo/prompts/compiler/layout/curve.md

Pre-edit prompt SHA-256: `c15b2a6624fa5a9487d5f4d2b97e47d622745039bc0a845197b0e8fe6656b36e`.

Exclusive consumer: `01_core/src/compiler/layout/curve.rs`; inspected SHA-256: `aeecad9162e1e6de0813545df10bfe889fabdccc7d99e6d29d799971c1e7d2d6`.

Historical lookup identifiers: `P1226`.

### 00_nucleo/prompts/compiler/layout/decorations.md

Pre-edit prompt SHA-256: `d718cf34aee7cb7c6c85d71b7392665319123a70fcba5427c7afb8f3120dc43c`.

Exclusive consumer: `01_core/src/compiler/layout/decorations.rs`; inspected SHA-256: `7d5d360688ea4639a00fc9c1a0e55098f25af1a9743600cb28b7d023f7ee3cbb`.

### 00_nucleo/prompts/compiler/layout/divider.md

Pre-edit prompt SHA-256: `d493003c735e1ab758b50e41787825fdd732dd4f1240934316608b0243c51d1b`.

Exclusive consumer: `01_core/src/compiler/layout/divider.rs`; inspected SHA-256: `3d2d210577f8a65c417ba030327ced7b1330d673a87803fc83d7a1e2cae40749`.

### 00_nucleo/prompts/compiler/layout/dynamic.md

Pre-edit prompt SHA-256: `87fb267d6d32b614352a4dd3dcb397d10968735b8eef12855f8668a97c80bd3e`.

Exclusive consumer: `01_core/src/compiler/layout/dynamic.rs`; inspected SHA-256: `df92fe875a217a43b8317057846d1479bf06c6b13492654bc2b99757c14bdf95`.

### 00_nucleo/prompts/compiler/layout/enum_item.md

Pre-edit prompt SHA-256: `3e43d490e439e5976b7d39cd6c2de1cb20354a76ed0b6f664e065dba482ac220`.

Exclusive consumer: `01_core/src/compiler/layout/enum_item.rs`; inspected SHA-256: `e8d61ed803bcb3c39b107506363f8575928b664d6f83944b219574692148e579`.

Historical lookup identifiers: `P380`, `P505`, `P864`, `P762`, `P1072`, `P1031`.

Historical source anchors: `enum.rs:152-154`, `enum.rs:149-150`, `enum.rs:89-90`, `enum.rs:68-72`, `enum.rs:74-78`, `crates/typst-library/src/model/enum.rs:74-78`.

### 00_nucleo/prompts/compiler/layout/equation.md

Pre-edit prompt SHA-256: `3914f4e9805935c6d08c87913397e73f970a58c9a7178fe02e3c551a30cfb5bc`.

Exclusive consumer: `01_core/src/compiler/layout/equation.rs`; inspected SHA-256: `4c955ea3d5bf0cd383c345dd3ce51be463428e6dfa7eeea1ee0547ee754d237f`.

Historical lookup identifiers: `P800`, `P813`, `P250`, `P1140`, `P896`, `P987`, `P451`, `P784`, `P893`, `P891`, `P895`, `P944`, `P921`, `P912`, `P945`, `P952`, `P1292`, `P1293`, `P967`, `P772y`, `P1291`, `P1493`.

Historical source anchors: `lab/typst-original/crates/typst-library/src/math/equation.rs:190`, `lab/typst-original/crates/typst-layout/src/flow/distribute.rs:589`, `lab/typst-original/crates/typst-library/src/layout/container.rs:342`, `lab/typst-original/crates/typst-layout/src/rules.rs:807`, `lab/typst-original/crates/typst-library/src/math/equation.rs:197-201`, `lab/typst-original/crates/typst-layout/src/math/mod.rs:217`, `lab/typst-original/crates/typst-layout/src/math/scripts.rs:306-310`, `lab/typst-original/crates/typst-library/src/math/equation.rs:189-195`, `flow/collect.rs:253-278`, `flow/distribute.rs:185-199`, `01_core/src/compiler/layout/equation.rs:128-145`, `typst-layout/src/math/mod.rs:49-101`, `typst-layout/src/inline/line.rs:559-633`, `typst-library/src/math/ir/resolve.rs:24-35`, `math/mod.rs:64-72`, `typst-layout/src/math/mod.rs:209-330`, `mod.rs:217`, `typst-pdf/tags/resolve/mod.rs:303-306`.

### 00_nucleo/prompts/compiler/layout/flow_suffix.md

Pre-edit prompt SHA-256: `b87a2e0d154a9b515e28859de045e48bb85b78055b8c2e94389e485736d8afcf`.

Exclusive consumer: `01_core/src/compiler/layout/flow_suffix.rs`; inspected SHA-256: `d6af784b0b42020b16758cd7db842984ae6f68343055d9601d3798c444584a74`.

Historical lookup identifiers: `P1504`, `P1503`.

Historical source anchors: `compiler/layout/cursor.rs:361-372`, `compiler/layout/mod.rs:797-1417`.

### 00_nucleo/prompts/compiler/layout/flush.md

Pre-edit prompt SHA-256: `7dd21b2522f4df15ba22705f5e2d6c5bd02f5df39d4caa4d51f14fa08ed17bc7`.

Exclusive consumer: `01_core/src/compiler/layout/flush.rs`; inspected SHA-256: `82d26c68eaaeca19a1678258c305dbaf846edb7095dcc4c0d60770eb23efc8d1`.

Historical lookup identifiers: `P1292`.

### 00_nucleo/prompts/compiler/layout/footnote.md

Pre-edit prompt SHA-256: `f14d0ebd3424498435dae1a9cbb54d080a46e1a90eb82c86dff369b78e91b19f`.

Exclusive consumer: `01_core/src/compiler/layout/footnote.rs`; inspected SHA-256: `ba402edffc930f9a236a5da08fc8f46568c5effe3f7b0d5b28578ba0cec361fb`.

Historical lookup identifiers: `P381`, `P1016`, `P793`, `P844`, `P448`, `P1031`.

Historical source anchors: `crates/typst-library/src/model/footnote.rs:79-80`, `footnote.rs:72-78`, `crates/typst-library/src/model/footnote.rs:24-31`, `footnote.rs:324`, `footnote.rs:303`.

### 00_nucleo/prompts/compiler/layout/format_options.md

Pre-edit prompt SHA-256: `165f02608bcb004b39a32ee197ceebfbe161832e29f34ec1f4bccedfa5ba854a`.

Exclusive consumer: `01_core/src/compiler/layout/format_options.rs`; inspected SHA-256: `15d2d952f03ddf0775609dbc7274cbea6503fc825c1777801414a0f896118e25`.

Historical lookup identifiers: `P1462`, `P1463`, `P1465`, `P1467`, `P1468`, `P1469`, `P1466`.

Historical source anchors: `lab/typst-original/crates/typst-realize/src/lib.rs:603-670`, `typst-realize/src/lib.rs:603-670`.

### 00_nucleo/prompts/compiler/layout/grid.md

Pre-edit prompt SHA-256: `2739932ffbd4dcf38765c2d455cb4399696dddfcf9e038f9fd793ea4c80528ff`.

Exclusive consumer: `01_core/src/compiler/layout/grid.rs`; inspected SHA-256: `c243d3faa60cb659fc5ac7e4125a75143d3048b007ead232eb2bee3741f65efc`.

### 00_nucleo/prompts/compiler/layout/grid_cell.md

Pre-edit prompt SHA-256: `cca006eb344e9a413fff8c49a69005498dd248d46eb4f4530bf8725be8424dcd`.

Exclusive consumer: `01_core/src/compiler/layout/grid_cell.rs`; inspected SHA-256: `a9f54c26ead5d3fe520ef05eb2dcb7310bc3222b5f7e1c9a07dbcaad2054b4c5`.

### 00_nucleo/prompts/compiler/layout/grid_footer.md

Pre-edit prompt SHA-256: `748667460c9fce18cf5fd6577ecf81dd7ec9396502cafb541901a2d8ff849a5c`.

Exclusive consumer: `01_core/src/compiler/layout/grid_footer.rs`; inspected SHA-256: `c7320e97802b4b646c4fce926d9089d7b7147b6711d76a313dbdeca0cddeb21c`.

### 00_nucleo/prompts/compiler/layout/grid_header.md

Pre-edit prompt SHA-256: `6dba31e1e8825636a95703627bcdf15ee6eccc9b301f47f21791bd84200cdbc1`.

Exclusive consumer: `01_core/src/compiler/layout/grid_header.rs`; inspected SHA-256: `750e27e378a38542e5d0ff3a927fc8221d6b5c70d37bb3ddf98873669c0d95fc`.

### 00_nucleo/prompts/compiler/layout/grid_placement.md

Pre-edit prompt SHA-256: `9eaf9c4ec0b4f92efae548af0a2f9bbdfd55fc0c008ba681cd3a27190da41035`.

Exclusive consumer: `01_core/src/compiler/layout/grid_placement.rs`; inspected SHA-256: `74d453fcad6abb599976f34660e74f64434968575df1cdb29f16388d717130fd`.

### 00_nucleo/prompts/compiler/layout/h_space.md

Pre-edit prompt SHA-256: `b576da784367615580bb7face6bf6e57d4336d18f5a163b105eb4375d2b6cebb`.

Exclusive consumer: `01_core/src/compiler/layout/h_space.rs`; inspected SHA-256: `31009aa203cc148d2db1e6171164dd995c80eeb4b6c02de0dd1f14390112fb5c`.

### 00_nucleo/prompts/compiler/layout/heading.md

Pre-edit prompt SHA-256: `b6bd1d86ae39132215aad2342d8bbdfedd602306a4c51f189b42805bec1b347d`.

Exclusive consumer: `01_core/src/compiler/layout/heading.rs`; inspected SHA-256: `a1f13549373ae35750440554bdfc7269afc26c6a0691569ac841b208726f0188`.

Historical lookup identifiers: `P451`, `P1031`, `P1036`, `P241`, `P978`, `P975`, `P1033`.

Historical source anchors: `crates/typst-library/src/model/heading.rs:288-308`, `01_core/src/compiler/layout/helpers.rs:260-266`, `lab/typst-original/crates/typst-library/src/model/heading.rs:281-285`, `heading.rs:288-289`, `compiler/introspect/heading.rs:34`, `compiler/introspect.rs:1105-1108`.

### 00_nucleo/prompts/compiler/layout/helpers.md

Pre-edit prompt SHA-256: `7e6ea4a0f893b658456e31cfa66005d3e9efd96be6280ea3dfb429be286c3d02`.

Exclusive consumer: `01_core/src/compiler/layout/helpers.rs`; inspected SHA-256: `649cfd5f8320b36c8f8bbcc23505491d6a1378fc5d1f7b310702aaee3dc7e6d5`.

Historical lookup identifiers: `P1493`.

Historical source anchors: `layout/html_frame.rs:30-45`, `helpers.rs:85-95`.

### 00_nucleo/prompts/compiler/layout/hide.md

Pre-edit prompt SHA-256: `9c7bbb3c4b63c66a1adbfc8202dc2a868c2eab72a7e6bf28f0b3f92882d64b2a`.

Exclusive consumer: `01_core/src/compiler/layout/hide.rs`; inspected SHA-256: `9ac9bb344a48cb7bb31b38883ecdcbe80f2be890fa934dec1ccb6b91793963c8`.

### 00_nucleo/prompts/compiler/layout/html_frame.md

Pre-edit prompt SHA-256: `c03177b5c785d32b6ddd67ae9d899f7ad1274540dc239a7448e9d66006f13bee`.

Exclusive consumer: `01_core/src/compiler/layout/html_frame.rs`; inspected SHA-256: `2b3d408db05abe8dad3e71c16b7c1405d7a2ea7054ab9e2b626df080206eca32`.

Historical lookup identifiers: `P1424`, `P1486`, `P1493`.

Historical source anchors: `typst-html/src/rules.rs:88-91`, `typst-html/src/convert.rs:140-154`, `01_core/src/compiler/layout/html_frame.rs:74-86`.

### 00_nucleo/prompts/compiler/layout/hyphenation.md

Pre-edit prompt SHA-256: `24c8ac168b1474043d49b1befdbc15edd69e661491ec0ad2cc4e9ecbdf675a25`.

Exclusive consumer: `01_core/src/compiler/layout/hyphenation.rs`; inspected SHA-256: `a3b854099b67c7de48eafb7f83d155e1da5643cdc581d13cb464bb7f044408b8`.

### 00_nucleo/prompts/compiler/layout/line_commit.md

Pre-edit prompt SHA-256: `3110a220ad4d50a496eb074e11aedfaf2be473a2fecff49618cb7665f8e6fd2f`.

Exclusive consumer: `01_core/src/compiler/layout/line_commit.rs`; inspected SHA-256: `1afe023a9901c6b6b1de87b98cea41884848702e8b56f34d53c48ae14ca83292`.

Historical lookup identifiers: `P1503`, `P1382`, `P1389`, `P1408`, `P1414`, `P1419`, `P1493`, `P1391`, `P1392`.

Historical source anchors: `01_core/src/compiler/layout/cursor.rs:406-1038`, `compiler/layout/mod.rs:326-1417`.

### 00_nucleo/prompts/compiler/layout/linebreak.md

Pre-edit prompt SHA-256: `c37ccf195edf48ee694cd940005182deb93b08c278c26b802f1549c2c82785e5`.

Exclusive consumer: `01_core/src/compiler/layout/linebreak.rs`; inspected SHA-256: `9cf7bc5c60585d4ff1943c07b7b6e6346b526a00173257168e9a977945178fbc`.

### 00_nucleo/prompts/compiler/layout/link.md

Pre-edit prompt SHA-256: `029359d007b74a5651c8028844f492a53a627f7372feccf5fad0f4f71670cb04`.

Exclusive consumer: `01_core/src/compiler/layout/link.rs`; inspected SHA-256: `e5dd0f5ebb026b68a1fe8c37f3c58c4eed70bc95f8b70de132b6179d657f9948`.

Historical lookup identifiers: `P422`, `P424`, `P1031`, `P1493`.

Historical source anchors: `link.rs:24`, `link.rs:41-49`, `link.rs:175-195`, `link.rs:259-262`, `link.rs:26-27`, `link.rs:51-103`.

### 00_nucleo/prompts/compiler/layout/list_item.md

Pre-edit prompt SHA-256: `07f34ce26547c3f44af33628dd682bf2b9eb04a83e47627cd9b8ad1da33c6579`.

Exclusive consumer: `01_core/src/compiler/layout/list_item.rs`; inspected SHA-256: `27b0e6145be19ee73a7b017ae08ca1e4fa792b3e737fefe91822afc0364caf7f`.

Historical lookup identifiers: `P380`, `P505`, `P864`, `P762`, `P1072`, `P1031`, `P504`.

Historical source anchors: `list.rs:100-102`, `list.rs:97-98`, `list.rs:65-66`, `list.rs:88-95`, `list.rs:52-64`.

### 00_nucleo/prompts/compiler/layout/measurement.md

Pre-edit prompt SHA-256: `bdaf1c86b727acf674aba1460fd6a322a3227c52dcfff3c1cbe7a89abe2e4435`.

Exclusive consumer: `01_core/src/compiler/layout/measurement.rs`; inspected SHA-256: `494322460b68d484d585fb464aee24408d4554341ff9857cb83a5552ac78a3df`.

### 00_nucleo/prompts/compiler/layout/metrics.md

Pre-edit prompt SHA-256: `0b159dd9d38d1f12123acc5d44d58a17c0569ab7af7439b1d467d69ab03f68df`.

Exclusive consumer: `01_core/src/compiler/layout/metrics.rs`; inspected SHA-256: `9577d8faf8864d14f9580a478a4ed8e7f20dc69348342a68f6915e9f08a7cb49`.

Historical lookup identifiers: `P1493`.

Historical source anchors: `metrics.rs:269-317`.

### 00_nucleo/prompts/compiler/layout/outline_entry.md

Pre-edit prompt SHA-256: `13653efbebdd7cbb0806911992711a738a00b2beaf1389e41c6c260bc7bc7f83`.

Exclusive consumer: `01_core/src/compiler/layout/outline_entry.rs`; inspected SHA-256: `a73c9d8475ab9ae60b19fdba105486d0579a3ee1c804b7809d163a80aa8b24cd`.

### 00_nucleo/prompts/compiler/layout/pad.md

Pre-edit prompt SHA-256: `07e0e4d8f06572b47c98b15240aac3ba7b4d15513a84ea0235e3c51b03fdc007`.

Exclusive consumer: `01_core/src/compiler/layout/pad.rs`; inspected SHA-256: `9ba162df017e0b23a5139b59642db38d646eda09d333b73d6d59a1e921000b18`.

### 00_nucleo/prompts/compiler/layout/page_canvas.md

Pre-edit prompt SHA-256: `a8f6876f8706675a9d9ac680361a005864430ec3363b76af966a12ae24a4bb84`.

Exclusive consumer: `01_core/src/compiler/layout/page_canvas.rs`; inspected SHA-256: `c3bdc2f0928cfcb2409da4a4bd3427e39289fdc3e95856347866fd5acdabff1c`.

### 00_nucleo/prompts/compiler/layout/page_commit.md

Pre-edit prompt SHA-256: `5ef64091f6e52bce3aabd421eb58bc91cfae50964d32730b022a009b4ffdbee6`.

Exclusive consumer: `01_core/src/compiler/layout/page_commit.rs`; inspected SHA-256: `558eba176752d381f81851a07eed229283c5422e834bd1a11de8237782bcf634`.

Historical lookup identifiers: `P1504`, `P1503`, `P1486`.

Historical source anchors: `compiler/layout/cursor.rs:361-618`.

### 00_nucleo/prompts/compiler/layout/page_geometry.md

Pre-edit prompt SHA-256: `da9463e63434c34f5a55b35cb86cb61f20a4c3bffe7ac37c7f2ead45e09f292a`.

Exclusive consumer: `01_core/src/compiler/layout/page_geometry.rs`; inspected SHA-256: `51851eacb6edb047ecaf6a2a2721ecb9e02dddf90f8421fa941969ec7986bc1d`.

### 00_nucleo/prompts/compiler/layout/page_run.md

Pre-edit prompt SHA-256: `abc6f2e4aec6ffc106ed12b7efe9ad7687c30a134992b961b168dcfbf08afd51`.

Exclusive consumer: `01_core/src/compiler/layout/page_run.rs`; inspected SHA-256: `1e2ca1585886a7722ea16750b6e559a0a3f44c42b06d9be457b1a2009ad484f0`.

Historical lookup identifiers: `P1493`.

### 00_nucleo/prompts/compiler/layout/page_running.md

Pre-edit prompt SHA-256: `93eaaa89de37743d3cff3545e05ef58ca00a5dfa676b288fcd601114887f4af7`.

Exclusive consumer: `01_core/src/compiler/layout/page_running.rs`; inspected SHA-256: `c60ec9a6f5d88f5bec6f14fb56f0c0f8ebed6609b11a81c06d94c280e3dadb80`.

### 00_nucleo/prompts/compiler/layout/pagebreak.md

Pre-edit prompt SHA-256: `b721744f0d430d6a79f312b24a679a0c408c961d1107fa19e44d087038155716`.

Exclusive consumer: `01_core/src/compiler/layout/pagebreak.rs`; inspected SHA-256: `0b3c3516f12e5713e5e14372d66290f88cbd3fb5d395fce1eccd1c39cd74ba12`.

### 00_nucleo/prompts/compiler/layout/par_line_callbacks.md

Pre-edit prompt SHA-256: `7a3de5972a789b06fa00d543104ac085df2bfd3b752498d222cd7f9407f01182`.

Exclusive consumer: `01_core/src/compiler/layout/par_line_callbacks.rs`; inspected SHA-256: `84196c32c5b919e40b4c046368a19886a4952e5b1c844c43909cbedd0e76a14b`.

### 00_nucleo/prompts/compiler/layout/parbreak.md

Pre-edit prompt SHA-256: `8d48f53cbbc415728a301cb0c453e619d10443805d1f0552d59f06c47875b138`.

Exclusive consumer: `01_core/src/compiler/layout/parbreak.rs`; inspected SHA-256: `b884c4b47cc9a18c9daaacc674aaf3fae9ffde514fd4fc919e72ee7fb4992e15`.

### 00_nucleo/prompts/compiler/layout/pdf_artifact.md

Pre-edit prompt SHA-256: `332ad01cad05700029040ecdb2448f3d5a82e783804e569f1c8757d89cc5d072`.

Exclusive consumer: `01_core/src/compiler/layout/pdf_artifact.rs`; inspected SHA-256: `b618eff4b99fa8bfcd0f0a17ac3764f589a7d018538ab4ff821d531940ee69ec`.

Historical lookup identifiers: `P1293`, `P1493`.

### 00_nucleo/prompts/compiler/layout/pdf_attach.md

Pre-edit prompt SHA-256: `ea37a1a2d8e5246abb2cd977c06945d4cc44d13b40f9d513dbcaec4a058f31fa`.

Exclusive consumer: `01_core/src/compiler/layout/pdf_attach.rs`; inspected SHA-256: `9e02de7a84a22709eb9581d641ff90bfb1d160ecf75975432498df4c057e3cf3`.

Historical lookup identifiers: `P1293`.

### 00_nucleo/prompts/compiler/layout/place.md

Pre-edit prompt SHA-256: `0030161e4a921385d3c49d5167e820dedad476ef810190acff64104125344822`.

Exclusive consumer: `01_core/src/compiler/layout/place.rs`; inspected SHA-256: `1a1c8aabb9098eb5c3a7729c7b5ba0e48040c7ac92efda70be64b0db2e9a8241`.

Historical lookup identifiers: `P1504`, `P1503`, `P1292`, `P1493`.

Historical source anchors: `compiler/layout/cursor.rs:35-65`, `layout/place.rs:195-245`, `flow/collect.rs:281-331`, `flow/compose.rs:298-377`, `flow/distribute.rs:504-510`, `flow/distribute.rs:630-662`.

### 00_nucleo/prompts/compiler/layout/placement.md

Pre-edit prompt SHA-256: `08bf4436f5a6f2b713513fd08e9598807b9aca14d99705e3ee7d794a77704474`.

Exclusive consumer: `01_core/src/compiler/layout/placement.rs`; inspected SHA-256: `5a3a3103859a575f8d279916969cfbd40a59bf61623480f2fcf1f4071fed4f35`.

### 00_nucleo/prompts/compiler/layout/quote.md

Pre-edit prompt SHA-256: `e61a31ec69f1b9c216fe1cdab2439f4c363a2718fd431697696f36f2f0058d06`.

Exclusive consumer: `01_core/src/compiler/layout/quote.rs`; inspected SHA-256: `a98f0f6b5a42f9b7c4c853b1570cbd4ceed1f3c8f2c4bab29007c51b24002e9d`.

### 00_nucleo/prompts/compiler/layout/raw_highlight.md

Pre-edit prompt SHA-256: `613364f5ba9ae79ccd50d755e73b46763293a1099eba81e844a7c63594593bd0`.

Exclusive consumer: `01_core/src/compiler/layout/raw.rs`; inspected SHA-256: `0e01cabd138f2f10fbc8c348ff383fe7e129c39540c6f30eb4cd2bb40b42b264`.

Historical lookup identifiers: `P785a`.

### 00_nucleo/prompts/compiler/layout/repeat.md

Pre-edit prompt SHA-256: `15b85efccf7ce8d37cb16c74216df11c9a0f9b3193039a0915af063b42779558`.

Exclusive consumer: `01_core/src/compiler/layout/repeat.rs`; inspected SHA-256: `c30c5c19b3234d9a8b0d147da61399587dcafc0d8ec4046e92c19fd6d8bb6c4b`.

Historical lookup identifiers: `P1407`.

### 00_nucleo/prompts/compiler/layout/sequence.md

Pre-edit prompt SHA-256: `8c180bc4d69f451f67fb31a31ec9cdbc2a4e71f0b9c7330672c1d8f487b18bb5`.

Exclusive consumer: `01_core/src/compiler/layout/sequence.rs`; inspected SHA-256: `5edffa26a9ae4bf0e103bd9b3b22c13f162a3b59a8011b381c56d7bff5686c09`.

Historical lookup identifiers: `P1494`, `P1374`, `P1375`, `P1376`, `P1379`, `P1378`, `P1380`, `P1381`, `P1383`, `P1382`, `P1384`, `P1385`, `P1386`, `P1387`, `P1388`, `P1389`, `P1390`, `P1391`, `P1392`, `P1493`.

Historical source anchors: `01_core/src/compiler/layout/sequence.rs:45-55`, `flow/collect.rs:244-264`, `flow/distribute.rs:589-681`, `flow/compose.rs:230-249`, `flow/collect.rs:128-145`, `flow/collect.rs:240-286`.

### 00_nucleo/prompts/compiler/layout/shape_block_behaviour.md

Pre-edit prompt SHA-256: `f064ba47d34d4f899051f58c4afa6d31b06ea6bcff135aa4de87f00f95b1ca75`.

Exclusive consumer: `01_core/src/compiler/layout/shape.rs`; inspected SHA-256: `4a388fbc6a46da93e663eb6e962ad80b11277353a25e5941152ef55b7ecf34fc`.

Historical lookup identifiers: `P767`, `P763h`, `P1227`, `P1228`, `P748`, `P750`, `P767c`, `P767a`, `P1031`, `P250`, `P763f`, `P745`, `P762`.

Historical source anchors: `01_core/src/compiler/layout/shape.rs:129`, `lab/typst-original/crates/typst-layout/src/engine.rs:765-803`, `container.rs:342-354`.

### 00_nucleo/prompts/compiler/layout/slicing.md

Pre-edit prompt SHA-256: `41d7bc630220ff8a6eaa7c1464eaa202312f34106f326f1b539286c806d43436`.

Exclusive consumer: `01_core/src/compiler/layout/slicing.rs`; inspected SHA-256: `e2551e5a59bf8595a36c934544aa09f46f60ba406a6d0b4ef97f871cdcdff002`.

### 00_nucleo/prompts/compiler/layout/smartquote.md

Pre-edit prompt SHA-256: `bec38936f98c9d5837893bc73227a126cae64c63c0668e9f79292c3d7c9bddec`.

Exclusive consumer: `01_core/src/compiler/layout/smartquote.rs`; inspected SHA-256: `4566f6dd6cfe2cb99025f9bbd0270b91dac1d144ae970b4229d33c411bb48943`.

Historical lookup identifiers: `P1286`.

Historical source anchors: `text/smartquote.rs:201-317`.

### 00_nucleo/prompts/compiler/layout/space.md

Pre-edit prompt SHA-256: `f36ce4e62ce5d21f75d53a3f990c4d5c76070600d1413a7b81e3691d761f2585`.

Exclusive consumer: `01_core/src/compiler/layout/space.rs`; inspected SHA-256: `5f80957d21f27bc3e91a0e7377ff2211c7101e9094386ae4513efa50af9312ea`.

### 00_nucleo/prompts/compiler/layout/stack.md

Pre-edit prompt SHA-256: `1b23efd329af7f168df70de0df99f8c647c455c756cdaba2b63f370b07292549`.

Exclusive consumer: `01_core/src/compiler/layout/stack.rs`; inspected SHA-256: `a07355c11afe52586307ee1c8dd5170e5e85d86ab5e2f261b2cc099872bbe6ad`.

Historical lookup identifiers: `P1493`.

Historical source anchors: `stack.rs:25-87`.

### 00_nucleo/prompts/compiler/layout/state_display.md

Pre-edit prompt SHA-256: `bc9876b9959c95577d6a5da62993a6235085247a586f59985a3fc6b2ccd2fa68`.

Exclusive consumer: `01_core/src/compiler/layout/state_display.rs`; inspected SHA-256: `dcc6ab1157af2b823ee848afbb5b86e29407e6173c8919004bd2dab6e41c6846`.

### 00_nucleo/prompts/compiler/layout/style_scope.md

Pre-edit prompt SHA-256: `2f0ff3c7767d844204d4974bbf00c5c23ca4db744e2f61ffb4ce737c883e1d77`.

Exclusive consumer: `01_core/src/compiler/layout/style_scope.rs`; inspected SHA-256: `4ed1eb6187a19987b832b4fe973721c65923943d687f043b668edc1076bdb801`.

### 00_nucleo/prompts/compiler/layout/sub_frame.md

Pre-edit prompt SHA-256: `43483cd3471095d3f75cb113ee57846070dc35eefed3af7a8c02c39354fe20f9`.

Exclusive consumer: `01_core/src/compiler/layout/sub_frame.rs`; inspected SHA-256: `295c622dd09af9a4f6c019efa0cee8fa4597ca9f0ac68f2c785dd49666d44361`.

Historical lookup identifiers: `P1391`, `P1392`, `P1493`.

### 00_nucleo/prompts/compiler/layout/table.md

Pre-edit prompt SHA-256: `03828cdffd79870d7bd68060ccbe5fc0bdee49863a4039e12834c38f29e157d3`.

Exclusive consumer: `01_core/src/compiler/layout/table.rs`; inspected SHA-256: `a348c2839e9cd09bb24a8c90921bf0776fb9baa962df7d4f519b74e43f8e5bbc`.

Historical lookup identifiers: `P380`, `P459`, `P157A`, `P224`, `P772v`, `P772i`, `P488`, `P661`, `P1288`.

Historical source anchors: `01_core/src/compiler/layout/table.rs:70-87`, `typst-pdf/src/tags/context/table.rs:129-135`.

### 00_nucleo/prompts/compiler/layout/table_cell.md

Pre-edit prompt SHA-256: `8bf51897b40fb7cd0caee5e2006325a4998056203b7520581abb28c791539af6`.

Exclusive consumer: `01_core/src/compiler/layout/table_cell.rs`; inspected SHA-256: `6cec982d39e5c0bfa17124c0d0ec34d12f06eb4f1fc7e3ee3f23eda984846e79`.

Historical lookup identifiers: `P1288`.

Historical source anchors: `01_core/src/compiler/layout/table_cell.rs:14-20`, `typst-pdf/src/tags/context/table.rs:98-126`.

### 00_nucleo/prompts/compiler/layout/table_footer.md

Pre-edit prompt SHA-256: `a6345f5927c4be1cba16c93572c531469c59841f82b75ddb55005023cf4e56ef`.

Exclusive consumer: `01_core/src/compiler/layout/table_footer.rs`; inspected SHA-256: `bf64eeed28d568b51e78c289776a4fd5c1d985c072d58189db382fa0651cb734`.

### 00_nucleo/prompts/compiler/layout/table_header.md

Pre-edit prompt SHA-256: `e189b1cd65bc92be68f42fb34c7d8d1ff8377c13f76f8c0e8dfed3af8c71d081`.

Exclusive consumer: `01_core/src/compiler/layout/table_header.rs`; inspected SHA-256: `fc4f77c089d780a0888cc26789c51c6ccc580b9884157e959e34f88d400f67bd`.

### 00_nucleo/prompts/compiler/layout/term_item.md

Pre-edit prompt SHA-256: `fe9ba7aed285800cd408de583cd8d1f039ce213af7d643251781d900b8f5308d`.

Exclusive consumer: `01_core/src/compiler/layout/term_item.rs`; inspected SHA-256: `13fda67903f8b6b5c684ecdd11d0d25e075c06fd5fb94617091e010058529b94`.

### 00_nucleo/prompts/compiler/layout/terms.md

Pre-edit prompt SHA-256: `71daa57aec09bcd12987fbbb18b97017cc8b524eedc5a91c5c1543d4834199ab`.

Exclusive consumer: `01_core/src/compiler/layout/terms.rs`; inspected SHA-256: `0556add021fef655f489ca9142a32009394060cf1ee3fcbdcf1e479b6e796b78`.

### 00_nucleo/prompts/compiler/layout/tests.md

Pre-edit prompt SHA-256: `666d2fd85278439e844c02862b3213dace79f2686cc3c56909b022920f458d02`.

Exclusive consumer: `01_core/src/compiler/layout/tests.rs`; inspected SHA-256: `668a905bb7392efd7a3b088b51cb445a0bcc7999554acec742e005ec767134a8`.

Historical lookup identifiers: `P1373`, `P1374`, `P1375`, `P1292`, `P245`, `P1390`, `P1389`, `P1388`, `P1391`, `P1392`, `P1466`.

### 00_nucleo/prompts/compiler/layout/text.md

Pre-edit prompt SHA-256: `98795a59bf8a374f5a16d2feff844a5285056161f761de649e17deb4c9bc097e`.

Exclusive consumer: `01_core/src/compiler/layout/text.rs`; inspected SHA-256: `0f8f0e78174deb402ad5fcce41014410bdf957db42c0c2e44e64d2464ff490fc`.

Historical lookup identifiers: `P1481`.

Historical source anchors: `text.rs:158`.

### 00_nucleo/prompts/compiler/layout/tiling.md

Pre-edit prompt SHA-256: `ef6797e5fe4d5dc9c1c7b1069067153c6813e8c22b60e803de5138f85af61cfd`.

Exclusive consumer: `01_core/src/compiler/layout/tiling.rs`; inspected SHA-256: `629f3824787c52563d3b9f86d8c9e2592a6dae359ef3e9fd959029382e7df73a`.

Historical lookup identifiers: `P1245`, `P1254`.

Historical source anchors: `crates/typst-library/src/visualize/tiling.rs:99-185`.

### 00_nucleo/prompts/compiler/layout/title.md

Pre-edit prompt SHA-256: `ee246b90a86063d5ac87906e14ba2c1827da8e4122aedf7ec25b14dcf1c8d56e`.

Exclusive consumer: `01_core/src/compiler/layout/title.rs`; inspected SHA-256: `2b1ef5d7903d6ee2a5f466d7627f050d559bb52503839088dda70c12bf00ec64`.

Historical lookup identifiers: `P765a`, `P765`.

### 00_nucleo/prompts/compiler/layout/transform.md

Pre-edit prompt SHA-256: `60f766aa9ac2e5ef2ddff4d4d92e34b45e2c03fee9659831b006bfeca5c933c8`.

Exclusive consumer: `01_core/src/compiler/layout/transform.rs`; inspected SHA-256: `3170116f0d65119bb39649a2d31b776b813e7fcb37d7248239f3a7948ebed75e`.

Historical lookup identifiers: `P1493`.

Historical source anchors: `transform.rs:48-68`.

### 00_nucleo/prompts/compiler/layout/v_space.md

Pre-edit prompt SHA-256: `0f1680668a9a6c5c32e5a78e7697794d94f8c35dec0a0930dcdf0a74bfc4323a`.

Exclusive consumer: `01_core/src/compiler/layout/v_space.rs`; inspected SHA-256: `038704a2e7984d36b21fb3710da11dc184a2911026f4e93832d69ca523a1ff16`.

### 00_nucleo/prompts/compiler/layout/vanilla_defaults.md

Pre-edit prompt SHA-256: `59a91c9b0cb53de1e8522e0f60a27cca6c0fb037f86515e0ff6915fd5b4d786d`.

Exclusive consumer: `01_core/src/compiler/layout/vanilla_defaults.rs`; inspected SHA-256: `1cd671191e551ee5b8a608963096d23bba44ccbcba01333d5dc87d37b2fe1700`.

Historical lookup identifiers: `P1056`, `P1057`.

Historical source anchors: `lab/typst-original/crates/typst-library/src/model/par.rs:224`, `lab/typst-original/crates/typst-library/src/model/par.rs:210`, `lab/typst-original/crates/typst-library/src/layout/container.rs:342`.

### 00_nucleo/prompts/compiler/layout/word_segment.md

Pre-edit prompt SHA-256: `7f78ddd20a7051e6ba66cf19ac18fa24b1925a4a7cd86f2bb4353eee20101bd3`.

Exclusive consumer: `01_core/src/compiler/layout/word_segment.rs`; inspected SHA-256: `17ab457feece7121917cc620c49b671455708d3a1b69a0e316bf5d9ada547523`.

Historical lookup identifiers: `P1504`, `P1503`.

Historical source anchors: `compiler/layout/cursor.rs:1192-1303`.

### 00_nucleo/prompts/compiler/layout_counters.md

Pre-edit prompt SHA-256: `0eb81045a221373ee7b32e19521ed13ffc16ef693c325b615f8acb72408abff9`.

Exclusive consumer: `01_core/src/compiler/layout/counters.rs`; inspected SHA-256: `ccf808917d88fca4ef86a4038ea5020ba5f84d773994bafe5ad5877d0ff8dd23`.

Historical lookup identifiers: `P1031`.

Historical source anchors: `counter.rs:38-41`, `counter.rs:481-497`, `counter.rs:501-512`, `counter.rs:374-381`.

### 00_nucleo/prompts/compiler/layout_figure.md

Pre-edit prompt SHA-256: `267955a87458e09f7094801d294a3d5b80fb72eebf7e2e20a183995f84f1eba9`.

Exclusive consumer: `01_core/src/compiler/layout/figure.rs`; inspected SHA-256: `fd5cc61e54783c3e7f6d59508d72f0cae5aa872bf5c8e30921abcc4a762d3a1b`.

Historical lookup identifiers: `P470`, `P1031`, `P488`, `P1034`, `P365`.

Historical source anchors: `crates/typst-library/src/model/figure.rs:296-299`, `crates/typst-library/src/text/mod.rs:473`, `text/lang.rs:232`.

### 00_nucleo/prompts/compiler/layout_outline.md

Pre-edit prompt SHA-256: `c58164d19b4d751736591cfbf55591736344a598015b859b3b6a3d49ea5767e5`.

Exclusive consumer: `01_core/src/compiler/layout/outline.rs`; inspected SHA-256: `1f4300110ffc558fd7401eca171940dd63c5367be28b236da23bcc42edb9117a`.

Historical lookup identifiers: `P457`, `P502`, `P359`, `P1031`, `P472`, `P488`, `P1034`, `P1406`, `P1407`, `P1411`, `P1493`.

Historical source anchors: `crates/typst-library/src/text/mod.rs:473`, `typst-layout/src/rules.rs:413-455`.

### 00_nucleo/prompts/compiler/layout_references.md

Pre-edit prompt SHA-256: `b0003fd67ff64f0db590d88cafafd35bdda2f1e9f6a8c446fd498f9678956d3a`.

Exclusive consumer: `01_core/src/compiler/layout/references.rs`; inspected SHA-256: `9e1767a087fac4a29416823c78a9ccab8270bf5274e5d8d78c2b20eaefdeefce`.

Historical lookup identifiers: `P462`, `P463`, `P1073`, `P788`, `P786`, `P856`, `P1459`, `P1140`, `P1157`, `P1458`.

Historical source anchors: `reference.rs:341-355`, `references.rs:235-249`, `references.rs:391-411`.

## Assigned working-tree provenance

The complete `git diff HEAD --stat` was captured during inspection; it includes substantial pre-existing work outside this scope and is not a measurement used to close any decision. The following porcelain status is restricted to assigned paths. Exact pre-edit fingerprints above, not the final HEAD diff, identify the original dirty text preserved by this operation.

```text
 M 00_nucleo/prompts/compiler/layout-image.md
 M 00_nucleo/prompts/compiler/layout.md
 M 00_nucleo/prompts/compiler/layout/bib_csl.md
 M 00_nucleo/prompts/compiler/layout/bibliography.md
 M 00_nucleo/prompts/compiler/layout/block.md
 M 00_nucleo/prompts/compiler/layout/boxed.md
 M 00_nucleo/prompts/compiler/layout/cell_tail_commit.md
 M 00_nucleo/prompts/compiler/layout/cite.md
 M 00_nucleo/prompts/compiler/layout/colbreak.md
 M 00_nucleo/prompts/compiler/layout/cursor.md
 M 00_nucleo/prompts/compiler/layout/curve.md
 M 00_nucleo/prompts/compiler/layout/decorations.md
 M 00_nucleo/prompts/compiler/layout/divider.md
 M 00_nucleo/prompts/compiler/layout/dynamic.md
 M 00_nucleo/prompts/compiler/layout/enum_item.md
 M 00_nucleo/prompts/compiler/layout/equation.md
 M 00_nucleo/prompts/compiler/layout/flow_suffix.md
 M 00_nucleo/prompts/compiler/layout/flush.md
 M 00_nucleo/prompts/compiler/layout/footnote.md
 M 00_nucleo/prompts/compiler/layout/format_options.md
 M 00_nucleo/prompts/compiler/layout/grid.md
 M 00_nucleo/prompts/compiler/layout/grid_cell.md
 M 00_nucleo/prompts/compiler/layout/grid_footer.md
 M 00_nucleo/prompts/compiler/layout/grid_header.md
 M 00_nucleo/prompts/compiler/layout/grid_placement.md
 M 00_nucleo/prompts/compiler/layout/h_space.md
 M 00_nucleo/prompts/compiler/layout/heading.md
 M 00_nucleo/prompts/compiler/layout/helpers.md
 M 00_nucleo/prompts/compiler/layout/hide.md
 M 00_nucleo/prompts/compiler/layout/html_frame.md
 M 00_nucleo/prompts/compiler/layout/hyphenation.md
 M 00_nucleo/prompts/compiler/layout/line_commit.md
 M 00_nucleo/prompts/compiler/layout/linebreak.md
 M 00_nucleo/prompts/compiler/layout/link.md
 M 00_nucleo/prompts/compiler/layout/list_item.md
 M 00_nucleo/prompts/compiler/layout/metrics.md
 M 00_nucleo/prompts/compiler/layout/pad.md
 M 00_nucleo/prompts/compiler/layout/page_canvas.md
 M 00_nucleo/prompts/compiler/layout/page_commit.md
 M 00_nucleo/prompts/compiler/layout/page_geometry.md
 M 00_nucleo/prompts/compiler/layout/page_run.md
 M 00_nucleo/prompts/compiler/layout/page_running.md
 M 00_nucleo/prompts/compiler/layout/pagebreak.md
 M 00_nucleo/prompts/compiler/layout/parbreak.md
 M 00_nucleo/prompts/compiler/layout/pdf_artifact.md
 M 00_nucleo/prompts/compiler/layout/pdf_attach.md
 M 00_nucleo/prompts/compiler/layout/place.md
 M 00_nucleo/prompts/compiler/layout/placement.md
 M 00_nucleo/prompts/compiler/layout/quote.md
 M 00_nucleo/prompts/compiler/layout/raw_highlight.md
 M 00_nucleo/prompts/compiler/layout/repeat.md
 M 00_nucleo/prompts/compiler/layout/sequence.md
 M 00_nucleo/prompts/compiler/layout/shape_block_behaviour.md
 M 00_nucleo/prompts/compiler/layout/slicing.md
 M 00_nucleo/prompts/compiler/layout/smartquote.md
 M 00_nucleo/prompts/compiler/layout/stack.md
 M 00_nucleo/prompts/compiler/layout/sub_frame.md
 M 00_nucleo/prompts/compiler/layout/table.md
 M 00_nucleo/prompts/compiler/layout/table_cell.md
 M 00_nucleo/prompts/compiler/layout/table_footer.md
 M 00_nucleo/prompts/compiler/layout/table_header.md
 M 00_nucleo/prompts/compiler/layout/term_item.md
 M 00_nucleo/prompts/compiler/layout/terms.md
 M 00_nucleo/prompts/compiler/layout/tests.md
 M 00_nucleo/prompts/compiler/layout/text.md
 M 00_nucleo/prompts/compiler/layout/tiling.md
 M 00_nucleo/prompts/compiler/layout/title.md
 M 00_nucleo/prompts/compiler/layout/transform.md
 M 00_nucleo/prompts/compiler/layout/v_space.md
 M 00_nucleo/prompts/compiler/layout/vanilla_defaults.md
 M 00_nucleo/prompts/compiler/layout/word_segment.md
 M 00_nucleo/prompts/compiler/layout_counters.md
 M 00_nucleo/prompts/compiler/layout_figure.md
 M 00_nucleo/prompts/compiler/layout_outline.md
 M 00_nucleo/prompts/compiler/layout_references.md
?? 00_nucleo/prompts/compiler/layout/align.md
?? 00_nucleo/prompts/compiler/layout/context_block.md
?? 00_nucleo/prompts/compiler/layout/counter_display.md
?? 00_nucleo/prompts/compiler/layout/counter_display_callback.md
?? 00_nucleo/prompts/compiler/layout/measurement.md
?? 00_nucleo/prompts/compiler/layout/outline_entry.md
?? 00_nucleo/prompts/compiler/layout/par_line_callbacks.md
?? 00_nucleo/prompts/compiler/layout/space.md
?? 00_nucleo/prompts/compiler/layout/state_display.md
?? 00_nucleo/prompts/compiler/layout/style_scope.md
```
