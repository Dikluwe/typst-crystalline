# L0 — entities/content
Hash do Código: 7800c661


**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/content.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:059eb35863a413593c98fb7195a521e3d019dadf461433e9c42422218be89065
- 00_nucleo/prompts/_nuclei/content/symbol-identity.toml sha256:5060f0989415e92d54eab2a82afd84ba11472aef635d15830f9fb6ee3469aea9
- 00_nucleo/prompts/_nuclei/math-attach-slot-presence.toml sha256:cf64af6ef56faaa1ba19fa32ec6c134dc3f4f16a4a74fde21eca40259cf3b6ba


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


## Approved native Symbol declaration — batch E

**Status: approved by the repository owner on 2026-10-23: “Confirmo a adição
de Content::Symbol(EcoString) proposta no L0.” The ADR-0127 public gate is satisfied.**
The approved Source declaration above remains unchanged. This proposal adds only
the following variant to the public enum owned by this prompt:

```rust
// Additional Content variant, approved:
Symbol(EcoString),
```

### Measurement before classification

The frozen pre-E working tree and complete input hashes are recorded in
`lab/results/symbol-identity-2026-10-23/baseline.json`, with source text in
`baseline-source.json.gz`. The fresh bilateral `before/receipt.json` reproduces
all 16 historical carrier differences: equal complete Unicode strings but
`func: text` locally versus `func: symbol` upstream. All 146 previously matching
cases still match; the 36 local extensions still succeed locally and fail
upstream and are not parity successes. Additional reflection probes are in
`additional/receipt.json`. These observations measure language morphology and
reflection, not Rust equality, implementation structure or output-byte parity.

Local `compiler/eval/markup_stream.rs:567` turns raw Symbol values into Text;
`compiler/eval/math/expression.rs:33-60,179` separately creates MathText or
MathIdent for symbol-producing expressions. The closed enum in the exclusive
consumer has no native Symbol leaf. Pinned upstream
`crates/typst-library/src/foundations/symbol.rs:451-475` declares a text-bearing
SymbolElem; `foundations/value.rs:191-205` inserts it for Symbol values and
`crates/typst-eval/src/math.rs:39,83-87` preserves grapheme/shorthand identity.
Upstream here means commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree
`cac836c07e4dec2a89219370428292393a3216b5`, not moving main.

### Approved declaration and limits

Symbol stores the selected, complete Unicode text in an EcoString, including
variation selectors and multi-character strings accepted by the content element
constructor. It does not store a symbol namespace, modifier table, variant name
or the raw Value::Symbol object. Equal selected text has equal native Symbol
morphology even when reached through different raw symbol values. It remains
distinct from Text with identical text. The raw symbol value type and its
variant/modifier operations retain their existing contract.

This owner declares data only. Existing exclusive owners must separately specify
and implement insertion, fields, native function identity/callability, selectors,
language equality, representation, traversal, realization, layout and export.
Required observable integration is a native symbol content element with sole
public field `text`, repr `[<text>]`, and a callable element function that is
**distinct from** the global raw-symbol value constructor. Its function can
construct symbol content from a string such as `"xy"` and can form a field
selector. Symbol content does not acquire raw symbol `.variants()` methods.
No new public struct, trait, function signature or ElementKind variant is
proposed. Do not implement native identity through Dynamic, styling markers,
Unicode-based inference, or a JSON-only alias.

Preserve all other Content variants, Source/Located/Realized transport, immutable
occurrence semantics, style and effect ordering. Preserve complete textual output
and math metrics. This declaration authorizes no eval/layout phase move: exact
existing realization boundaries must be measured and specified by their owners.
In particular, symbol identity before realization does not imply immunity from
textual show rules: pinned upstream `typst-realize/src/lib.rs:317-345,1040`
handles regex in math and converts symbols to textual content outside math.
The numeric MathText field mismatch is separate evidence, not authorization to
reclassify all MathText as Symbol. No changes to the 36 extension classifications
or to the oracle pin are authorized.

### Acceptance after approval

Update each affected exclusive L0 before its productive consumer, share any
cross-owner declarative obligations only through pinned Tekt Nuclei, and validate
V15/V26 before resealing. Add meaningful failing Rust tests before implementation
and retain RED evidence, then GREEN evidence. Recheck the 16 differences, the 146
matches, the 36 extensions and reflection of markup/code-tail/math/emoji/runtime
symbols, full Unicode text, fields/has/at, function equality/callability, selector
matching, language equality and styled bodies. Measure show/regex behavior and
rendered text/geometry against the pinned oracle; preserve A–D controls.
Run the complete repository validation runner, workspace build, strict/reverse
lineage, full crystalline-lint with zero violations, and diff whitespace checks.
Record exact dirty state, input hashes, binary identities, UTC and commands for
each measurement. Until implementation and those gates, batch E remains open.

## Approved batch E integration

Measured inputs: `lab/results/symbol-identity-2026-10-23/baseline.json`,
`before/receipt.json` and `additional/receipt.json`; see the shared nucleus for
the approved native leaf contract. This owner retains its existing responsibility.
Declare the approved Symbol(EcoString) enum variant; no additional public types or signatures.
Preserve all pre-existing working-tree changes, effects and other variants.
