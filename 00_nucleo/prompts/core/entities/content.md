# L0 — entities/content
Hash do Código: 91785886

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:059eb35863a413593c98fb7195a521e3d019dadf461433e9c42422218be89065
- 00_nucleo/prompts/_nuclei/math-attach-slot-presence.toml sha256:cf64af6ef56faaa1ba19fa32ec6c134dc3f4f16a4a74fde21eca40259cf3b6ba

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/content.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Own the closed Content enum, SetPageDelta and StyledContent declarations and
the composition of private behavior/test modules. Native content uses exhaustive
static matching, not vtables, unsafe, proc-macro dispatch or property maps.
Dynamic remains the explicit extension boundary, not native dispatch.
SetPage and Styled hold immutable Arc payloads; sequences share Arc slices.
Preserve every variant, public payload type and omission distinction below.
Nested Option page deltas distinguish unspecified settings from explicit
clearing; do not flatten them. Text carries semantic text rather than baked
render styles. SpaceFromNewline preserves causal origin, independently of
language-level space equivalence. Native element structs retain their own owners.

This consumer must not implement constructors, normalization, projections,
identity/debug, equality/hash, morphology or traversal delegated to children.
It must not validate stdlib arguments, perform eval/layout/export, infer file
targets or access I/O. Their behavior remains under their exclusive owners.

The exclusive public data contract is:

```rust
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SetPageDelta {
    pub paper: Option<crate::entities::page_geometry::Paper>,
    pub flipped: Option<bool>,
    pub binding: Option<crate::entities::page_geometry::PageBinding>,
    pub width: Option<crate::entities::layout_types::PageDimension>,
    pub height: Option<crate::entities::layout_types::PageDimension>,
    pub margin: Option<crate::entities::layout_types::PageMarginSpec>,
    pub numbering: Option<Option<crate::entities::numbering::Numbering>>,
    pub number_align: Option<crate::entities::page_running::PageNumberAlign>,
    pub header: Option<crate::entities::page_running::PageMarginal>,
    pub header_ascent: Option<crate::entities::page_running::PageMarginalOffset>,
    pub footer: Option<crate::entities::page_running::PageMarginal>,
    pub footer_descent: Option<crate::entities::page_running::PageMarginalOffset>,
    pub supplement: Option<crate::entities::page_supplement::PageSupplement>,
    pub columns: Option<usize>,
    pub bleed: Option<crate::entities::page_canvas::PageBleedSpec>,
    pub fill: Option<crate::entities::page_canvas::PageFill>,
    pub background: Option<Option<Arc<Content>>>,
    pub foreground: Option<Option<Arc<Content>>>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct StyledContent {
    pub body: Content,
    pub styles: crate::entities::style::Styles,
}
#[derive(Clone)]
pub enum Content {
    Empty,
    Text(EcoString),
    Space,
    SpaceFromNewline,
    Parbreak,
    Sequence(Arc<[Content]>),
    HtmlElem(Arc<crate::entities::html::HtmlElem>),
    HtmlFrame(Arc<crate::entities::html::HtmlFrameElem>),
    Par {
        body: Box<Content>,
    },
    Heading(Arc<HeadingElem>),
    Title(Arc<TitleElem>),
    Strong(Arc<StrongElem>),
    Emph(Arc<EmphElem>),
    Raw(Arc<RawElem>),
    ListItem(Arc<ListItemElem>),
    EnumItem(Arc<EnumItemElem>),
    Link(Arc<LinkElem>),
    Equation(Arc<EquationElem>),
    MathSequence(Arc<[Content]>),
    MathIdent(EcoString),
    MathText(EcoString),
    MathFrac(Arc<MathFracElem>),
    MathAttach(Arc<MathAttachElem>),
    MathPrimes(Arc<MathPrimesElem>),
    MathRoot(Arc<MathRootElem>),
    MathStretch(Arc<MathStretchElem>),
    MathDelimited(Arc<MathDelimitedElem>),
    MathLr(Arc<MathLrElem>),
    MathMid(Arc<MathMidElem>),
    MathOverbrace(Arc<MathOverbraceElem>),
    MathUnderbrace(Arc<MathUnderbraceElem>),
    MathUnderbracket(Arc<MathUnderbracketElem>),
    MathUnderparen(Arc<MathUnderparenElem>),
    MathUndershell(Arc<MathUndershellElem>),
    MathOverbracket(Arc<MathOverbracketElem>),
    MathOverparen(Arc<MathOverparenElem>),
    MathOvershell(Arc<MathOvershellElem>),
    MathAlignPoint(Arc<MathAlignPointElem>),
    Linebreak(Arc<LinebreakElem>),
    MathMatrix(Arc<MathMatrixElem>),
    MathCases(Arc<MathCasesElem>),
    MathAccent(Arc<MathAccentElem>),
    MathCancel(Arc<MathCancelElem>),
    MathUnderline(Arc<MathUnderlineElem>),
    MathOverline(Arc<MathOverlineElem>),
    MathVec(Arc<MathVecElem>),
    MathClassOverride(Arc<MathClassOverrideElem>),
    MathLimitsOverride(Arc<MathLimitsOverrideElem>),
    MathUnderover(Arc<MathUnderoverElem>),
    MathOp(Arc<MathOpElem>),
    MathStyled(Arc<MathStyledElem>),
    Label(Arc<LabelElem>),
    Ref(Arc<RefElem>),
    CounterDisplay(Arc<CounterDisplayElem>),
    CounterUpdate(Arc<CounterUpdateElem>),
    Outline(Arc<OutlineElem>),
    OutlineEntry(Arc<OutlineEntryElem>),
    Figure(Arc<FigureElem>),
    FigureCaption(Arc<FigureCaptionElem>),
    Image(Arc<ImageElem>),
    Shape(Arc<ShapeElem>),
    Curve(Arc<CurveElem>),
    Transform(Arc<TransformElem>),
    Grid(Arc<GridElem>),
    GridHeader(Arc<GridHeaderElem>),
    GridFooter(Arc<GridFooterElem>),
    GridCell(Arc<GridCellElem>),
    SetPage(Arc<SetPageDelta>),
    PageRun(Arc<PageRunElem>),
    Align(Arc<AlignElem>),
    Place(Arc<PlaceElem>),
    Flush(Arc<FlushElem>),
    Styled(Arc<StyledContent>),
    Divider(Arc<DividerElem>),
    Terms(Arc<TermsElem>),
    TermItem(Arc<TermItemElem>),
    Quote(Arc<QuoteElem>),
    Document {
        path: EcoString,
        body: Box<Content>,
        format: Option<EcoString>,
        title: Option<Box<Content>>,
        author: Vec<EcoString>,
        date: Option<Datetime>,
        keywords: Vec<EcoString>,
    },
    Asset {
        path: EcoString,
        data: Arc<[u8]>,
    },
    SmartQuote(Arc<SmartQuoteElem>),
    PdfAttach(Arc<PdfAttachElem>),
    PdfArtifact(Arc<PdfArtifactElem>),
    Underline(Arc<UnderlineElem>),
    Strike(Arc<StrikeElem>),
    Overline(Arc<OverlineElem>),
    SmallCaps {
        body: Box<Content>,
    },
    Pad(Arc<PadElem>),
    Hide(Arc<HideElem>),
    HSpace(Arc<HSpaceElem>),
    VSpace(Arc<VSpaceElem>),
    Pagebreak(Arc<PagebreakElem>),
    Colbreak(Arc<ColbreakElem>),
    Stack(Arc<StackElem>),
    Boxed(Arc<BoxedElem>),
    Block(Arc<BlockElem>),
    TableCell(Arc<TableCellElem>),
    Bibliography(Arc<BibliographyElem>),
    Cite(Arc<CiteElem>),
    Footnote(Arc<FootnoteElem>),
    FootnoteEntry(Arc<FootnoteEntryElem>),
    TableHeader(Arc<TableHeaderElem>),
    TableFooter(Arc<TableFooterElem>),
    GridHLine(Arc<GridHLineElem>),
    GridVLine(Arc<GridVLineElem>),
    TableHLine(Arc<TableHLineElem>),
    TableVLine(Arc<TableVLineElem>),
    Table(Arc<TableElem>),
    Repeat(Arc<RepeatElem>),
    Columns(Arc<ColumnsElem>),
    Metadata(Arc<MetadataElem>),
    State(Arc<StateElem>),
    StateUpdate(Arc<StateUpdateElem>),
    StateDisplay(Arc<StateDisplayElem>),
    CounterDisplayCallback(Arc<CounterDisplayCallbackElem>),
    ContextBlock(Arc<ContextBlockElem>),
    Dynamic(Arc<dyn DynElement>),
}

```

## Acceptance

Verify the owned behavior through local tests and its existing integrations.
Preserve public paths and observable results. Ownership, nucleus integrity and
lineage checks must remain valid; no unrelated consumer is authorized by this prompt.



## Immutable occurrence transport

Content additionally declares ShowRecipe(Arc<ShowRecipeContent>), Realized(Arc<RealizedContent>) and Located(Arc<LocatedContent>). ShowRecipeContent retains body, rules:Arc<[ShowRule]>, styles:StyleChain, guards:Vec<RuleId>, file:FileId, textual:bool and context_id_floor:u64. Realized retains original and visual; Located retains location:Location and body. These additions supplement the base enum excerpt, not replace its variants.

Located preserves a prepared occurrence through recipes and aliases. Fresh insertion creates occurrences before preparation without merging ordinary equal content. Realized retains consumed origin without reemitting its previous visual. Language projections are transparent. Locator remains the sole Location constructor. textual distinguishes captured causal text groups from element recipes; text realization reuses its owner's grouping/splice after group elements are realized. context_id_floor is the captured environment's contextual-ID watermark; realization begins after that range to avoid collisions with captured bindings.

## Source-origin evidence and approval

`lab/results/construction/diagnostics/table-origens-d08b3c2-2026-10-07/baseline.json.gz` preserves eight complete probes, sources/commands/UTC/HEAD/tree/diffstat/hashes/binaries at historical pin `9f2b6e8715237cb086899a42873660fe744622e8`. The validated C executable was frozen; source-verification.json matches productive/L0 hashes to C's tested-final snapshot. The root target was unavailable, so no rebuild was claimed.

bare/alias/spread-conflict matched messages/hints but differed in caret/trace. Vanilla points to alias definition [a] on line 1 rather than call line 3, and [a] rather than the whole spread. local-body is local sequence(metadata+prepared Cell) versus upstream styled(raw Cell), with matching fixture colors/effects. plain-func agrees on cell; set-func/show-func/set-show-func differ cell/sequence versus styled. The first before round used str(function) and was invalid for morphology; only baseline uses repr(function).

Upstream eval/code.rs:148 applies Value.spanned; value.rs:210–216 handles Content/Func, content/mod.rs:109–114 preserves existing origin, table.rs:436–437 transfers value.span to implicit Cell, and eval/code.rs:26–57 retains set/show as styled tail. Local expression_dispatch.rs:125–136 realized Cell during join, table_grid.rs:160–175 omitted child origin, realize/table_cells.rs:176–183 fabricated detached spans and row_groups.rs:109 consumed them. These are diagnostic/morphology/effect observations, not upstream intent, Rust equality or PDF-byte requirements.

The owner confirmed `confirmo` on 2026-10-07 in the final record `lab/history/construction/records/00_nucleo/materialization/table-origens-d08b3c2-2026-10-07.md`; the exact contract `lab/history/construction/records/00_nucleo/materialization/table-origens-d08b3c2-2026-10-07/contract.md` remains the earlier candidate text. The earlier candidate gate is satisfied for this exact Source contract and causal insertion boundary.

## Source declaration and boundary

```rust
#[derive(Debug, Clone)]
pub struct SourceContent {
    pub body: Content,
    pub span: crate::entities::span::Span,
}
// Additional Content variant:
// Source(Arc<SourceContent>),
```

Source is transparent origin metadata, never a new language element or introspectable location. This owner declares data only; projection/traversal remain with their exclusive owners. Capture lexical value and realize at effective insertion, superseding immediate Cell realization at join. Retain Table→placement→Cell→body and surviving-content effects; no eval/layout phase move is authorized. Preserve first origin through reference/clone/spread, distinct from location; synthetic unproven origin remains detached. New public additions beyond the approved declaration need a separate gate.

Acceptance includes baseline probes and cross-Source/import aliases, multiple cells/headers, reuse/spread/forwarded Args, synthetic detached origin, raw-field equivalence, explicit-constructor origin, hidden/replaced Cell, metadata order/count and local/global guard/depth. Preserve C geometry/text/paints and Table/Raw/math/matrix/query macros; debug/release builds, appropriate suites and no owner V5/V6/V15/V26. Record state/hashes/UTC for measurements. The original implementation package's no-commit/reset/automatic-cleanup restriction is historical logistics, not additional authority to alter data here.

## Historical record

`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/content.md` preserves the original evidence and chronology.
