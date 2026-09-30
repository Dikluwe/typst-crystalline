# L0 — entities/content
Hash do Código: 765a49a7

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/math-attach-slot-presence.toml sha256:81b492ca5d01377da0b54b6deb21b6cb24b20919009ea3ea21b7350959779715

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
