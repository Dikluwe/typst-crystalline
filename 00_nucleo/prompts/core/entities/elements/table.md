# Prompt L0 — `entities/elements/table` — `TableElem`
Hash do Código: b92c676f

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/map-content-change.toml sha256:6343f553bd5f842f83190bd6533a9a9642118a34214550d8289c9daaba219043
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:2b9e828f5e0d958f4328d98a35320ba2ea321997e541e6ab90520d2e8811e7bf

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/table.rs`

## Base carrier and evidence

The archived base model uses a Content::Table Arc carrier with optional caption, real header/footer row groups and manually Debug-based Hash because TrackSizing/Stroke/Color contain f64. Upstream model/table.rs:495,525 identifies header/footer as child elements, not named arguments. Preserve the original base fields:

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct TableElem {
    pub columns:  Vec<TrackSizing>,
    pub rows:     Vec<TrackSizing>,
    pub children: Vec<Content>,
    pub hlines:   Vec<TableHLineElem>,
    pub vlines:   Vec<TableVLineElem>,
    pub header:   Option<Content>,
    pub footer:   Option<Content>,
    pub stroke:   Option<Stroke>,
    pub fill:     Option<Color>,
    pub caption:  Option<Content>,
}
```

Header/footer each hold at most one corresponding TableHeader/TableFooter, with an explicit duplicate error during native-table child resolution. Layout consumes them as row groups through the same mechanism as Grid. Constructors table(columns,rows,children) and table_with_caption retain compatibility; table defaults caption to None. A numbered caption does not turn Table into Figure or match figure.where(kind: table).

The old nonlocatable heading conflicts with its own element_kind/to_payload exception and the later Table kind contract. Preserve the existing Table payload/indexing boundary; do not use that stale heading to remove locatability. Exact extraction remains with its owner. Derived PartialEq covers every field, not only the original ten; Hash retains the established mechanics.

For raw tables plain_text prefixes caption when present and joins children with spaces; is_empty is children.is_empty() && caption.as_ref().is_none_or(|c| c.is_empty()). Maps recurse through children/header/footer/caption while retaining tracks, lines, stroke, fill and metadata. The prepared contract below qualifies which contributions are visited.

## Accessibility summary proposal and outstanding approval

Historical table.rs:29–42 lacked summary; upstream table.rs:271–277 stores optional text and pdf/accessibility.rs:137–143 changes only it through pdf.table-summary. The archived accessibility proposal requires optional semantic summary separate from caption/visible text: omission→None, explicit none rejected by the cast, empty string retained. Constructors/clone/hash/mechanical equality/maps preserve it; plain_text/is_empty ignore it. Transport to tagged PDF /Table must not change text/pages/boxes/geometry. The original P1288 record still describes a public gate; this consolidation has not established its approval receipt and does not promote the proposal to a newly authorized contract. Keep the requirement and approval-verification debt visible.

## Prepared table evidence and approval

`lab/results/construction/diagnostics/table-cell-realizacao-d08b3-2026-10-07/{before,additional,order,aliases}.json.gz` records HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/UTC/diffstat/status/sources/binaries/argv, historical vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`, oracle SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea` and Tekt `7c0957f45fe40290aa809e3358c9a87979aa1e28058511ad0ee978feea6713a2`. Upstream grid resolve.rs:204–278 resolves fields,1500–1530 ordered conflicts,1665–1703 empty cells and 1945–1988 breakable. Local grid_placement.rs:91–112 prioritizes explicit cells and 262–270 only knows GridCell; realize/mod.rs:160–200 visits bottom-up and table.rs:127–152 strips wrappers without attributes. Probes show zero cell effects when Table is replaced, zero hidden-heading effects and aliases `LOCAL a`, `OUTER LOCAL a`, `NEW a`. Unknown fields, hidden effects or wrong order refute the hypothesis. Cache/pagination equivalence is not inferred, and Rust mechanics/bytes are not the acceptance target.

The original D08B3-A gate was satisfied by `lab/history/construction/records/00_nucleo/materialization/table-cell-realizacao-d08b3-2026-10-07.md`, manifest `5fec42835e38e9a28ee2ea2714b30135f069f111c140bb5e918a0902a06cc82b`. It covers the specified five public fields, two carriers, facade and logical phase before Cell show, without authorizing later extensions.

## Prepared carrier contract

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct TableGridResolved {
    pub cells: Vec<Content>,
    pub num_cols: usize,
    pub num_rows: usize,
    pub header_rows: usize,
    pub footer_start: usize,
    pub(crate) complete: bool,
}
```

TableElem additionally retains public source_span:Span, rows_explicit:bool and resolved:Option<Arc<TableGridResolved>>. Raw construction has resolved=None. Explicit rows differ from omitted fallback [Auto]; no children and no explicit rows must not fabricate a cell. Raw children/header/footer remain lexical source. The plan holds actual/empty cell origins once, not duplicate span entries, in row-major order. header_rows describes [0,n); absent footer uses footer_start=num_rows. Getters/serialization expose no new language fields for these carriers.

With resolved Some, visual traversal/emptiness/plain_text consume complete cells once, not raw children/header/footer in parallel. Maps/clone preserve lexical source and transport visual content without executing recipes.

## Change-aware traversal evidence and contract

`lab/results/construction/diagnostics/table-local-scaling-followup-2026-10-09/change-aware-red-gate.json.gz` records the same HEAD/uncommitted provenance plus UTC/script/fixtures/interpositor/CPU/argv, release `29745a9e80ca43160b34f4807949f00e803070392653df0fc5e214b2f42ac33a`:483823 glibc requests on local-set-1600 exceeded the predefined 470000 gate. traversal.rs:25–55,235–264 and table.rs:85–118/table_cell.rs:308–344 rebuild unchanged containers/metadata. Pinned upstream realize/lib.rs:242–291,353–410 visits references and materializes after change; this motivates a refutable cost hypothesis without requiring its vtable/equality.

Provide only a crate-private reconstruction helper returning None when every visited child is unchanged. Raw order is children,header,footer,caption; prepared order is exclusively resolved.cells then caption, preserving raw source/metadata. Delegate descent to the explicit proof in `00_nucleo/prompts/core/entities/content/traversal.md`, allocating Table/plan only after first change. Existing Element signature/semantics reuse this implementation without a new trait method. Do not use equality or process effects twice. Verify child/prepared/caption/order tests and paired gates, without certifying excluded variants. The pinned no-change nucleus shares only the proof; this owner retains algorithm/fields/order/preservation.

## Prepared table acceptance and incomplete scope

Require RED→GREEN, suites/build/lint, restricted preservation and the original D08A/B2, D05/D06/D07/query collections. Accept language, effects and actual geometry, never byte equality or whitespace normalization to close render defects. The isolated-cell package separately completes raw-cell-no-placement/outside-fields defaults and fresh show-cell-reconstruct generation; the first prepared-table package alone is explicitly incomplete.

Generic Table defaults/fields (`table-root-equality`), function styles, rich paints/strokes/insets, lines/gutters, multipage spans and PDF/UA remain outside certification. Do not change legacy extra casts, syntax, flags or global nonlocatable policy through this slice.

## Historical record

`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/table.md` preserves the original evidence and chronology.
