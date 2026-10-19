//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/content.md
//! @prompt-hash 9a99b543
//! @layer L1
//! @updated 2026-10-07
//!
//! Closed domain model for document content. This file owns the data
//! representation; behavior is organized by responsibility in child modules.

mod construction;
mod emptiness;
mod equality;
mod fields;
mod identity;
mod morphology;
mod page_structure;
mod plain_text;
mod rust_hash;
mod sequence;
mod source;
mod traversal;

use std::sync::Arc;

use ecow::EcoString;

use crate::entities::corners::Corners;
use crate::entities::counter_update::CounterUpdate as CounterAction;
use crate::entities::dir::Dir;
use crate::entities::geometry::{ShapeKind, Stroke};
#[allow(unused_imports)]
use crate::entities::layout_types::{
    Align2D, Color, Length, PlaceScope, Pt, TrackSizing, TransformMatrix,
};
use crate::entities::math_style::MathStyleKind;
#[allow(unused_imports)]
use crate::entities::paint::Paint;
use crate::entities::parity::Parity;
use crate::entities::ptr_eq_arc::PtrEqArc;
use crate::entities::rel::Rel;
use crate::entities::sides::Sides;
use crate::entities::world_types::Datetime;
// Modelo D (ADR-0105, lote piloto P316): variantes delegadas a módulos.
use crate::entities::elements::divider::DividerElem;
use crate::entities::elements::emph::EmphElem;
use crate::entities::elements::flush::FlushElem;
use crate::entities::elements::heading::HeadingElem;
use crate::entities::elements::label::LabelElem;
use crate::entities::elements::math_styled::MathStyledElem;
use crate::entities::elements::strong::StrongElem;
use crate::entities::elements::title::TitleElem;
use crate::entities::elements::DynElement;
use crate::entities::elements::Element;
// Lote 2 P317 — família math element-shaped (11 variantes).
use crate::entities::elements::math_accent::{MathAccentElem, MathAccentExplicit};
use crate::entities::elements::math_align_point::MathAlignPointElem;
use crate::entities::elements::math_attach::{
    MathAttachElem, MathAttachSlot, MathPrimesElem,
};
use crate::entities::elements::math_cancel::{
    MathCancelAngle, MathCancelElem, MathCancelExplicit,
};
use crate::entities::elements::math_cases::{MathCasesElem, MathCasesExplicit};
use crate::entities::elements::math_class_override::MathClassOverrideElem;
use crate::entities::elements::math_delimited::{
    MathDelimitedElem, MathLrElem, MathMidElem,
};
use crate::entities::elements::math_frac::MathFracElem;
use crate::entities::elements::math_limits_override::MathLimitsOverrideElem;
use crate::entities::elements::math_matrix::{
    MathMatrixAugment, MathMatrixElem, MathMatrixExplicit,
};
use crate::entities::elements::math_op::MathOpElem;
use crate::entities::elements::math_root::MathRootElem;
use crate::entities::elements::math_stretch::MathStretchElem;
use crate::entities::elements::math_underline::{MathOverlineElem, MathUnderlineElem};
use crate::entities::elements::math_underover::{
    MathOverbraceElem, MathOverbracketElem, MathOverparenElem, MathOvershellElem,
    MathUnderbraceElem, MathUnderbracketElem, MathUnderoverElem, MathUnderparenElem,
    MathUndershellElem,
};
use crate::entities::elements::math_vec::{MathVecElem, MathVecExplicit};
// Lote 3 P318 — família lista/termos (5 variantes).
use crate::entities::elements::enum_item::EnumItemElem;
use crate::entities::elements::link::LinkElem;
use crate::entities::elements::list_item::ListItemElem;
use crate::entities::elements::term_item::TermItemElem;
use crate::entities::elements::terms::TermsElem;
// Lote 4 P319 — decorações de texto (3 variantes).
use crate::entities::elements::overline::OverlineElem;
use crate::entities::elements::strike::StrikeElem;
use crate::entities::elements::underline::UnderlineElem;
// Lote 5 P320 — quebras/espaços + grid/table header/footer (9 variantes).
use crate::entities::elements::colbreak::ColbreakElem;
use crate::entities::elements::grid_footer::GridFooterElem;
use crate::entities::elements::grid_header::GridHeaderElem;
use crate::entities::elements::h_space::HSpaceElem;
use crate::entities::elements::linebreak::LinebreakElem;
use crate::entities::elements::page_run::PageRunElem;
use crate::entities::elements::pagebreak::PagebreakElem;
use crate::entities::elements::table_footer::TableFooterElem;
use crate::entities::elements::table_header::TableHeaderElem;
use crate::entities::elements::v_space::VSpaceElem;
// Lote 6 P321 — família state/counter + Metadata (7 variantes).
use crate::entities::elements::context_block::ContextBlockElem;
use crate::entities::elements::counter_display::CounterDisplayElem;
use crate::entities::elements::counter_display_callback::CounterDisplayCallbackElem;
use crate::entities::elements::counter_update::CounterUpdateElem;
use crate::entities::elements::metadata::MetadataElem;
use crate::entities::elements::state::StateElem;
use crate::entities::elements::state_display::StateDisplayElem;
use crate::entities::elements::state_update::StateUpdateElem;
// Lote 7 P322 — por largura (5 variantes).
use crate::entities::elements::align::AlignElem;
use crate::entities::elements::bibliography::BibliographyElem;
use crate::entities::elements::block::BlockElem;
use crate::entities::elements::boxed::BoxedElem;
use crate::entities::elements::cite::CiteElem;
use crate::entities::elements::columns::ColumnsElem;
use crate::entities::elements::curve::CurveElem;
use crate::entities::elements::equation::EquationElem;
use crate::entities::elements::figure::FigureElem;
// P1473 — `figure.caption`/`footnote.entry`, escopo reduzido (owners 1:1).
use crate::entities::elements::figure_caption::FigureCaptionElem;
use crate::entities::elements::footnote::FootnoteElem;
use crate::entities::elements::footnote_entry::FootnoteEntryElem;
use crate::entities::elements::grid::GridElem;
use crate::entities::elements::grid_cell::GridCellElem;
use crate::entities::elements::grid_hline::GridHLineElem;
use crate::entities::elements::grid_vline::GridVLineElem;
use crate::entities::elements::hide::HideElem;
use crate::entities::elements::image::ImageElem;
use crate::entities::elements::outline::OutlineElem;
use crate::entities::elements::outline_entry::OutlineEntryElem;
use crate::entities::elements::pad::PadElem;
use crate::entities::elements::pdf_artifact::PdfArtifactElem;
use crate::entities::elements::pdf_attach::PdfAttachElem;
use crate::entities::elements::place::PlaceElem;
use crate::entities::elements::quote::QuoteElem;
use crate::entities::elements::r#ref::RefElem;
use crate::entities::elements::raw::RawElem;
use crate::entities::elements::repeat::RepeatElem;
use crate::entities::elements::shape::ShapeElem;
use crate::entities::elements::smartquote::SmartQuoteElem;
use crate::entities::elements::stack::StackElem;
use crate::entities::elements::table::TableElem;
use crate::entities::elements::table_cell::TableCellElem;
use crate::entities::elements::table_hline::TableHLineElem;
use crate::entities::elements::table_vline::TableVLineElem;
use crate::entities::elements::transform::TransformElem;

/// Conteúdo declarativo produzido por `eval()`.
///
/// Diverge intencionalmente do original (`typst-library/foundations/content/`),
/// que usa vtable (`unsafe trait NativeElement`), proc macros e Arc manual.
/// Replicar essa metaprogramação em L1 seria arquitecturalmente inferior.
/// Enum linear com variantes declarativas — mais simples e testável.
///
/// **Invariante L1**: não desenha, não mede, não renderiza.
/// Qualquer operação que precise de métricas de fonte ou I/O pertence a L3.
///
/// `PartialEq` implementado manualmente — `Arc<[Content]>` compara por ponteiro
/// com `derive`, não por conteúdo (ADR-0026 revisão).
/// Immutable payload for the progressive `set page` marker.
///
/// Kept behind `Arc` so this rare, wide payload does not determine the size
/// of every `Content` value (P1505 stage B).
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

/// Immutable body and style delta carried by `Content::Styled`.
///
/// The payload is shared so the 328-byte `Styles` value does not determine
/// the size of every `Content` value (P1505 stage C).
#[derive(Debug, Clone, PartialEq)]
pub struct StyledContent {
    pub body: Content,
    pub styles: crate::entities::style::Styles,
}

/// Captured lexical environment for deferred show application.
#[derive(Debug, Clone)]
pub struct ShowRecipeContent {
    pub body: Content,
    pub textual: bool,
    pub context_id_floor: u64,
    pub rules: Arc<[crate::entities::show::ShowRule]>,
    pub styles: crate::entities::style_chain::StyleChain,
    pub guards: Vec<crate::entities::show::RuleId>,
    pub file: crate::entities::file_id::FileId,
}

/// A semantic occurrence and the content emitted by its transformation.
#[derive(Debug, Clone)]
pub struct RealizedContent {
    pub original: Content,
    pub visual: Content,
}

/// Identity of an occurrence, independent of its visual replacement.
#[derive(Debug, Clone)]
pub struct LocatedContent {
    pub location: crate::entities::location::Location,
    pub body: Content,
}

#[derive(Debug, Clone)]
pub struct SourceContent {
    pub body: Content,
    pub span: crate::entities::span::Span,
}

#[derive(Clone)]
pub enum Content {
    Source(Arc<SourceContent>),
    ShowRecipe(Arc<ShowRecipeContent>),
    Realized(Arc<RealizedContent>),
    Located(Arc<LocatedContent>),
    /// Conteúdo vazio.
    Empty,
    /// Texto simples. **F-5b fatia 2 (P372/P373, §3a.13/§3a.14)**: perdeu o
    /// `TextStyle` assado — o render do `#set text`/`#set par` vive **só na chain**
    /// (canal `custom` `"text.<campo>"`, transparente à morfologia; transporte
    /// aninhado P373). O layout resolve o render da chain. Fonte única do render.
    Text(EcoString),
    /// Espaço entre elementos (SpaceElem).
    Space,
    /// Espaço de markup inferido por newline; preserva a origem até a realização.
    SpaceFromNewline,
    /// Quebra de parágrafo semântica (linha em branco no markup).
    /// **P622**: distinta de `Space` e de `Linebreak` (`\\` explícito).
    Parbreak,
    /// Sequência de elementos — clone O(1) via Arc (ADR-0026 revisão).
    Sequence(Arc<[Content]>),

    /// Nó HTML explícito produzido por `html.elem` sob a feature HTML.
    HtmlElem(Arc<crate::entities::html::HtmlElem>),
    /// Ilha de layout paginado embutida no target HTML.
    HtmlFrame(Arc<crate::entities::html::HtmlFrameElem>),

    /// Parágrafo — contentor sintético que agrupa o conteúdo entre
    /// `Content::Parbreak`s durante a paragraph realization (P863).
    Par {
        body: Box<Content>,
    },

    // ── Rich text (Passo 22, consolidado no Passo 101) ───────────────────
    // `Content::Strong` e `Content::Emph` removidos no Passo 101
    // (ADR-0038/0039): `*bold*` e `_italic_` passam a emitir
    // `Content::Styled(body, Styles::from_iter([Style::bold(true) | Italic(true)]))`.
    // Os construtores `Content::strong(body)` e `Content::emph(body)` foram
    // redefinidos para preservar a API pública.
    /// Cabeçalho com nível 1–6 (`= Heading`).
    ///
    /// **Modelo D (ADR-0105, P316)**: lógica delegada a
    /// `entities::elements::heading::HeadingElem` (locatável).
    Heading(Arc<HeadingElem>),
    /// Título do documento (`#title()`).
    ///
    /// **P765a**: paridade com vanilla CLI 0.15.0; renderiza o corpo em
    /// tamanho aumentado e negrito.
    Title(Arc<TitleElem>),
    /// **`strong` (`*bold*`) — F-5b fatia 1 (P371, §3a.12)**: variante própria
    /// (modelo D), distinta de `Styled` (era `Styled([Bold])` antes do retorno ao
    /// modelo de variantes que a 0026 `:63` prescreve). Dá a fidelidade ADR-0107
    /// (`*bold* ≠ #set text(bold)`). Render (bold) replicado no layout.
    Strong(Arc<StrongElem>),
    /// **`emph` (`_italic_`) — F-5b fatia 1 (P371, §3a.12)**: par simétrico do
    /// `strong` (variante própria, modelo D). Era `Styled([Italic])`.
    Emph(Arc<EmphElem>),

    // ── Passo 23 ────────────────────────────────────────────────────────────
    /// Código raw inline ou em bloco (`` `...` `` ou ```` ``` ... ``` ````).
    /// **Modelo D (Lote 7 P322)**: `entities::elements::raw::RawElem`.
    Raw(Arc<RawElem>),
    /// Item de lista não ordenada (`- ...`).
    /// **Modelo D (Lote 3 P318)**: `entities::elements::list_item::ListItemElem`.
    ListItem(Arc<ListItemElem>),
    /// Item de lista ordenada (`+ ...` ou `1. ...`).
    /// **Modelo D (Lote 3 P318)**: `entities::elements::enum_item::EnumItemElem`.
    EnumItem(Arc<EnumItemElem>),
    /// Hiperligação (`https://...`).
    /// **Modelo D (Lote 3 P318)**: `entities::elements::link::LinkElem`.
    Link(Arc<LinkElem>),

    // ── Matemática (Passo 34) ────────────────────────────────────────────────
    /// Equação matemática (`$...$` inline, `$ ... $` block).
    /// `block: true` → equação em linha própria (display mode).
    /// O motor de equações (Passo 36+) processa `body`.
    /// **Modelo D (Lote 10 P325)**: `entities::elements::equation::EquationElem`
    /// (locatável P186B; contentor assimétrico — map_content recursa,
    /// map_text terminal).
    Equation(Arc<EquationElem>),

    /// Sequência de nós matemáticos — corpo interno de uma equação.
    MathSequence(Arc<[Content]>),

    /// Identificador matemático: variável, função, símbolo (`x`, `sin`, `alpha`).
    MathIdent(EcoString),

    /// Texto literal em modo matemático (`"texto"` dentro de `$...$`).
    MathText(EcoString),

    /// Fracção matemática (`a/b` ou `frac(a, b)`).
    /// **Modelo D (Lote 2 P317)**: `entities::elements::math_frac::MathFracElem`.
    MathFrac(Arc<MathFracElem>),

    /// Base com índice e/ou expoente (`x_1^2`, `{}^{14}_6 C`).
    /// `tl`/`bl` = pre-scripts à esquerda (Passo 46).
    /// `sub`/`sup` = scripts à direita.
    /// **Modelo D (Lote 2 P317)**: `entities::elements::math_attach::MathAttachElem`.
    MathAttach(Arc<MathAttachElem>),

    /// Grupo semântico de primes (`math.primes` e apóstrofos matemáticos).
    MathPrimes(Arc<MathPrimesElem>),

    /// Raiz matemática (`√x`, `∛x`, `∜x`).
    /// `index`: None = raiz quadrada, Some(n) = raiz n-ésima.
    /// **Modelo D (Lote 2 P317)**: `entities::elements::math_root::MathRootElem`.
    MathRoot(Arc<MathRootElem>),

    /// Glifo matemático explicitamente extensível.
    MathStretch(Arc<MathStretchElem>),

    /// Expressão entre delimitadores (`(...)`, `[...]`, `{...}`).
    /// `open`/`close` são os caracteres delimitadores.
    /// Mantida como variante própria para que o layout possa
    /// seleccionar variantes de tamanho (Passo 42).
    /// **Modelo D (Lote 2 P317)**: `entities::elements::math_delimited::MathDelimitedElem`.
    MathDelimited(Arc<MathDelimitedElem>),
    /// Wrapper público de escalonamento left/right.
    MathLr(Arc<MathLrElem>),
    /// Delimitador intermediário escalado pelo `MathLr` mais próximo.
    MathMid(Arc<MathMidElem>),

    /// Chave horizontal superior pública com anotação opcional.
    MathOverbrace(Arc<MathOverbraceElem>),

    /// Chave horizontal inferior pública com anotação opcional.
    MathUnderbrace(Arc<MathUnderbraceElem>),

    /// Barra horizontal inferior pública com anotação opcional.
    MathUnderbracket(Arc<MathUnderbracketElem>),

    /// Parêntesis horizontal inferior público com anotação opcional.
    MathUnderparen(Arc<MathUnderparenElem>),

    /// Casco horizontal inferior público com anotação opcional.
    MathUndershell(Arc<MathUndershellElem>),

    /// Barra horizontal superior pública com anotação opcional.
    MathOverbracket(Arc<MathOverbracketElem>),

    /// Parêntesis horizontal superior público com anotação opcional.
    MathOverparen(Arc<MathOverparenElem>),

    /// Casco horizontal superior público com anotação opcional.
    MathOvershell(Arc<MathOvershellElem>),

    /// Ponto de alinhamento em equações matemáticas (`&`).
    /// Separa colunas no layout de grelha (Passo 51).
    /// **Modelo D (Lote 2 P317)**: `entities::elements::math_align_point::MathAlignPointElem`.
    MathAlignPoint(Arc<MathAlignPointElem>),

    /// Quebra de linha em contexto matemático (`\\`).
    /// Separa linhas no layout de grelha (Passo 51).
    /// **Modelo D (Lote 5 P320)**: `entities::elements::linebreak::LinebreakElem`.
    Linebreak(Arc<LinebreakElem>),

    /// Matriz matemática produzida pela função `mat(...)`.
    /// `rows`: lista de linhas, cada linha é uma lista de células.
    /// `delim`: par de delimitadores (`('(', ')')` por defeito).
    /// **Modelo D (Lote 2 P317)**: `entities::elements::math_matrix::MathMatrixElem`.
    MathMatrix(Arc<MathMatrixElem>),

    /// Função definida por ramos, produzida pela função `cases(...)`.
    /// `rows`: lista de ramos; cada ramo é um array de células (separadas por `&`).
    /// Delimitador esquerdo `{`; sem delimitador direito.
    /// **Modelo D (Lote 2 P317)**: `entities::elements::math_cases::MathCasesElem`.
    MathCases(Arc<MathCasesElem>),

    // ── Passo 296 — Math accent + cancel (P-math-accent-cancel) ─────────
    /// Acento matemático — vanilla `AccentElem`.
    ///
    /// **P296 (HIV + (a) minimal)**: variant minimal com `base` e
    /// `accent` apenas. Layouter posiciona `accent` centrado
    /// horizontalmente acima da `base`. Cosméticos vanilla (`size`,
    /// `dotless`) **scope-out** per ADR-0054 graded.
    ///
    /// **A.0.0 N=4 refuta classificação Tabela A.4 linha 118**:
    /// `accent` estava marcado `parcial` mas zero hits no L1 pré-P296;
    /// classificação corrigida `ausente` → `implementado`.
    /// **Modelo D (Lote 2 P317)**: `entities::elements::math_accent::MathAccentElem`.
    MathAccent(Arc<MathAccentElem>),

    /// Linha de cancelamento sobre conteúdo matemático — vanilla
    /// `CancelElem`.
    ///
    /// **P296 (HIV + (a) minimal)**: variant minimal com `body`
    /// apenas. Layouter emite `FrameItem::Line` diagonal sobre o
    /// bbox do `body`. Cosméticos vanilla (`length`, `inverted`,
    /// `cross`, `angle`, `stroke`) **scope-out** per ADR-0054 graded.
    /// `inverted`/`cross` toggles funcionais podem materializar em
    /// passo dedicado P296.X futuro.
    ///
    /// **A.0.0 N=4 refuta classificação Tabela A.4 linha 119**:
    /// `cancel` estava marcado `parcial` mas zero hits no L1 pré-P296;
    /// classificação corrigida `ausente` → `implementado`.
    /// **Modelo D (Lote 2 P317)**: `entities::elements::math_cancel::MathCancelElem`.
    MathCancel(Arc<MathCancelElem>),

    /// Underline matemático body-only, distinto da decoração textual.
    MathUnderline(Arc<MathUnderlineElem>),

    /// Overline matemático body-only, distinto da decoração textual.
    MathOverline(Arc<MathOverlineElem>),

    /// Vetor matemático variádico, identidade distinta de matriz.
    MathVec(Arc<MathVecElem>),

    // ── P772y — `MathClassOverride` (`math.class(class, body)`) ─────────
    /// Força a `MathClass` de um símbolo/expressão — override do valor
    /// inferido automaticamente por `default_math_class`/`node_math_class`.
    /// Vanilla `ClassElem` (`math/mod.rs`). Afecta apenas espaçamento
    /// automático (`rules/math/layout/spacing.rs`); `body` é layoutado
    /// normalmente.
    MathClassOverride(Arc<MathClassOverrideElem>),

    // ── Passo 992 — `MathLimitsOverride` (`limits()`/`scripts()`) ────────
    /// Força o discriminador `is_limits` do `MathAttach` pai (empilhado vs
    /// lateral) — `limits(body, inline:)` (`limits: true`) ou
    /// `scripts(body)` (`limits: false`). Vanilla: dois elementos
    /// separados, `LimitsElem`/`ScriptsElem` (`math/attach.rs`),
    /// consolidados aqui num só (ADR-0107). Não afecta `MathClass`/
    /// layout do `body`.
    MathLimitsOverride(Arc<MathLimitsOverrideElem>),

    // ── Passo 297 — `MathUnderover` (P296.1) ─────────────────────────────
    /// Anotações verticais sobre/sob conteúdo matemático — agregação
    /// cristalina do cluster vanilla `UnderlineElem`/`OverlineElem`/
    /// `UnderbraceElem`/`OverbraceElem`/`UnderbracketElem`/etc.
    ///
    /// **P297 (HV'.a + (b) Option fields)**: variant agregado per
    /// ADR-0054 graded. Vanilla typst fragmenta em 12 elementos
    /// separados (cada com `body` + `annotation: Option<Content>`);
    /// cristalino agrega num único variant com `base` + `under: Option`
    /// \+ `over: Option`. Layouter empilha verticalmente conforme
    /// presença de cada.
    ///
    /// **A.0.0 N=5 refuta spec P297** que assumia `UnderoverElem`
    /// vanilla unificado — vanilla NÃO tem este wrapper. Magnitude
    /// alta (paralelo P294); refuta categoricamente hipótese
    /// degenerescência §6.6 P295.
    ///
    /// **"Variant rico" N=5 candidato genuíno** — primeira
    /// qualificação Option `Box<Content>` estrutural (não cosmético)
    /// desde P287 refutação. Promoção adiada per P273.17 §0 (uma
    /// ADR meta por passo).
    /// **Modelo D (Lote 2 P317)**: `entities::elements::math_underover::MathUnderoverElem`.
    MathUnderover(Arc<MathUnderoverElem>),

    // ── Passo 298 — `MathOp` (P296.2 fecho cluster math 4/4) ─────────────
    /// Operador textual matemático — vanilla `OpElem`. Paradigma
    /// cross-variant: `limits` afecta layout de `MathAttach` pai.
    ///
    /// **P298 (HV'' adaptado)**: cristalino já tinha heurística
    /// limits-style hardcoded em `attach.rs` via
    /// `symbols::is_limit_function`/`is_large_operator`. P298 estende
    /// para suportar `MathOp { limits: true }` user-customizable
    /// (paridade vanilla `op("lim", limits: true)`).
    ///
    /// **A.0.0 N=6 magnitude alta**: refutação significativa spec
    /// — heurística limits-style já existia parcialmente; P298
    /// estende sem substituir (fallback `MathIdent("lim")` preservado).
    ///
    /// **`bool limits` discriminador estrutural ambíguo**: caso
    /// intermédio para "variant rico" N=5 (P297 estabeleceu como
    /// Option estrutural; `bool` não qualifica). Promoção adiada.
    /// **Modelo D (Lote 2 P317)**: `entities::elements::math_op::MathOpElem`.
    MathOp(Arc<MathOpElem>),

    // ── Passo 311b.2 — Math style wrapper (Caminho I per P311a) ─────────
    /// Wrapper de variant glyph / flags math style — vanilla
    /// `bb`/`bold`/`cal`/`frak`/`italic`/`mono`/`sans`/`scr`/`script`/
    /// `serif`/`sscript`/`upright`.
    ///
    /// Todos os campos override são `Option<_>`: `None` = inherit
    /// (preserva valor activo no walker); `Some(_)` = override (outer
    /// força este valor, inner não sobrescreve se já set).
    ///
    /// - `kind`: variant glyph (DoubleStruck/Chancery/etc.) ou size
    ///   (Script/SScript com factor multiplicativo). `bb`/`cal`/etc.
    ///   usam `Some(_)`; `bold`/`italic`/`upright` usam `None`.
    /// - `bold`/`italic`: flags ortogonais (e.g. Sans Bold Italic).
    /// - `cramped`: propaga estado de script-cramping para
    ///   `MathLayouter`.
    ///
    /// Composição (per diagnóstico P311a §3.3): outer-wins para
    /// kind/italic; bold é ortogonal (bitwise OR ao descer);
    /// cramped propaga; size compõe multiplicativamente.
    ///
    /// **Modelo D (ADR-0105, P316)**: lógica delegada a
    /// `entities::elements::math_styled::MathStyledElem`.
    MathStyled(Arc<MathStyledElem>),

    /// Destino nomeado para referências cruzadas (P460 / P464).
    /// A `Label` é metainformação posicional — não tem presença visual.
    /// Produzida por `#label("sec1", body)` (user-created, `auto: false`) ou por
    /// sintaxe `<label>` em headings/figures/equations (auto-generated, `auto: true`).
    /// O body é renderizado normalmente e o nome é registado como destino no PDF
    /// (`/Dests`).
    /// **Modelo D (P464)**: `entities::elements::label::LabelElem`
    /// (wrapper de label; recurse body; não-locatável no trait).
    Label(Arc<LabelElem>),

    /// Referência cruzada (Passo 56).
    /// Enquanto não existe motor de introspecção, renderiza literalmente `@nome`.
    /// **Modelo D (Lote 8 P323)**: `entities::elements::r#ref::RefElem`
    /// (não-locatável, leaf).
    Ref(Arc<RefElem>),

    // ── Introspecção / Contadores (Passo 57) ────────────────────────────────

    // Lote F-2 S5 (P335): SetHeadingNumbering/SetEquationNumbering removidos
    // — numeração migrada para a chain léxica (assada nos elementos).
    /// Valor actual de um contador no ponto de inserção.
    /// Produzida por `counter(heading).get()` / `counter(heading).display()`.
    /// O Layouter resolve o valor no momento do layout (single-pass).
    /// DEBT-10: single-pass não suporta referências para a frente.
    /// **Modelo D (Lote 6 P321)**: `entities::elements::counter_display::CounterDisplayElem`
    /// (não-locatável, legacy single-pass).
    CounterDisplay(Arc<CounterDisplayElem>),

    /// Instrução de modificação de um contador (Passo 58).
    /// **Modelo D (Lote 6 P321)**: `entities::elements::counter_update::CounterUpdateElem`
    /// (locatável).
    CounterUpdate(Arc<CounterUpdateElem>),

    /// Marcador para a Tabela de Conteúdos (Passo 61).
    /// O layouter substitui este nó pela lista de títulos do documento.
    /// **Modelo D (Lote 8 P323)**: `entities::elements::outline::OutlineElem`
    /// (locatável; unit struct — campos do vanilla pendentes de cobertura).
    Outline(Arc<OutlineElem>),

    /// **P1402** — entrada pública realizada por `outline()`.
    OutlineEntry(Arc<OutlineEntryElem>),

    /// Elemento com numeração própria e legenda opcional (Passo 62, DEBT-15 Passo 75).
    /// `kind` discrimina o contador: "image", "table", "raw", etc.
    /// `numbering` baked-in em eval via `#set figure(numbering: "1")` (DEBT-14).
    /// **Modelo D (Lote 13 P328)**: `entities::elements::figure::FigureElem`
    /// (locatável M1; contentor — recurse body + caption; kind/numbering
    /// `Option<String>` — `kind` None↔Auto, default "image" resolvido em uso).
    Figure(Arc<FigureElem>),

    /// **P1473** — `figure.caption(body, position?, separator?)` qualificado,
    /// escopo reduzido (construção sem síntese). Owner:
    /// `entities::elements::figure_caption::FigureCaptionElem`. Não
    /// participa da síntese (`kind`/`supplement`/`numbering`/`counter`) nem
    /// do layout de figuras nesta fatia.
    FigureCaption(Arc<FigureCaptionElem>),

    // Lote F-2 S5 (P335): SetFigureNumbering removido — assado em FigureElem.numbering.
    /// Imagem carregada do disco (Passo 71, DEBT-24).
    ///
    /// `data: PtrEqArc<Vec<u8>>` — clones partilham a mesma alocação (O(1) clone)
    /// e PartialEq compara por ponteiro em vez de por valor (DEBT-26).
    /// `width`/`height` usam `Box<Value>` para quebrar o ciclo de tipos
    /// `Content → Value → Content` (sem Box seria recursão infinita).
    /// **Modelo D (Lote 7 P322)**: `entities::elements::image::ImageElem`.
    Image(Arc<ImageElem>),

    /// Forma geométrica primitiva (Passo 76).
    ///
    /// `width`/`height`: dimensões opcionais no AST — o layouter resolve os valores
    /// finais e emite `FrameItem::Shape` com `f64` concretos.
    /// `fill`/`stroke` resolvidos na stdlib — nunca por resolver no layouter.
    /// **Modelo D (Lote 11 P326)**: `entities::elements::shape::ShapeElem`
    /// (não-locatável, leaf — geometria pura).
    Shape(Arc<ShapeElem>),

    /// Path de curva composto por segmentos (Passo 513).
    ///
    /// `CurveElem` agrupa `Move`/`Line`/`Cubic`/`Quad`/`Close`. O layouter
    /// converte para `FrameItem::Shape { kind: Path(...) }`.
    /// **Modelo D**: `entities::elements::curve::CurveElem`.
    Curve(Arc<CurveElem>),

    /// Aplica uma transformação afim ao conteúdo interno (Passo 78).
    ///
    /// O layouter calcula a AABB do conteúdo transformado e reserva o espaço
    /// correcto na página. O exportador emite q → cm → conteúdo → Q.
    /// **Modelo D (Lote 9 P324)**: `entities::elements::transform::TransformElem`
    /// (não-locatável, contentor — recurse body).
    Transform(Arc<TransformElem>),

    /// Grid de colunas com células posicionadas por ordem de leitura (Passo 80).
    ///
    /// `rows` é consumido pelo layouter desde o Passo 83 (DEBT-34b encerrado).
    /// Comentário obsoleto removido na auditoria do Passo 105.
    ///
    /// **Refino P224** (Fase 4 Layout candidata sub-passo 3 — fecha série
    /// α "terminar Layout"): 5 fields aditivos cumulativos:
    /// - `gutter: Option<Length>` (P224.A) — espaço uniforme entre cells
    ///   (default `None` == zero).
    /// - `align: Option<Align2D>` (P224.A) — alignment uniforme (default
    ///   `None` == top-left).
    /// - `inset: Sides<Length>` (P224.A) — margem interna em cada cell
    ///   (default zero; paridade P156G+H+I).
    /// - `header: Option<Box<Content>>` (P224.B) — header opcional.
    /// - `footer: Option<Box<Content>>` (P224.B) — footer opcional.
    ///
    /// Atributos vanilla `stroke`/`fill` cosméticos scope-out per ADR-0054
    /// graded (Fase 5 candidata NÃO-reservada per política P158).
    /// Per-cell `align`/`inset`/`fill`/`stroke`/`breakable` em `GridCell`
    /// scope-out (subset paridade P157B literal). Placement algorítmico
    /// completo via `Content::GridCell` + `grid_placement.rs` (P224.C
    /// fecha DEBT-34e).
    /// **Modelo D (Lote 12 P327)**: `entities::elements::grid::GridElem`
    /// (não-locatável, contentor — recurse cells + header + footer; 10 campos).
    Grid(Arc<GridElem>),

    // ── Passo 224.B (ADR-0061 Fase 4 candidata sub-3) — Grid header/footer ──
    /// Grid header — vanilla `GridHeader` (paridade P157C TableHeader
    /// literal). Layouter renderiza `body` no contexto Grid; **`repeat`
    /// armazenado mas semantic adiada** per ADR-0054 graded (paridade
    /// pattern N=5 cumulativo weak/breakable/float/repeat).
    /// **Modelo D (Lote 5 P320)**: `entities::elements::grid_header::GridHeaderElem`.
    GridHeader(Arc<GridHeaderElem>),

    /// Grid footer — vanilla `GridFooter` (paridade P157C TableFooter
    /// literal). Par simétrico com `GridHeader` (mesmos fields).
    /// **Modelo D (Lote 5 P320)**: `entities::elements::grid_footer::GridFooterElem`.
    GridFooter(Arc<GridFooterElem>),

    // ── Passo 224.C (ADR-0061 Fase 4 candidata sub-3) — Grid cell + placement ──
    /// Grid cell estruturado — vanilla `GridCell` (paridade P157B
    /// TableCell literal; 5 fields). Placement algorítmico real
    /// resolvido via `compiler/layout/grid_placement.rs` (P224.C fecha
    /// DEBT-34e colspan/rowspan).
    ///
    /// `x`/`y`: posição explícita opcional (auto-placement se `None`).
    /// `colspan`/`rowspan`: ocupação adjacente opcional (default 1).
    ///
    /// `align`/`inset`/`breakable` per-cell vanilla scope-out
    /// (subset paridade P157B literal; refino candidato Categoria
    /// B.3 Fase 5 NÃO-reservado).
    ///
    /// **Refino P230** (Fase 5 Layout Categoria A.3): +2 fields
    /// cosméticos per-cell `stroke` + `fill` (precedência override
    /// Grid-level via `.or()` resolution). Per ADR-0079 PROPOSTO
    /// Categoria A.3 + **ADR-0080 EM VIGOR aplicação automática**
    /// (L0 não tocado por defeito; pattern "L0 minimal" formalizado
    /// P229).
    /// **Modelo D (Lote 12 P327)**: `entities::elements::grid_cell::GridCellElem`
    /// (não-locatável, contentor — recurse body; 9 cosméticos P230/P235,
    /// gémeo de `TableCell`).
    GridCell(Arc<GridCellElem>),

    /// Altera a configuração da página a partir deste ponto do documento (Passo 81).
    ///
    /// Se existir conteúdo na página actual, força uma quebra de página antes
    /// de aplicar a nova configuração. Se a página actual estiver vazia, aplica
    /// directamente sem quebra.
    SetPage(Arc<SetPageDelta>),

    /// Configuração lexical de página aplicada somente ao body (P1140.19).
    PageRun(Arc<PageRunElem>),

    /// Altera a posição do conteúdo dentro do espaço disponível no fluxo (Passo 82).
    /// O cursor avança após o bloco — o espaço é consumido normalmente.
    /// **Modelo D (Lote 7 P322)**: `entities::elements::align::AlignElem`.
    Align(Arc<AlignElem>),

    /// Posiciona o conteúdo de forma absoluta na página sem consumir espaço (Passo 82).
    /// O cursor não avança. Usado para cabeçalhos, rodapés e marcas de água.
    ///
    /// `scope` (Passo 84.6, encerra DEBT-37): `PlaceScope::Column` (default)
    /// ancora ao contentor activo (célula de Grid se houver, página caso
    /// contrário); `PlaceScope::Parent` ancora à página independentemente.
    ///
    /// **Refino P223** (Fase 4 Layout candidata sub-passo 2): 2 fields
    /// adicionados — `float: bool` (default `false`) e `clearance:
    /// Option<Length>` (default `None`). Ambos armazenados mas semantic
    /// real adiada per ADR-0054 graded (pattern N=4 cumulativo
    /// `weak`/`breakable`/`float`). DEBT-37 §"Divergência" fechada
    /// em P223 — `scope: Parent` agora exige `float: true` (paridade
    /// vanilla literal restaurada).
    /// **Modelo D (Lote 9 P324)**: `entities::elements::place::PlaceElem`
    /// (não-locatável, contentor — recurse body; 6 campos cosméticos:
    /// alignment/dx/dy/scope/float/clearance, P223 ADR-0054 graded).
    Place(Arc<PlaceElem>),

    /// Sentinela de fluxo que realiza os floats pendentes no ponto corrente.
    Flush(Arc<FlushElem>),

    /// Conteúdo estilizado — aplica um delta `Styles` ao corpo (Passo 99,
    /// ADR-0038).
    ///
    /// Fundação tipada para `#set`/`#show`. Ainda **não** é consumida pelo
    /// Layouter actual — é construída em testes de integração e pelo
    /// pipeline futuro quando o eval activar estilos. Acessores por
    /// `StyleChain::push_styles` garantem que o delta é aplicado na
    /// resolução.
    Styled(Arc<StyledContent>),

    // ── Estruturas de listas (Passo 154B, ADR-0060 Fase 1) ──────────────
    /// Separador horizontal estrutural (`#divider()`).
    /// **Modelo D (ADR-0105, P316)**: `entities::elements::divider::DividerElem`
    /// (singleton). Layouter emite linha horizontal.
    Divider(Arc<DividerElem>),

    /// Lista de pares termo-descrição (`#terms(...)`) — Passo 154B.
    /// Cada item é tipicamente `Content::TermItem`.
    /// **Modelo D (Lote 3 P318)**: `entities::elements::terms::TermsElem`.
    Terms(Arc<TermsElem>),

    /// Par individual termo-descrição (Passo 154B).
    /// Aparece tipicamente dentro de `Content::Terms`, mas pode também
    /// surgir standalone (e.g. show rules futuras).
    /// **Modelo D (Lote 3 P318)**: `entities::elements::term_item::TermItemElem`.
    TermItem(Arc<TermItemElem>),

    // ── Citação estrutural (Passo 155, ADR-0060 Fase 1, sub-passo 2) ────
    /// Citação estrutural com 4 atributos (vanilla `QuoteElem`).
    ///
    /// - `body`: conteúdo citado.
    /// - `attribution`: autor/fonte opcional.
    /// - `block`: `true` = parágrafo dedicado; `false` = inline.
    /// - `quotes`: `true` = aspas locale-apropriadas em torno do body.
    ///
    /// Smart-quotes resolvidas no layouter via
    /// `crate::compiler::lang::quotes::localize_quotes(lang)` consultando
    /// `text.lang` activo (per ADR-0057).
    /// **Modelo D (Lote 8 P323)**: `entities::elements::quote::QuoteElem`
    /// (não-locatável, contentor — recurse body + attribution).
    Quote(Arc<QuoteElem>),

    // ── Passo 397 — metadata do documento + placeholder de resource ───────
    /// Wrapper de metadata do documento (`#document(...)`).
    /// Não emite frames — é metadata pura; PDF Info dict continua scope-out
    /// per ADR-0054 graded.
    Document {
        path: EcoString,
        body: Box<Content>,
        format: Option<EcoString>,
        title: Option<Box<Content>>,
        author: Vec<EcoString>,
        date: Option<Datetime>,
        keywords: Vec<EcoString>,
    },
    /// Placeholder de resource externo (`#asset(...)`).
    /// Extensão cristalina (não existe no vanilla como elemento standalone).
    /// Registry real de resources continua scope-out per ADR-0054 graded.
    Asset {
        path: EcoString,
        data: Arc<[u8]>,
    },

    // ── Passo 287 (frente `P-smartquote`) — função stdlib smartquote ──────
    //
    // Variant **leaf** (não container). Representa uma chamada programática
    // a `#smartquote(double: bool)` — paralelo arquitectural ao markup
    // `"foo"`/`'bar'` (P155) que pré-resolve glyph em `eval_markup`.
    //
    // `double = true`  → próximo glyph é aspa dupla open/close lang-aware.
    // `double = false` → próximo glyph é aspa simples (always ASCII `'` em
    //                    cristalino — smart-apostrophes scope-out P155).
    //
    // Estado de alternância (open/close) **não** vive no variant — está em
    // `Layouter.smartquote_*_open` (campos pos-P287). Diagnóstico
    // `diagnostico-smartquote-passo-287.md` §A.3 opção (γ′): markup mantém
    // estado próprio em `eval_markup`; função mantém estado próprio no
    // Layouter; divergência aceite per ADR-0054 graded.
    //
    // **Não qualifica como "variant rico com cosméticos opcionais"** —
    // leaf-like com 1 campo `bool` required; padrão N=4 cumulativo
    // (P156G/H/I+P284) inalterado (diagnóstico §A.2.2 honestidade
    // epistémica).
    /// **Modelo D (Lote 9 P324)**: `entities::elements::smartquote::SmartQuoteElem`
    /// (não-locatável, leaf).
    SmartQuote(Arc<SmartQuoteElem>),

    /// Marker invisível de attachment, coletado pelo layout.
    PdfAttach(Arc<PdfAttachElem>),

    /// Conteúdo visual preservado como artifact semântico no PDF.
    PdfArtifact(Arc<PdfArtifactElem>),

    // ── Passo 284 (ADR-0054 graded) — text decoration ─────────────────────
    //
    // Três variants distintos (paridade vanilla `UnderlineElem`/`StrikeElem`/
    // `OverlineElem`). Atributos cosméticos opcionais materializados em
    // bucket 1 do diagnóstico P284 §A.1; `evade` e `background` ficam fora
    // (ADR-0054 graded); objecto `Stroke` rico adiado (Tabela A.7 linha 201
    // `stroke(...)` parcial).
    //
    // - `body`: conteúdo a decorar.
    // - `stroke`: paint da linha (default `Color::rgb(0, 0, 0)` quando `None`).
    // - `offset`: override do offset Y default (em pt no espaço Layouter);
    //   `None` = constante por kind (`+0.10/-0.25/-0.80 em` em-units).
    // - `extent`: extensão horizontal além do body (positiva ou negativa);
    //   `None` = 0pt.
    //
    // Emit reutiliza `FrameItem::Line` existente (precedente Passo 38 frac);
    // hash L0 `export.rs` `bc7b8b95` preservado (per spec §5).
    /// **Modelo D (Lote 4 P319)**: `entities::elements::underline::UnderlineElem`.
    Underline(Arc<UnderlineElem>),

    /// **Modelo D (Lote 4 P319)**: `entities::elements::strike::StrikeElem`.
    Strike(Arc<StrikeElem>),

    /// **Modelo D (Lote 4 P319)**: `entities::elements::overline::OverlineElem`.
    Overline(Arc<OverlineElem>),

    // ── Passo 408 (ADR-0054 graded) — small caps ──────────────────────────
    //
    // Paridade vanilla `SmallcapsElem`: elemento de texto que transforma
    // o body em small capitals. Consumer real requer shaping OpenType
    // (`smcp` / `c2sc`) — DEBT-53 scope-out XL. Neste passo materializa-se
    // o variant e o stdlib; o consumer em layout é stub transparente.
    //
    // Modelo minimal: só `body` (sem atributos opcionais). Não segue o
    // Modelo D para evitar um ficheiro de elemento só para um campo.
    SmallCaps {
        body: Box<Content>,
    },

    // ── Passo 156C (ADR-0061 Fase 1 sub-passo 1) — pad + hide ───────────
    // ── Passo 156L (ADR-0061 Fase 3 sub-passo 2; refino) — sides
    //    individualizadas: `padding: Sides<Length>` → `sides:
    //    Sides<Option<Length>>` per ADR-0064 Caso C (segunda
    //    aplicação concreta). `None` per lado ↔ default vanilla zero;
    //    distingue lado declarado vs lado não declarado para futura
    //    introspecção e show rules.
    /// Container que aplica padding ao body durante layout.
    ///
    /// `sides` em quatro lados (`Sides<Option<Length>>`); cada lado
    /// `None` ↔ default vanilla zero (resolvido em momento de uso no
    /// Layouter). Vanilla `PadElem` em
    /// `lab/typst-original/.../layout/pad.rs`. Atributos de stdlib
    /// (`left`/`right`/`top`/`bottom`/`x`/`y`/`rest`) resolvidos em
    /// `native_pad` com precedência específico > eixo > rest antes de
    /// chegar a este variant.
    /// **Modelo D (Lote 10 P325)**: `entities::elements::pad::PadElem`
    /// (não-locatável, contentor — recurse body).
    Pad(Arc<PadElem>),

    /// Container que calcula dimensões mas não emite items visuais.
    ///
    /// Útil para placeholders e equilíbrio. Vanilla `HideElem` em
    /// `lab/typst-original/.../layout/hide.rs`. Cristalino preserva o
    /// avanço de cursor (consistente com vanilla "layout-aware mas não
    /// rende").
    /// **Modelo D (Lote 7 P322)**: `entities::elements::hide::HideElem`.
    Hide(Arc<HideElem>),

    // ── Passo 156D (ADR-0061 Fase 1 sub-passo 2) — h + v spacing ─────────
    /// Spacing primitive horizontal (vanilla `HElem`).
    ///
    /// `amount` em `Length`; `weak` armazenado mas comportamento de
    /// collapse adiado (perfil ADR-0054 graded). Layouter avança
    /// `cursor_x` por `amount`. Vanilla aceita `Fraction`; cristalino
    /// só aceita `Length` neste passo (ADR-0061 §6.3 refino futuro).
    /// **Modelo D (Lote 5 P320)**: `entities::elements::h_space::HSpaceElem`.
    HSpace(Arc<HSpaceElem>),

    /// Spacing primitive vertical (vanilla `VElem`).
    ///
    /// Análogo a `HSpace` mas em eixo Y. Layouter força `flush_line`
    /// antes de avançar `cursor_y` (caso contrário texto na linha
    /// actual fica meio-render).
    /// **Modelo D (Lote 5 P320)**: `entities::elements::v_space::VSpaceElem`.
    VSpace(Arc<VSpaceElem>),

    // ── Passo 156E (ADR-0061 Fase 1 sub-passo 3) — pagebreak manual ──────
    /// Quebra de página manual (vanilla `PagebreakElem`).
    ///
    /// `weak` armazenado mas comportamento de collapse adiado
    /// (perfil ADR-0054 graded; consistente com P156D HSpace/VSpace).
    /// `to: Some(parity)` força a próxima página a ter paridade
    /// especificada — Layouter insere página vazia se necessário.
    /// `to: None` == Auto (sem ajuste).
    /// **Modelo D (Lote 5 P320)**: `entities::elements::pagebreak::PagebreakElem`.
    Pagebreak(Arc<PagebreakElem>),

    // ── Passo 220 (ADR-0078 PROPOSTO sub-fase b 4/4) — colbreak manual ──
    /// Quebra de coluna manual — Fase 3 Layout per ADR-0078
    /// PROPOSTO. Adicionado em P220 (DEBT-56 sub-fase b
    /// 4/4 — fecha sub-fase b estructuralmente).
    ///
    /// **Semantic graded P220**: em cristalino pós-P219
    /// (Opção B graded — sem multi-region flow real), colbreak
    /// downgrade a pagebreak literal. Paridade vanilla:
    /// vanilla também downgrade fora de columns context.
    ///
    /// Quando consumer multi-region real existir
    /// (P-Layout-Fase4 candidato), arm pode ser refinado para
    /// salto entre regions reais.
    ///
    /// `weak: bool` armazenado mas semantic de collapse adiada
    /// (paridade `Pagebreak.weak` P156E e HSpace/VSpace P156D).
    /// Sem `to: Option<Parity>` — vanilla `ColbreakElem` não tem
    /// (paridade só faz sentido em páginas).
    /// **Modelo D (Lote 5 P320)**: `entities::elements::colbreak::ColbreakElem`.
    Colbreak(Arc<ColbreakElem>),

    // ── Passo 156I (ADR-0061 Fase 2 sub-passo 3) — stack compositivo ──
    /// Container compositivo — vanilla `StackElem`. **Último sub-passo
    /// Fase 2; atinge target 72% Layout** declarado em ADR-0061.
    ///
    /// Distinção material face a Block/Boxed: **Arc<[Content]>** em vez
    /// de body único; atributos próprios `dir` (4 direcções) e
    /// `spacing` entre children.
    ///
    /// Decisão arquitectural reusada de P156G/H (variant rico) com
    /// adaptação para `Arc<[Content]>` (clone O(1) per ADR-0026
    /// revisão, consistente com `Sequence`/`MathSequence`).
    /// **Modelo D (Lote 9 P324)**: `entities::elements::stack::StackElem`
    /// (não-locatável, contentor — recurse children).
    Stack(Arc<StackElem>),

    // ── Passo 156H (ADR-0061 Fase 2 sub-passo 2) — box inline container ──
    /// Container inline — vanilla `BoxElem`.
    ///
    /// Distinção material face a `Block` (P156G): **posicionamento
    /// inline** (não força flush_line). Atributos comuns com Block:
    /// `body`, `width`, `height`, `inset`. Atributo único: `baseline`.
    ///
    /// Decisão arquitectural P156H: variant rico (Opção A) reusada do
    /// padrão estabelecido em P156G — containers ricos preferem
    /// variants explícitos quando atributos não são propriedades de
    /// texto.
    ///
    /// Atributos vanilla scope-out per ADR-0054 graded: `outset`,
    /// `fill`, `stroke`, `radius`, `clip`, `stroke-overhang`.
    ///
    /// Naming: variant Rust é `Boxed` (não `Box`) para evitar confusão
    /// com `std::boxed::Box`; stdlib expõe `#box(...)` (paridade
    /// vanilla). Construtor Rust: `Content::boxed(...)`.
    /// **Modelo D (Lote 14 P329)**: `entities::elements::boxed::BoxedElem`
    /// (não-locatável, contentor — recurse body; 9 cosméticos de caixa; o
    /// construtor `boxed` cobre 5 → construções completas via Arc-wrap, C2).
    Boxed(Arc<BoxedElem>),

    // ── Passo 156G (ADR-0061 Fase 2 sub-passo 1) — block container ───────
    /// Container block — vanilla `BlockElem`.
    ///
    /// Forma minimalista per ADR-0054 graded. Atributos Fase 1
    /// (P156G): `body`, `width`, `height`, `inset`, `breakable`.
    /// Scope-out (refino futuro): `outset`, `fill`, `stroke`, `radius`,
    /// `clip`, `spacing`, `above`/`below`, `sticky`.
    ///
    /// Decisão arquitectural P156G.2: variant rico (Opção A) em vez de
    /// `Content::Styled`. Rationale: Block é container de layout (width/
    /// height/inset/breakable) — vocabulário diferente do `Style` enum
    /// que cobre propriedades de texto (Bold/Italic/Size/Fill/HeadingLevel).
    /// Coerente com `Content::Pad` (P156C) que também tem fields explícitos
    /// para padding.
    /// **Modelo D (Lote 15 P330)**: `entities::elements::block::BlockElem`
    /// (não-locatável, contentor — recurse body; 13 cosméticos; a mais densa.
    /// `block` cobre 5/14 → construções completas via Arc-wrap, C2).
    Block(Arc<BlockElem>),

    // ── Passo 157B (ADR-0060 Fase 2 sub-passo 2) — table cell ───────────
    /// Cell estruturada de Table — vanilla `TableCell`.
    /// **Segundo sub-passo Model Fase 2**.
    ///
    /// Subset minimal per ADR-0054 graded e diagnóstico P157B §1:
    /// 5 fields críticos (body/x/y/colspan/rowspan); 6 atributos
    /// vanilla scope-out (align/stroke/fill/inset/breakable +
    /// internal fields kind/is_repeated).
    ///
    /// Decisões arquitecturais (per diagnóstico P157B):
    /// - `x`/`y`: ADR-0064 **Caso A** (`Smart<usize>` → `Option<usize>`;
    ///   None ↔ Auto auto-placement). **Primeira aplicação concreta
    ///   de Caso A em domínio Model** (P156G/H/I aplicaram-no em
    ///   Layout).
    /// - `colspan`/`rowspan`: ADR-0064 **Caso C** (`NonZeroUsize`
    ///   default 1 → `Option<usize>` com `None` ↔ default 1; zero
    ///   rejeitado em stdlib). **Primeira variação `usize` do Caso
    ///   C**; anteriores eram `Length`.
    ///
    /// Layouter renderiza `body` no contexto actual; `x`/`y`/colspan/
    /// rowspan **armazenados mas ignorados** per ADR-0054 graded —
    /// algoritmo de placement diferido em **DEBT-34e** (refactor
    /// dedicado a placement Grid completo).
    /// **Modelo D (Lote 12 P327)**: `entities::elements::table_cell::TableCellElem`
    /// (não-locatável, contentor — recurse body; 9 cosméticos P230/P235).
    TableCell(Arc<TableCellElem>),

    // ── Passo 159A (ADR-0060 Fase 2 — Bibliography + Cite par acoplado) ──
    /// Lista bibliográfica — vanilla `BibliographyElem`.
    /// **Primeiro sub-passo Bibliography + Cite Model Fase 2**
    /// (par acoplado com `Cite`).
    ///
    /// Subset minimal per ADR-0054 graded e diagnóstico P159A §1:
    /// 2 fields críticos (entries/title); 6+ fields vanilla
    /// scope-out (sources/full/style/lang/region) + acoplamento
    /// hayagriva diferido per ADR-0062 reserva sem ficheiro.
    ///
    /// **Input cristalino literal `Vec<BibEntry>`** — sem
    /// parsing externo (`.bib`/`.yaml`). Refinos futuros
    /// (hayagriva, CSL) NÃO reservados per política P158.
    ///
    /// `title: Option<Box<Content>>` per ADR-0064 **Caso A**
    /// (`Smart<Option<Content>>` vanilla → `Option<Box<Content>>`
    /// cristalino). Patamar Caso A cresce N=4 → 5 com P159A.
    ///
    /// Layouter renderiza title (se Some) seguido de lista de
    /// entries formatadas como `"[{key}] {author}. {title}
    /// ({year})."` per ADR-0033 + ADR-0054 graded — paridade
    /// vanilla observable mínima.
    /// **Modelo D (Lote 10 P325)**: `entities::elements::bibliography::BibliographyElem`
    /// (locatável P181C; contentor — recurse title).
    Bibliography(Arc<BibliographyElem>),

    /// Citação inline — vanilla `CiteElem`.
    /// Par com `Bibliography` (acoplamento semântico vanilla
    /// inseparável: cite referencia entries de bibliography).
    ///
    /// Subset minimal per ADR-0054 graded:
    /// 3 fields (key/supplement/form); 1+ field vanilla
    /// scope-out (style override CSL).
    ///
    /// `key: String` directo (paridade vanilla `Label` simplificado).
    /// `supplement: Option<Box<Content>>` per ADR-0064 Caso A
    /// (page/chapter override).
    /// `form: Option<CitationForm>` per ADR-0064 Caso A (Passo
    /// 159C; vanilla `Smart<Option<CiteForm>>` → cristalino
    /// `Option<CitationForm>`; None ↔ Normal default).
    ///
    /// Layouter renderiza per form com lookup Bibliography
    /// same-document (Passo 159C):
    /// - `Normal`/None: `[key]` placeholder.
    /// - `Prose`: `Author (Year)` quando key existe; fallback `[key]`.
    /// - `Author`/`Year`: campo correspondente; fallback `[key]`.
    ///
    /// **Sem validação cross-reference** `key ∈ Bibliography.keys`
    /// — fallback `[key]` é silencioso; ADR-0017 Introspection
    /// runtime adiada (cite cross-document ficaria como TODO).
    /// **Modelo D (Lote 9 P324)**: `entities::elements::cite::CiteElem`
    /// (locatável M1; recurse supplement). `CitationForm` ganhou `Hash`
    /// por derive (dependência do lote).
    Cite(Arc<CiteElem>),

    // ── Passo 295 — `Footnote` cluster Fase 1 (marker only) ─────────────
    /// Footnote inline — vanilla `FootnoteElem`.
    ///
    /// **Fase 1 P295 (HE marker only)**: variant minimal com `body`
    /// armazenado mas **não renderizado** no rodapé nesta fase. Layouter
    /// emite apenas marker `[N]` superscript inline onde a footnote
    /// aparece. Numeração via walker counter simples (sem
    /// Introspector/Counter machinery).
    ///
    /// Cristalino simplifications per ADR-0054 graded vs vanilla:
    /// - `numbering: Numbering` (default `"1"`) **scope-out** (cosmético;
    ///   numeração arábica default implícita). Padrão "variant rico"
    ///   N=4 cumulativo preservado inalterado.
    /// - `FootnoteBody::Reference(Label)` **scope-out** (multi-ref
    ///   footnotes — frente futura P295.X).
    ///
    /// Frentes pendentes pós-P295:
    /// - **P295.1** — nota corpo renderizada no rodapé da página
    ///   correspondente (requer 2-pass layout; magnitude L).
    /// - **P295.2** — overflow multi-página (footnote ocupa páginas
    ///   subsequentes se rodapé não chega).
    /// - **P295.X** — footnote reference via `#footnote(<label>)`
    ///   bloqueado por scope methods em stdlib.
    ///
    /// **Modelo D (Lote 11 P326)**: `entities::elements::footnote::FootnoteElem`
    /// (não-locatável P295 Fase 1; contentor — recurse body).
    Footnote(Arc<FootnoteElem>),

    /// **P1473** — `footnote.entry(note, separator?, clearance?, gap?,
    /// indent?)` qualificado, escopo reduzido (construção sem realização).
    /// Owner: `entities::elements::footnote_entry::FootnoteEntryElem`. Não
    /// implementa `.realize()`/`ShowSet`; `compiler/layout/footnote.rs` não
    /// consulta este constructor nesta fatia.
    FootnoteEntry(Arc<FootnoteEntryElem>),

    // ── Passo 157C (ADR-0060 Fase 2 sub-passo 3 — fecha table foundations) ──
    /// Header repetível de Table — vanilla `TableHeader`.
    /// **Terceiro e último sub-passo Model Fase 2**.
    ///
    /// Par simétrico com `TableFooter`. Subset minimal per
    /// ADR-0054 graded e diagnóstico P157C §1.3:
    /// 2 fields (body/repeat); fields diferidos: `level: NonZeroU32`
    /// (hierarquia Header), `repeat-rows: Smart<usize>`, children
    /// variádicos estruturados.
    ///
    /// **Divergência aceite per ADR-0033**: vanilla usa
    /// `#[variadic] children: Vec<TableItem>`; cristalino usa
    /// `body: Box<Content>` para uniformidade com containers
    /// existentes (use Sequence se múltiplos children necessários).
    ///
    /// `repeat: bool` ADR-0064 **Caso D** (`bool` directo com
    /// default `true` paridade vanilla — **primeira aplicação
    /// Caso D em domínio Model**; P156D weak / P156G breakable /
    /// P156J justify aplicaram-no em Layout).
    ///
    /// Layouter renderiza `body` no contexto actual; **`repeat`
    /// armazenado mas ignorado** per ADR-0054 graded — algoritmo
    /// de repetição em page breaks diferido em **DEBT-56**
    /// (refactor multi-region).
    /// **Modelo D (Lote 5 P320)**: `entities::elements::table_header::TableHeaderElem`.
    TableHeader(Arc<TableHeaderElem>),

    /// Footer repetível de Table — vanilla `TableFooter`.
    /// Par simétrico com `TableHeader` (paridade absoluta:
    /// mesmos fields; `level` vanilla diferido em ambos para
    /// preservar simetria cristalina).
    ///
    /// Mesma divergência `body: Box<Content>` aceite per ADR-0033.
    /// Mesma decisão `repeat: bool` ADR-0064 Caso D.
    /// Mesma limitação per ADR-0054 graded (DEBT-56).
    /// **Modelo D (Lote 5 P320)**: `entities::elements::table_footer::TableFooterElem`.
    TableFooter(Arc<TableFooterElem>),

    // ── Passo 512 — linhas em grid/table ────────────────────────────────
    /// Linha horizontal num grid.
    GridHLine(Arc<GridHLineElem>),
    /// Linha vertical num grid.
    GridVLine(Arc<GridVLineElem>),
    /// Linha horizontal numa tabela.
    TableHLine(Arc<TableHLineElem>),
    /// Linha vertical numa tabela.
    TableVLine(Arc<TableVLineElem>),

    // ── Passo 157A (ADR-0060 Fase 2 sub-passo 1) — table minimal ────────
    /// Container tabular semântico — vanilla `TableElem`.
    /// **Primeiro sub-passo Model Fase 2**.
    ///
    /// Subset minimal per ADR-0054 graded e diagnóstico P157 §3:
    /// 3 fields críticos (columns/rows/children); ~9 atributos vanilla
    /// scope-out (gutter/inset/align/fill/stroke/summary; cells
    /// estruturadas + header/footer diferidos para P157B/C).
    ///
    /// Estruturalmente análogo a `Content::Grid` mas semanticamente
    /// distinto per ADR-0060 §"Decisão 4" (Model structural exige
    /// variant dedicado; reaproveitamento de Grid vive só no
    /// algoritmo de layout, não no enum). Field `children` (não `cells`)
    /// segue nomenclatura vanilla `Vec<TableChild>`; pequena
    /// divergência intra-cristalino vs `Grid.cells` documentada
    /// em diagnóstico P157A §3.2.
    ///
    /// Layouter delega a `layout_grid` clone simples — sem
    /// modificação de `grid.rs` per diagnóstico P157A §10.
    /// **Modelo D (Lote 12 P327)**: `entities::elements::table::TableElem`
    /// (não-locatável, contentor — recurse children; columns/rows/stroke/fill).
    Table(Arc<TableElem>),

    // ── Passo 156J (ADR-0061 Fase 3 sub-passo 1) — repeat ────────────────
    /// Repetição de body para preencher espaço (vanilla `RepeatElem`).
    /// **Primeira aplicação Fase 3** declarada em ADR-0061.
    ///
    /// Caso de uso primário: TOC dot leaders `#box(width: 1fr,
    /// repeat[.])`. Em vanilla, o algoritmo de runtime calcula
    /// quantidade-para-encher dinamicamente; em cristalino, P156J
    /// implementa **paridade estrutural** (variant + stdlib + medição
    /// estática + layout single-render) per ADR-0054 graded —
    /// algoritmo dinâmico diferido para refino futuro (mesmo critério
    /// aceite em P156G/H/I para containers complexos).
    ///
    /// **Atributos** (paridade vanilla; total 3 fields):
    /// - `body`: conteúdo a repetir (obrigatório).
    /// - `gap: Option<Length>`: espaço entre cópias; `None` == zero
    ///   (padrão Smart→Option N=6 da série P156D-I).
    /// - `justify: bool`: default `true` (paridade vanilla;
    ///   distribuição de espaço residual diferida per ADR-0054).
    ///
    /// **Modelo D (Lote 7 P322)**: `entities::elements::repeat::RepeatElem`.
    Repeat(Arc<RepeatElem>),

    /// **P217 (DEBT-56 sub-fase b — Layout Fase 3)** — Multi-column
    /// container per ADR-0078 PROPOSTO.
    ///
    /// Distinção material face a `Block`/`Boxed`/`Stack`/`Repeat`:
    /// `Columns` é o **primeiro consumer estrutural** da abstracção
    /// `Region`/`Regions` (P216A+P216B). Vanilla `ColumnsElem`.
    ///
    /// **P217 layouter arm é stub transparente** — delega a
    /// `layout_content(body)` ignorando `count`/`gutter`. Consumer
    /// multi-region real em P219 (sub-fase b consumer).
    ///
    /// **Atributos** (paridade vanilla; total 3 fields):
    /// - `count`: número de colunas. `usize`. Validação `>= 1`
    ///   diferida a `native_columns` (P218); construtor Rust aceita
    ///   `count = 0` como caso degenerate.
    /// - `gutter`: espaço entre colunas (`Option<Length>`). `None`
    ///   ↔ default vanilla (~4% width) — **ADR-0064 Caso C** (cumulativo
    ///   N=cresce; precedentes P156I Stack.spacing, P156L
    ///   `Sides<Option<Length>>`).
    /// - `body`: conteúdo a fluir entre N colunas.
    ///
    /// Stdlib `native_columns` em P218 (atomização ADR-0036).
    /// Consumer multi-region em P219.
    /// **Modelo D (Lote 8 P323)**: `entities::elements::columns::ColumnsElem`
    /// (não-locatável, contentor — recurse body).
    Columns(Arc<ColumnsElem>),

    /// **P169 (M9 sub-passo 1)** — Metadata embebido para introspecção.
    /// Vanilla `metadata(value)` em `introspection/metadata.rs`.
    ///
    /// Content invisível em layout (zero-size, sem caixa); o `value`
    /// fica disponível via `Introspector::query_metadata` para querying
    /// pelo utilizador. Usado em conjunto com `Content::Label` para
    /// associar metadata a uma label específica.
    ///
    /// Walk arm em `introspect.rs` é terminal — não desce em filhos
    /// (não há) e não muta state. `extract_payload` produz
    /// `ElementPayload::Metadata { value }`. Layouter arm é no-op
    /// (zero-size).
    /// **Modelo D (Lote 6 P321)**: `entities::elements::metadata::MetadataElem`
    /// (locatável).
    Metadata(Arc<MetadataElem>),

    /// **P171 (M9 sub-passo 3)** — runtime mutable state, init.
    /// Vanilla `state(key, init)` em `introspection/state.rs`.
    ///
    /// Define o valor inicial de um state identificado por `key`.
    /// Invisível em layout. Consumer: `Introspector::state_value`.
    /// **Modelo D (Lote 6 P321)**: `entities::elements::state::StateElem` (locatável).
    State(Arc<StateElem>),

    /// **P171 (M9 sub-passo 3)** — runtime mutable state, update.
    /// Vanilla `state.update(key, value)` (Set variant; Func adiada).
    ///
    /// Aplica `update` ao state identificado por `key` no ponto onde
    /// este nó aparece. Invisível em layout.
    /// **Modelo D (Lote 6 P321)**: `entities::elements::state_update::StateUpdateElem`
    /// (locatável).
    StateUpdate(Arc<StateUpdateElem>),

    /// **P240 (M9d / M7+1)** — render-mediated state display.
    /// Vanilla `state.display(callback)` em `introspection/state.rs`.
    ///
    /// Emite tag durante walk; valor + callback aplicado em
    /// `apply_state_displays` pós-fixpoint (paralelo
    /// `apply_state_funcs` P171). Resultado pre-rendered guardado
    /// em `Introspector.state_displays`; layout arm consome via
    /// `state_display_value(key, loc)` — Layouter permanece puro
    /// (sem Engine+ctx em signature).
    ///
    /// `callback: None` renderiza `Value` directo (Value::Content
    /// passa-through; Value::Str via Content::text; outros tipos
    /// fallback Content::Empty).
    /// **Modelo D (Lote 6 P321)**: `entities::elements::state_display::StateDisplayElem`
    /// (locatável).
    StateDisplay(Arc<StateDisplayElem>),

    /// **P241 (M9d / M7+2)** — render-mediated counter display real
    /// walk-time. Vanilla `counter.display(callback)` em
    /// `introspection/counter.rs`.
    ///
    /// Paralelo absoluto a `Content::StateDisplay` P240. Coexiste
    /// com `Content::CounterDisplay { kind }` legacy single-pass
    /// que continua a servir display simples sem callback no
    /// Layouter directo. Walk emite tag via `extract_payload`;
    /// `apply_counter_displays` pós-fixpoint pre-renderiza Content
    /// via `apply_func(callback, [Value::Array(counter_state)],
    /// ctx, engine)` e armazena em `Introspector.counter_displays`.
    /// Layout arm consome via `counter_display_value(key, loc)` —
    /// Layouter permanece puro.
    ///
    /// **Forma do Value passado ao callback** (Decisão 4 P241):
    /// `Value::Array(Vec<Value::Int>)` representando counter state
    /// actual (paridade vanilla `CounterState = SmallVec<[u64; 3]>`).
    /// Counter inexistente → `Value::Array(vec![])`.
    ///
    /// **Sem callback** (`callback: None`): formato default
    /// "1.2.3" via join "." (paridade `formatted_counter_at` P177).
    /// **Modelo D (Lote 6 P321)**: `entities::elements::counter_display_callback::CounterDisplayCallbackElem`
    /// (locatável).
    CounterDisplayCallback(Arc<CounterDisplayCallbackElem>),

    /// **P506** — Bloco de delayed evaluation (`context { expr }`).
    /// `entities::elements::context_block::ContextBlockElem` (locatável).
    ContextBlock(Arc<ContextBlockElem>),

    /// **Lote F-1 (P334) — a fronteira de extensão E1** (ADR-0106; L0
    /// `entities/f_fronteira_e1.md` §3a). A **única** porta de extensão: um
    /// elemento de utilizador (`impl Element`) entra aqui via `Arc<dyn
    /// DynElement>` (object-safe; o blanket bridga `Element → DynElement`). Os
    /// 6 matches do hub despacham por 1 linha, idêntico aos 65 nativos; o `dyn`
    /// só toca **esta** folha (os nativos ficam monomórficos — §4* da tabela
    /// P332, ADR-0029/0030). O enum continua **fechado** (ADR-0026): o dinâmico
    /// é o *conteúdo* da variante, não o enum.
    Dynamic(Arc<dyn DynElement>),
}

#[cfg(test)]
mod tests;
