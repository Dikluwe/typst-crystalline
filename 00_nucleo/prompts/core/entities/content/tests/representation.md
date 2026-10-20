# L0 — entities/content/tests/representation
Hash do Código: faf4b690

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/smartquote-semantics.toml sha256:d92a80d952b2cb79d90b96a75a0387cfde2da24f2fd41327de8ab0e647a97f8c
- 00_nucleo/prompts/_nuclei/table/cell-occurrence.toml sha256:bba33b336e651ccd6d41f88b0e16449932d0a986de1ee2cc3c1fdf5c8fb8c6dc
- 00_nucleo/prompts/_nuclei/table/cell-presence.toml sha256:f7d3050154813e1ea46c26e235383ae337781a4b28d10ef3248f6eaec96ce221
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:2b9e828f5e0d958f4328d98a35320ba2ea321997e541e6ab90520d2e8811e7bf

**Layer:** L1 test-only
**Exclusive consumer:** `01_core/src/entities/content/tests/representation.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract and acceptance

Own independent regression cases/fixtures for construction, closed-variant representation, structural equality, clone, defaults and payload preservation. Observe public/crate-visible behavior, not private structure or mirrored implementation; add no productive dependency, dynamic dispatch or reverse layer dependency. Execute within the Content suite.

## SmartQuote expectation evidence

The historical core-quotes-green.log records type failures in old tests treating double as bool. The ratified semantic contract distinguishes omitted None and explicit Some; enabled:false remains SmartQuote. Update expectations to those fields while retaining argument/validation coverage. This is continuous ADR-0127 test alignment, with no additional productive change.

## Lexical Cell presence evidence and scope

`lab/results/construction/diagnostics/table-cell-realizacao-2026-10-07/before.json.gz` and presence-refuters.json retain HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, tree/diffstat/UTC/binaries/argv against historical pin `9f2b6e8715237cb086899a42873660fe744622e8`. Upstream table.rs:734–778 retains Smart/Option; local table_cell.rs:40–51 collapsed x:auto/omission and fill:none/stroke:none. Probes allow auto in x/y/fill/align/inset/breakable, reject it in colspan/rowspan/stroke, and allow none only in fill/stroke. These are language distinctions without a mandate to copy storage.

Fixtures must supply presence matching explicit arguments. Keep all existing render observations/assertions/tolerances and never fabricate known defaults. Approval on 2026-10-07 is in `lab/history/construction/records/00_nucleo/materialization/table-cell-presenca-aprovacao-2026-10-07.json`; it does not change render phase/defaults. Require body/presence/method/JSON/clone/map/observation/rejection-span RED→GREEN, builds/suites/lint and D08A/D05/D07/new-refuter collections with exact provenance. Position/style/padding/show realization has its separate bounded approval.

## Prepared table evidence and approval

`lab/results/construction/diagnostics/table-cell-realizacao-d08b3-2026-10-07/{before,additional,order,aliases}.json.gz` records HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/UTC/diffstat/status/sources/binaries/argv, historical vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`, oracle SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea` and Tekt `7c0957f45fe40290aa809e3358c9a87979aa1e28058511ad0ee978feea6713a2`. Upstream grid resolve.rs:204–278 resolves fields,1500–1530 ordered conflicts,1665–1703 empty cells and 1945–1988 breakable. Local grid_placement.rs:91–112 prioritizes explicit cells and 262–270 only knows GridCell; realize/mod.rs:160–200 visits bottom-up and table.rs:127–152 strips wrappers without attributes. Probes show zero cell effects when Table is replaced, zero hidden-heading effects and aliases `LOCAL a`, `OUTER LOCAL a`, `NEW a`. Unknown fields, hidden effects or wrong order refute the hypothesis. Cache/pagination equivalence is not inferred, and Rust mechanics/bytes are not the acceptance target.

The original D08B3-A gate was satisfied by `lab/history/construction/records/00_nucleo/materialization/table-cell-realizacao-d08b3-2026-10-07.md`, manifest `5fec42835e38e9a28ee2ea2714b30135f069f111c140bb5e918a0902a06cc82b`. It covers the specified five public fields, two carriers, facade and logical phase before Cell show, without authorizing later extensions.

## Prepared-table regression responsibility

Adapt raw/default constructor fields while preserving fixture meaning, observations and tolerances. Add meaningful RED→GREEN controls for fields, order, effects, aliases, hidden content, HTML and geometry. Tests authorize nothing beyond the ratified contract.

## Prepared table acceptance and incomplete scope

Require RED→GREEN, suites/build/lint, restricted preservation and the original D08A/B2, D05/D06/D07/query collections. Accept language, effects and actual geometry, never byte equality or whitespace normalization to close render defects. The isolated-cell package separately completes raw-cell-no-placement/outside-fields defaults and fresh show-cell-reconstruct generation; the first prepared-table package alone is explicitly incomplete.

Generic Table defaults/fields (`table-root-equality`), function styles, rich paints/strokes/insets, lines/gutters, multipage spans and PDF/UA remain outside certification. Do not change legacy extra casts, syntax, flags or global nonlocatable policy through this slice.

## Isolated cell evidence and approval

`lab/results/construction/diagnostics/table-cell-isolada-d08b3b-2026-10-07/complete.json.gz` contains 53 probes from 2026-10-07T14:47:56.340144+00:00 to 14:48:12.375245+00:00, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/status/diffstat/hashes/source/argv/cwd/environment/output; historical pin `9f2b6e8715237cb086899a42873660fe744622e8`, oracle `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`, Tekt `6ca44e46ae2c1b1f8b44207052ea2fd63e3b24b4136c3fc3d25b6d20e9b6a20f`. before/refuters/complete are not independent additional rounds.

Vanilla isolated-default exposes x/y/inset/align/fill/breakable auto, spans 1 and empty stroke dict; Tekt omits them and emits two metadata items instead of one. isolated-x succeeds upstream but fails locally. raw-fields is body-only and raw-x rejects in both. Table defaults do not become isolated Cell defaults. raw-vs-prepared is false upstream and true locally. nested-effects/body-effects/late-rule refute single execution and lexical origin. Local fresh-isolated/table, fresh-block, fresh-styled and multi-fresh wrongly succeed where upstream rejects `maximum show rule depth exceeded` at the new constructor span. identity/body/alias and fresh-conditional/hidden stay valid. multi-identity/multi-return-table require returned it to permit an earlier unconsumed recipe. isolated-query rejects `cell is not locatable` in both, location stays none; cell-set is a separate excluded negative.

Upstream model/table.rs:734–778 defines defaults; realize/lib.rs:367–386 prepares/guards the argument,418–429 visits product with depth,491–505 separates element guards,571–586 materializes properties. Local table_cell.rs:111–143 projects lexical/positioned only, realize/table_cells.rs:110–129 reapplies raw cells and 280–304 terminates a whole snapshot, show_rule_termination.rs:65–70 treats same shape as fixpoint and rules/apply.rs:139–166 disables the rule globally during callback. Fields/effects/order/termination/causal errors are language; upstream guard structure is not mandatory. Aliases, two rules, fresh generations and hidden content can refute the nominal design; no upstream intent is inferred.

The separate D08B3-B public-field/type gate was satisfied in `lab/history/construction/records/00_nucleo/materialization/table-cell-isolada-d08b3b-2026-10-07.md`, manifest `3375b0a09a4d39d02c2f0bafbcaf3016c6afde27acdf632331da7a98de31e1df`. Approval A alone did not authorize it.

## Isolated-cell regression responsibility

Adapt literals with standalone=None without changing old controls. Add only necessary owned visual/clone/map/representation/HTML/geometry behavior controls; do not mirror the implementation.

## Isolated cell acceptance and limits

Install only the ratified candidates, tests RED before implementation and reseal only changed owners. Require the recorded and additional bilateral collections, exactly-once effects, equality/getters, selection, HTML product and actual text/styles/geometry; preserve message/span differences. Build debug/release, appropriate core/infra/shell suites and linter without errors or owner V5/V6/V15/V26; revalidate D08A/B2/B3-A, Raw defaults/blocks, Equation/math/matrix/query. Preserve the prior tree by snapshot/restricted diff; the original construction package prohibited commit/reset/global resealing.

This slice completes intrinsic isolated defaults and alias-versus-fresh-show distinction. Style defaults/set table.cell (`cell-set`) require their separate measured owner contract, with no promised implementation implied by scope-out. General Table defaults/equality, rich casts/paints/strokes/insets, lines/gutters/multipage spans, PDF/UA and general nonlocatable policy remain open. The preexisting empty-file eval warning is neither corrected nor normalized to claim parity.

## Historical record

`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/content/tests/representation.md` preserves the original evidence and chronology.


## Approved Figure/Grid occurrence amendment (2026-10-19)

### Evidence before specification

Baseline `731f0ffdf4c7c152f590551e72ff63f7bd85ef85` preserves the measured proposal in `lab/results/v16-corrections-2026-10-19/figure-kind/review.md`, its `agents/contracts/design.md`, semantic partition, independent audit and counter receipts. The fixed oracle is commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree `cac836c07e4dec2a89219370428292393a3216b5`. Constructor string kind loses function identity; constructor inference precedes occurrence styles; separated Grid fields lose source order. Typed identity, lexical presence, field traversal and counter results are language semantics/morphology. The chosen Rust carriers are implementation mechanics. The owner explicitly approved the linked public fields, contracts, phases and compatibility changes on 2026-10-19 under ADR-0127; this amendment materializes that bounded decision.

### Exclusive contract

Migrate direct Grid fixtures with source_order:None (legacy synthetic order), preserving representation, physical-size and existing semantic assertions. No invented carrier data or relaxed bounds.

### Acceptance and limits

Preserve all unrelated prior obligations and nucleus pins. This amendment supersedes only conflicting Figure/Grid clauses stated above. Require independent semantic RED→GREEN, exhaustive decision extension builds, existing workspace/build/tests/all-targets/Clippy/Rustdoc/fmt, canonical paths, strict forward/reverse lineage and valid V15/V26 before scoped resealing. No suppression, severity downgrade, weakened assertion or global hash repair. The bounded package does not certify full Figure/Grid parity: standalone HTML rendering, complete caption synthesis, supplement-dependent selector preview, external Dynamic enumeration, Grid footer phase and complex Table supplemental target order remain visible debt.
