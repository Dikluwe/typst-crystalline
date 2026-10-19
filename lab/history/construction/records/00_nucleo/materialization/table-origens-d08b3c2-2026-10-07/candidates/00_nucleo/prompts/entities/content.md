# L0 — entities/content
Hash do Código: 89942c1a

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


## Realização com avaliação única

Content possui ShowRecipe(Arc<ShowRecipeContent>) com body, rules Arc<[ShowRule]>, styles StyleChain, guards Vec<RuleId>, file FileId; Realized(Arc<RealizedContent>) com original e visual; Located(Arc<LocatedContent>) com location Location e body. São transportes internos imutáveis; Located conserva a ocorrência durante receitas, Realized conserva a origem consumida sem emitir seu visual anterior. Projeções da linguagem são transparentes. Locator permanece único construtor de Location.


## Realização com avaliação única

ShowRecipeContent inclui textual: bool para distinguir captura de grupo textual causal da receita de elemento. A realização textual reutiliza agrupamento/splice do owner vigente depois de realizar elementos do grupo.

Located é referência a uma ocorrência já preparada: múltiplos aliases conservam a identidade. A criação de novas ocorrências por inserção acontece antes de essa preparação, sem colapsar conteúdo comum por igualdade.


## Realização com avaliação única

ShowRecipeContent transporta context_id_floor: u64, watermark de ids contextuais já alocados no ambiente capturado. A realização começa depois dessa faixa; não colidir produtor novo com conteúdo contextual capturado em bindings.


## D08B3-C2 — candidato não vigente antes do gate ADR-0127

### Medição anterior à decisão

`00_nucleo/diagnosticos/table-origens-d08b3c2-2026-10-07/baseline.json.gz` registra oito sondas completas: fontes, comandos, UTC, HEAD, working tree/diffstat, hashes e binários. Pin 9f2b6e8715237cb086899a42873660fe744622e8. Executável Tekt congelado do C validado; `source-verification.json` confirma hashes produtivos/L0 iguais ao snapshot tested-final do C. O target da raiz não estava disponível nesta retomada; não foi fingido um rebuild.

bare/alias/spread-conflict: mensagens e hints iguais, caret/trace divergentes. No alias o vanilla usa a definição [a] (linha 1), não a chamada (linha 3). No spread usa [a], não o spread inteiro. local-body publica sequence com metadata+Cell preparada no Tekt, styled contendo Cell crua no vanilla; cores/efeitos coincidem no fixture. plain-func coincide cell; set-func/show-func/set-show-func medem cell/sequence contra styled. A primeira rodada `before` tinha str(function), foi invalidada para decidir morfologia; somente baseline usa repr(function).

Fonte: vanilla `typst-eval/src/code.rs:148` aplica Value.spanned; `typst-library/src/foundations/value.rs:210–216` só assenta Content/Func; `foundations/content/mod.rs:109–114` preserva span já assentado. `model/table.rs:436–437` transporta value.span ao fabricar Cell. `typst-eval/src/code.rs:26–57` conserva set/show como styled sobre a cauda. Tekt `eval/expression_dispatch.rs:125–136` realiza Cell no join; `stdlib/structural/table_grid.rs:160–175` guarda children sem origem; `realize/table_cells.rs:176–183` fabrica Cell com span detached; `layout/grid/row_groups.rs:109` usa esse span. É língua (diagnóstico/morfologia/efeitos), não igualdade Rust ou bytes PDF. Não inferir intenção upstream.

### Responsabilidade exclusiva candidata

Declaração pública exata SourceContent e Content::Source conforme contract.md; preservação da transparência e separação de origem/localização. Este owner só declara dados; projeções e travessias permanecem nos owners exclusivos existentes.

A especificação detalhada e limites estão no contrato candidato do pacote C2. Antes de materializar, instalar este conteúdo no caminho canônico do owner, auditar seus núcleos e selar hashes somente depois de RED→GREEN. A fonte vigente permanece intacta até aprovação.

A cláusula C2 supersede a realização imediata da Cell no join descrita pelo refinamento A: capturar valor lexical e realizar somente na inserção efetiva. Essa mudança de fronteira/ordem também integra o gate ADR-0127 §2.3; preservar a ordem Table→placement→Cell→body do núcleo vigente.
