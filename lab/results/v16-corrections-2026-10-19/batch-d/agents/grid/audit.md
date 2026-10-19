# Grid phase 1 audit

Owners: V16-029–036 and V16-077. All corresponding full L0s, relevant ADRs, repository guides, case rows, and the original cause correction plan were read before editing. Phase 1 changes only L0 clauses and new cfg(test) controls. No productive implementation, seal, formatter, Cargo, linter, or oracle invocation occurred.

## Measurement and causal audit

`phase1-before.json` preserves HEAD, UTC, full diffstat/status, exact sources and owner hashes. `phase1-ready.json` preserves completion state and hashes. `anchors.json` identifies each decision by file, enclosing function and unique-in-function match anchor; mutation domain is Content for 029–036 and Value for 077. No nested enum extension is introduced. PlacementPolicy is already closed; keep its existing policy calculation and explicitly handle guard residuals within each TableCell/GridCell arm.

029: emit_row_borders reads only PlacedCell.body. Raw wrapper is not a cell metadata lookup; prepared TableCell's known None must suppress inherited stroke. Existing d08b3c_border_reads_known_stroke_and_known_none complements the new raw-wrapper control.

030: emit_grid_cell_body is called by cell_emission. The tuple extraction feeds independent effective-fill/stroke and prepared-default inset branches. Existing fill-survives-replacement and bidi controls remain; the new control checks raw inherited fill/inset versus prepared known None and state restoration after real emission.

031/032: MeasuredCells::new combines ambient and simple_placement. The adjacent unselected `_ => true` in new only accepts ordinary sequential-index assumptions; it is not a cache certificate. capture requires token plus one identical Text product, finite metrics, no orphan alignments and identical environment. take removes entry before token/geometry/style confirmation. Refusal returns to cell_body's layout_sub_frame. Thus neutral placement cannot admit Styled/Text-with-space products by itself. New controls exercise normal fallback geometry after refusal. Existing tests cover source origin, explicit coordinates, effective style invalidation, failed consumption, punctuation, and overflow. No new catchall is relocated into these checks.

033/034: the selected inset matches operate on outer cell/item. Both prepared variants use prepared_default; absent surviving default yields zero inset instead of container inset. Raw content keeps inherited inset. Actual column/row measurements are compared in the new control. Adjacent prepared-coordinate partition matches and composed_visual's zero/composite classifier are different operations and remain untouched.

035: map_content_pre processes Source before callback; None continues existing reconstruction. Table's Some(node.clone()) deliberately stops recursion. Staged TableCell uses visual; unstaged residual is no replacement. Controls cover staged text under Source/Styled, raw cell, and complete nested Table barrier. Recipes are not executed here. Existing full layout/realizer tests must cover alias/effect-once integration; no global parity claim follows from structural projection.

036: both adapters reach one engine. Wrong-policy cell metadata must remain raw unit placement, including impossible coordinates/spans; matching Table cells keep causal overflow diagnostic. Controls also distinguish Grid explicit-first ordering from Table's document-order conflict. Existing tests cover holes/padding and detailed overlap/hints.

077: current set caller is set.rs:262–267 (the original ledger's 205–210 shifted). It accepts Auto separately and diagnoses other failed projections at the named value span. realizer table_cells.rs:675–682 records Auto presence and stores projection Option. style_inset accepts Length or an all-valid partial dictionary. Unknown keys/non-Length values reject the entire dictionary. Rich Ratio/Relative/function casts remain out of scope. New controls cover precedence, partial/empty dict, and bounded rejection.

## Validation obligations still open

All new tests in tests.json await coordinator execution. Their expected role is conservation GREEN before/after; structural baseline lacks E0004 at each selected match and must become exact-site E0004 afterward. Phase 1 is not a semantic RED claim. The tests have not yet been compiled and require ordinary fixture correction if compilation identifies a mismatch. Test-output/formatting and heavy checks belong to coordinator.

Selected source evidence establishes internal preservation, not current upstream parity. Existing rich-inset, multipage, fixed-row alignment, aliases, effects, clipping/tails and broader cache exclusions remain unchanged and uncertified by these new controls alone. No shared enum/facade expansion is needed. The parent should run pertinent existing layout/realizer/eval suites and the baseline structural mutations before authorizing implementation.

## Authorized implementation

After coordinator reported all grid conservation controls GREEN and froze baseline-final receipts, the nine selected matches were closed over the exact current enum variants. Existing prepared TableCell guard fallthrough remains explicit in its own TableCell arm. Styled's failed transport guard and measurement_visual's unstaged TableCell now return their prior None inside their unconditional owning arm. Placement reads coordinates only inside each matching policy branch and keeps wrong-policy variants raw; no policy/default/cast/cache eligibility was expanded. No shared enum/facade was edited. No build/lint/seal/formatter ran in this agent.

`anchors-final.json` retains exact source lines, unique-in-function anchors and source hashes. `implementation-ready.json` records final UTC/HEAD/whole-tree diffstat and owned hashes before coordinator validation. Parent runs GREEN controls and future-variant E0004 checks before any closure claim.
