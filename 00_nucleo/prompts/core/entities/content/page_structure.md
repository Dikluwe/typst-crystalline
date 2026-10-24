# L0 — core/entities/content/page_structure
Hash do Código: f7563f73


**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/content/page_structure.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/map-content-change.toml sha256:6343f553bd5f842f83190bd6533a9a9642118a34214550d8289c9daaba219043
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:059eb35863a413593c98fb7195a521e3d019dadf461433e9c42422218be89065
- 00_nucleo/prompts/_nuclei/content/symbol-identity.toml sha256:5060f0989415e92d54eab2a82afd84ba11472aef635d15830f9fb6ee3469aea9
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:2b9e828f5e0d958f4328d98a35320ba2ea321997e541e6ab90520d2e8811e7bf


## Evidence before specification

Table realization evidence is in `lab/results/construction/diagnostics/table-cell-realizacao-d08b3-2026-10-07/{before,additional,order,aliases}.json.gz`: HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/status/diffstat, UTC, sources, argv and binaries; historical vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`, oracle SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`, Tekt `7c0957f45fe40290aa809e3358c9a87979aa1e28058511ad0ee978feea6713a2`. Upstream grid `resolve.rs:204–278,1500–1530,1665–1703,1945–1988` resolves fields, ordered conflicts, empty cells and breakable. Local `grid_placement.rs:91–112,262–270` prioritizes explicit placement and knows only GridCell; `realize/mod.rs:160–200` visits recipes bottom-up; `table.rs:127–152` unwraps without attributes. Probes observe zero cell effects when the table is replaced, zero heading effects when hidden by a cell, and alias output `LOCAL a`, `OUTER LOCAL a`, `NEW a`. Unknown fields, hidden effects or incorrect order would refute the proposed language behavior; cache/pagination equivalence is not inferred.

Source-origin evidence is in `lab/results/construction/diagnostics/table-origens-d08b3c2-2026-10-07/baseline.json.gz`, with HEAD/tree/UTC/sources/argv/binaries at that historical pin. Existing ShowRecipe/Located/Realized branches establish the corresponding transparent operation. Source must not become an observable element.

The current single-traversal decision follows `lab/results/construction/diagnostics/content-preflight-removal-2026-10-10/{before,measurements,confirmation,allocation-red,allocation-green,validation}.json.gz`, with the same HEAD/uncommitted provenance. Previous release `2d7f5dc93f6a7f15c81ff93b74270c80a14077d6753c8f79c73c9575d113d5f1`, candidate `9ae0a3d9285b8d0b4bae0da1db058483e7f5543003fabff46bb7d93b17e474f2`, oracle pin/hash above. `content/traversal.rs:24–82` supplies explicit no-change proof. Removing the two borrowed guards improved paired blocks-400 time by 1.21% in both collections, six rounds with negative medians; six cases retained allocation counts,34 text/geometry/raster/query controls and the core suite passed. Mathematics/repeated blocks remain outside equivalent-work vanilla comparisons. These historical results are not a new performance or current-pin parity certification.

## Structural contract

Own pure page/column segmentation and `wrap_page_columns`. Preserve source order, nested content, page/column markers, page configuration and degenerate normalization. Perform no layout, measurement, rendering or I/O. Preserve occurrence carriers as causal units: never discard identity or reemit consumed origin. Preserve prepared plans, stages and sources during segmentation; descend only into actual prepared visual contributions, never reemit the original body.

Source is transparent, delegates to its body, neither exposes span as language identity nor executes recipes, and retains origin per segment where applicable. Keep exclusive ownership and static dispatch, without new signatures, traits, flags or an eval/layout phase change.

Use `map_content` directly without a duplicate borrowed preflight. This supersedes the former no-column borrowed guard, whose complete evidence and proposal remain in the historical record. Follow the pinned explicit no-change protocol while preserving callback boundaries, order, errors, origin, location and transformations. Do not detect change through equality, hashes or pointer identity, expand variants or replace the enum with dynamic dispatch. Source retains its first origin under the pinned nucleus.

## Approval and incomplete scope

The table realization package originally named D08B3-A changed five public fields, two public carriers, a facade and the logical phase before Cell show. Its formerly pending gate was satisfied in `lab/history/construction/records/00_nucleo/materialization/table-cell-realizacao-d08b3-2026-10-07.md`, manifest `5fec42835e38e9a28ee2ea2714b30135f069f111c140bb5e918a0902a06cc82b`. The separate isolated-cell package is recorded in `lab/history/construction/records/00_nucleo/materialization/table-cell-isolada-d08b3b-2026-10-07.md`, manifest `3375b0a09a4d39d02c2f0bafbcaf3016c6afde27acdf632331da7a98de31e1df`. These bounded approvals do not certify the whole Cell universe: isolated defaults (`raw-cell-no-placement`, `outside-fields`) and fresh `show-cell-reconstruct` generation retain their separate scope and checks.

Generic Table defaults/fields (`table-root-equality`), function styles, rich paints/strokes/insets, rich lines/gutters, multipage spans and PDF/UA remain outside certification. Preserve legacy extra casts, syntax, flags and global nonlocatable policy. The exact Source-origin contract remains `lab/history/construction/records/00_nucleo/materialization/table-origens-d08b3c2-2026-10-07/contract.md`; approval `confirmo` on 2026-10-07 is recorded in the final `lab/history/construction/records/00_nucleo/materialization/table-origens-d08b3c2-2026-10-07.md`.

## Acceptance

Existing page/column tests pass unchanged; include local tests. Require RED→GREEN, suites/build/lint, restricted-preservation checks and the original D08A/B2, D05/D06/D07/query and C2/C/macro collections before certification. Accept language, effects and actual geometry, never byte equality or whitespace normalization to close render defects.

Retain with/without-column controls, located/pending occurrences, origin, effects and queries. The external paired-time gate requires repeated benefit over the no-gain baseline, no robust regression and no allocation increase. Arc reuse is auxiliary engineering evidence; text, geometry, morphology and effects establish language acceptance. The small historical improvement establishes neither global parity nor reduced resident memory. No public contract, default behavior, phase or compatibility change is authorized here.

## Historical record

`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/content/page_structure.md` preserves the original evidence and chronology.

## Complete-suite Source transparency regression — 2026-10-22

### Evidence before correction

`lab/results/remaining-parity-2026-10-22/candidate-7/validation/tests-domain-infra.log`
SHA-256 `e5900986d7df6f773083e7b95a967d76d451bccb7afe12a93e326d04b98ac8da`
records the complete unfiltered core/infra run. Its frozen inputs are retained in
`candidate-7/validation/validation-state.json.gz` and the unchanged-input check in
`candidate-7/validation/validation-freeze.json` at
`2026-10-22T14:53:49.321659+00:00`, under that same result root. The source-bound
build receipt `candidate-7/candidate-build/receipt.json`, SHA-256
`bcdd2171a19fd1e93bc4934388c35e8fb79563c957994fd9d6bae176930dd9fe`, retains
UTC `2026-10-22T14:44:18.153459+00:00`–`2026-10-22T14:44:36.818708+00:00`,
exact HEAD/uncommitted diffstat, inputs, source archive and product SHA-256
`91cba2e532c185020023aed9a0c031540e57f4b966673649b9a5b1cd8935ec95`.
The existing test bodies are unchanged from the initial baseline archive.
Six page-column layout regressions show missing implicit Columns after actual
SetPage gained its proven Source origin: four one-page requirements observe two
pages, and two RTL requirements begin in the wrong column. Bare-only SetPage
classification in content/page_structure.rs prevents the transform. Four further
observer failures stop at Source instead of examining the actual SetPage/Heading
body. Three separate paragraph failures have another owner and are not evidence
to change page configuration or remove Source. These actual failures supersede
focused-suite confidence; no warning/test/geometry threshold is waived.

### Exclusive correction and acceptance

Read Source.body through existing source_body only at the existing SetPage and
Pagebreak kind-classification points in wrap_page_columns/page_column_segments.
Keep original carrier objects and source spans in emitted output, original body
order, marker scope boundaries, child indices and all page/column settings.
Do not project ShowRecipe/Located/Realized, execute recipes, remove source
origins, change public API/phase or broaden implicit-column scope. Existing
nominal no-change protocol remains intact. Add a local sourced SetPage/Pagebreak
regression and confirm RED before productive classification changes. The six
original layout assertions and physical-page figure/query test remain unchanged.

## Approved batch E integration

Measurement: pre-E inputs and bilateral reflection in
`lab/results/symbol-identity-2026-10-23/{baseline.json,before/receipt.json,additional/receipt.json}`.
The owner approved the native Symbol declaration on 2026-10-23.
Handle Symbol explicitly as a native childless textual leaf in exhaustive complements and existing projections. Preserve the complete Unicode payload and reconstruct the same variant when mapping. Do not silently classify Symbol as a bare equation outside math.
Preserve the existing unit responsibility and all other variants/effects; no public contract beyond the approved Content addition or phase move.
