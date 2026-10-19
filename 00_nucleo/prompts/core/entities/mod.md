# L0 — Domain entity facade
Hash do Código: 17024a21

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:96c5b3ef9be3d171d3ca4c57f30fe826068c568fc17437f83b2b9b5d7372aeff

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/mod.rs`

## Evidence before specification

The bidi evidence in `lab/results/construction/diagnostics/bidi-contexto-2026-10-05/initial.json.gz` records HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, the uncommitted working tree, UTC, sources, binaries and thirteen CLI inputs against historical vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`. In `rtl_auto_mixed`, ABC starts at x=124.020510 in Tekt and 152.949220 in vanilla; `ltr_numeric` places Arabic on the left and reverses 123/456, unlike vanilla. Historical `layout_bidi.rs:73–82,227–240` and `shaper/bidi.rs:34` resolve isolated lines/strings; the private paragraph context cannot reach them. Upstream `inline/mod.rs:189,232` shares direction, and `inline/collect.rs:156–178` preserves explicit embeddings. `probe.json.gz` preserves the recipe, rustc and unicode-bidi 0.3.18 hashes: the line `456 789` inside `ABC 123 456 789 مرحبا` is one LTR run; isolated with the same RTL base it becomes runs `789`, space, `456`. This refutes direction-only transport and isolated resolution, without certifying candidate geometry. Language semantics, syntax and morphology are obligations; library/ranges/Arc/cache are internal mechanics.

The conditional-test evidence at the same HEAD with an uncommitted working tree is in `lab/results/construction/diagnostics/cfg-test-ci-2026-10-13/{before,refinement-before}.json.gz`, including sources, L0, UTC, status and diffstat. `line_cap_tests.rs:5` and its parent at line 128 duplicated `cfg(test)`. Removing the inner condition retained Rust tests but caused architectural V2 because the pinned linter does not propagate the parent's condition.

Prepared-paint evidence in `lab/results/construction/diagnostics/tiling-transporte-2026-10-14/{parent-before,solid-controls}/results.json.gz` records the same HEAD/uncommitted provenance, sources, argv and binaries. Historical vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`, tree `6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4`, oracle SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`, original P1489 recipe. `tiling.rs:124–127` returns None above 10000 cells; `shape.rs:151–185` materializes only fill; export `shape.rs:151,201` reduces tiling to color; `layout_types.rs:505,531` lacks cell/root transport; raster `render.rs:217,579` reduces fill and ignores clip_mask. The earlier coverage record's over-limit 200×100pitch1 solid/stroke observation uses unchanged sources and is not a fresh binary measurement. On 240×192 SVG rendered over white with librsvg, place-parent differs by 1194 pixels above 16, box-fixed/auto-parent by 1192, while Solid controls agree exactly; block-parent differs by 2, maximum 23. Upstream SVG `lib.rs:346–358`, inline `box.rs:35–49` and `frame.rs:459–474` establish offset/rotation, inherited-transform inverse and local roots before inset. They do not require copying FrameKind/Packed/Rust equality. Solid controls also refute general box-place/rotate geometry (3200/5296/3645 pixels above 16); transport cannot close those defects. The earlier quarter control agrees between Tekt PDF/SVG and vanilla SVG while vanilla PDF differs; do not copy that defect.

## Facade contract

Declare static L1 entity modules, the inline `frame_visitor` facade reexporting `walk_frame_items` and `FrameVisitor` from `layout_types`, and private `line_cap_tests`. Preserve all existing public modules, including `font_variations` and `document_format_options`. Their child owners retain domain behavior, defaults, parsing, document extraction and export precedence. No general reexports, module removals or business logic are authorized.

The approved bidi addition is exclusively `pub mod bidi_context;`, owned by `00_nucleo/prompts/core/entities/bidi_context.md` for `01_core/src/entities/bidi_context.rs`. Do not reexport its types, decide Unicode levels, add state or duplicate child validation. The approved paint addition is exclusively `pub mod frame_paint;`, without child logic or general reexports. These bounded approvals supersede the earlier no-new-module restriction only for these declarations.

Keep `line_cap_tests` unconditionally declared in this facade, retaining its own inner `#![cfg(test)]` to exclude its whole file, imports and helpers from production. Preserve module paths, names, visibility and path attributes. Its test bodies retain their separate L0 owner. Add no dependency, API, default mode or pipeline phase.

## Approval and incomplete integration

Bidi approval is recorded as `confirmo` in `lab/history/construction/records/00_nucleo/materialization/bidi-contexto-aprovacao-2026-10-05.json`, manifest `1ae7513eb3af426754909954aef165c97257a9f62d3911fbb111e2002fac4043`. Frozen versions are in the implementation's before receipt; code hashes do not prove approval.

Paint approval is in `lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`, manifest `d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d`. It satisfies the earlier public-contract gate without expanding scope. The original T1a slice installs pure carriers and this declaration; T1b must complete FrameItem fields and producers/consumers, T2 PDF/SVG/raster resources and clipping, and T3 geometric integration. Carrier presence is explicitly incomplete and proves no render correction or legitimacy of legacy fallbacks. Each implementation owner must be updated before its consumer, with pinned shared claims and RED→GREEN.

## Acceptance

Verify static declarations, the unchanged test inventory, core suite, production build, Clippy and architecture. Integration checks must retain full Solid/Gradient transport, prepared-cell Arc retention without repetition-proportional copying, full stroke geometry and Stroke::default equivalence. Offer no total conversion from declarative Paint/Tiling. Missing PartialEq on new carriers does not alter language equality. Compile tests verify the boundary; child tests remain separately owned.

## Historical record

`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/mod.md` preserves the original evidence and chronology.
