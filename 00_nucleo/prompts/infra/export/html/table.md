# L0 — html/table
Hash do Código: 7f2d1c03

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:2b9e828f5e0d958f4328d98a35320ba2ea321997e541e6ab90520d2e8811e7bf

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/html/table.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/html/table.rs:12` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Table encoding

Own html_table and flatten/row helpers. Use columns.len().max(1); a present header
emits thead/th and ordinary children tbody/td. Flatten Sequence/TableHeader/TableFooter
bodies under the existing rules. Cell colspan/rowspan default to one and attributes
are emitted only above one. Preserve chunk-based row grouping, including its current
span-occupancy limitation; do not invent a separate footer or expand supported tables.
Delegate cell content to inline with the same mode, resolver and StyleChain.

## Verification

Check empty/ordinary/header tables, flattened wrappers, spans and style/frame
propagation without paginated layout.

## Ratified resolved-table and isolated-cell support

### Evidence

`lab/results/construction/diagnostics/table-cell-realizacao-d08b3-2026-10-07/{before,additional,order,aliases}.json.gz`
retains HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/UTC/
diffstat/status/sources/binaries/argv/results at historical pin
`9f2b6e8715237cb086899a42873660fe744622e8`; oracle SHA
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`, Tekt SHA
`7c0957f45fe40290aa809e3358c9a87979aa1e28058511ad0ee978feea6713a2`.
Vanilla grid/resolve.rs:204–278 resolves fields, :1500–1530 ordered conflicts,
:1665–1703 empties, :1945–1988 breakable. Tekt grid_placement.rs:91–112 prioritizes
explicit cells and :262–270 knows only GridCell; realize/mod.rs:160–200 visits
bottom-up; table.rs:127–152 unwraps without attributes. Order probes observe zero
cell effects under table replacement and no hidden-heading effects; aliases
observe LOCAL a/OUTER LOCAL a/NEW a. Unknown fields/hidden effects/wrong order
refute the language hypothesis; no cache/pagination equivalence or Rust/byte
requirement is inferred.

`table-cell-isolada-d08b3b-2026-10-07/complete.json.gz` contains 53 probes from
2026-10-07T14:47:56.340144+00:00 through 14:48:12.375245+00:00, same HEAD/dirty
tree with diffstat/status/hashes/sources/argv/cwd/environment/full outputs. Same
vanilla pin/SHA; Tekt SHA
`6ca44e46ae2c1b1f8b44207052ea2fd63e3b24b4136c3fc3d25b6d20e9b6a20f`.
Do not count before/refuters/complete as independent extra rounds. Vanilla isolated
defaults expose x/y/inset/align/fill/breakable auto, spans 1, empty stroke dict;
Tekt omits them and emits two metadata versus one. isolated-x works only upstream;
raw-fields is only body in both and raw-x rejects in both. Table style is not
isolated Cell default. raw-vs-prepared is false upstream/true Tekt; nested/body/
late-rule controls refute once-only effects/lexical origin. fresh-isolated/table/
fresh-block/fresh-styled/multi-fresh incorrectly succeed in Tekt versus
`maximum show rule depth exceeded` with the new-constructor span. identity/body/
alias/fresh-conditional/hidden remain valid; multi-identity/multi-return-table show
returning it must permit earlier unconsumed recipes. Cell query is nonlocatable
in both, location none, no fabricated location. cell-set is a separate excluded
negative.

Upstream table.rs:734–778 declares defaults/properties; realize/lib.rs:367–386
prepares/guards arguments, :418–429 depth-visits product, :491–505 distinguishes
per-element guards, :571–586 materializes properties. Tekt table_cell.rs:111–143
projects lexical/positioned plan only; realize/table_cells.rs:110–129 reapplies raw
Cell, :280–304 terminates whole snapshot; show_rule_termination.rs:65–70 treats same
form as fixed point; rules/apply.rs:139–166 globally revokes callback rule. Fields/
effects/order/termination/causal errors are language; upstream guards/materialization
structure is mechanics. Nominal choice is an inference refutable by alias/two-rule/
new-generation/hidden-content controls, not upstream intent.

### Owner responsibility and recorded approval

Historical ratification receipts resolve the old candidate-status wording:
`lab/history/construction/records/00_nucleo/materialization/table-cell-realizacao-d08b3-2026-10-07.md:3`
records owner `comfirmo` on 2026-10-07, manifesto
`5fec42835e38e9a28ee2ea2714b30135f069f111c140bb5e918a0902a06cc82b`;
`lab/history/construction/records/00_nucleo/materialization/table-cell-isolada-d08b3b-2026-10-07.md:3`
records owner `confirmo`, materialized status and manifesto
`3375b0a09a4d39d02c2f0bafbcaf3016c6afde27acdf632331da7a98de31e1df`.
These approvals supersede only the corresponding pre-approval candidate gates.
The insertion package excludes isolated defaults until that separate approval;
the isolated-default package still excludes cell-set/style defaults and all other
uncompleted scope listed below. Historical candidates are receipts, never a
replacement for current canonical owning L0.

Complete TableGridResolved encoding uses row/col and header/footer
ranges with spans and visual Complete, no paginated layout or blind origin chunks.
Derived x/y are target-independent logical positions. Raw fallback stays legacy;
escaping/modes/resolver remain unchanged, no global HTML byte claim.

The isolated-cell approval covers its one public field/type independently of the
insertion package. Flatten/cell helpers use prepared isolated visuals when present,
without invented position, retaining positioned-plan/span contract. No double-body
visit or recipe execution in export.

### Acceptance, incompleteness and limits

The ratified packages require only their reviewed candidates/current owning L0,
with tests real RED
before implementation, owner-only seals, bilateral collections/additional reviews,
effects once/equality/getters/selection, HTML and actual text/style/geometry,
unaltered messages/spans. Build debug/release and appropriate core/infra/shell
suites, lint without errors/owner V5/V6/V15/V26; preserve prior tree by snapshot/
restricted diff, no commit/reset/global resealing. Revalidate prior table lexical/
realization, Raw defaults/blocks, Equation/math/matrix/query controls identified in
the archived contract. Do not use byte equality or whitespace normalization to
close render, change legacy extra casts/syntax/flags/global nonlocatable policy.

The insertion package was explicitly incomplete for isolated intrinsic defaults
(raw-cell-no-placement/outside-fields) and fresh show-cell-reconstruct generation;
these are completed only by the separate isolated-default approval. That package
completes only intrinsic defaults and
alias/new-show-generation distinction; cell-set/style defaults require a third
measured L0/gate with no promised schedule/implementation. Generic Table defaults/
equality/fields, rich function styles/paints/strokes/insets/lines/gutters,
multipage spans/PDF-UA/global nonlocatable policy remain open. Do not correct or
normalize the existing empty-file eval warning to assert parity. Historical
regression collections retain table lexical/raw/smartquote/matrix/query obligations
without claiming those earlier runs certify the new upstream pin. Full original:
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/infra/export/html/table.md`.
