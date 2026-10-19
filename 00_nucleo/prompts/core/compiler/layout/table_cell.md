# L0 — table cell
Hash do Código: 1985dbd5

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:059eb35863a413593c98fb7195a521e3d019dadf461433e9c42422218be89065
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:7510f3bba27256221d4447db608839af562ac71b4eacc0c042f693b531a95184
- 00_nucleo/prompts/_nuclei/table/cell-occurrence.toml sha256:bba33b336e651ccd6d41f88b0e16449932d0a986de1ee2cc3c1fdf5c8fb8c6dc
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:2b9e828f5e0d958f4328d98a35320ba2ea321997e541e6ab90520d2e8811e7bf


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/table_cell.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/table_cell.rs:15`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. The former reference `9f2b6e8715237cb086899a42873660fe744622e8` and older vanilla pins are historical snapshots. They do not certify the current ratified reference `bd5ebec285505889e8ff1d4f68804ab147a699f3` without a new measurement.

`lab/results/construction/diagnostics/table-cell-realizacao-d08b3-2026-10-07/{before,additional,order,aliases}.json.gz` preserves HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree, historical pin `9f2b6e8715237cb086899a42873660fe744622e8`, UTC/diffstat/status/sources/binaries/arguments/results. Oracle SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`; Tekt `7c0957f45fe40290aa809e3358c9a87979aa1e28058511ad0ee978feea6713a2`. Pinned `layout/grid/resolve.rs:204–278,1500–1530,1665–1703,1945–1988` resolves fields/conflicts/padding/breakability. Tekt `grid_placement.rs:91–112,262–270`, `realize/mod.rs:160–200`, `table.rs:127–152` expose explicit-first/Grid-only placement, bottom-up recipes and discarded wrappers. order measures no cell effects under replaced Table/no Heading effects under hidden cell; aliases measures LOCAL a, OUTER LOCAL a, NEW a. Language/effects, not copied mechanics/bytes/Rust equality. Unknown fields/hidden effects/wrong order refute correction; cache/pagination equivalence is not inferred.

`lab/results/construction/diagnostics/table-cell-isolada-d08b3b-2026-10-07/complete.json.gz` contains 53 probes, UTC2026-10-07T14:47:56.340144+00:00 through 14:48:12.375245+00:00, same HEAD/uncommitted tree/full diffstat/status/hashes/sources/argv/cwd/environment/output. Vanilla historical pin `9f2b6e8715237cb086899a42873660fe744622e8`, binary `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`; Tekt `6ca44e46ae2c1b1f8b44207052ea2fd63e3b24b4136c3fc3d25b6d20e9b6a20f`. before/refuters/complete are not independent additional rounds.

isolated-default exposes vanilla auto x/y/inset/align/fill/breakable, spans 1, empty stroke dict; Tekt lacks defaults/emits two metadata versus one. isolated-x differs; raw-fields only body/raw-x rejected match; isolated-table-style excludes Table defaults from standalone Cell. raw-vs-prepared is false vanilla/true Tekt. nested-effects/body-effects/late-rule refute single execution/lexical origin. fresh-isolated/table/block/styled/multi-fresh wrongly pass Tekt versus vanilla `maximum show rule depth exceeded` at new constructor span; identity/body/alias/fresh-conditional/hidden remain valid. multi-identity/multi-return-table show returned it must allow earlier unconsumed recipe. isolated-query remains `cell is not locatable`, location none; cell-set is separate and excluded here.

Pinned model/table.rs:734–778 declares defaults/properties; typst-realize/src/lib.rs:367–386,418–429,491–505,571–586 covers prepared argument/product depth/per-element guards/materialization. Tekt table_cell.rs:111–143 projects lexical/positioned only; realize/table_cells.rs:110–129 reapplies unprepared Cell,`:280–304` terminates whole snapshot; eval/show_rule_termination.rs:65–70 treats same form fixed-point; eval/rules/apply.rs:139–166 globally revokes rule during callback. Fields/effects/order/termination/causal error are language, not upstream guard structure; no intent inference. Alias/two-rule/new-generation/hidden-content controls can refute the chosen representation.

`lab/results/construction/diagnostics/table-origens-d08b3c2-2026-10-07/baseline.json.gz` records HEAD/working tree/UTC/sources/argv/binaries against historical pin `9f2b6e8715237cb086899a42873660fe744622e8`. Existing ShowRecipe/Located/Realized branches contain the corresponding transparent operation; Source must not leak as observable content. Human approval `confirmo` was received 2026-10-07; contract: `lab/history/construction/records/00_nucleo/materialization/table-origens-d08b3c2-2026-10-07/contract.md`.

geometry-before/nested-local-text has matching source/glyph/size/color but all four Tekt words on one y versus vanilla OUTER then surviving default at x+5pt/y+25.43798pt. Pinned typst-layout/src/rules.rs:704–724 materializes Cell default through nonzero-inset Pad then Align. geometry-third preserves words/fonts/x but retains 5pt vertical difference: nonzero padding realization is a block, not manual parbreak then inline Pad.

The owner approved insertion-time resolved Table (historical D08B3-A) on 2026-10-07 (`comfirmo`), archived at `lab/history/construction/records/00_nucleo/materialization/table-cell-realizacao-d08b3-2026-10-07.md:3`, manifest `5fec42835e38e9a28ee2ea2714b30135f069f111c140bb5e918a0902a06cc82b`: five public fields, two public carriers, one public facade and logical preparation before Cell show. This supersedes its stale proposed-only gate only within the approved scope. Standalone Cell defaults/generation (historical D08B3-B) were separately ratified/materialized (`confirmo`), archived at `lab/history/construction/records/00_nucleo/materialization/table-cell-isolada-d08b3b-2026-10-07.md:3`, manifest `3375b0a09a4d39d02c2f0bafbcaf3016c6afde27acdf632331da7a98de31e1df`. A did not authorize B's added public field/type; B's separate confirmation did. Neither closes broader incomplete scope.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/core/compiler/layout/mod.md`; the contract below defines this unit's operations, not ownership of that hub.

Dispatch the cell body exactly once. Table semantic classification belongs to `00_nucleo/prompts/core/compiler/layout/table.md`; visual cell placement belongs to `00_nucleo/prompts/core/compiler/layout/grid.md`. Do not derive Header/Data from appearance here.

Static dispatch for Complete standalone/positioned prepared Cell uses resolved.visual once; raw fallback still uses body. No Func, Location allocation, repeated padding/cosmetics or local default/guard/recipe resolution. Pending CellRecipeApplied must realize before render. A standalone unoccupied Cell acquires no Table position/inset.

In a mixed product after existing inline text, positioned Cell's known surviving default with nonzero inset reuses an existing Block with that default's inset/breakable and visual body once. Apply Align to body when known. This supersedes manual prior-paragraph closure plus inline Pad, which the 5pt refuter rejected. Zero inset remains inline; standalone Cell stays inline; default already applied externally by Grid never doubles inset/paint. No compensation constant, new snapshot/Location/API/phase or recipe replay. Source/body transparency preserves classification under existing owners.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

Require RED→GREEN, suites/build/lint/restricted preservation and D08A/B2, D05/D06/D07/query collections. Accept real language/effects/geometry, not byte equality or whitespace normalization to close render. General Table defaults/fields (table-root-equality), rich style functions/paint/strokes/insets, rich lines/gutters, multipage spans and PDF/UA are not certified. Preserve extra legacy D08B2 casts, syntax/flags and global nonlocatable policy.

Separate standalone ratification requires only approved candidates, tests with real preimplementation RED and resealing edited owners only. Accept bilateral probes/additional revisions, once-only effects/equality/getters/selection/HTML product/actual text/styles/geometry and exact messages/spans without hiding differences. Require debug/release builds, relevant core/infra/shell suites, lint without errors and owners without V5/V6/V15/V26; revalidate D08A/B2/B3-A, Raw defaults/blocks, Equation/math/matrix/query. Preserve earlier tree via snapshot/restricted diff without commit/reset/global reseal.

This slice completes intrinsic standalone defaults and alias-versus-new-generation distinction only. Separate Cell style-default set work (historical D08B3-C/cell-set) requires its own measurement/L0 and carries no promised delivery. General Table defaults/equality, rich casts/paint/strokes/insets, rich lines/gutters/multipage spans/PDF-UA and global nonlocatable policy remain open. Existing empty-file eval warning is neither repaired nor normalized to claim parity.

Surviving-default geometry requires RED→GREEN, real glyph geometry, zero-inset control and C/macro controls; preserve body exactly once, source origin and standalone/root behavior.

Preserve exclusive ownership/static dispatch without I/O or a new signature/trait/flag or eval↔layout phase. Source-origin tests/C2/C/macro collections/builds/lineage are required before certification.
