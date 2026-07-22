//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/content.md
//! @prompt-hash 9c2a3110
//! @layer L1
//! @updated 2026-06-22
//!
//! Excepção Regra 6 da ADR-0037: o enum `Content` é a entidade
//! fundamental do domínio visual — representa toda a árvore de
//! conteúdo do documento (markup, math, grid, figure, shape,
//! transform, align, place, etc.). Dividir por variante destruiria
//! a fonte única da verdade estrutural e forçaria consumidores
//! (eval, layout) a re-assemblar o enum via re-exports. ~1070 linhas
//! aceitas como custo de coesão por domínio.

use std::fmt;
use std::sync::Arc;

use ecow::EcoString;

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
use crate::entities::sides::Sides;
use crate::entities::world_types::Datetime;
// Modelo D (ADR-0105, lote piloto P316): variantes delegadas a módulos.
use crate::entities::elements::divider::DividerElem;
use crate::entities::elements::emph::EmphElem;
use crate::entities::elements::heading::HeadingElem;
use crate::entities::elements::label::LabelElem;
use crate::entities::elements::math_styled::MathStyledElem;
use crate::entities::elements::strong::StrongElem;
use crate::entities::elements::title::TitleElem;
use crate::entities::elements::DynElement;
use crate::entities::elements::Element;
// Lote 2 P317 — família math element-shaped (11 variantes).
use crate::entities::elements::math_accent::MathAccentElem;
use crate::entities::elements::math_align_point::MathAlignPointElem;
use crate::entities::elements::math_attach::MathAttachElem;
use crate::entities::elements::math_cancel::MathCancelElem;
use crate::entities::elements::math_cases::MathCasesElem;
use crate::entities::elements::math_class_override::MathClassOverrideElem;
use crate::entities::elements::math_delimited::MathDelimitedElem;
use crate::entities::elements::math_frac::MathFracElem;
use crate::entities::elements::math_matrix::MathMatrixElem;
use crate::entities::elements::math_op::MathOpElem;
use crate::entities::elements::math_root::MathRootElem;
use crate::entities::elements::math_underover::MathUnderoverElem;
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
use crate::entities::elements::footnote::FootnoteElem;
use crate::entities::elements::grid::GridElem;
use crate::entities::elements::grid_cell::GridCellElem;
use crate::entities::elements::grid_hline::GridHLineElem;
use crate::entities::elements::grid_vline::GridVLineElem;
use crate::entities::elements::hide::HideElem;
use crate::entities::elements::image::ImageElem;
use crate::entities::elements::outline::OutlineElem;
use crate::entities::elements::pad::PadElem;
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
#[derive(Clone)]
pub enum Content {
    /// Conteúdo vazio.
    Empty,
    /// Texto simples. **F-5b fatia 2 (P372/P373, §3a.13/§3a.14)**: perdeu o
    /// `TextStyle` assado — o render do `#set text`/`#set par` vive **só na chain**
    /// (canal `custom` `"text.<campo>"`, transparente à morfologia; transporte
    /// aninhado P373). O layout resolve o render da chain. Fonte única do render.
    Text(EcoString),
    /// Espaço entre elementos (SpaceElem).
    Space,
    /// Quebra de parágrafo semântica (linha em branco no markup).
    /// **P622**: distinta de `Space` e de `Linebreak` (`\\` explícito).
    Parbreak,
    /// Sequência de elementos — clone O(1) via Arc (ADR-0026 revisão).
    Sequence(Arc<[Content]>),

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

    /// Raiz matemática (`√x`, `∛x`, `∜x`).
    /// `index`: None = raiz quadrada, Some(n) = raiz n-ésima.
    /// **Modelo D (Lote 2 P317)**: `entities::elements::math_root::MathRootElem`.
    MathRoot(Arc<MathRootElem>),

    /// Expressão entre delimitadores (`(...)`, `[...]`, `{...}`).
    /// `open`/`close` são os caracteres delimitadores.
    /// Mantida como variante própria para que o layout possa
    /// seleccionar variantes de tamanho (Passo 42).
    /// **Modelo D (Lote 2 P317)**: `entities::elements::math_delimited::MathDelimitedElem`.
    MathDelimited(Arc<MathDelimitedElem>),

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

    // ── P772y — `MathClassOverride` (`math.class(class, body)`) ─────────
    /// Força a `MathClass` de um símbolo/expressão — override do valor
    /// inferido automaticamente por `default_math_class`/`node_math_class`.
    /// Vanilla `ClassElem` (`math/mod.rs`). Afecta apenas espaçamento
    /// automático (`rules/math/layout/spacing.rs`); `body` é layoutado
    /// normalmente.
    MathClassOverride(Arc<MathClassOverrideElem>),

    // ── Passo 297 — `MathUnderover` (P296.1) ─────────────────────────────
    /// Anotações verticais sobre/sob conteúdo matemático — agregação
    /// cristalina do cluster vanilla `UnderlineElem`/`OverlineElem`/
    /// `UnderbraceElem`/`OverbraceElem`/`UnderbracketElem`/etc.
    ///
    /// **P297 (HV'.a + (b) Option fields)**: variant agregado per
    /// ADR-0054 graded. Vanilla typst fragmenta em 12 elementos
    /// separados (cada com `body` + `annotation: Option<Content>`);
    /// cristalino agrega num único variant com `base` + `under: Option`
    /// + `over: Option`. Layouter empilha verticalmente conforme
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

    /// Elemento com numeração própria e legenda opcional (Passo 62, DEBT-15 Passo 75).
    /// `kind` discrimina o contador: "image", "table", "raw", etc.
    /// `numbering` baked-in em eval via `#set figure(numbering: "1")` (DEBT-14).
    /// **Modelo D (Lote 13 P328)**: `entities::elements::figure::FigureElem`
    /// (locatável M1; contentor — recurse body + caption; kind/numbering
    /// `Option<String>` — `kind` None↔Auto, default "image" resolvido em uso).
    Figure(Arc<FigureElem>),

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
    /// resolvido via `engine/layout/grid_placement.rs` (P224.C fecha
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
    SetPage {
        width: Option<f64>,
        height: Option<f64>,
        margin: Option<f64>,
        /// **P532** — padrão de numeração automática de páginas.
        numbering: Option<EcoString>,
        /// **P537b** — número de colunas definido por `#set page(columns: N)`.
        columns: Option<usize>,
    },

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

    /// Conteúdo estilizado — aplica um delta `Styles` ao corpo (Passo 99,
    /// ADR-0038).
    ///
    /// Fundação tipada para `#set`/`#show`. Ainda **não** é consumida pelo
    /// Layouter actual — é construída em testes de integração e pelo
    /// pipeline futuro quando o eval activar estilos. Acessores por
    /// `StyleChain::push_styles` garantem que o delta é aplicado na
    /// resolução.
    Styled(Box<Content>, crate::entities::style::Styles),

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
    /// `crate::engine::lang::quotes::localize_quotes(lang)` consultando
    /// `text.lang` activo (per ADR-0057).
    /// **Modelo D (Lote 8 P323)**: `entities::elements::quote::QuoteElem`
    /// (não-locatável, contentor — recurse body + attribution).
    Quote(Arc<QuoteElem>),

    // ── Passo 397 — metadata do documento + placeholder de resource ───────
    /// Wrapper de metadata do documento (`#document(...)`).
    /// Não emite frames — é metadata pura; PDF Info dict continua scope-out
    /// per ADR-0054 graded.
    Document {
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
        kind: Option<EcoString>,
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
    /// (Smart<Option<Content>> vanilla → Option<Box<Content>>
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
    /// **Modelo D (Lote 11 P326)**: `entities::elements::footnote::FootnoteElem`
    /// (não-locatável P295 Fase 1; contentor — recurse body).
    Footnote(Arc<FootnoteElem>),

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
    ///   Sides<Option<Length>>).
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

impl fmt::Debug for Content {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        thread_local! {
            static DEPTH: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
        }
        const MAX_DEPTH: usize = 8;

        let depth = DEPTH.get();
        if depth >= MAX_DEPTH {
            return write!(f, "...");
        }
        DEPTH.set(depth + 1);
        let result = fmt_content(self, f);
        DEPTH.set(depth);
        result
    }
}

fn fmt_content(c: &Content, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match c {
        Content::Empty => write!(f, "empty"),
        Content::Text(t) => write!(f, "text({:?})", t),
        Content::Space => write!(f, "space"),
        Content::Parbreak => write!(f, "parbreak"),
        Content::Sequence(seq) => f.debug_tuple("sequence").field(&seq.as_ref()).finish(),
        Content::Heading(h) => write!(f, "heading({:?})", h),
        Content::Title(t) => write!(f, "title({:?})", t),
        Content::Strong(s) => write!(f, "strong({:?})", s),
        Content::Emph(e) => write!(f, "emph({:?})", e),
        Content::Raw(r) => write!(f, "raw({:?})", r),
        Content::ListItem(li) => write!(f, "list.item({:?})", li),
        Content::EnumItem(ei) => write!(f, "enum.item({:?})", ei),
        Content::Link(l) => write!(f, "link({:?})", l),
        Content::Equation(e) => write!(f, "equation({:?})", e),
        Content::MathSequence(seq) => {
            f.debug_tuple("math.sequence").field(&seq.as_ref()).finish()
        }
        Content::MathIdent(s) => write!(f, "math.ident({:?})", s),
        Content::MathText(s) => write!(f, "math.text({:?})", s),
        Content::MathFrac(fr) => write!(f, "math.frac({:?})", fr),
        Content::MathAttach(a) => write!(f, "math.attach({:?})", a),
        Content::MathRoot(r) => write!(f, "math.root({:?})", r),
        Content::MathDelimited(d) => write!(f, "math.delimited({:?})", d),
        Content::MathAlignPoint(_) => write!(f, "math.align.point"),
        Content::Linebreak(_) => write!(f, "linebreak"),
        Content::MathMatrix(_) => write!(f, "math.matrix"),
        Content::MathCases(_) => write!(f, "math.cases"),
        Content::MathAccent(a) => write!(f, "math.accent({:?})", a),
        Content::MathCancel(c) => write!(f, "math.cancel({:?})", c),
        Content::MathClassOverride(c) => write!(f, "math.class({:?})", c),
        Content::MathUnderover(u) => write!(f, "math.underover({:?})", u),
        Content::MathOp(o) => write!(f, "math.op({:?})", o),
        Content::MathStyled(s) => write!(f, "math.styled({:?})", s),
        Content::Label(l) => write!(f, "label({:?})", l),
        Content::Ref(r) => write!(f, "ref({:?})", r),
        Content::CounterDisplay(_) => write!(f, "counter.display"),
        Content::CounterUpdate(_) => write!(f, "counter.update"),
        Content::Outline(o) => write!(f, "outline({:?})", o),
        Content::Figure(fig) => write!(f, "figure({:?})", fig),
        Content::Image(i) => write!(f, "image({:?})", i),
        Content::Shape(_) => write!(f, "shape"),
        Content::Curve(_) => write!(f, "curve"),
        Content::Transform(t) => write!(f, "transform({:?})", t),
        Content::Grid(_) => write!(f, "grid"),
        Content::GridHeader(_) => write!(f, "grid.header"),
        Content::GridFooter(_) => write!(f, "grid.footer"),
        Content::GridCell(_) => write!(f, "grid.cell"),
        Content::GridHLine(_) => write!(f, "grid.hline"),
        Content::GridVLine(_) => write!(f, "grid.vline"),
        Content::SetPage { .. } => write!(f, "set.page"),
        Content::Align(a) => write!(f, "align({:?})", a),
        Content::Place(p) => write!(f, "place({:?})", p),
        Content::Styled(child, styles) => {
            write!(f, "styled({:?}, {:?})", child, styles)
        }
        Content::Divider(_) => write!(f, "divider"),
        Content::Terms(_) => write!(f, "terms"),
        Content::TermItem(ti) => write!(f, "term.item({:?})", ti),
        Content::Quote(q) => write!(f, "quote({:?})", q),
        Content::SmartQuote(_) => write!(f, "smart.quote"),
        Content::Underline(u) => write!(f, "underline({:?})", u),
        Content::Strike(s) => write!(f, "strike({:?})", s),
        Content::Overline(o) => write!(f, "overline({:?})", o),
        Content::SmallCaps { body } => write!(f, "small.caps({:?})", body),
        Content::Pad(p) => write!(f, "pad({:?})", p),
        Content::Hide(h) => write!(f, "hide({:?})", h),
        Content::HSpace(_) => write!(f, "hspace"),
        Content::VSpace(_) => write!(f, "vspace"),
        Content::Pagebreak(_) => write!(f, "pagebreak"),
        Content::Colbreak(_) => write!(f, "colbreak"),
        Content::Stack(_) => write!(f, "stack"),
        Content::Boxed(b) => write!(f, "box({:?})", b),
        Content::Block(b) => write!(f, "block({:?})", b),
        Content::TableCell(_) => write!(f, "table.cell"),
        Content::Bibliography(b) => write!(f, "bibliography({:?})", b),
        Content::Cite(c) => write!(f, "cite({:?})", c),
        Content::Footnote(foot) => write!(f, "footnote({:?})", foot),
        Content::TableHeader(_) => write!(f, "table.header"),
        Content::TableFooter(_) => write!(f, "table.footer"),
        Content::TableHLine(_) => write!(f, "table.hline"),
        Content::TableVLine(_) => write!(f, "table.vline"),
        Content::Table(_) => write!(f, "table"),
        Content::Repeat(_) => write!(f, "repeat"),
        Content::Columns(_) => write!(f, "columns"),
        Content::Metadata(_) => write!(f, "metadata"),
        Content::State(_) => write!(f, "state"),
        Content::StateUpdate(_) => write!(f, "state.update"),
        Content::StateDisplay(_) => write!(f, "state.display"),
        Content::CounterDisplayCallback(_) => write!(f, "counter.display.callback"),
        Content::Dynamic(_) => write!(f, "dynamic"),
        Content::Document { title, .. } => {
            if let Some(t) = title {
                write!(f, "document({:?})", t)
            } else {
                write!(f, "document")
            }
        }
        Content::Asset { path, .. } => write!(f, "asset({:?})", path),
        Content::ContextBlock(e) => write!(f, "context({:?})", e),
    }
}

impl Content {
    /// Nome do elemento de conteúdo para mensagens de erro e diagnósticos.
    pub fn elem_name(&self) -> &'static str {
        match self {
            Self::Text(_) => "text",
            Self::Space => "space",
            Self::Empty => "empty",
            Self::Sequence(_) => "sequence",
            Self::Styled(..) => "styled",
            Self::Heading(_) => "heading",
            Self::Title(_) => "title",
            Self::Strong(_) => "strong",
            Self::Emph(_) => "emph",
            Self::Raw(_) => "raw",
            Self::ListItem(_) => "list.item",
            Self::EnumItem(_) => "enum.item",
            Self::Link(_) => "link",
            Self::Equation(_) => "equation",
            Self::MathSequence(_) => "math.sequence",
            Self::MathIdent(_) => "math.ident",
            Self::MathText(_) => "math.text",
            Self::MathFrac(_) => "math.frac",
            Self::MathAttach(_) => "math.attach",
            Self::MathRoot(_) => "math.root",
            Self::MathDelimited(_) => "math.delimited",
            Self::MathAlignPoint(_) => "math.align.point",
            Self::Linebreak(_) => "linebreak",
            Self::MathMatrix(_) => "math.matrix",
            Self::MathCases(_) => "math.cases",
            Self::MathAccent(_) => "math.accent",
            Self::MathCancel(_) => "math.cancel",
            Self::MathClassOverride(_) => "math.class",
            Self::MathUnderover(_) => "math.underover",
            Self::MathOp(_) => "math.op",
            Self::MathStyled(_) => "math.styled",
            Self::Label(_) => "label",
            Self::Ref(_) => "ref",
            Self::CounterDisplay(_) => "counter.display",
            Self::CounterUpdate(_) => "counter.update",
            Self::Outline(_) => "outline",
            Self::Figure(_) => "figure",
            Self::Image(_) => "image",
            Self::Shape(_) => "shape",
            Self::Curve(_) => "curve",
            Self::Transform(_) => "transform",
            Self::Grid(_) => "grid",
            Self::GridHeader(_) => "grid.header",
            Self::GridFooter(_) => "grid.footer",
            Self::GridCell(_) => "grid.cell",
            Self::GridHLine(_) => "grid.hline",
            Self::GridVLine(_) => "grid.vline",
            Self::SetPage { .. } => "set.page",
            Self::Align(_) => "align",
            Self::Place(_) => "place",
            Self::Divider(_) => "divider",
            Self::Terms(_) => "terms",
            Self::TermItem(_) => "term.item",
            Self::Quote(_) => "quote",
            Self::SmartQuote(_) => "smart.quote",
            Self::Underline(_) => "underline",
            Self::Strike(_) => "strike",
            Self::Overline(_) => "overline",
            Self::SmallCaps { .. } => "small.caps",
            Self::Pad(_) => "pad",
            Self::Columns(_) => "columns",
            Self::Boxed(_) => "box",
            Self::Block(_) => "block",
            Self::Table(_) => "table",
            Self::TableCell(_) => "table.cell",
            Self::TableHeader(_) => "table.header",
            Self::TableFooter(_) => "table.footer",
            Self::TableHLine(_) => "table.hline",
            Self::TableVLine(_) => "table.vline",
            Self::Bibliography(_) => "bibliography",
            Self::Cite(_) => "cite",
            Self::Footnote(_) => "footnote",
            Self::Metadata(_) => "metadata",
            Self::State(_) => "state",
            Self::StateUpdate(_) => "state.update",
            Self::StateDisplay(_) => "state.display",
            Self::CounterDisplayCallback(_) => "counter.display.callback",
            Self::Dynamic(_) => "dynamic",
            Self::Document { .. } => "document",
            Self::Asset { .. } => "asset",
            Self::ContextBlock(_) => "context",
            _ => "content",
        }
    }
    /// Cria conteúdo de texto. **F-5b fatia 2 (P373)**: sem estilo assado — o render
    /// do `#set text`/`#set par` viaja na chain (`custom`), resolvido no layout.
    pub fn text(s: impl Into<EcoString>) -> Self {
        Self::Text(s.into())
    }

    /// Cria conteúdo vazio.
    pub fn empty() -> Self {
        Self::Empty
    }

    /// Cria uma sequência, normalizando casos degenerados.
    ///
    /// - 0 partes → `Empty`
    /// - 1 parte → desembrulha (evita `Sequence([x])`)
    /// - n > 1 → `Sequence(parts)`
    /// Negrito (`*bold*`) — **F-5b fatia 1 (P371, §3a.12)**: produz a **variante
    /// própria** `Content::Strong` (modelo D), distinta de `Styled` (era
    /// `Styled([Bold])` no colapso P101, agora **superado** pelo modelo de
    /// variantes que a 0026 `:63`/0105-D prescrevem). Render (bold) replicado no
    /// layout; o `==`/`#show`/`morph_canon` distinguem `strong` de `#set text(bold)`
    /// (fidelidade ADR-0107).
    pub fn strong(body: Content) -> Self {
        Self::Strong(Arc::new(StrongElem::new(body)))
    }

    /// Itálico (`_italic_`) — **F-5b fatia 1 (P371, §3a.12)**: variante própria
    /// `Content::Emph` (era `Styled([Italic])`).
    pub fn emph(body: Content) -> Self {
        Self::Emph(Arc::new(EmphElem::new(body)))
    }
    pub fn heading(level: u8, body: Content) -> Self {
        Self::Heading(Arc::new(HeadingElem::new(level, body)))
    }

    /// Cria um título de documento (`#title(...)`).
    pub fn title(body: Content) -> Self {
        Self::Title(Arc::new(TitleElem::new(body)))
    }
    /// **P605/P606** — heading com controlo explícito de `outlined` e `bookmarked`.
    pub fn heading_with_outlined(level: u8, body: Content, outlined: bool) -> Self {
        Self::Heading(Arc::new(HeadingElem::new_with_outlined(level, body, outlined)))
    }
    /// **P606** — heading com controlo separado de `outlined` e `bookmarked`.
    pub fn heading_with_outlined_and_bookmarked(
        level: u8,
        body: Content,
        outlined: bool,
        bookmarked: Option<bool>,
    ) -> Self {
        Self::Heading(Arc::new(HeadingElem::new_with_outlined_and_bookmarked(
            level, body, outlined, bookmarked,
        )))
    }
    /// **P829** — heading via função nativa `#heading(...)`: regista na
    /// máscara `set_fields` quais campos foram explicitamente assentes
    /// (`HEADING_SET_*`) — paridade vanilla `has`/`at`/`fields`, que só vêem
    /// campos assentes no constructor (medido: `heading[H].has("level")` →
    /// false; `heading(level: 2)[H].has("level")` → true).
    pub fn heading_native(
        level: u8,
        body: Content,
        outlined: bool,
        bookmarked: Option<bool>,
        set_fields: u8,
    ) -> Self {
        let mut elem =
            HeadingElem::new_with_outlined_and_bookmarked(level, body, outlined, bookmarked);
        elem.set_fields = set_fields;
        Self::Heading(Arc::new(elem))
    }
    /// Heading numerado **na forma de transporte** (F-5a de-bake, P364,
    /// `f_fronteira_e1.md` §3a.9). O campo assado `numbering_active` foi
    /// removido; a numeração vive **só na chain** — este construtor produz o
    /// `Content::Styled` que carrega `custom("heading.numbering")=true`, a forma
    /// canônica que `#set heading(numbering:)` gera em produção (transporte
    /// fatia-1). Consumidor (layout/introspect) lê o gate da chain. Usado pelos
    /// fixtures para construir um heading numerado sem passar pelo eval.
    pub fn heading_numbered(level: u8, body: Content) -> Self {
        Self::heading_numbered_with_pattern(level, body, None)
    }
    /// Heading numerado **na forma de transporte** com pattern configurável
    /// (P451). O pattern é transportado na chain como
    /// `custom("heading.numbering.pattern")`; o gate `heading.numbering` continua
    /// a ser o booleano de activação.
    pub fn heading_numbered_with_pattern(
        level: u8,
        body: Content,
        pattern: Option<ecow::EcoString>,
    ) -> Self {
        Self::heading_numbered_with_pattern_and_outlined(level, body, pattern, true)
    }
    /// **P605/P606** — variant numerado com controlo de `outlined`.
    pub fn heading_numbered_with_pattern_and_outlined(
        level: u8,
        body: Content,
        pattern: Option<ecow::EcoString>,
        outlined: bool,
    ) -> Self {
        Self::heading_numbered_with_pattern_outlined_bookmarked(
            level, body, pattern, outlined, None,
        )
    }
    /// **P606** — variant numerado com controlo separado de `outlined` e `bookmarked`.
    pub fn heading_numbered_with_pattern_outlined_bookmarked(
        level: u8,
        body: Content,
        pattern: Option<ecow::EcoString>,
        outlined: bool,
        bookmarked: Option<bool>,
    ) -> Self {
        Self::heading_numbered_native(level, body, pattern, outlined, bookmarked, 0)
    }
    /// **P829** — variant numerado para a função nativa `#heading(numbering:)`:
    /// como `heading_numbered_with_pattern_outlined_bookmarked` mas com a
    /// máscara `set_fields` dos campos explicitamente assentes (ver
    /// `heading_native`).
    pub fn heading_numbered_native(
        level: u8,
        body: Content,
        pattern: Option<ecow::EcoString>,
        outlined: bool,
        bookmarked: Option<bool>,
        set_fields: u8,
    ) -> Self {
        use crate::entities::style::Styles;
        use crate::entities::value::Value;
        let mut styles =
            Styles::new().push_custom("heading.numbering", Value::Bool(true));
        if let Some(pattern) = pattern {
            styles = styles.push_custom("heading.numbering.pattern", Value::Str(pattern));
        }
        Self::Styled(
            Box::new(Self::heading_native(level, body, outlined, bookmarked, set_fields)),
            styles,
        )
    }
    /// Construtor do separador estrutural (Modelo D, P316).
    pub fn divider() -> Self {
        Self::Divider(Arc::new(DividerElem))
    }
    /// Construtor da fronteira de extensão E1 (Lote F-1, P334). Embrulha um
    /// elemento de utilizador (qualquer `impl Element`) na variante dinâmica.
    /// `Arc<E>` faz coerção para `Arc<dyn DynElement>` via o blanket.
    pub fn dynamic<E: DynElement>(elem: E) -> Self {
        Self::Dynamic(Arc::new(elem))
    }
    /// Construtor de `MathStyled` (Modelo D, P316).
    pub fn math_styled(
        kind: Option<MathStyleKind>,
        bold: Option<bool>,
        italic: Option<bool>,
        body: Content,
        cramped: Option<bool>,
    ) -> Self {
        Self::MathStyled(Arc::new(MathStyledElem { kind, bold, italic, body, cramped }))
    }

    // ── Construtores ergonómicos da família math (Modelo D, Lote 2 P317) ──────
    /// Construtor de `MathFrac`.
    pub fn math_frac(num: Content, den: Content) -> Self {
        Self::MathFrac(Arc::new(MathFracElem { num, den }))
    }
    /// Construtor de `MathAttach` (base + pre/pos-scripts opcionais).
    pub fn math_attach(
        base: Content,
        tl: Option<Content>,
        bl: Option<Content>,
        sub: Option<Content>,
        sup: Option<Content>,
    ) -> Self {
        Self::MathAttach(Arc::new(MathAttachElem { base, tl, bl, sub, sup }))
    }
    /// Construtor de `MathRoot` (`index: None` = raiz quadrada).
    pub fn math_root(index: Option<Content>, radicand: Content) -> Self {
        Self::MathRoot(Arc::new(MathRootElem { index, radicand }))
    }
    /// Construtor de `MathDelimited`.
    pub fn math_delimited(open: char, body: Content, close: char) -> Self {
        Self::MathDelimited(Arc::new(MathDelimitedElem { open, body, close }))
    }
    /// Construtor de `MathAlignPoint` (marcador `&`).
    pub fn math_align_point() -> Self {
        Self::MathAlignPoint(Arc::new(MathAlignPointElem))
    }
    /// Construtor de `MathMatrix`.
    pub fn math_matrix(rows: Vec<Vec<Content>>, delim: (char, char)) -> Self {
        Self::MathMatrix(Arc::new(MathMatrixElem { rows, delim }))
    }
    /// Construtor de `MathCases`.
    pub fn math_cases(rows: Vec<Vec<Content>>) -> Self {
        Self::MathCases(Arc::new(MathCasesElem { rows }))
    }
    /// Construtor de `MathAccent`.
    pub fn math_accent(base: Content, accent: Content) -> Self {
        Self::MathAccent(Arc::new(MathAccentElem { base, accent }))
    }
    /// Construtor de `MathCancel`.
    pub fn math_cancel(body: Content) -> Self {
        Self::MathCancel(Arc::new(MathCancelElem { body }))
    }
    /// Construtor de `MathClassOverride` — `math.class(class, body)`.
    pub fn math_class_override(
        class: crate::entities::math_class::MathClass,
        body: Content,
    ) -> Self {
        Self::MathClassOverride(Arc::new(MathClassOverrideElem { class, body }))
    }
    /// Construtor de `MathUnderover`.
    pub fn math_underover(
        base: Content,
        under: Option<Content>,
        over: Option<Content>,
    ) -> Self {
        Self::MathUnderover(Arc::new(MathUnderoverElem { base, under, over }))
    }
    /// Construtor de `MathOp` (`limits` é discriminador de layout).
    pub fn math_op(text: Content, limits: bool) -> Self {
        Self::MathOp(Arc::new(MathOpElem { text, limits }))
    }

    pub fn raw(text: impl Into<EcoString>, lang: Option<EcoString>, block: bool) -> Self {
        Self::Raw(Arc::new(RawElem { text: text.into(), lang, block }))
    }
    /// `align(alignment, body)` — Modelo D (Lote 7 P322).
    pub fn align(alignment: Align2D, body: Content) -> Self {
        Self::Align(Arc::new(AlignElem { alignment, body }))
    }
    /// `image(path, data, width, height, fit)` — Modelo D (Lote 7 P322) + P502.
    pub fn image(
        path: impl Into<String>,
        data: PtrEqArc<Vec<u8>>,
        width: Option<Box<crate::entities::value::Value>>,
        height: Option<Box<crate::entities::value::Value>>,
        fit: impl Into<EcoString>,
    ) -> Self {
        Self::Image(Arc::new(ImageElem {
            path: path.into(),
            data,
            width,
            height,
            fit: fit.into(),
        }))
    }
    // ── Construtores ergonómicos família lista/termos (Modelo D, Lote 3 P318) ──
    pub fn list_item(body: Content) -> Self {
        Self::ListItem(Arc::new(ListItemElem {
            body,
            marker: None,
            marker_align: None,
            indent: None,
            body_indent: None,
            tight: None,
        }))
    }
    /// P470 — construtor com marcador customizado.
    pub fn list_item_with_marker(
        body: Content,
        marker: crate::entities::list_marker::ListMarker,
    ) -> Self {
        Self::ListItem(Arc::new(ListItemElem {
            body,
            marker: Some(marker),
            marker_align: None,
            indent: None,
            body_indent: None,
            tight: None,
        }))
    }
    /// **P504/P505** — construtor completo com marcador, alinhamento e indentação.
    pub fn list_item_full(
        body: Content,
        marker: Option<crate::entities::list_marker::ListMarker>,
        marker_align: Option<crate::entities::layout_types::Align2D>,
        indent: Option<crate::entities::layout_types::Length>,
        body_indent: Option<crate::entities::layout_types::Length>,
        tight: Option<bool>,
    ) -> Self {
        Self::ListItem(Arc::new(ListItemElem {
            body,
            marker,
            marker_align,
            indent,
            body_indent,
            tight,
        }))
    }
    pub fn enum_item(number: Option<u32>, body: Content) -> Self {
        Self::EnumItem(Arc::new(EnumItemElem {
            number,
            body,
            numbering: None,
            indent: None,
            body_indent: None,
            tight: None,
        }))
    }
    /// P470 — construtor com esquema de numeração.
    pub fn enum_item_with_numbering(
        number: Option<u32>,
        body: Content,
        numbering: crate::entities::enum_numbering::EnumNumbering,
    ) -> Self {
        Self::EnumItem(Arc::new(EnumItemElem {
            number,
            body,
            numbering: Some(numbering),
            indent: None,
            body_indent: None,
            tight: None,
        }))
    }
    /// **P505** — construtor completo com numeração e indentação.
    pub fn enum_item_full(
        number: Option<u32>,
        body: Content,
        numbering: Option<crate::entities::enum_numbering::EnumNumbering>,
        indent: Option<crate::entities::layout_types::Length>,
        body_indent: Option<crate::entities::layout_types::Length>,
        tight: Option<bool>,
    ) -> Self {
        Self::EnumItem(Arc::new(EnumItemElem {
            number,
            body,
            numbering,
            indent,
            body_indent,
            tight,
        }))
    }
    pub fn link(url: impl Into<EcoString>, body: Content) -> Self {
        Self::Link(Arc::new(LinkElem { url: url.into(), body }))
    }
    pub fn terms(items: Vec<Content>) -> Self {
        Self::Terms(Arc::new(TermsElem { items }))
    }
    pub fn term_item(term: Content, description: Content) -> Self {
        Self::TermItem(Arc::new(TermItemElem { term, description }))
    }

    // ── Construtores ergonómicos decorações de texto (Modelo D, Lote 4 P319) ──
    pub fn overline(
        body: Content,
        stroke: Option<Color>,
        offset: Option<Length>,
        extent: Option<Length>,
    ) -> Self {
        Self::Overline(Arc::new(OverlineElem { body, stroke, offset, extent }))
    }
    pub fn strike(
        body: Content,
        stroke: Option<Color>,
        offset: Option<Length>,
        extent: Option<Length>,
    ) -> Self {
        Self::Strike(Arc::new(StrikeElem { body, stroke, offset, extent }))
    }
    pub fn underline(
        body: Content,
        stroke: Option<Color>,
        offset: Option<Length>,
        extent: Option<Length>,
    ) -> Self {
        Self::Underline(Arc::new(UnderlineElem { body, stroke, offset, extent }))
    }

    /// **Passo 408** — `smallcaps(body)`.
    ///
    /// Consumer real de small caps requer shaping OpenType (`smcp`/`c2sc`);
    /// até lá, o layout trata como stub transparente (ADR-0054 graded).
    pub fn smallcaps(body: Content) -> Self {
        Self::SmallCaps { body: Box::new(body) }
    }

    /// **Passo 448** — `sub(body)`.
    pub fn sub(body: Content) -> Self {
        use crate::entities::style::{Style, Styles};
        Self::Styled(Box::new(body), Styles::from_iter([Style::subscript(true)]))
    }

    /// **P471** — `sub(body, size: length)` com tamanho explícito.
    pub fn sub_with_size(
        body: Content,
        size: Option<crate::entities::layout_types::Length>,
    ) -> Self {
        use crate::entities::style::{Style, Styles};
        let mut styles = vec![Style::subscript(true)];
        if let Some(s) = size {
            styles.push(Style::subscript_size(s));
        }
        Self::Styled(Box::new(body), Styles::from_iter(styles))
    }

    /// **Passo 448** — `super(body)` (`super` é keyword em Rust, logo o
    /// construtor chama-se `superscript`).
    pub fn superscript(body: Content) -> Self {
        use crate::entities::style::{Style, Styles};
        Self::Styled(Box::new(body), Styles::from_iter([Style::superscript(true)]))
    }

    /// **P471** — `super(body, size: length)` com tamanho explícito.
    pub fn superscript_with_size(
        body: Content,
        size: Option<crate::entities::layout_types::Length>,
    ) -> Self {
        use crate::entities::style::{Style, Styles};
        let mut styles = vec![Style::superscript(true)];
        if let Some(s) = size {
            styles.push(Style::superscript_size(s));
        }
        Self::Styled(Box::new(body), Styles::from_iter(styles))
    }

    /// **Passo 449** — `highlight(body, fill)`.
    pub fn highlight(
        body: Content,
        fill: Option<crate::entities::layout_types::Color>,
    ) -> Self {
        use crate::entities::style::{Style, Styles};
        Self::Styled(Box::new(body), Styles::from_iter([Style::highlight(fill)]))
    }

    /// **P471** — `highlight(body, fill, radius, extent)` com parâmetros cosmésticos.
    pub fn highlight_full(
        body: Content,
        fill: Option<crate::entities::layout_types::Color>,
        radius: Option<crate::entities::layout_types::Length>,
        extent: Option<crate::entities::layout_types::Length>,
    ) -> Self {
        use crate::entities::style::{Style, Styles};
        let mut styles = vec![Style::highlight(fill)];
        if let Some(r) = radius {
            styles.push(Style::highlight_radius(r));
        }
        if let Some(e) = extent {
            styles.push(Style::highlight_extent(e));
        }
        Self::Styled(Box::new(body), Styles::from_iter(styles))
    }

    /// `pad(body, sides)` — Passo 156C (ADR-0061 Fase 1) /
    /// Passo 156L (refino sides individualizadas per ADR-0064 Caso C).
    pub fn pad(body: Content, sides: Sides<Option<Length>>) -> Self {
        Self::Pad(Arc::new(PadElem { body, sides }))
    }

    /// **Lote 10 P325** — `Content::Equation` (equação matemática).
    pub fn equation(body: Content, block: bool) -> Self {
        Self::Equation(Arc::new(EquationElem::new(body, block)))
    }
    /// Equação numerada **na forma de transporte** (F-5a de-bake, P364,
    /// `f_fronteira_e1.md` §3a.9). Campo assado removido; a numeração vive **só
    /// na chain** — produz o `Content::Styled` com `custom("equation.numbering")
    /// ="(1)"`, a forma que `#set math.equation(numbering:)` gera em produção. O
    /// gate efetivo continua `block && numbering` no consumidor.
    /// P456: pattern default "(1)" (format_counter) em vez de Bool.
    pub fn equation_numbered(body: Content, block: bool) -> Self {
        use crate::entities::style::Styles;
        use crate::entities::value::Value;
        Self::Styled(
            Box::new(Self::equation(body, block)),
            Styles::new().push_custom("equation.numbering", Value::Str("(1)".into())),
        )
    }

    /// **Lote 11 P326** — `Content::Footnote` (nota de rodapé).
    pub fn footnote(body: Content) -> Self {
        Self::Footnote(Arc::new(FootnoteElem { body, numbering: None }))
    }

    /// **P502** — `Content::Footnote` com `numbering` customizado.
    pub fn footnote_with_numbering(body: Content, numbering: Option<EcoString>) -> Self {
        Self::Footnote(Arc::new(FootnoteElem { body, numbering }))
    }

    /// **Lote 11 P326** — `Content::Shape` (geometria).
    pub fn shape(
        kind: ShapeKind,
        width: Option<Box<crate::entities::value::Value>>,
        height: Option<Box<crate::entities::value::Value>>,
        fill: Option<crate::entities::paint::Paint>,
        stroke: Option<Stroke>,
    ) -> Self {
        Self::Shape(Arc::new(ShapeElem { kind, width, height, fill, stroke }))
    }

    /// `hide(body)` — Passo 156C (ADR-0061 Fase 1).
    pub fn hide(body: Content) -> Self {
        Self::Hide(Arc::new(HideElem { body }))
    }

    /// `h(amount, weak)` — Passo 156D (ADR-0061 Fase 1 sub-passo 2).
    pub fn h_space(amount: Length, weak: bool) -> Self {
        Self::HSpace(Arc::new(HSpaceElem {
            amount: crate::entities::elements::h_space::Spacing::Absolute(amount),
            weak,
        }))
    }

    /// **P842 (#38)** — `h(amount.fr, weak)`: fração do espaço restante da
    /// linha (paridade vanilla `Spacing::Fractional`, `layout/spacing.rs`).
    pub fn h_space_fraction(fr: f64, weak: bool) -> Self {
        Self::HSpace(Arc::new(HSpaceElem {
            amount: crate::entities::elements::h_space::Spacing::Fractional(fr),
            weak,
        }))
    }

    /// `v(amount, weak)` — Passo 156D (ADR-0061 Fase 1 sub-passo 2).
    pub fn v_space(amount: Length, weak: bool) -> Self {
        Self::VSpace(Arc::new(VSpaceElem { amount, weak }))
    }

    /// `pagebreak(weak, to)` — Passo 156E (ADR-0061 Fase 1 sub-passo 3).
    pub fn pagebreak(weak: bool, to: Option<Parity>) -> Self {
        Self::Pagebreak(Arc::new(PagebreakElem { weak, to }))
    }

    /// `colbreak(weak)` — Passo 220 (ADR-0078 PROPOSTO sub-fase b 4/4).
    pub fn colbreak(weak: bool) -> Self {
        Self::Colbreak(Arc::new(ColbreakElem { weak }))
    }

    /// `linebreak()` — Modelo D (Lote 5 P320).
    pub fn linebreak() -> Self {
        Self::Linebreak(Arc::new(LinebreakElem))
    }
    /// `grid_header(body, repeat)` — Modelo D (Lote 5 P320).
    pub fn grid_header(body: Content, repeat: bool) -> Self {
        Self::GridHeader(Arc::new(GridHeaderElem { body, repeat }))
    }
    /// `grid_footer(body, repeat)` — Modelo D (Lote 5 P320).
    pub fn grid_footer(body: Content, repeat: bool) -> Self {
        Self::GridFooter(Arc::new(GridFooterElem { body, repeat }))
    }
    /// `grid_hline(start, end, row, stroke, position)` — Passo 512.
    pub fn grid_hline(
        start: usize,
        end: Option<usize>,
        row: usize,
        stroke: Option<Stroke>,
        position: EcoString,
    ) -> Self {
        Self::GridHLine(Arc::new(GridHLineElem { start, end, row, stroke, position }))
    }
    /// `grid_vline(start, end, col, stroke, position)` — Passo 512.
    pub fn grid_vline(
        start: usize,
        end: Option<usize>,
        col: usize,
        stroke: Option<Stroke>,
        position: EcoString,
    ) -> Self {
        Self::GridVLine(Arc::new(GridVLineElem { start, end, col, stroke, position }))
    }

    // ── Construtores ergonómicos família state/counter (Modelo D, Lote 6 P321) ──
    /// **P460/P464** — `Content::Label` criado explicitamente pelo utilizador
    /// (`auto: false`).
    pub fn label(name: impl Into<EcoString>, body: Content) -> Self {
        Self::Label(Arc::new(LabelElem { name: name.into(), body, auto: false }))
    }

    /// **P464** — `Content::Label` gerado automaticamente pela sintaxe `<label>`
    /// (`auto: true`).
    pub fn label_auto(name: impl Into<EcoString>, body: Content) -> Self {
        Self::Label(Arc::new(LabelElem { name: name.into(), body, auto: true }))
    }

    /// **Lote 13 P328** — `Content::Figure` (figura locatável M1).
    ///
    /// **F-5a de-bake (P365, `f_fronteira_e1.md` §3a.9):** o campo assado
    /// `numbering` foi removido de `FigureElem`. Quando `numbering` é `Some(padrão)`
    /// este construtor produz a **forma de transporte** (`Content::Styled` com
    /// `custom("figure.numbering")`), a forma canônica que `#set figure(numbering:)`
    /// gera — usada pelos fixtures. `None` → figura simples. Em produção
    /// `native_figure` passa `None` (a fatia-1 carrega o gate).
    pub fn figure(
        body: Content,
        caption: Option<Content>,
        kind: Option<String>,
        numbering: Option<String>,
    ) -> Self {
        let fig = Self::Figure(Arc::new(FigureElem { body, caption, kind }));
        match numbering {
            Some(pat) => {
                use crate::entities::style::Styles;
                use crate::entities::value::Value;
                Self::Styled(
                    Box::new(fig),
                    Styles::new().push_custom("figure.numbering", Value::Str(pat.into())),
                )
            }
            None => fig,
        }
    }

    pub fn counter_display(kind: impl Into<String>) -> Self {
        Self::CounterDisplay(Arc::new(CounterDisplayElem { kind: kind.into() }))
    }
    pub fn counter_update(key: impl Into<String>, action: CounterAction) -> Self {
        Self::CounterUpdate(Arc::new(CounterUpdateElem { key: key.into(), action }))
    }
    pub fn metadata(value: crate::entities::value::Value) -> Self {
        Self::Metadata(Arc::new(MetadataElem { value: Box::new(value) }))
    }
    pub fn state(key: impl Into<String>, init: crate::entities::value::Value) -> Self {
        Self::State(Arc::new(StateElem { key: key.into(), init: Box::new(init) }))
    }
    pub fn state_update(
        key: impl Into<String>,
        update: crate::entities::state_update::StateUpdate,
    ) -> Self {
        Self::StateUpdate(Arc::new(StateUpdateElem { key: key.into(), update }))
    }
    pub fn state_display(
        key: impl Into<String>,
        callback: Option<crate::entities::func::Func>,
    ) -> Self {
        Self::StateDisplay(Arc::new(StateDisplayElem { key: key.into(), callback }))
    }
    pub fn counter_display_callback(
        key: impl Into<String>,
        callback: Option<crate::entities::func::Func>,
    ) -> Self {
        Self::CounterDisplayCallback(Arc::new(CounterDisplayCallbackElem {
            key: key.into(),
            callback,
        }))
    }

    /// `block(body, width, height, inset, breakable)` — Passo 156G
    /// (ADR-0061 Fase 2 sub-passo 1). Construtor com defaults sensatos
    /// (None/zero/true) para uso programático.
    pub fn block(
        body: Content,
        width: Option<Length>,
        height: Option<Length>,
        inset: Sides<Length>,
        breakable: bool,
    ) -> Self {
        Self::Block(Arc::new(BlockElem {
            body,
            width,
            height,
            inset,
            breakable,
            outset: Sides::new(
                Length::pt(0.0),
                Length::pt(0.0),
                Length::pt(0.0),
                Length::pt(0.0),
            ),
            // P242 — radius `Corners<Length>` substitui `Option<Length>` P231.
            radius: crate::entities::corners::Corners::uniform(Length::ZERO),
            clip: false,
            // P247 — fill/stroke default `None` (paridade pattern P242).
            fill: None,
            stroke: None,
            // P250 — spacing/above/below/sticky defaults (None×3 + false).
            spacing: None,
            above: None,
            below: None,
            sticky: false,
        }))
    }

    /// `box(body, width, height, inset, baseline)` — Passo 156H
    /// (ADR-0061 Fase 2 sub-passo 2). Naming `boxed` evita conflito com
    /// `std::boxed::Box`; stdlib expõe `#box(...)` (paridade vanilla).
    pub fn boxed(
        body: Content,
        width: Option<Length>,
        height: Option<Length>,
        inset: Sides<Length>,
        baseline: Length,
    ) -> Self {
        Self::Boxed(Arc::new(BoxedElem {
            body,
            width,
            height,
            inset,
            baseline,
            outset: Sides::new(
                Length::pt(0.0),
                Length::pt(0.0),
                Length::pt(0.0),
                Length::pt(0.0),
            ),
            // P242 — radius `Corners<Length>` substitui `Option<Length>` P231.
            radius: crate::entities::corners::Corners::uniform(Length::ZERO),
            clip: false,
            // P247 — fill/stroke default `None` paralelo Block.
            fill: None,
            stroke: None,
        }))
    }

    /// `stack(dir, spacing, ..children)` — Passo 156I (ADR-0061 Fase 2
    /// sub-passo 3). Atinge target 72% Layout. Aceita Vec<Content> que
    /// converte para `Arc<[Content]>` (clone O(1) per ADR-0026 revisão).
    pub fn stack(children: Vec<Content>, dir: Dir, spacing: Option<Length>) -> Self {
        Self::Stack(Arc::new(StackElem { children: children.into(), dir, spacing }))
    }

    /// **Lote 9 P324** — `Content::SmartQuote` (aspa lang-aware).
    pub fn smartquote(double: bool) -> Self {
        Self::SmartQuote(Arc::new(SmartQuoteElem { double }))
    }

    /// **Lote 9 P324** — `Content::Transform` (transformação afim 2D).
    pub fn transform(matrix: TransformMatrix, body: Content) -> Self {
        Self::Transform(Arc::new(TransformElem { matrix, body }))
    }

    /// **Lote 9 P324** — `Content::Place` (posicionamento absoluto/flutuante).
    #[allow(clippy::too_many_arguments)]
    pub fn place(
        alignment: Align2D,
        dx: f64,
        dy: f64,
        scope: PlaceScope,
        float: bool,
        clearance: Option<Length>,
        body: Content,
    ) -> Self {
        Self::Place(Arc::new(PlaceElem {
            alignment,
            dx,
            dy,
            scope,
            float,
            clearance,
            body,
        }))
    }

    /// `repeat(body, gap, justify)` — Passo 156J (ADR-0061 Fase 3
    /// sub-passo 1). **Primeira Fase 3**. Default `justify == true`
    /// (paridade vanilla); algoritmo dinâmico de quantidade-para-encher
    /// diferido per ADR-0054 graded.
    pub fn repeat(body: Content, gap: Option<Length>, justify: bool) -> Self {
        Self::Repeat(Arc::new(RepeatElem { body, gap, justify }))
    }

    /// **P217** — Construtor `Content::Columns` (multi-column container).
    /// Stdlib `native_columns` em P218 com validação `count >= 1`.
    /// Consumer multi-region real em P219.
    pub fn columns(body: Content, count: usize, gutter: Option<Length>) -> Self {
        Self::Columns(Arc::new(ColumnsElem { count, gutter, body, page_columns: false }))
    }

    /// **P462** — `Content::Ref` (referência cruzada `@label` / `ref("label")`).
    /// Nome `reference` (não `r#ref`: evita raw identifier nos call-sites).
    pub fn reference(name: impl Into<EcoString>) -> Self {
        Self::reference_with_supplement(name, None)
    }

    /// **P462** — `Content::Ref` com supplement explícito (ex: `ref("fig1", supplement: "Fig. ")`).
    pub fn reference_with_supplement(
        name: impl Into<EcoString>,
        supplement: Option<Content>,
    ) -> Self {
        Self::Ref(Arc::new(RefElem { name: name.into(), supplement }))
    }

    /// **Lote 8 P323** — `Content::Outline` (índice). Unit struct.
    /// P457: adiciona parâmetros `title`, `depth`, `indent` com defaults vanilla.
    /// P502: `indent` é `OutlineIndent` (auto/bool/length/function).
    pub fn outline() -> Self {
        use crate::entities::elements::outline::OutlineIndent;
        Self::outline_with(None, 3, OutlineIndent::Auto)
    }

    pub fn outline_with(
        title: Option<Content>,
        depth: usize,
        indent: crate::entities::elements::outline::OutlineIndent,
    ) -> Self {
        Self::Outline(Arc::new(OutlineElem::new(title, depth, indent)))
    }

    /// **P472** — List of Figures (`lof()`).
    pub fn lof(title: Option<Content>) -> Self {
        use crate::entities::elements::outline::{OutlineIndent, OutlineTarget};
        Self::Outline(Arc::new(OutlineElem::with_target(
            title,
            1,
            OutlineIndent::Bool(false),
            OutlineTarget::Figures,
        )))
    }

    /// **P472** — List of Tables (`lot()`).
    pub fn lot(title: Option<Content>) -> Self {
        use crate::entities::elements::outline::{OutlineIndent, OutlineTarget};
        Self::Outline(Arc::new(OutlineElem::with_target(
            title,
            1,
            OutlineIndent::Bool(false),
            OutlineTarget::Tables,
        )))
    }

    /// **Lote 8 P323** — `Content::Quote` (citação).
    pub fn quote(
        body: Content,
        attribution: Option<Content>,
        block: bool,
        quotes: bool,
    ) -> Self {
        Self::Quote(Arc::new(QuoteElem { body, attribution, block, quotes }))
    }

    /// **Passo 397** — `Content::Document` (metadata pura).
    pub fn document(
        title: Option<Content>,
        author: Vec<EcoString>,
        date: Option<Datetime>,
        keywords: Vec<EcoString>,
    ) -> Self {
        Self::Document { title: title.map(Box::new), author, date, keywords }
    }

    /// **Passo 397** — `Content::Asset` (placeholder de resource).
    pub fn asset(path: EcoString, kind: Option<EcoString>) -> Self {
        Self::Asset { path, kind }
    }

    /// `table(columns, rows, ..children)` — Passo 157A (ADR-0060
    /// Fase 2 sub-passo 1; **primeiro sub-passo Model Fase 2**).
    /// Subset minimal: cells distribuídas como `Content::Grid`;
    /// TableCell estruturado + Header/Footer diferidos para P157B/C.
    pub fn table(
        columns: Vec<TrackSizing>,
        rows: Vec<TrackSizing>,
        children: Vec<Content>,
    ) -> Self {
        Self::Table(Arc::new(TableElem {
            columns,
            rows,
            children,
            hlines: vec![],
            vlines: vec![],
            header: None,
            footer: None,
            stroke: None,
            fill: None,
            caption: None,
        }))
    }

    /// `table_with_caption(columns, rows, children, caption)` — P459.
    /// Variante programática com caption opcional para numeração automática.
    pub fn table_with_caption(
        columns: Vec<TrackSizing>,
        rows: Vec<TrackSizing>,
        children: Vec<Content>,
        caption: Option<Content>,
    ) -> Self {
        Self::Table(Arc::new(TableElem {
            columns,
            rows,
            children,
            hlines: vec![],
            vlines: vec![],
            header: None,
            footer: None,
            stroke: None,
            fill: None,
            caption,
        }))
    }

    /// `table_cell(body, x, y, colspan, rowspan)` — Passo 157B
    /// (ADR-0060 Fase 2 sub-passo 2). `x`/`y` ADR-0064 Caso A;
    /// `colspan`/`rowspan` Caso C. Placement algorítmico diferido
    /// em DEBT-34e — fields armazenados mas ignorados em layout.
    pub fn table_cell(
        body: Content,
        x: Option<usize>,
        y: Option<usize>,
        colspan: Option<usize>,
        rowspan: Option<usize>,
    ) -> Self {
        Self::TableCell(Arc::new(TableCellElem {
            body,
            x,
            y,
            colspan,
            rowspan,
            stroke: None,
            fill: None,
            align: None,
            inset: None,
            breakable: None,
        }))
    }

    /// `table_header(body, repeat)` — Passo 157C (ADR-0060 Fase 2
    /// sub-passo 3 — **fecha table foundations**). `repeat: bool`
    /// ADR-0064 Caso D (default `true` paridade vanilla — primeira
    /// aplicação Caso D em Model). Algoritmo de repetição diferido
    /// em DEBT-56.
    pub fn table_header(body: Content, repeat: bool) -> Self {
        Self::TableHeader(Arc::new(TableHeaderElem { body, repeat }))
    }

    /// `table_footer(body, repeat)` — par simétrico de `table_header`
    /// (Passo 157C). Mesma decisão Caso D + DEBT-56.
    pub fn table_footer(body: Content, repeat: bool) -> Self {
        Self::TableFooter(Arc::new(TableFooterElem { body, repeat }))
    }
    /// `table_hline(start, end, row, stroke, position)` — Passo 512.
    pub fn table_hline(
        start: usize,
        end: Option<usize>,
        row: usize,
        stroke: Option<Stroke>,
        position: EcoString,
    ) -> Self {
        Self::TableHLine(Arc::new(TableHLineElem { start, end, row, stroke, position }))
    }
    /// `table_vline(start, end, col, stroke, position)` — Passo 512.
    pub fn table_vline(
        start: usize,
        end: Option<usize>,
        col: usize,
        stroke: Option<Stroke>,
        position: EcoString,
    ) -> Self {
        Self::TableVLine(Arc::new(TableVLineElem { start, end, col, stroke, position }))
    }

    // ── Passo 513 — curve elements ────────────────────────────────────────────
    /// `curve.move(point)`.
    pub fn curve_move(x: Length, y: Length) -> Self {
        Self::Curve(Arc::new(crate::entities::elements::curve::CurveElem {
            segments: vec![crate::entities::elements::curve::CurveSegment::Move(
                crate::entities::elements::curve::CurvePoint { x, y },
            )],
        }))
    }

    /// `curve.line(point)`.
    pub fn curve_line(x: Length, y: Length) -> Self {
        Self::Curve(Arc::new(crate::entities::elements::curve::CurveElem {
            segments: vec![crate::entities::elements::curve::CurveSegment::Line(
                crate::entities::elements::curve::CurvePoint { x, y },
            )],
        }))
    }

    /// `curve.cubic(control1, control2, end)`.
    pub fn curve_cubic(
        c1x: Length,
        c1y: Length,
        c2x: Length,
        c2y: Length,
        ex: Length,
        ey: Length,
    ) -> Self {
        Self::Curve(Arc::new(crate::entities::elements::curve::CurveElem {
            segments: vec![crate::entities::elements::curve::CurveSegment::Cubic(
                crate::entities::elements::curve::CurvePoint { x: c1x, y: c1y },
                crate::entities::elements::curve::CurvePoint { x: c2x, y: c2y },
                crate::entities::elements::curve::CurvePoint { x: ex, y: ey },
            )],
        }))
    }

    /// `curve.quad(control, end)`.
    pub fn curve_quad(cx: Length, cy: Length, ex: Length, ey: Length) -> Self {
        Self::Curve(Arc::new(crate::entities::elements::curve::CurveElem {
            segments: vec![crate::entities::elements::curve::CurveSegment::Quad(
                crate::entities::elements::curve::CurvePoint { x: cx, y: cy },
                crate::entities::elements::curve::CurvePoint { x: ex, y: ey },
            )],
        }))
    }

    /// `curve.close()`.
    pub fn curve_close() -> Self {
        Self::Curve(Arc::new(crate::entities::elements::curve::CurveElem {
            segments: vec![crate::entities::elements::curve::CurveSegment::Close],
        }))
    }

    /// `bibliography(entries, title)` — Passo 159A (par acoplado
    /// com `cite`). Subset minimal per ADR-0054 graded; input
    /// cristalino literal `Vec<BibEntry>`; sem hayagriva.
    pub fn bibliography(
        entries: Vec<crate::entities::bib_entry::BibEntry>,
        title: Option<Content>,
    ) -> Self {
        Self::Bibliography(Arc::new(BibliographyElem {
            entries,
            path: None,
            title,
            style: None,
            locale: None,
        }))
    }

    /// **P418** — `bibliography(entries, title, style, locale)` com CSL.
    pub fn bibliography_with_style(
        entries: Vec<crate::entities::bib_entry::BibEntry>,
        title: Option<Content>,
        style: Option<EcoString>,
        locale: Option<EcoString>,
    ) -> Self {
        Self::Bibliography(Arc::new(BibliographyElem {
            entries,
            path: None,
            title,
            style,
            locale,
        }))
    }

    /// **P419** — `bibliography(path, title, style, locale)` carregado de disco.
    pub fn bibliography_from_path(
        path: impl Into<EcoString>,
        title: Option<Content>,
        style: Option<EcoString>,
        locale: Option<EcoString>,
    ) -> Self {
        Self::Bibliography(Arc::new(BibliographyElem {
            entries: Vec::new(),
            path: Some(path.into()),
            title,
            style,
            locale,
        }))
    }

    /// `cite(key, supplement, form, style)` — Passo 159A (par acoplado com
    /// `bibliography`) + Passo 159C (form variants) + P468 (style).
    /// Sem validação cross-reference (ADR-0017 Introspection runtime adiada).
    pub fn cite(
        key: impl Into<String>,
        supplement: Option<Content>,
        form: Option<crate::entities::citation_form::CitationForm>,
    ) -> Self {
        Self::cite_with_style(key, supplement, form, None)
    }

    /// **P468** — constructor de `Content::Cite` com estilo explícito.
    pub fn cite_with_style(
        key: impl Into<String>,
        supplement: Option<Content>,
        form: Option<crate::entities::citation_form::CitationForm>,
        style: Option<crate::entities::citation_style::CitationStyle>,
    ) -> Self {
        Self::Cite(Arc::new(CiteElem { key: key.into(), supplement, form, style }))
    }

    pub fn sequence(parts: Vec<Content>) -> Self {
        match parts.len() {
            0 => Self::Empty,
            1 => parts.into_iter().next().unwrap(),
            _ => Self::Sequence(parts.into()), // Vec<Content> → Arc<[Content]>
        }
    }

    /// **P627** — Divide o body de um `Content::Columns` sintético nos
    /// `Content::Pagebreak` que aparecem na sua `Sequence`, mesmo quando
    /// aninhados dentro de `Content::Styled`. Cada segmento (e os `Pagebreak`
    /// originais, preservados) são devolvidos como uma lista de `Content`,
    /// pronta para ser envolvida em `ColumnsElem` ou emitida directamente.
    fn page_column_segments(body: &Self) -> Vec<Self> {
        match body {
            Self::Pagebreak(_) => vec![body.clone()],
            Self::Sequence(seq) => {
                let mut pieces: Vec<Self> = Vec::new();
                let mut current: Vec<Self> = Vec::new();
                for child in seq.iter() {
                    for piece in Self::page_column_segments(child) {
                        if matches!(piece, Self::Pagebreak(_)) {
                            if !current.is_empty() {
                                pieces.push(Self::sequence(current));
                                current = Vec::new();
                            }
                            pieces.push(piece);
                        } else {
                            current.push(piece);
                        }
                    }
                }
                if !current.is_empty() {
                    pieces.push(Self::sequence(current));
                }
                pieces
            }
            Self::Styled(inner, styles) => {
                let inner_pieces = Self::page_column_segments(inner);
                inner_pieces
                    .into_iter()
                    .map(|piece| match piece {
                        Self::Pagebreak(_) => piece,
                        _ => Self::Styled(Box::new(piece), styles.clone()),
                    })
                    .collect()
            }
            _ => vec![body.clone()],
        }
    }

    /// **P537b** — Liga `#set page(columns: N)` ao consumer `Content::Columns`.
    ///
    /// Percorre a árvore de `Content` e, sempre que encontra um
    /// `Content::SetPage { columns: Some(n) }` dentro de uma `Sequence`, envolve
    /// os nós subsequentes (até outro `SetPage`, `Pagebreak` ou fim da sequência)
    /// num `Content::Columns { count: n, body: ... }`.
    ///
    /// **P627**: se o body resultante for uma `Sequence` que contém
    /// `Content::Pagebreak` ao seu nível, o body é partido em segmentos;
    /// cada segmento (e cada `Pagebreak` preservado) é emitido como elemento
    /// separado. Isto permite que secções de páginas subsequentes, com
    /// `text.dir` diferentes, obtenham a sua própria direcção de preenchimento.
    ///
    /// O `SetPage` original permanece imediatamente antes do `Columns`, para que
    /// `set_page::layout` actualize `page_config` antes de `columns::layout`
    /// consumir o body.
    ///
    /// Se uma `Sequence` não contiver `SetPage { columns: Some(_) }`, é
    /// preservada exactamente como está (zero reescrita estrutural).
    pub fn wrap_page_columns(self) -> Self {
        self.map_content(&mut |content| {
            if let Self::Sequence(parts) = content {
                // Só reescrever Sequences que efectivamente têm um set-rule de
                // colunas. Isto evita alterar a estrutura AST de documentos que
                // não usam `#set page(columns:)` (preserva snapshots P307b).
                let needs_wrap = parts
                    .iter()
                    .any(|p| matches!(p, Self::SetPage { columns: Some(_), .. }));
                if !needs_wrap {
                    return Ok(None);
                }

                let parts_vec: Vec<Self> = parts.iter().cloned().collect();
                let mut out = Vec::with_capacity(parts_vec.len());
                let mut i = 0;
                while i < parts_vec.len() {
                    if let Self::SetPage { columns: Some(count), .. } = &parts_vec[i] {
                        let count = *count;
                        // Encontrar a fronteira do body implícito: próximo SetPage
                        // ou Pagebreak termina o grupo deste set-rule.
                        let mut j = i + 1;
                        while j < parts_vec.len()
                            && !matches!(
                                &parts_vec[j],
                                Self::SetPage { .. } | Self::Pagebreak(_)
                            )
                        {
                            j += 1;
                        }
                        // Preservar o SetPage original (actualiza page_config).
                        out.push(parts_vec[i].clone());
                        // **P627** — partir o body nos Pagebreaks ao nível da
                        // Sequence, para que cada secção de página tenha o seu
                        // próprio ColumnsElem (e portanto a sua própria
                        // direcção de preenchimento).
                        let body = Self::sequence(parts_vec[i + 1..j].to_vec());
                        let segments = Self::page_column_segments(&body);
                        for piece in segments {
                            match piece {
                                Self::Pagebreak(_) => out.push(piece),
                                _ => out.push(Self::Columns(Arc::new(ColumnsElem {
                                    count,
                                    gutter: None,
                                    body: piece,
                                    page_columns: true,
                                }))),
                            }
                        }
                        i = j;
                    } else {
                        out.push(parts_vec[i].clone());
                        i += 1;
                    }
                }
                Ok(Some(Self::sequence(out)))
            } else {
                Ok(None)
            }
        })
        .unwrap_or(self)
    }

    /// Retorna `true` se este conteúdo não contém informação visível.
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Empty => true,
            // P622: Parbreak é marker estrutural — nunca vazio.
            Self::Parbreak => false,
            Self::Sequence(v) => v.is_empty(),
            Self::Label(e) => e.is_empty(),
            // Figura: não está vazia se tiver body OU caption com conteúdo.
            Self::Figure(e) => e.is_empty(),
            Self::Grid(e) => e.is_empty(),
            // P224.B — GridHeader/GridFooter vazio se body for (paridade P157C).
            // Modelo D (Lote 5 P320): grid/table header/footer delegam ao elemento.
            Self::GridHeader(e) => e.is_empty(),
            Self::GridFooter(e) => e.is_empty(),
            // P224.C — GridCell vazio se body for (paridade P157B TableCell).
            Self::GridCell(e) => e.is_empty(),
            // Passo 512 — linhas em grid/table são sempre visíveis.
            Self::GridHLine(_) | Self::GridVLine(_) => false,
            Self::TableHLine(_) | Self::TableVLine(_) => false,
            // Passo 157A (ADR-0060 Fase 2): Table é vazio se children
            // for vazio (paridade com Grid; cells / children indistintos
            // semanticamente para is_empty).
            Self::Table(e) => e.is_empty(),
            // Passo 157B (ADR-0060 Fase 2 sub-passo 2): TableCell vazio
            // se body for (atributos x/y/colspan/rowspan não tornam o
            // container não-vazio — paridade Block/Boxed).
            Self::TableCell(e) => e.is_empty(),
            // Passo 157C (ADR-0060 Fase 2 sub-passo 3): par simétrico
            // TableHeader/TableFooter vazio se body for (atributo
            // repeat não torna o container não-vazio — paridade Block/Boxed).
            Self::TableHeader(e) => e.is_empty(),
            Self::TableFooter(e) => e.is_empty(),
            // Passo 159A (ADR-0060 Fase 2 — Bibliography + Cite par
            // acoplado). Bibliography vazio se entries vazias E title
            // None. Cite nunca vazio (key sempre presente; placeholder
            // `[key]` é sempre observable).
            Self::Bibliography(e) => e.is_empty(),
            Self::Cite(e) => e.is_empty(),
            // P295 — Footnote nunca vazio (marker `[N]` é sempre observable).
            Self::Footnote(e) => e.is_empty(),
            // Passo 154B: Divider é singleton estrutural, nunca vazio.
            // Terms vazio (sem items) é considerado vazio; TermItem vazio
            // se ambos os lados forem vazios.
            Self::Divider(d) => d.is_empty(),
            // Modelo D (Lote 3 P318): Terms/TermItem delegam ao elemento.
            Self::Terms(e) => e.is_empty(),
            Self::TermItem(e) => e.is_empty(),
            // Passo 155: Quote vazio se body for vazio.
            Self::Quote(e) => e.is_empty(),
            // P397: Document/Asset são metadata/resources; não produzem
            // conteúdo observável no layout.
            Self::Document { .. } => true,
            Self::Asset { .. } => true,
            // P284: decoração vazia se o body for vazio (cosméticos não
            // criam observable se não há conteúdo).
            // Modelo D (Lote 4 P319): decorações delegam ao elemento.
            Self::Underline(e) => e.is_empty(),
            Self::Strike(e) => e.is_empty(),
            Self::Overline(e) => e.is_empty(),
            // P408: smallcaps é vazio sse o body for vazio (stub transparente).
            Self::SmallCaps { body } => body.is_empty(),
            // P287 — SmartQuote: nunca vazio (sempre emite 1 glyph).
            Self::SmartQuote(e) => e.is_empty(),
            // Passo 156C (ADR-0061 Fase 1): Pad/Hide vazios se o body for.
            Self::Pad(e) => e.is_empty(),
            Self::Hide(e) => e.is_empty(),
            // Modelo D (Lote 5 P320): espaços/breaks delegam ao elemento
            // (HSpace/VSpace = amount.is_zero(); Pagebreak/Colbreak = false).
            Self::HSpace(e) => e.is_empty(),
            Self::VSpace(e) => e.is_empty(),
            Self::Pagebreak(e) => e.is_empty(),
            Self::Colbreak(e) => e.is_empty(),
            // Passo 156G: Block é vazio se o body for (atributos de
            // dimensão/inset não fazem o container deixar de ser vazio
            // semanticamente — análogo a Pad em P156C).
            Self::Block(e) => e.is_empty(),
            // Passo 156H: Boxed (Box inline) — proxy análogo a Block.
            Self::Boxed(e) => e.is_empty(),
            // Passo 156I: Stack é vazio se TODOS os children forem vazios
            // (consistente com Sequence; stack vazio é semanticamente
            // sem conteúdo).
            Self::Stack(e) => e.is_empty(),
            // Passo 156J: Repeat é vazio se body for (atributos não
            // tornam o container não-vazio — análogo a Block/Boxed).
            Self::Repeat(e) => e.is_empty(),
            // P217: Columns é vazio se body for (count/gutter não tornam
            // não-vazio — análogo a Block/Boxed/Repeat).
            Self::Columns(e) => e.is_empty(),
            // Lote F-1 (P334): a fronteira dinâmica delega ao elemento.
            Self::Dynamic(e) => e.dyn_is_empty(),
            // Lote F-4 S1 (P338) — fecha **B2**: um nó estilizado é vazio sse o
            // body for (o estilo não adiciona observable — paridade com
            // Block/Pad/Boxed). Antes caía em `_ => false` (styled-de-vazio
            // reportava não-vazio incorretamente).
            Self::Styled(body, _) => body.is_empty(),
            // F-5b fatia 1 (P371): strong/emph vazios se o body for (como Styled).
            Self::Strong(e) => e.is_empty(),
            Self::Emph(e) => e.is_empty(),
            _ => false,
        }
    }

    /// Extrai texto plano recursivamente — para verificação em testes.
    pub fn plain_text(&self) -> String {
        match self {
            Self::Empty => String::new(),
            Self::Text(s) => s.to_string(),
            Self::Space => " ".to_string(),
            // P622: representação textual de separação de parágrafos.
            Self::Parbreak => "\n".to_string(),
            Self::Sequence(v) => v.iter().map(|c| c.plain_text()).collect(),
            // Passo 101: Content::Strong/Emph removidos — cobertos por
            // Content::Styled(body, _) => body.plain_text() no fim do match.
            // Modelo D (Lote 6 P321): família state/counter delega ao elemento (vazio).
            Self::Metadata(e) => e.plain_text(),
            Self::State(e) => e.plain_text(),
            Self::StateUpdate(e) => e.plain_text(),
            Self::StateDisplay(e) => e.plain_text(),
            Self::CounterDisplayCallback(e) => e.plain_text(),
            Self::Heading(h) => h.plain_text(),
            Self::Title(t) => t.plain_text(),
            // F-5b fatia 1 (P371): strong/emph transparentes ao plain_text (só body).
            Self::Strong(e) => e.plain_text(),
            Self::Emph(e) => e.plain_text(),
            Self::Raw(e) => e.plain_text(),
            // Modelo D (Lote 3 P318): família lista/termos delega ao elemento.
            Self::ListItem(e) => e.plain_text(),
            Self::EnumItem(e) => e.plain_text(),
            Self::Link(e) => e.plain_text(),
            // Modelo D (Lote 4 P319): decorações delegam ao elemento
            // (transparente — só body; cosméticos não afetam plain_text).
            Self::Underline(e) => e.plain_text(),
            Self::Strike(e) => e.plain_text(),
            Self::Overline(e) => e.plain_text(),
            // P408: smallcaps é transparente para plain_text (stub).
            Self::SmallCaps { body } => body.plain_text(),
            // P287 — SmartQuote: paridade vanilla `PlainText for
            // Packed<SmartQuoteElem>` — emite fallback ASCII (`"` ou `'`).
            // Layouter resolve lang-aware (consumer pós-P287); plain_text
            // é vista textual sem contexto lang.
            Self::SmartQuote(e) => e.plain_text(),
            Self::Equation(e) => e.plain_text(),
            Self::MathSequence(nodes) => nodes.iter().map(|n| n.plain_text()).collect(),
            Self::MathIdent(s) => s.to_string(),
            Self::MathText(s) => s.to_string(),
            // Modelo D (Lote 2 P317): família math delega ao elemento.
            Self::MathFrac(e) => e.plain_text(),
            Self::MathAttach(e) => e.plain_text(),
            Self::MathRoot(e) => e.plain_text(),
            Self::MathDelimited(e) => e.plain_text(),
            Self::MathAlignPoint(e) => e.plain_text(),
            Self::Linebreak(e) => e.plain_text(),
            Self::MathMatrix(e) => e.plain_text(),
            Self::MathCases(e) => e.plain_text(),
            Self::MathAccent(e) => e.plain_text(),
            Self::MathCancel(e) => e.plain_text(),
            Self::MathClassOverride(e) => e.plain_text(),
            Self::MathUnderover(e) => e.plain_text(),
            Self::MathOp(e) => e.plain_text(),
            // P311b.2 — MathStyled é transparente para plain_text (wraps body).
            Self::MathStyled(m) => m.plain_text(),
            Self::Label(e) => e.plain_text(),
            Self::Ref(e) => e.plain_text(),
            Self::CounterDisplay(e) => e.plain_text(),
            Self::CounterUpdate(e) => e.plain_text(),
            Self::Outline(e) => e.plain_text(),
            Self::Figure(e) => e.plain_text(),
            Self::Image(e) => e.plain_text(),
            Self::Shape(e) => e.plain_text(),
            Self::Curve(e) => e.plain_text(),
            Self::Transform(e) => e.plain_text(),
            Self::Grid(e) => e.plain_text(),
            // P224.B — GridHeader/GridFooter transparentes (paridade P157C).
            Self::GridHeader(e) => e.plain_text(),
            Self::GridFooter(e) => e.plain_text(),
            // P224.C — GridCell transparente (paridade P157B TableCell).
            Self::GridCell(e) => e.plain_text(),
            // Passo 512 — linhas em grid/table não contribuem para texto plano.
            Self::GridHLine(_) | Self::GridVLine(_) => String::new(),
            Self::TableHLine(_) | Self::TableVLine(_) => String::new(),
            // Passo 157A: Table concatena children com space (paridade
            // com Grid em plain_text — semântica de "células visíveis
            // em sequência").
            Self::Table(e) => e.plain_text(),
            // Passo 157B: TableCell é transparente para texto plano —
            // recurse no body sem multiplicar por colspan/rowspan
            // (paridade não visível em texto plano; spans são
            // runtime-only e diferidos em DEBT-34e).
            Self::TableCell(e) => e.plain_text(),
            // Passo 157C: par simétrico TableHeader/TableFooter
            // transparente para texto plano — recurse no body sem
            // multiplicar por repeat (semântica de page-break
            // repetição não visível em texto plano; diferida em
            // DEBT-56).
            Self::TableHeader(e) => e.plain_text(),
            Self::TableFooter(e) => e.plain_text(),
            // Passo 159A: Bibliography concatena title (se Some) +
            // entries formatadas. Cite emite `"[{key}]"` placeholder
            // + supplement.
            Self::Bibliography(e) => e.plain_text(),
            Self::Cite(e) => e.plain_text(),
            // P295 — Footnote plain_text: incorporar corpo (paridade
            // semântica de plain_text para search/screen readers).
            // Marker `[N]` real é resolvido em layout-time.
            Self::Footnote(e) => e.plain_text(),
            Self::SetPage { .. } => String::new(),
            Self::Align(e) => e.plain_text(),
            Self::Place(e) => e.plain_text(),
            Self::Styled(body, _) => body.plain_text(),
            // Passo 154B: Divider é structural sem texto; Terms concatena
            // pares por linha; TermItem produz "term: description".
            Self::Divider(d) => d.plain_text(),
            Self::Terms(e) => e.plain_text(),
            Self::TermItem(e) => e.plain_text(),
            // Passo 155: Quote em texto plain usa ASCII fallback (sem
            // smart-quotes — interaction com lang só vive no layouter).
            // Com attribution: `"body" — attribution`; sem: `"body"`.
            // Passo 156C: Pad é transparente para texto plano (recurse no
            // body sem alterar texto). Hide produz string vazia (não rende).
            Self::Pad(e) => e.plain_text(),
            Self::Hide(e) => e.plain_text(),
            // Modelo D (Lote 5 P320): espaços/breaks delegam ao elemento (vazio).
            Self::HSpace(e) => e.plain_text(),
            Self::VSpace(e) => e.plain_text(),
            Self::Pagebreak(e) => e.plain_text(),
            Self::Colbreak(e) => e.plain_text(),
            // Passo 156G: Block é transparente para texto plano (recurse
            // no body; análogo a Pad em P156C).
            Self::Block(e) => e.plain_text(),
            // Passo 156H: Boxed (Box) — análogo a Block.
            Self::Boxed(e) => e.plain_text(),
            // Passo 156I: Stack concatena plain_text de children
            // (análogo a Sequence; preserva ordem).
            Self::Stack(e) => e.plain_text(),
            // Passo 156J: Repeat é transparente para texto plano —
            // recurse no body sem multiplicar (paridade não visível em
            // texto plano; semântica de repetição é runtime-only).
            Self::Repeat(e) => e.plain_text(),
            // P217: Columns transparente para texto plano (recurse no body).
            Self::Columns(e) => e.plain_text(),
            Self::Quote(e) => e.plain_text(),
            // P397: Document devolve texto plano do título; Asset não tem
            // representação textual.
            Self::Document { title, .. } => {
                title.as_ref().map_or(String::new(), |t| t.plain_text())
            }
            Self::Asset { .. } => String::new(),
            // P506: ContextBlock é terminal para plain_text (conteúdo real só
            // existe após expansão pós-introspecção).
            Self::ContextBlock(_) => String::new(),
            // Lote F-1 (P334): a fronteira dinâmica delega ao elemento.
            Self::Dynamic(e) => e.dyn_plain_text(),
        }
    }
}

impl PartialEq for Content {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Empty, Self::Empty) => true,
            (Self::Text(a), Self::Text(b)) => a == b,
            (Self::Space, Self::Space) => true,
            (Self::Parbreak, Self::Parbreak) => true,
            (Self::Sequence(a), Self::Sequence(b)) => a.as_ref() == b.as_ref(),
            // Passo 101: Content::Strong/Emph removidos — Content::Styled cobre.
            (Self::Heading(a), Self::Heading(b)) => a == b,
            // F-5b fatia 1 (P371): strong/emph variantes próprias — `==` por tipo
            // (distinto de `Styled[Bold]` → `strong ≠ #set text`, fidelidade 0107).
            (Self::Strong(a), Self::Strong(b)) => a == b,
            (Self::Emph(a), Self::Emph(b)) => a == b,
            // Modelo D (Lote 7 P322): Raw delega ao `Arc<…Elem>`.
            (Self::Raw(a), Self::Raw(b)) => a == b,
            (Self::ListItem(a), Self::ListItem(b)) => a == b,
            (Self::EnumItem(a), Self::EnumItem(b)) => a == b,
            (Self::Link(a), Self::Link(b)) => a == b,
            // Modelo D (Lote 10 P325): Equation delega ao `Arc<…Elem>`.
            (Self::Equation(a), Self::Equation(b)) => a == b,
            (Self::MathSequence(a), Self::MathSequence(b)) => a.as_ref() == b.as_ref(),
            (Self::MathIdent(a), Self::MathIdent(b)) => a == b,
            (Self::MathText(a), Self::MathText(b)) => a == b,
            // Modelo D (Lote 2 P317): família math delega ao `Arc<…Elem>`
            // (PartialEq estrutural derivado em cada `…Elem`).
            (Self::MathFrac(a), Self::MathFrac(b)) => a == b,
            (Self::MathAttach(a), Self::MathAttach(b)) => a == b,
            (Self::MathRoot(a), Self::MathRoot(b)) => a == b,
            (Self::MathDelimited(a), Self::MathDelimited(b)) => a == b,
            (Self::MathAlignPoint(a), Self::MathAlignPoint(b)) => a == b,
            (Self::Linebreak(a), Self::Linebreak(b)) => a == b,
            (Self::MathMatrix(a), Self::MathMatrix(b)) => a == b,
            (Self::MathCases(a), Self::MathCases(b)) => a == b,
            (Self::MathAccent(a), Self::MathAccent(b)) => a == b,
            (Self::MathCancel(a), Self::MathCancel(b)) => a == b,
            (Self::MathClassOverride(a), Self::MathClassOverride(b)) => a == b,
            (Self::MathUnderover(a), Self::MathUnderover(b)) => a == b,
            (Self::MathOp(a), Self::MathOp(b)) => a == b,
            // MathStyled PartialEq estrutural (Modelo D P316: delega ao Arc<Elem>).
            (Self::MathStyled(a), Self::MathStyled(b)) => a == b,
            // P464: Label delega ao `Arc<LabelElem>` (único tipo de label).
            (Self::Label(a), Self::Label(b)) => a == b,
            (Self::Ref(a), Self::Ref(b)) => a == b,
            // Modelo D (Lote 6 P321): delegam ao `Arc<…Elem>`.
            (Self::CounterDisplay(a), Self::CounterDisplay(b)) => a == b,
            (Self::CounterUpdate(a), Self::CounterUpdate(b)) => a == b,
            (Self::Outline(a), Self::Outline(b)) => a == b,
            // Modelo D (Lote 13 P328): Figure delega ao `Arc<…Elem>`.
            (Self::Figure(a), Self::Figure(b)) => a == b,
            // Modelo D (Lote 7 P322): Image delega ao `Arc<…Elem>`.
            (Self::Image(a), Self::Image(b)) => a == b,
            // Modelo D (Lote 11 P326): Shape delega ao `Arc<…Elem>`.
            (Self::Shape(a), Self::Shape(b)) => a == b,
            // Passo 513: Curve delega ao `Arc<CurveElem>`.
            (Self::Curve(a), Self::Curve(b)) => a == b,
            // Modelo D (Lote 9 P324): Transform delega ao `Arc<…Elem>`.
            (Self::Transform(a), Self::Transform(b)) => a == b,
            // P224+P227+P228 — Grid refino +7 fields (gutter/align/inset/header/footer/stroke/fill).
            // Modelo D (Lote 12 P327): Grid delega ao `Arc<…Elem>`.
            (Self::Grid(a), Self::Grid(b)) => a == b,
            // P224.B — GridHeader / GridFooter (paridade P157C literal).
            // Modelo D (Lote 5 P320): delegam ao `Arc<…Elem>`.
            (Self::GridHeader(a), Self::GridHeader(b)) => a == b,
            (Self::GridFooter(a), Self::GridFooter(b)) => a == b,
            // P224.C + P230 + P235 — GridCell +5 fields cumulativos.
            // Modelo D (Lote 12 P327): GridCell delega ao `Arc<…Elem>`.
            (Self::GridCell(a), Self::GridCell(b)) => a == b,
            // Passo 512 — linhas em grid/table.
            (Self::GridHLine(a), Self::GridHLine(b)) => a == b,
            (Self::GridVLine(a), Self::GridVLine(b)) => a == b,
            (Self::TableHLine(a), Self::TableHLine(b)) => a == b,
            (Self::TableVLine(a), Self::TableVLine(b)) => a == b,
            // Modelo D (Lote 12 P327): Table delega ao `Arc<…Elem>`.
            (Self::Table(a), Self::Table(b)) => a == b,
            // Passo 157B + P230 + P235 — TableCell +5 fields cumulativos.
            // Modelo D (Lote 12 P327): TableCell delega ao `Arc<…Elem>`.
            (Self::TableCell(a), Self::TableCell(b)) => a == b,
            // Modelo D (Lote 5 P320): par simétrico TableHeader/TableFooter.
            (Self::TableHeader(a), Self::TableHeader(b)) => a == b,
            (Self::TableFooter(a), Self::TableFooter(b)) => a == b,
            // Passo 159A — par acoplado Bibliography + Cite.
            // Modelo D (Lote 10 P325): Bibliography delega ao `Arc<…Elem>`.
            (Self::Bibliography(a), Self::Bibliography(b)) => a == b,
            // Modelo D (Lote 9 P324): Cite delega ao `Arc<…Elem>`.
            (Self::Cite(a), Self::Cite(b)) => a == b,
            // P295 — Footnote PartialEq: body == body.
            // Modelo D (Lote 11 P326): Footnote delega ao `Arc<…Elem>`.
            (Self::Footnote(a), Self::Footnote(b)) => a == b,
            (
                Self::SetPage {
                    width: wa,
                    height: ha,
                    margin: ma,
                    numbering: na,
                    columns: ca,
                },
                Self::SetPage {
                    width: wb,
                    height: hb,
                    margin: mb,
                    numbering: nb,
                    columns: cb,
                },
            ) => wa == wb && ha == hb && ma == mb && na == nb && ca == cb,
            // Modelo D (Lote 7 P322): Align delega ao `Arc<…Elem>`.
            (Self::Align(a), Self::Align(b)) => a == b,
            // Modelo D (Lote 9 P324): Place delega ao `Arc<…Elem>`.
            (Self::Place(a), Self::Place(b)) => a == b,
            (Self::Styled(ba, sa), Self::Styled(bb, sb)) => ba == bb && sa == sb,
            // Passo 154B — terms + divider.
            (Self::Divider(a), Self::Divider(b)) => a == b,
            (Self::Terms(a), Self::Terms(b)) => a == b,
            (Self::TermItem(a), Self::TermItem(b)) => a == b,
            // Modelo D (Lote 8 P323): Quote delega ao `Arc<…Elem>`.
            (Self::Quote(a), Self::Quote(b)) => a == b,
            // P397: Document/Asset comparam estruturalmente por campos.
            (
                Self::Document { title: a, author: b, date: c, keywords: d },
                Self::Document { title: e, author: f, date: g, keywords: h },
            ) => a == e && b == f && c == g && d == h,
            (Self::Asset { path: a, kind: b }, Self::Asset { path: c, kind: d }) => {
                a == c && b == d
            }
            // Modelo D (Lote 4 P319): decorações delegam ao `Arc<…Elem>`.
            (Self::Underline(a), Self::Underline(b)) => a == b,
            (Self::Strike(a), Self::Strike(b)) => a == b,
            (Self::Overline(a), Self::Overline(b)) => a == b,
            // P408: smallcaps compara pelo body.
            (Self::SmallCaps { body: a }, Self::SmallCaps { body: b }) => a == b,
            // Modelo D (Lote 9 P324): SmartQuote delega ao `Arc<…Elem>`.
            (Self::SmartQuote(a), Self::SmartQuote(b)) => a == b,
            // Passo 156C / 156L — Pad / Hide.
            // Modelo D (Lote 10 P325): Pad delega ao `Arc<…Elem>`.
            (Self::Pad(a), Self::Pad(b)) => a == b,
            (Self::Hide(a), Self::Hide(b)) => a == b,
            // Modelo D (Lote 5 P320): espaços/breaks delegam ao `Arc<…Elem>`.
            (Self::HSpace(a), Self::HSpace(b)) => a == b,
            (Self::VSpace(a), Self::VSpace(b)) => a == b,
            (Self::Pagebreak(a), Self::Pagebreak(b)) => a == b,
            (Self::Colbreak(a), Self::Colbreak(b)) => a == b,
            // Passo 156G + P231 + P247 + P250 — Block +9 cosméticos
            // (outset/radius/clip/fill/stroke/spacing/above/below/sticky).
            // Modelo D (Lote 15 P330): Block delega ao `Arc<…Elem>`.
            (Self::Block(a), Self::Block(b)) => a == b,
            // Passo 156H + P231 + P247 — Boxed +5 cosméticos paralelo Block.
            // Modelo D (Lote 14 P329): Boxed delega ao `Arc<…Elem>`.
            (Self::Boxed(a), Self::Boxed(b)) => a == b,
            // Modelo D (Lote 9 P324): Stack delega ao `Arc<…Elem>`.
            (Self::Stack(a), Self::Stack(b)) => a == b,
            // Modelo D (Lote 7 P322): Repeat delega ao `Arc<…Elem>`.
            (Self::Repeat(a), Self::Repeat(b)) => a == b,
            // Modelo D (Lote 8 P323): Columns delega ao `Arc<…Elem>`.
            (Self::Columns(a), Self::Columns(b)) => a == b,
            // Modelo D (Lote 6 P321): StateDisplay/CounterDisplayCallback delegam.
            (Self::StateDisplay(a), Self::StateDisplay(b)) => a == b,
            (Self::CounterDisplayCallback(a), Self::CounterDisplayCallback(b)) => a == b,
            // Quirk pré-existente preservado (Lote 6 P321): Metadata/State/
            // StateUpdate NÃO têm arm — caem em `_ => false` (sempre desiguais).
            // Lote F-1 (P334): a fronteira dinâmica compara por downcast
            // estrutural (`dyn_eq`); kinds diferentes ⇒ `false`.
            (Self::Dynamic(a), Self::Dynamic(b)) => a.dyn_eq(b.as_ref()),
            _ => false,
        }
    }
}

// P204B (M8): impl Hash via hash_content (existing Debug-based hash
// function from P162). Necessária para `#[comemo::track]` no trait
// `Introspector` per ADR-0073 — métodos como `headings_for_toc` que
// retornam `&[(Label, Option<String>, Content, usize)]` exigem `Content: Hash`.
// Estratégia: delega ao hash_content u128 que já existe, hashing-o
// como tuple no hasher genérico.
impl std::hash::Hash for Content {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        crate::entities::content_hash::hash_content(self).hash(state);
    }
}

impl Content {
    /// Acesso a campos de elementos estruturados — usado pelas show rules (Passo 68).
    ///
    /// Ex: `it.body` onde `it` é um `Content::Heading` retorna `Some(Value::Content(body))`.
    /// Retorna `None` para campos inexistentes ou variantes sem campos nomeados.
    pub fn get_field(&self, field: &str) -> Option<crate::entities::value::Value> {
        use crate::entities::value::Value;
        match (self, field) {
            // Modelo D (P316): Heading delega ao elemento.
            (Content::Heading(h), f) => h.get_field(f),
            // P765a: Title delega ao elemento.
            (Content::Title(t), f) => t.get_field(f),
            // F-5b fatia 1 (P371): strong/emph delegam (ex.: `it.body`).
            (Content::Strong(e), f) => e.get_field(f),
            (Content::Emph(e), f) => e.get_field(f),
            (Content::Figure(e), "body") => Some(Value::Content(e.body.clone())),
            // P408: smallcaps expõe `body` para show rules (`it.body`).
            (Content::SmallCaps { body }, "body") => {
                Some(Value::Content(body.as_ref().clone()))
            }
            // Lote F-1 (P334): leitura de campos da fronteira dinâmica (S7) —
            // o que o closure de `#show` usará (F-2+).
            (Content::Dynamic(e), f) => e.dyn_get_field(f),
            // **P844** (achado #47 de P831) — `query(<meta>).first().value`
            // (paridade vanilla `MetadataElem.value`).
            (Content::Metadata(e), "value") => Some(e.value.as_ref().clone()),
            _ => None,
        }
    }

    /// Percorre a árvore bottom-up, aplicando `transform` a cada nó após processar os filhos.
    ///
    /// `transform` retorna `Some(new)` → substituir (sem reentrada no novo nó).
    /// `transform` retorna `None` → manter o nó processado (com filhos já transformados).
    ///
    /// O `match` lista explicitamente todos os containers e terminais — sem `_ =>`.
    /// Containers com `Box<Content>` ou `Vec<Content>` recursam; terminais clonam directamente.
    pub fn map_content<F>(
        &self,
        transform: &mut F,
    ) -> crate::entities::source_result::SourceResult<Self>
    where
        F: FnMut(
            &Content,
        )
            -> crate::entities::source_result::SourceResult<Option<Content>>,
    {
        // Passo 1: processar os filhos (bottom-up) para obter o nó com filhos transformados.
        let processed = match self {
            // ── Containers: propagar recursivamente ─────────────────────────
            Content::Sequence(seq) => {
                let new_seq: crate::entities::source_result::SourceResult<Vec<Content>> =
                    seq.iter().map(|c| c.map_content(transform)).collect();
                Content::Sequence(Arc::from(new_seq?))
            },
            // Passo 101: Content::Strong/Emph removidos — cobertos pelo
            // arm Content::Styled abaixo (que já propaga transform recursivamente).
            // Modelo D (P316): Heading delega ao elemento.
            Content::Heading(h) => h.map_content(transform)?,
            Content::Title(t) => t.map_content(transform)?,
            Content::Strong(e) => e.map_content(transform)?,
            Content::Emph(e)   => e.map_content(transform)?,
            // Modelo D (Lote 3 P318): família lista/termos delega ao elemento.
            Content::ListItem(e) => e.map_content(transform)?,
            Content::EnumItem(e) => e.map_content(transform)?,
            Content::Link(e)     => e.map_content(transform)?,
            Content::Label(e)    => e.map_content(transform)?,
            // Modelo D (Lote 13 P328): Figure container delega ao elemento.
            Content::Figure(e) => e.map_content(transform)?,
            // Content::Equation tem body: Box<Content> → container.
            // Modelo D (Lote 10 P325): Equation container delega ao elemento.
            Content::Equation(e) => e.map_content(transform)?,
            Content::MathSequence(seq) => {
                let new_seq: crate::entities::source_result::SourceResult<Vec<Content>> =
                    seq.iter().map(|c| c.map_content(transform)).collect();
                Content::MathSequence(Arc::from(new_seq?))
            },
            // Modelo D (Lote 2 P317): família math delega ao elemento
            // (cada `…Elem::map_content` recurse nos filhos e re-embrulha).
            Content::MathFrac(e)      => e.map_content(transform)?,
            Content::MathAttach(e)    => e.map_content(transform)?,
            Content::MathRoot(e)      => e.map_content(transform)?,
            Content::MathDelimited(e) => e.map_content(transform)?,
            Content::MathMatrix(e)    => e.map_content(transform)?,
            Content::MathCases(e)     => e.map_content(transform)?,
            Content::MathAccent(e)    => e.map_content(transform)?,
            Content::MathCancel(e)    => e.map_content(transform)?,
            Content::MathClassOverride(e) => e.map_content(transform)?,
            Content::MathUnderover(e) => e.map_content(transform)?,
            Content::MathOp(e)        => e.map_content(transform)?,
            Content::MathAlignPoint(e) => e.map_content(transform)?,
            // P311b.2 — MathStyled map_content recursivo (Modelo D P316: delega).
            Content::MathStyled(m) => m.map_content(transform)?,

            // Modelo D (Lote 3 P318): Terms/TermItem delegam ao elemento.
            Content::Terms(e)    => e.map_content(transform)?,
            Content::TermItem(e) => e.map_content(transform)?,

            // Modelo D (Lote 8 P323): Quote container delega ao elemento.
            Content::Quote(e) => e.map_content(transform)?,

            // P284 — text decoration containers — recurse em body;
            // atributos cosméticos são Copy primitivos.
            // Modelo D (Lote 4 P319): decorações delegam ao elemento.
            Content::Underline(e) => e.map_content(transform)?,
            Content::Strike(e)    => e.map_content(transform)?,
            Content::Overline(e)  => e.map_content(transform)?,

            // P408: smallcaps container — recurse em body (stub transparente).
            Content::SmallCaps { body } => Content::SmallCaps {
                body: Box::new(body.map_content(transform)?),
            },

            // Passo 156C / 156L: Pad / Hide containers — recurse em body;
            // sides é Copy primitivo (Sides<Option<Length>>).
            // Modelo D (Lote 10 P325): Pad container delega ao elemento.
            Content::Pad(e) => e.map_content(transform)?,
            // Modelo D (Lote 7 P322): Hide delega ao elemento.
            Content::Hide(e) => e.map_content(transform)?,

            // Passo 156G + P231 + P247 + P250: Block container — recurse em
            // body; preserva 9 cosméticos (Sides/Corners/bool/Color Copy;
            // Stroke Clone; Option<Length> Copy; bool Copy).
            Content::Block(e) => e.map_content(transform)?,

            // Passo 156H + P231 + P247: Boxed (Box inline) — recurse análogo a Block; preserva 5 cosméticos.
            Content::Boxed(e) => e.map_content(transform)?,

            // Modelo D (Lote 9 P324): Stack container delega ao elemento.
            Content::Stack(e) => e.map_content(transform)?,

            // Passo 156J: Repeat container — recurse em body; gap e
            // justify são Copy primitivos (Option<Length>, bool).
            Content::Repeat(e) => e.map_content(transform)?,

            // Modelo D (Lote 8 P323): Columns container delega ao elemento.
            Content::Columns(e) => e.map_content(transform)?,

            // ── Terminais: clonar directamente ──────────────────────────────
            // Listados explicitamente — variantes novas não passam em silêncio.
            // Passo 156D: HSpace/VSpace são leaves (sem body), terminais.
            // Passo 156E: Pagebreak é leaf (event sem body), terminal.
            // P622: Parbreak é leaf estrutural — terminal.
            Content::Text(_)
            | Content::Space
            | Content::Parbreak
            | Content::Empty
            | Content::Linebreak(_)
            | Content::Outline(_)
            | Content::Raw(_)
            | Content::Ref(_)
            | Content::SetPage { .. }
            | Content::CounterUpdate(_)
            | Content::CounterDisplay(_)
            | Content::MathIdent(_)
            | Content::MathText(_)
            | Content::Image(_)
            | Content::Divider(_)
            // P287 — SmartQuote leaf (sem body — terminal).
            | Content::SmartQuote(_)
            | Content::HSpace(_)
            | Content::VSpace(_)
            | Content::Pagebreak(_)
            // P220: Colbreak é leaf (event sem body), terminal.
            | Content::Colbreak(_)
            | Content::Shape(_)
            // Passo 513: Curve é leaf — terminal.
            | Content::Curve(_)
            // P169 (M9): Metadata é terminal — clonar directamente.
            | Content::Metadata(_)
            // P171 (M9): State e StateUpdate são terminais.
            | Content::State(_)
            | Content::StateUpdate(_)
            // P240 (M9d/M7+1): StateDisplay é terminal (callback opcional
            // mas não atravessa Content; resolvido pós-fixpoint).
            | Content::StateDisplay(_)
            // P241 (M9d/M7+2): CounterDisplayCallback terminal paralelo
            // StateDisplay; resolvido pós-fixpoint via apply_counter_displays.
            | Content::CounterDisplayCallback(_) => self.clone(),
            // Modelo D (Lote 9 P324): Transform container delega ao elemento.
            Content::Transform(e) => e.map_content(transform)?,
            // P224+P227+P228 — Grid refino +7 fields (gutter/align/inset/header/footer/stroke/fill).
            // Modelo D (Lote 12 P327): Grid container delega ao elemento.
            Content::Grid(e) => e.map_content(transform)?,
            // Modelo D (Lote 5 P320): GridHeader/GridFooter delegam ao elemento.
            Content::GridHeader(e) => e.map_content(transform)?,
            Content::GridFooter(e) => e.map_content(transform)?,
            // Modelo D (Lote 12 P327): GridCell container delega ao elemento.
            Content::GridCell(e) => e.map_content(transform)?,
            // Passo 512 — linhas em grid/table são terminais.
            Content::GridHLine(_) | Content::GridVLine(_) => self.clone(),
            Content::TableHLine(_) | Content::TableVLine(_) => self.clone(),
            // Passo 157A + P227 + P228: Table — mapear children; preservar stroke + fill.
            // Modelo D (Lote 12 P327): Table container delega ao elemento.
            Content::Table(e) => e.map_content(transform)?,
            // Modelo D (Lote 12 P327): TableCell container delega ao elemento.
            Content::TableCell(e) => e.map_content(transform)?,
            // Modelo D (Lote 5 P320): TableHeader/TableFooter delegam ao elemento.
            Content::TableHeader(e) => e.map_content(transform)?,
            Content::TableFooter(e) => e.map_content(transform)?,
            // Passo 159A: Bibliography recurse em title; preserva
            // entries (BibEntry é dados puros, sem Content recursivo).
            // Cite recurse em supplement; preserva key.
            // Modelo D (Lote 10 P325): Bibliography delega ao elemento (recurse title).
            Content::Bibliography(e) => e.map_content(transform)?,
            // Modelo D (Lote 9 P324): Cite delega ao elemento (recurse supplement).
            Content::Cite(e) => e.map_content(transform)?,
            // Modelo D (Lote 11 P326): Footnote container delega ao elemento.
            Content::Footnote(e) => e.map_content(transform)?,
            Content::Align(e) => e.map_content(transform)?,
            // Modelo D (Lote 9 P324): Place container delega ao elemento.
            Content::Place(e) => e.map_content(transform)?,
            Content::Styled(body, styles) => Content::Styled(
                Box::new(body.map_content(transform)?),
                styles.clone(),
            ),
            // P397: Document recursa no título (único Content aninhado);
            // Asset é terminal (sem Content aninhado).
            Content::Document { title, author, date, keywords } => Content::Document {
                title: match title {
                    Some(body) => Some(Box::new(body.map_content(transform)?)),
                    None       => None,
                },
                author: author.clone(),
                date:   *date,
                keywords: keywords.clone(),
            },
            Content::Asset { .. } => self.clone(),
            // P506: ContextBlock é terminal em map_content (o corpo é closure,
            // não content; expansão acontece pós-introspecção).
            Content::ContextBlock(_) => self.clone(),
            // Lote F-1 (P334): a fronteira dinâmica recursa nos filhos via o
            // elemento (mesmo contrato dos 65: devolve o nó com filhos
            // transformados; o hub aplica `transform` ao nó abaixo).
            Content::Dynamic(e) => e.dyn_map_content(transform)?,
        };

        // Passo 2: aplicar a transformação ao nó já processado.
        match transform(&processed)? {
            Some(new_content) => Ok(new_content),
            None => Ok(processed),
        }
    }

    /// Forma canônica para o `==` **morfológico** da linguagem (Passo 345,
    /// ADR-0107). Remove o estilo de **render** sem o apagar da árvore real:
    /// - `Content::Text`: o `TextStyle` **assado** (heading bold, `#set text`
    ///   ambiente — render, P343 #1) → `TextStyle::default()`. A morfologia do
    ///   texto é a **string**, não o estilo resolvido.
    /// - `Content::Styled` **semanticamente vazio** (só transporte `custom`,
    ///   ex.: numbering β1) → **transparente** (desce no body). O estilo
    ///   semântico (`*bold*`/`_italic_`, em campos tipados) **permanece** —
    ///   é morfologia (o `#show strong` o vê, ADR-0038).
    /// - `numbering_active`/`numbering` (heading/equation/figure — assados da
    ///   chain `#set …(numbering:)`, P343 #2/#3/#4) → neutro. Medido contra o
    ///   vanilla (P345 N1: `#set` numbering **não** entra na igualdade).
    ///
    /// Comparar duas formas canônicas com o `==` **estrutural** (`PartialEq`)
    /// dá a igualdade **morfológica**. **Não** altera o `#[derive(PartialEq)]`
    /// do Rust — são dois sistemas de propósito (ADR-0025): o derivado serve
    /// testes/coleções; este caminho serve o `==` da linguagem (`eval`).
    /// `it.body == [a]` casa por morfologia **como consequência** (Achado 2,
    /// P342), não como alvo.
    pub fn morph_canon(&self) -> Content {
        let mut transform = |node: &Content| -> crate::entities::source_result::SourceResult<
            Option<Content>,
        > {
            Ok(match node {
                Content::Text(s) => Some(Content::Text(s.clone())),
                // F-5a de-bake (P364): heading/equation numbering deixaram de
                // viver em campo assado — viajam como `custom` num
                // `Content::Styled` semanticamente vazio, já tratado
                // transparente pelo arm acima (desce no body). Os arms
                // dedicados (que zeravam `numbering_active`) tornaram-se
                // redundantes e foram removidos: o mecanismo é único (o custom
                // é render, não morfologia — P345 N1).
                Content::Styled(body, styles) if styles.is_semantically_empty() => {
                    Some((**body).clone())
                }
                _ => None,
            })
        };
        // `transform` é total (nunca devolve `Err`) → `map_content` não falha.
        self.map_content(&mut transform)
            .expect("morph_canon: transform total nunca devolve Err")
    }

    /// Aplica uma função de transformação a todos os nós `Content::Text`,
    /// preservando a estrutura da árvore (Passo 67).
    ///
    /// O uso de `&mut F` permite que a closure carregue estado entre chamadas
    /// (ex: um contador de substituições restantes), o que é necessário para
    /// que `replace(count: N)` funcione correctamente através de múltiplos nós.
    pub fn map_text<F>(&self, transform: &mut F) -> Self
    where
        F: FnMut(&str) -> String,
    {
        match self {
            // O caso alvo: aplicar a transformação (o render vive na chain, F-5b).
            Content::Text(s) => Content::Text(transform(s.as_str()).into()),

            // ── Containers com filhos (propagação recursiva) ──────────────
            // Cada variante listada explicitamente — sem `_ =>` ou `other =>`.
            Content::Sequence(seq) => {
                Content::Sequence(
                    seq.iter().map(|c| c.map_text(transform)).collect::<Vec<_>>().into()
                )
            }
            // Modelo D (P316): Heading delega ao elemento.
            Content::Heading(h) => h.map_text(transform),
            Content::Title(t) => t.map_text(transform),
            Content::Strong(e) => e.map_text(transform),
            Content::Emph(e)   => e.map_text(transform),
            // Passo 101: Content::Strong/Emph removidos — cobertos pelo
            // arm Content::Styled abaixo (map_text recursivo).
            Content::Label(e)    => e.map_text(transform),
            // Modelo D (Lote 13 P328): Figure container delega ao elemento.
            Content::Figure(e) => e.map_text(transform),
            // Modelo D (Lote 3 P318): família lista/termos delega ao elemento
            // (contentores de prosa — map_text recurse, precedente Heading).
            Content::ListItem(e) => e.map_text(transform),
            Content::EnumItem(e) => e.map_text(transform),
            Content::Link(e)     => e.map_text(transform),
            Content::Terms(e)    => e.map_text(transform),
            Content::TermItem(e) => e.map_text(transform),

            // Modelo D (Lote 8 P323): Quote container delega ao elemento.
            Content::Quote(e) => e.map_text(transform),

            // Modelo D (Lote 4 P319): decorações delegam ao elemento
            // (contentores de prosa — map_text recurse no body).
            Content::Underline(e) => e.map_text(transform),
            Content::Strike(e)    => e.map_text(transform),
            Content::Overline(e)  => e.map_text(transform),

            // P408: smallcaps container — map_text recurse no body (stub).
            Content::SmallCaps { body } => Content::SmallCaps {
                body: Box::new(body.map_text(transform)),
            },

            // Passo 156C / 156L: Pad / Hide containers — recurse em body.
            // Modelo D (Lote 10 P325): Pad container delega ao elemento.
            Content::Pad(e) => e.map_text(transform),
            // Modelo D (Lote 7 P322): Hide delega ao elemento.
            Content::Hide(e) => e.map_text(transform),

            // Passo 156G + P247 + P250: Block container — recurse em body;
            // preserva 9 cosméticos (P247 fill/stroke + P250 spacing/above/
            // below/sticky incluídos).
            Content::Block(e) => e.map_text(transform),

            // Passo 156H + P231 + P247: Boxed (Box inline) — recurse análogo a Block; preserva 5 cosméticos.
            Content::Boxed(e) => e.map_text(transform),

            // Modelo D (Lote 9 P324): Stack container delega ao elemento.
            Content::Stack(e) => e.map_text(transform),

            // Passo 156J: Repeat container — map_text no body.
            Content::Repeat(e) => e.map_text(transform),

            // Modelo D (Lote 8 P323): Columns container delega ao elemento.
            Content::Columns(e) => e.map_text(transform),

            // ── Terminais — clonar directamente ──────────────────────────
            // Nós matemáticos e estruturais sem markup Text — não contêm
            // Content::Text, portanto clonar em bloco é correcto e seguro.
            // Passo 156D: HSpace/VSpace são leaves (sem body), terminais.
            // Passo 156E: Pagebreak é leaf (event sem body), terminal.
            // P622: Parbreak é leaf estrutural — terminal.
            Content::Empty
            | Content::Space
            | Content::Parbreak
            | Content::Linebreak(_)
            | Content::Outline(_)
            | Content::Raw(_)
            | Content::Ref(_)
            | Content::SetPage { .. }
            | Content::CounterUpdate(_)
            | Content::CounterDisplay(_)
            | Content::MathIdent(_)
            | Content::MathText(_)
            // P287 — SmartQuote leaf (sem texto interno — map_text não recurse).
            | Content::SmartQuote(_)
            | Content::Equation(_)
            | Content::MathSequence(_)
            // Modelo D (Lote 2 P317): família math é terminal em map_text
            // (math structural; não desce — paralelo MathStyled/Divider). O
            // bloco clona em bloco; `…Elem::map_text` existe pelo contrato.
            | Content::MathAlignPoint(_)
            | Content::MathFrac(_)
            | Content::MathAttach(_)
            | Content::MathRoot(_)
            | Content::MathDelimited(_)
            | Content::MathMatrix(_)
            | Content::MathCases(_)
            | Content::MathAccent(_)
            | Content::MathCancel(_)
            | Content::MathClassOverride(_)
            | Content::MathUnderover(_)
            | Content::MathOp(_)
            // P311b.2 — MathStyled terminal em map_text (math structural).
            | Content::MathStyled(_)
            | Content::Image(_)
            | Content::Divider(_)
            | Content::HSpace(_)
            | Content::VSpace(_)
            | Content::Pagebreak(_)
            // P220: Colbreak é leaf (event sem body), terminal.
            | Content::Colbreak(_)
            | Content::Shape(_)
            // Passo 513: Curve é leaf — terminal.
            | Content::Curve(_)
            // P169 (M9): Metadata é terminal — clonar directamente.
            | Content::Metadata(_)
            // P171 (M9): State e StateUpdate são terminais.
            | Content::State(_)
            | Content::StateUpdate(_)
            // P240 (M9d/M7+1): StateDisplay é terminal em map_text.
            | Content::StateDisplay(_)
            // P241 (M9d/M7+2): CounterDisplayCallback terminal em map_text.
            | Content::CounterDisplayCallback(_) => self.clone(),
            // Modelo D (Lote 9 P324): Transform container delega ao elemento.
            Content::Transform(e) => e.map_text(transform),
            // P224+P227+P228 — Grid refino +7 fields (map_text).
            // Modelo D (Lote 12 P327): Grid container delega ao elemento.
            Content::Grid(e) => e.map_text(transform),
            // Modelo D (Lote 5 P320): GridHeader/GridFooter delegam ao elemento.
            Content::GridHeader(e) => e.map_text(transform),
            Content::GridFooter(e) => e.map_text(transform),
            // P224.C + P230 + P235 — GridCell recurse no body (map_text);
            // Modelo D (Lote 12 P327): GridCell container delega ao elemento.
            Content::GridCell(e) => e.map_text(transform),
            // Passo 512 — linhas em grid/table são terminais.
            Content::GridHLine(_) | Content::GridVLine(_) => self.clone(),
            Content::TableHLine(_) | Content::TableVLine(_) => self.clone(),
            // Passo 157A + P227 + P228: Table — map_text em children; preservar stroke + fill.
            // Modelo D (Lote 12 P327): Table container delega ao elemento.
            Content::Table(e) => e.map_text(transform),
            // Modelo D (Lote 12 P327): TableCell container delega ao elemento.
            Content::TableCell(e) => e.map_text(transform),
            // Modelo D (Lote 5 P320): TableHeader/TableFooter delegam ao elemento.
            Content::TableHeader(e) => e.map_text(transform),
            Content::TableFooter(e) => e.map_text(transform),
            // Passo 159A: Bibliography map_text em title; entries
            // são dados puros (String fields) — sem map_text recursivo
            // em entries (mapeamento de strings em fields entities é
            // out of scope per ADR-0033 paridade observable).
            // Modelo D (Lote 10 P325): Bibliography delega ao elemento (recurse title).
            Content::Bibliography(e) => e.map_text(transform),
            // Modelo D (Lote 9 P324): Cite delega ao elemento (recurse supplement).
            Content::Cite(e) => e.map_text(transform),
            // Modelo D (Lote 11 P326): Footnote container delega ao elemento.
            Content::Footnote(e) => e.map_text(transform),
            Content::Align(e) => e.map_text(transform),
            // Modelo D (Lote 9 P324): Place container delega ao elemento.
            Content::Place(e) => e.map_text(transform),
            Content::Styled(body, styles) => Content::Styled(
                Box::new(body.map_text(transform)),
                styles.clone(),
            ),
            // P397: Document recursa no título; Asset é terminal.
            Content::Document { title, author, date, keywords } => Content::Document {
                title: title.as_ref().map(|body| Box::new(body.map_text(transform))),
                author: author.clone(),
                date:   *date,
                keywords: keywords.clone(),
            },
            Content::Asset { .. } => self.clone(),
            // P506: ContextBlock é terminal em map_text.
            Content::ContextBlock(_) => self.clone(),
            // Lote F-1 (P334): a fronteira dinâmica delega ao elemento.
            Content::Dynamic(e) => e.dyn_map_text(transform),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_plain_text() {
        assert_eq!(Content::text("hello").plain_text(), "hello");
        assert_eq!(Content::text("").plain_text(), "");
    }

    #[test]
    fn empty_is_empty() {
        assert!(Content::empty().is_empty());
        assert_eq!(Content::empty().plain_text(), "");
    }

    #[test]
    fn space_nao_e_empty() {
        assert!(!Content::Space.is_empty());
        assert_eq!(Content::Space.plain_text(), " ");
    }

    #[test]
    fn parbreak_e_marker_estrutural() {
        assert!(!Content::Parbreak.is_empty());
        assert_eq!(Content::Parbreak.plain_text(), "\n");
        assert_eq!(Content::Parbreak, Content::Parbreak);
        assert_ne!(Content::Parbreak, Content::Space);
    }

    #[test]
    fn sequence_zero_partes_e_empty() {
        let c = Content::sequence(vec![]);
        assert!(c.is_empty());
        assert_eq!(c, Content::Empty);
    }

    #[test]
    fn sequence_uma_parte_desembrulha() {
        let c = Content::sequence(vec![Content::text("a")]);
        assert_eq!(c, Content::text("a"));
    }

    #[test]
    fn sequence_multiplas_partes() {
        let c = Content::sequence(vec![
            Content::text("a"),
            Content::Space,
            Content::text("b"),
        ]);
        assert_eq!(c.plain_text(), "a b");
        assert!(!c.is_empty());
    }

    #[test]
    fn sequence_is_empty_para_vec_vazio() {
        let c = Content::Sequence(Arc::from(Vec::<Content>::new().into_boxed_slice()));
        assert!(c.is_empty());
    }

    #[test]
    fn clone_e_partial_eq() {
        let c1 = Content::text("hello");
        let c2 = c1.clone();
        assert_eq!(c1, c2);
        assert_ne!(Content::text("a"), Content::text("b"));
        assert_ne!(Content::text("a"), Content::Space);
    }

    #[test]
    fn strong_plain_text_preservado() {
        assert_eq!(Content::strong(Content::text("bold")).plain_text(), "bold");
    }

    #[test]
    fn emph_plain_text_preservado() {
        assert_eq!(Content::emph(Content::text("em")).plain_text(), "em");
    }

    #[test]
    fn f5b_strong_distinto_de_set_text_bold() {
        // F-5b fatia 1 (P371): `strong` é variante própria, **distinta** do
        // `Styled[Bold]` que `#set text(bold)` produz → `*bold* ≠ #set text(bold)`
        // (fidelidade ADR-0107; o vanilla trata StrongElem ≠ StyledElem). Antes do
        // retorno ao modelo de variantes (colapso P101), ambos eram `Styled[Bold]`
        // e davam `==` (divergência).
        use crate::entities::style::{Style, Styles};
        let strong = Content::strong(Content::text("x"));
        let set_text_bold = Content::Styled(
            Box::new(Content::text("x")),
            Styles::from_iter([Style::bold(true)]),
        );
        assert_ne!(
            strong, set_text_bold,
            "strong ≠ #set text(bold) — distinção de tipo (0107)"
        );
        assert!(matches!(strong, Content::Strong(_)), "strong é variante própria");
        // emph idem, e strong ≠ emph (tipos distintos).
        assert_ne!(
            Content::strong(Content::text("x")),
            Content::emph(Content::text("x")),
            "strong ≠ emph (tipos distintos, como StrongElem ≠ EmphElem no vanilla)"
        );
        // Auto-igualdade preservada (o α depende disto).
        assert_eq!(
            Content::strong(Content::text("x")),
            Content::strong(Content::text("x"))
        );
    }

    #[test]
    fn heading_level_clamped() {
        assert!(
            matches!(Content::heading(0, Content::Empty), Content::Heading(h) if h.level == 1)
        );
        assert!(
            matches!(Content::heading(9, Content::Empty), Content::Heading(h) if h.level == 6)
        );
        assert!(
            matches!(Content::heading(3, Content::Empty), Content::Heading(h) if h.level == 3)
        );
    }

    #[test]
    fn heading_plain_text() {
        let h = Content::heading(1, Content::text("Title"));
        assert_eq!(h.plain_text(), "Title");
    }

    #[test]
    fn nested_sequence_plain_text() {
        let inner = Content::sequence(vec![Content::text("x"), Content::text("y")]);
        let outer = Content::sequence(vec![inner, Content::Space, Content::text("z")]);
        assert_eq!(outer.plain_text(), "xy z");
    }

    // ── Passo 23 ────────────────────────────────────────────────────────────

    #[test]
    fn raw_plain_text() {
        assert_eq!(
            Content::raw("fn main() {}", None, false).plain_text(),
            "fn main() {}"
        );
    }

    #[test]
    fn list_item_tem_bullet_em_plain_text() {
        assert!(Content::list_item(Content::text("Apple"))
            .plain_text()
            .contains("Apple"));
    }

    #[test]
    fn enum_item_com_numero() {
        let t = Content::enum_item(Some(1), Content::text("First")).plain_text();
        assert!(t.contains("1") && t.contains("First"));
    }

    #[test]
    fn link_plain_text_e_o_corpo() {
        assert_eq!(
            Content::link("https://typst.app", Content::text("Typst")).plain_text(),
            "Typst",
        );
    }

    // ── Passo 34 — variantes matemáticas ─────────────────────────────────────

    #[test]
    fn content_equation_inline_plain_text() {
        let eq = Content::equation(Content::MathIdent("x".into()), false);
        assert_eq!(eq.plain_text(), "x");
    }

    #[test]
    fn content_equation_block_plain_text() {
        let eq = Content::equation(Content::MathIdent("x".into()), true);
        assert_eq!(eq.plain_text(), "\nx\n");
    }

    #[test]
    fn content_math_frac_plain_text() {
        let frac = Content::math_frac(
            Content::MathIdent("a".into()),
            Content::MathIdent("b".into()),
        );
        assert_eq!(frac.plain_text(), "(a)/(b)");
    }

    #[test]
    fn content_math_attach_plain_text() {
        let attach = Content::math_attach(
            Content::MathIdent("x".into()),
            None,
            None,
            None,
            Some(Content::MathText("2".into())),
        );
        assert_eq!(attach.plain_text(), "x^2");
    }

    #[test]
    fn content_math_root_quadrada() {
        let root = Content::math_root(None, Content::MathIdent("x".into()));
        assert_eq!(root.plain_text(), "sqrt(x)");
    }

    #[test]
    fn content_math_root_cubica() {
        let root = Content::math_root(
            Some(Content::MathText("3".into())),
            Content::MathIdent("x".into()),
        );
        assert_eq!(root.plain_text(), "root(3, x)");
    }

    #[test]
    fn content_math_sequence_plain_text() {
        let seq = Content::MathSequence(Arc::from(
            vec![
                Content::MathIdent("x".into()),
                Content::MathText("+".into()),
                Content::MathIdent("y".into()),
            ]
            .into_boxed_slice(),
        ));
        assert_eq!(seq.plain_text(), "x+y");
    }

    #[test]
    fn content_math_partialeq() {
        let a = Content::MathIdent("x".into());
        let b = Content::MathIdent("x".into());
        let c = Content::MathIdent("y".into());
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    // ── Passo 26 — Content::Sequence com Arc (ADR-0026 revisão) ─────────────

    #[test]
    fn sequence_clone_e_o1() {
        let seq = Content::sequence(vec![
            Content::text("a"),
            Content::text("b"),
            Content::text("c"),
        ]);
        let clone = seq.clone();
        // PartialEq por conteúdo — não por ponteiro
        assert_eq!(seq, clone);
    }

    #[test]
    fn sequence_partialeq_por_conteudo() {
        let s1 = Content::sequence(vec![Content::text("hello")]);
        let s2 = Content::sequence(vec![Content::text("hello")]);
        // Dois Arc distintos com mesmo conteúdo → iguais
        assert_eq!(s1, s2);
    }

    #[test]
    fn sequence_partialeq_conteudos_diferentes() {
        let s1 = Content::sequence(vec![Content::text("a")]);
        let s2 = Content::sequence(vec![Content::text("b")]);
        assert_ne!(s1, s2);
    }

    // ── Passo 67 — map_text ───────────────────────────────────────────────────

    #[test]
    fn map_text_transforma_texto_simples() {
        let content = Content::text("hello");
        let result = content.map_text(&mut |s| s.to_uppercase());
        assert_eq!(result, Content::text("HELLO"));
    }

    #[test]
    fn map_text_desce_em_strong() {
        // F-5b fatia 1 (P371): `Content::strong(..)` produz `Content::Strong` (variante própria).
        let content = Content::strong(Content::text("hello"));
        let result = content.map_text(&mut |s| s.to_uppercase());
        assert_eq!(result, Content::strong(Content::text("HELLO")));
    }

    #[test]
    fn map_text_preserva_terminais_sem_texto() {
        let content = Content::Space;
        let result = content.map_text(&mut |s| s.to_uppercase());
        assert_eq!(result, Content::Space);
    }

    #[test]
    fn map_text_closure_com_estado_entre_nos() {
        // Validar que o estado da closure (FnMut) persiste entre nós distintos.
        let content = Content::Sequence(
            vec![
                Content::text("a"),
                Content::strong(Content::text("a")),
                Content::text("a"),
            ]
            .into(),
        );
        let mut count = 0usize;
        content.map_text(&mut |s| {
            count += 1;
            s.to_string()
        });
        assert_eq!(count, 3, "A closure deve ser chamada uma vez por nó Text");
    }

    // ── map_content (Passo 69 — DEBT-19) ─────────────────────────────────────

    #[test]
    fn map_content_substitui_heading_em_sequence() {
        let content = Content::Sequence(Arc::from(vec![
            Content::text("Antes"),
            Content::heading(1, Content::text("Titulo")),
            Content::text("Depois"),
        ]));

        let result = content
            .map_content(&mut |node| {
                if matches!(node, Content::Heading(_)) {
                    Ok(Some(Content::text("SUBSTITUIDO")))
                } else {
                    Ok(None)
                }
            })
            .unwrap();

        assert_eq!(result.plain_text(), "AntesSUBSTITUIDODepois");
    }

    #[test]
    fn map_content_bottom_up_pai_ve_filhos_transformados() {
        // F-5b fatia 1 (P371): `Content::strong` é variante própria `Content::Strong`
        // (o colapso P101 → `Styled[Bold]` foi superado). O bottom-up: o pai
        // (`Strong`) vê o filho já transformado.
        let content = Content::strong(Content::text("original"));

        let result = content
            .map_content(&mut |node| match node {
                Content::Text(s) => Ok(Some(Content::text(s.to_uppercase()))),
                Content::Strong(e) => {
                    let text = e.body.plain_text();
                    assert_eq!(
                        text, "ORIGINAL",
                        "Strong deve receber filho já transformado: {:?}",
                        text
                    );
                    Ok(None)
                }
                _ => Ok(None),
            })
            .unwrap();

        assert_eq!(result.plain_text(), "ORIGINAL");
    }

    #[test]
    fn map_content_nao_reavaliar_no_substituido() {
        let content = Content::heading(1, Content::text("X"));
        let mut call_count = 0usize;

        content
            .map_content(&mut |node| {
                if matches!(node, Content::Heading(_)) {
                    call_count += 1;
                    Ok(Some(Content::text("substituido")))
                } else {
                    Ok(None)
                }
            })
            .unwrap();

        assert_eq!(call_count, 1, "Heading deve ser processado exactamente uma vez");
    }

    // ── Passo 99 (ADR-0038): Content::Styled ─────────────────────────────

    use crate::entities::style::{Style, Styles};

    #[test]
    fn styled_plain_text_transparente() {
        let inner = Content::text("hello");
        let styles = Styles::from_iter([Style::bold(true), Style::Size(Pt(18.0))]);
        let styled = Content::Styled(Box::new(inner), styles);
        assert_eq!(styled.plain_text(), "hello");
    }

    #[test]
    fn styled_partial_eq() {
        let s1 = Content::Styled(
            Box::new(Content::text("x")),
            Styles::from_iter([Style::bold(true)]),
        );
        let s2 = Content::Styled(
            Box::new(Content::text("x")),
            Styles::from_iter([Style::bold(true)]),
        );
        let s3 = Content::Styled(
            Box::new(Content::text("x")),
            Styles::from_iter([Style::bold(false)]),
        );
        assert_eq!(s1, s2);
        assert_ne!(s1, s3);
    }

    #[test]
    fn styled_preserva_estilos_em_map_text() {
        let inner = Content::text("abc");
        let styles = Styles::from_iter([Style::italic(true)]);
        let styled = Content::Styled(Box::new(inner), styles.clone());
        let transformed = styled.map_text(&mut |s: &str| s.to_uppercase());
        match transformed {
            Content::Styled(body, st) => {
                assert_eq!(body.plain_text(), "ABC");
                assert_eq!(st, styles);
            }
            other => panic!("esperado Content::Styled, obteve {:?}", other),
        }
    }

    // ── Passo 154B (ADR-0060 Fase 1) — terms + divider ────────────────────

    #[test]
    fn divider_constructor_devolve_variant_correcto() {
        let c = Content::divider();
        assert!(matches!(c, Content::Divider(_)));
        // Divider é singleton estrutural: nunca empty.
        assert!(!c.is_empty());
    }

    #[test]
    fn divider_plain_text_devolve_vazio() {
        assert_eq!(Content::divider().plain_text(), "");
    }

    #[test]
    fn terms_constructor_devolve_variant_correcto() {
        let t = Content::terms(vec![Content::term_item(
            Content::text("a"),
            Content::text("b"),
        )]);
        assert!(matches!(t, Content::Terms(_)));
        assert!(!t.is_empty());
        // Terms vazio é considerado empty.
        assert!(Content::terms(vec![]).is_empty());
    }

    #[test]
    fn terms_plain_text_concatena_pares() {
        let t = Content::terms(vec![
            Content::term_item(Content::text("Apple"), Content::text("fruit")),
            Content::term_item(Content::text("Banana"), Content::text("yellow")),
        ]);
        assert_eq!(t.plain_text(), "Apple: fruit\nBanana: yellow");
    }

    #[test]
    fn term_item_plain_text() {
        let t = Content::term_item(Content::text("key"), Content::text("value"));
        assert_eq!(t.plain_text(), "key: value");
    }

    #[test]
    fn terms_map_text_recurse() {
        let t = Content::terms(vec![Content::term_item(
            Content::text("apple"),
            Content::text("fruit"),
        )]);
        let upper = t.map_text(&mut |s| s.to_uppercase());
        assert_eq!(upper.plain_text(), "APPLE: FRUIT");
    }

    #[test]
    fn terms_partial_eq() {
        let mk = || {
            Content::terms(vec![Content::term_item(
                Content::text("k"),
                Content::text("v"),
            )])
        };
        assert_eq!(mk(), mk());
        assert_ne!(mk(), Content::divider());
        assert_eq!(Content::divider(), Content::divider());
    }

    // ── Passo 155 (ADR-0060 Fase 1, sub-passo 2) — quote ─────────────────

    #[test]
    fn quote_constructor_devolve_variant_correcto() {
        let q = Content::quote(Content::text("hello"), None, false, true);
        assert!(matches!(q, Content::Quote(_)));
        assert!(!q.is_empty());
    }

    #[test]
    fn quote_plain_text_sem_attribution() {
        let q = Content::quote(Content::text("hello"), None, false, true);
        assert_eq!(q.plain_text(), "\"hello\"");
    }

    #[test]
    fn quote_plain_text_com_attribution() {
        let q = Content::quote(
            Content::text("Errare humanum est"),
            Some(Content::text("Seneca")),
            true,
            true,
        );
        assert_eq!(q.plain_text(), "\"Errare humanum est\" — Seneca");
    }

    #[test]
    fn quote_plain_text_quotes_false_omite_aspas() {
        let q = Content::quote(Content::text("texto"), None, false, false);
        assert_eq!(q.plain_text(), "texto");
    }

    #[test]
    fn quote_is_empty_proxy_para_body() {
        let empty = Content::quote(Content::Empty, None, false, true);
        assert!(empty.is_empty());
        let nonempty = Content::quote(Content::text("x"), None, false, true);
        assert!(!nonempty.is_empty());
    }

    #[test]
    fn quote_map_text_recurse_em_body_e_attribution() {
        let q = Content::quote(
            Content::text("hello"),
            Some(Content::text("seneca")),
            true,
            true,
        );
        let upper = q.map_text(&mut |s| s.to_uppercase());
        assert_eq!(upper.plain_text(), "\"HELLO\" — SENECA");
    }

    #[test]
    fn quote_partial_eq() {
        let mk = || Content::quote(Content::text("x"), None, false, true);
        assert_eq!(mk(), mk());
        let other =
            Content::quote(Content::text("x"), None, true /* diferente */, true);
        assert_ne!(mk(), other);
    }

    // ── Passo 284 (ADR-0054 graded) — text decoration ─────────────────────

    #[test]
    fn decoration_variants_construtores_basicos() {
        let u = Content::underline(Content::text("hi"), None, None, None);
        let s = Content::strike(Content::text("hi"), None, None, None);
        let o = Content::overline(Content::text("hi"), None, None, None);
        assert!(matches!(u, Content::Underline(_)));
        assert!(matches!(s, Content::Strike(_)));
        assert!(matches!(o, Content::Overline(_)));
        // Variants distintos não colapsam mesmo com body idêntico.
        assert_ne!(u, s);
        assert_ne!(s, o);
        assert_ne!(u, o);
    }

    #[test]
    fn decoration_plain_text_delega_no_body() {
        let mk = |body| Content::underline(body, None, None, None);
        assert_eq!(mk(Content::text("hello")).plain_text(), "hello");
        // strike/overline têm a mesma regra — paridade Quote/Link.
        let s = Content::strike(Content::text("x"), None, None, None);
        let o = Content::overline(Content::text("y"), None, None, None);
        assert_eq!(s.plain_text(), "x");
        assert_eq!(o.plain_text(), "y");
    }

    #[test]
    fn decoration_is_empty_proxy_para_body() {
        let u_empty = Content::underline(Content::Empty, None, None, None);
        let u_full = Content::underline(Content::text("a"), None, None, None);
        assert!(u_empty.is_empty());
        assert!(!u_full.is_empty());
    }

    #[test]
    fn decoration_partial_eq_distingue_cosmeticos() {
        let base = || Content::underline(Content::text("x"), None, None, None);
        assert_eq!(base(), base());
        let with_offset =
            Content::underline(Content::text("x"), None, Some(Length::pt(2.0)), None);
        assert_ne!(base(), with_offset, "offset diferente quebra igualdade");
        let with_stroke = Content::underline(
            Content::text("x"),
            Some(Color::rgb(255, 0, 0)),
            None,
            None,
        );
        assert_ne!(base(), with_stroke);
    }

    #[test]
    fn decoration_map_text_recurse_no_body() {
        let u =
            Content::underline(Content::text("hello"), None, Some(Length::pt(3.0)), None);
        let upper = u.map_text(&mut |s| s.to_uppercase());
        assert_eq!(upper.plain_text(), "HELLO");
        // Cosméticos preservados após map_text.
        if let Content::Underline(e) = upper {
            assert_eq!(e.offset, Some(Length::pt(3.0)));
        } else {
            panic!("map_text quebrou o variant kind");
        }
    }

    // ── Passo 287 — Content::SmartQuote leaf ────────────────────────────

    #[test]
    fn smartquote_variant_construtor_basico() {
        let sd = Content::smartquote(true);
        let ss = Content::smartquote(false);
        assert!(matches!(&sd, Content::SmartQuote(e) if e.double));
        assert!(matches!(&ss, Content::SmartQuote(e) if !e.double));
        // Variants distintos por valor `double`.
        assert_ne!(sd, ss);
    }

    #[test]
    fn smartquote_plain_text_ascii_paridade_vanilla() {
        // Paridade `PlainText for Packed<SmartQuoteElem>` — emite ASCII
        // fallback (Layouter resolve lang-aware via consumer; plain_text
        // é vista sem contexto).
        assert_eq!(Content::smartquote(true).plain_text(), "\"");
        assert_eq!(Content::smartquote(false).plain_text(), "'");
    }

    #[test]
    fn smartquote_is_empty_nunca_vazio() {
        // SmartQuote sempre emite 1 glyph — nunca empty.
        assert!(!Content::smartquote(true).is_empty());
        assert!(!Content::smartquote(false).is_empty());
    }

    #[test]
    fn smartquote_partial_eq() {
        let a = Content::smartquote(true);
        let b = Content::smartquote(true);
        let c = Content::smartquote(false);
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    // ── Passo 156C / 156L (ADR-0061 Fase 1 + Fase 3 refino) — pad + hide ──

    #[test]
    fn pad_constructor_envolve_body() {
        use crate::entities::layout_types::Length;
        use crate::entities::sides::Sides;
        // P156L: cada side é Option<Length>; Some(...) ↔ lado declarado.
        let p = Content::pad(Content::text("x"), Sides::uniform(Some(Length::pt(10.0))));
        if let Content::Pad(e) = &p {
            assert_eq!(e.body.plain_text(), "x");
            assert_eq!(e.sides.left, Some(Length::pt(10.0)));
            assert_eq!(e.sides.right, Some(Length::pt(10.0)));
            assert_eq!(e.sides.top, Some(Length::pt(10.0)));
            assert_eq!(e.sides.bottom, Some(Length::pt(10.0)));
        } else {
            panic!("esperado Content::Pad");
        }
    }

    #[test]
    fn hide_constructor_envolve_body() {
        let h = Content::hide(Content::text("placeholder"));
        if let Content::Hide(e) = &h {
            assert_eq!(e.body.plain_text(), "placeholder");
        } else {
            panic!("esperado Content::Hide");
        }
    }

    #[test]
    fn pad_e_hide_is_empty_proxy_para_body() {
        use crate::entities::layout_types::Length;
        use crate::entities::sides::Sides;
        // Pad/Hide com body Empty são considerados vazios.
        let pad_empty =
            Content::pad(Content::Empty, Sides::uniform(Some(Length::pt(5.0))));
        let hide_empty = Content::hide(Content::Empty);
        assert!(pad_empty.is_empty());
        assert!(hide_empty.is_empty());
        // Com body com texto, não vazios.
        let pad_text = Content::pad(Content::text("a"), Sides::uniform(None));
        let hide_text = Content::hide(Content::text("a"));
        assert!(!pad_text.is_empty());
        assert!(!hide_text.is_empty());
    }

    #[test]
    fn pad_plain_text_recurse_no_body() {
        use crate::entities::layout_types::Length;
        use crate::entities::sides::Sides;
        let p =
            Content::pad(Content::text("hello"), Sides::uniform(Some(Length::pt(2.0))));
        assert_eq!(p.plain_text(), "hello");
    }

    #[test]
    fn hide_plain_text_e_string_vazia() {
        // Hide é layout-aware mas não rende — plain_text vazio.
        let h = Content::hide(Content::text("invisivel"));
        assert_eq!(h.plain_text(), "");
    }

    #[test]
    fn pad_partial_eq() {
        use crate::entities::layout_types::Length;
        use crate::entities::sides::Sides;
        let mk =
            || Content::pad(Content::text("x"), Sides::uniform(Some(Length::pt(3.0))));
        assert_eq!(mk(), mk());
        // Padding diferente → diferente (bottom 5pt em vez de 3pt).
        let other = Content::pad(
            Content::text("x"),
            Sides::new(
                Some(Length::pt(3.0)),
                Some(Length::pt(3.0)),
                Some(Length::pt(3.0)),
                Some(Length::pt(5.0)),
            ),
        );
        assert_ne!(mk(), other);
        // P156L: distinção semântica nova — Some(zero) ≠ None.
        let some_zero =
            Content::pad(Content::text("x"), Sides::uniform(Some(Length::ZERO)));
        let none = Content::pad(Content::text("x"), Sides::uniform(None));
        assert_ne!(
            some_zero, none,
            "P156L: Some(zero) e None são semanticamente distintos"
        );
    }

    #[test]
    fn hide_partial_eq() {
        let a = Content::hide(Content::text("x"));
        let b = Content::hide(Content::text("x"));
        let c = Content::hide(Content::text("y"));
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn pad_e_hide_map_text_recurse_no_body() {
        use crate::entities::layout_types::Length;
        use crate::entities::sides::Sides;
        let pad =
            Content::pad(Content::text("hello"), Sides::uniform(Some(Length::pt(1.0))));
        let hide = Content::hide(Content::text("hello"));
        let pad_upper = pad.map_text(&mut |s| s.to_uppercase());
        let hide_upper = hide.map_text(&mut |s| s.to_uppercase());
        // Pad expõe via plain_text (recurse); Hide oculta plain_text mas
        // o body interno foi transformado — verificamos isso desembrulhando.
        assert_eq!(pad_upper.plain_text(), "HELLO");
        if let Content::Hide(e) = &hide_upper {
            assert_eq!(e.body.plain_text(), "HELLO");
        } else {
            panic!("esperado Content::Hide após map_text");
        }
    }

    // ── Passo 156D (ADR-0061 Fase 1, sub-passo 2) — h + v spacing ─────────

    #[test]
    fn hspace_constructor() {
        use crate::entities::elements::h_space::Spacing;
        use crate::entities::layout_types::Length;
        let h = Content::h_space(Length::pt(12.0), false);
        if let Content::HSpace(e) = h {
            assert_eq!(e.amount, Spacing::Absolute(Length::pt(12.0)));
            assert!(!e.weak);
        } else {
            panic!("esperado Content::HSpace");
        }
        // P842 (#38) — constructor de fração.
        let hfr = Content::h_space_fraction(1.5, true);
        if let Content::HSpace(e) = hfr {
            assert_eq!(e.amount, Spacing::Fractional(1.5));
            assert!(e.weak);
        } else {
            panic!("esperado Content::HSpace");
        }
    }

    #[test]
    fn vspace_constructor() {
        use crate::entities::layout_types::Length;
        let v = Content::v_space(Length::pt(8.0), true);
        if let Content::VSpace(e) = v {
            assert_eq!(e.amount, Length::pt(8.0));
            assert!(e.weak);
        } else {
            panic!("esperado Content::VSpace");
        }
    }

    #[test]
    fn hspace_e_vspace_is_empty_se_amount_zero() {
        use crate::entities::layout_types::Length;
        // Amount zero → considerado vazio (consistente com Sequence vazia).
        let h_zero = Content::h_space(Length::ZERO, false);
        let v_zero = Content::v_space(Length::ZERO, false);
        assert!(h_zero.is_empty());
        assert!(v_zero.is_empty());
        // Amount não-zero → não vazio.
        let h_nonzero = Content::h_space(Length::pt(1.0), false);
        let v_nonzero = Content::v_space(Length::pt(1.0), false);
        assert!(!h_nonzero.is_empty());
        assert!(!v_nonzero.is_empty());
    }

    #[test]
    fn hspace_e_vspace_plain_text_vazio() {
        use crate::entities::layout_types::Length;
        let h = Content::h_space(Length::pt(5.0), false);
        let v = Content::v_space(Length::pt(5.0), false);
        // Spacing primitives não rendem texto.
        assert_eq!(h.plain_text(), "");
        assert_eq!(v.plain_text(), "");
    }

    #[test]
    fn hspace_partial_eq() {
        use crate::entities::layout_types::Length;
        let a = Content::h_space(Length::pt(3.0), false);
        let b = Content::h_space(Length::pt(3.0), false);
        let c = Content::h_space(Length::pt(3.0), true); // weak diferente
        let d = Content::h_space(Length::pt(4.0), false); // amount diferente
        assert_eq!(a, b);
        assert_ne!(a, c);
        assert_ne!(a, d);
    }

    #[test]
    fn vspace_partial_eq() {
        use crate::entities::layout_types::Length;
        let a = Content::v_space(Length::pt(3.0), false);
        let b = Content::v_space(Length::pt(3.0), false);
        let c = Content::v_space(Length::pt(3.0), true);
        assert_eq!(a, b);
        assert_ne!(a, c);
        // VSpace e HSpace com mesmos campos NÃO são iguais (variantes
        // diferentes).
        let h = Content::h_space(Length::pt(3.0), false);
        assert_ne!(a, h);
    }

    #[test]
    fn hspace_e_vspace_map_text_preserva() {
        use crate::entities::layout_types::Length;
        // Spacing primitives são leaves — map_text não muda nada.
        let h = Content::h_space(Length::pt(7.0), false);
        let v = Content::v_space(Length::pt(7.0), true);
        let h_mapped = h.map_text(&mut |s| s.to_uppercase());
        let v_mapped = v.map_text(&mut |s| s.to_uppercase());
        assert_eq!(h, h_mapped);
        assert_eq!(v, v_mapped);
    }

    // ── Passo 156E (ADR-0061 Fase 1, sub-passo 3) — pagebreak ─────────────

    #[test]
    fn pagebreak_constructor() {
        use crate::entities::parity::Parity;
        let p = Content::pagebreak(false, None);
        if let Content::Pagebreak(e) = p {
            assert!(!e.weak);
            assert_eq!(e.to, None);
        } else {
            panic!("esperado Content::Pagebreak");
        }
        let p2 = Content::pagebreak(true, Some(Parity::Even));
        if let Content::Pagebreak(e) = p2 {
            assert!(e.weak);
            assert_eq!(e.to, Some(Parity::Even));
        } else {
            panic!("esperado Content::Pagebreak");
        }
    }

    #[test]
    fn pagebreak_is_empty_returns_false() {
        // Pagebreak é event observável mesmo "vazio" — análogo a Divider.
        let p = Content::pagebreak(false, None);
        assert!(
            !p.is_empty(),
            "Content::Pagebreak nunca é considerado vazio (event com efeito)"
        );
    }

    #[test]
    fn pagebreak_plain_text_vazio() {
        let p = Content::pagebreak(false, None);
        assert_eq!(p.plain_text(), "");
    }

    #[test]
    fn pagebreak_partial_eq() {
        use crate::entities::parity::Parity;
        let a = Content::pagebreak(false, None);
        let b = Content::pagebreak(false, None);
        let c = Content::pagebreak(true, None); // weak diferente
        let d = Content::pagebreak(false, Some(Parity::Even)); // to diferente
        let e = Content::pagebreak(false, Some(Parity::Odd)); // to diferente
        assert_eq!(a, b);
        assert_ne!(a, c);
        assert_ne!(a, d);
        assert_ne!(d, e);
    }

    #[test]
    fn pagebreak_map_text_preserva() {
        use crate::entities::parity::Parity;
        let p = Content::pagebreak(true, Some(Parity::Even));
        // Pagebreak é leaf — map_text não tem effect.
        let p_mapped = p.map_text(&mut |s| s.to_uppercase());
        assert_eq!(p, p_mapped);
    }

    // ── Passo 156G (ADR-0061 Fase 2, sub-passo 1) — block container ───────

    #[test]
    fn block_constructor_default_field_values() {
        use crate::entities::layout_types::Length;
        use crate::entities::sides::Sides;
        let b = Content::block(
            Content::text("body"),
            None,
            None,
            Sides::uniform(Length::ZERO),
            true,
        );
        if let Content::Block(e) = &b {
            assert_eq!(e.body.plain_text(), "body");
            assert_eq!(e.width, None);
            assert_eq!(e.height, None);
            assert_eq!(e.inset.left, Length::ZERO);
            assert!(e.breakable);
        } else {
            panic!("esperado Content::Block");
        }
    }

    #[test]
    fn block_with_explicit_width_height_inset() {
        use crate::entities::layout_types::Length;
        use crate::entities::sides::Sides;
        let b = Content::block(
            Content::text("x"),
            Some(Length::pt(100.0)),
            Some(Length::pt(50.0)),
            Sides::uniform(Length::pt(8.0)),
            false,
        );
        if let Content::Block(e) = &b {
            assert_eq!(e.width, Some(Length::pt(100.0)));
            assert_eq!(e.height, Some(Length::pt(50.0)));
            assert_eq!(e.inset.left, Length::pt(8.0));
            assert!(!e.breakable);
        } else {
            panic!("esperado Content::Block");
        }
    }

    #[test]
    fn block_is_empty_proxy_para_body() {
        use crate::entities::layout_types::Length;
        use crate::entities::sides::Sides;
        // Block com body Empty é vazio (atributos não-nulos não mudam isso).
        let b_empty = Content::block(
            Content::Empty,
            Some(Length::pt(100.0)),
            None,
            Sides::uniform(Length::pt(5.0)),
            true,
        );
        assert!(b_empty.is_empty());
        // Com texto, não vazio.
        let b_text = Content::block(
            Content::text("a"),
            None,
            None,
            Sides::uniform(Length::ZERO),
            true,
        );
        assert!(!b_text.is_empty());
    }

    #[test]
    fn styled_is_empty_proxy_para_body_b2() {
        // Lote F-4 S1 (P338) — fecha **B2**: um nó estilizado delega `is_empty`
        // ao body (o estilo não cria observable). Antes caía em `_ => false`
        // (styled-de-vazio reportava não-vazio).
        let styled_vazio = Content::strong(Content::Empty);
        assert!(styled_vazio.is_empty(), "strong(Empty) deve ser vazio (B2)");
        let styled_emph_vazio = Content::emph(Content::Empty);
        assert!(styled_emph_vazio.is_empty(), "emph(Empty) deve ser vazio (B2)");
        // Com conteúdo, não vazio.
        let styled_texto = Content::strong(Content::text("a"));
        assert!(!styled_texto.is_empty(), "strong(texto) não é vazio");
    }

    #[test]
    fn block_plain_text_recurse_no_body() {
        use crate::entities::layout_types::Length;
        use crate::entities::sides::Sides;
        let b = Content::block(
            Content::text("hello"),
            None,
            None,
            Sides::uniform(Length::pt(2.0)),
            true,
        );
        assert_eq!(b.plain_text(), "hello");
    }

    #[test]
    fn block_partial_eq() {
        use crate::entities::layout_types::Length;
        use crate::entities::sides::Sides;
        let mk = || {
            Content::block(
                Content::text("x"),
                Some(Length::pt(50.0)),
                None,
                Sides::uniform(Length::pt(3.0)),
                true,
            )
        };
        assert_eq!(mk(), mk());
        // Width diferente → diferente.
        let other_width = Content::block(
            Content::text("x"),
            Some(Length::pt(60.0)),
            None,
            Sides::uniform(Length::pt(3.0)),
            true,
        );
        assert_ne!(mk(), other_width);
        // breakable diferente → diferente.
        let other_breakable = Content::block(
            Content::text("x"),
            Some(Length::pt(50.0)),
            None,
            Sides::uniform(Length::pt(3.0)),
            false,
        );
        assert_ne!(mk(), other_breakable);
    }

    #[test]
    fn block_map_text_recurse_no_body() {
        use crate::entities::layout_types::Length;
        use crate::entities::sides::Sides;
        let b = Content::block(
            Content::text("hello"),
            None,
            None,
            Sides::uniform(Length::pt(1.0)),
            true,
        );
        let upper = b.map_text(&mut |s| s.to_uppercase());
        assert_eq!(upper.plain_text(), "HELLO");
        // Atributos preservados após map_text.
        if let Content::Block(e) = upper {
            assert_eq!(e.inset.left, Length::pt(1.0));
        } else {
            panic!("esperado Content::Block após map_text");
        }
    }

    // ── Passo 156H (ADR-0061 Fase 2, sub-passo 2) — box inline container ──

    #[test]
    fn boxed_constructor_default() {
        use crate::entities::layout_types::Length;
        use crate::entities::sides::Sides;
        let b = Content::boxed(
            Content::text("body"),
            None,
            None,
            Sides::uniform(Length::ZERO),
            Length::ZERO,
        );
        if let Content::Boxed(e) = &b {
            assert_eq!(e.body.plain_text(), "body");
            assert_eq!(e.width, None);
            assert_eq!(e.height, None);
            assert_eq!(e.inset.left, Length::ZERO);
            assert_eq!(e.baseline, Length::ZERO);
        } else {
            panic!("esperado Content::Boxed");
        }
    }

    #[test]
    fn boxed_constructor_explicit_atributos() {
        use crate::entities::layout_types::Length;
        use crate::entities::sides::Sides;
        let b = Content::boxed(
            Content::text("x"),
            Some(Length::pt(60.0)),
            Some(Length::pt(20.0)),
            Sides::uniform(Length::pt(2.0)),
            Length::pt(-3.0), // baseline negativo aceito
        );
        if let Content::Boxed(e) = &b {
            assert_eq!(e.width, Some(Length::pt(60.0)));
            assert_eq!(e.height, Some(Length::pt(20.0)));
            assert_eq!(e.inset.right, Length::pt(2.0));
            assert_eq!(e.baseline, Length::pt(-3.0));
        } else {
            panic!("esperado Content::Boxed");
        }
    }

    #[test]
    fn boxed_is_empty_proxy_para_body() {
        use crate::entities::layout_types::Length;
        use crate::entities::sides::Sides;
        let empty = Content::boxed(
            Content::Empty,
            Some(Length::pt(50.0)),
            None,
            Sides::uniform(Length::pt(5.0)),
            Length::ZERO,
        );
        assert!(empty.is_empty());
        let nonempty = Content::boxed(
            Content::text("a"),
            None,
            None,
            Sides::uniform(Length::ZERO),
            Length::ZERO,
        );
        assert!(!nonempty.is_empty());
    }

    #[test]
    fn boxed_plain_text_recurse_no_body() {
        use crate::entities::layout_types::Length;
        use crate::entities::sides::Sides;
        let b = Content::boxed(
            Content::text("hello"),
            None,
            None,
            Sides::uniform(Length::pt(1.0)),
            Length::ZERO,
        );
        assert_eq!(b.plain_text(), "hello");
    }

    #[test]
    fn boxed_partial_eq() {
        use crate::entities::layout_types::Length;
        use crate::entities::sides::Sides;
        let mk = || {
            Content::boxed(
                Content::text("x"),
                Some(Length::pt(40.0)),
                None,
                Sides::uniform(Length::pt(1.0)),
                Length::pt(2.0),
            )
        };
        assert_eq!(mk(), mk());
        // baseline diferente → diferente.
        let other_baseline = Content::boxed(
            Content::text("x"),
            Some(Length::pt(40.0)),
            None,
            Sides::uniform(Length::pt(1.0)),
            Length::pt(5.0),
        );
        assert_ne!(mk(), other_baseline);
    }

    #[test]
    fn boxed_map_text_recurse_no_body() {
        use crate::entities::layout_types::Length;
        use crate::entities::sides::Sides;
        let b = Content::boxed(
            Content::text("hello"),
            None,
            None,
            Sides::uniform(Length::pt(1.0)),
            Length::pt(2.0),
        );
        let upper = b.map_text(&mut |s| s.to_uppercase());
        assert_eq!(upper.plain_text(), "HELLO");
        // Atributos preservados.
        if let Content::Boxed(e) = upper {
            assert_eq!(e.baseline, Length::pt(2.0));
            assert_eq!(e.inset.left, Length::pt(1.0));
        } else {
            panic!("esperado Content::Boxed após map_text");
        }
    }

    // ── Passo 156I (ADR-0061 Fase 2, sub-passo 3) — stack compositivo ──────

    #[test]
    fn stack_constructor_default() {
        use crate::entities::dir::Dir;
        let s = Content::stack(
            vec![Content::text("a"), Content::text("b")],
            Dir::default(),
            None,
        );
        if let Content::Stack(e) = &s {
            assert_eq!(e.children.len(), 2);
            assert_eq!(e.dir, Dir::TTB);
            assert_eq!(e.spacing, None);
        } else {
            panic!("esperado Content::Stack");
        }
    }

    #[test]
    fn stack_constructor_explicit_dir_spacing() {
        use crate::entities::dir::Dir;
        use crate::entities::layout_types::Length;
        let s = Content::stack(
            vec![Content::text("x"), Content::text("y")],
            Dir::LTR,
            Some(Length::pt(5.0)),
        );
        if let Content::Stack(e) = &s {
            assert_eq!(e.dir, Dir::LTR);
            assert_eq!(e.spacing, Some(Length::pt(5.0)));
        } else {
            panic!("esperado Content::Stack");
        }
    }

    #[test]
    fn stack_is_empty_se_todos_children_vazios() {
        use crate::entities::dir::Dir;
        // Children todos Empty → stack é vazio.
        let s_empty =
            Content::stack(vec![Content::Empty, Content::Empty], Dir::TTB, None);
        assert!(s_empty.is_empty());
        // Stack sem children → também vazio.
        let s_zero = Content::stack(vec![], Dir::TTB, None);
        assert!(s_zero.is_empty());
        // Algum child com texto → não vazio.
        let s_nonempty =
            Content::stack(vec![Content::Empty, Content::text("a")], Dir::TTB, None);
        assert!(!s_nonempty.is_empty());
    }

    #[test]
    fn stack_plain_text_concatena_children() {
        use crate::entities::dir::Dir;
        let s = Content::stack(
            vec![Content::text("Hello "), Content::text("world")],
            Dir::TTB,
            None,
        );
        // Plain text concatena (consistente com Sequence).
        assert_eq!(s.plain_text(), "Hello world");
    }

    #[test]
    fn stack_partial_eq() {
        use crate::entities::dir::Dir;
        use crate::entities::layout_types::Length;
        let mk = || {
            Content::stack(
                vec![Content::text("a"), Content::text("b")],
                Dir::TTB,
                Some(Length::pt(3.0)),
            )
        };
        assert_eq!(mk(), mk());
        // Dir diferente → diferente.
        let other_dir = Content::stack(
            vec![Content::text("a"), Content::text("b")],
            Dir::LTR,
            Some(Length::pt(3.0)),
        );
        assert_ne!(mk(), other_dir);
        // Spacing diferente → diferente.
        let other_spacing =
            Content::stack(vec![Content::text("a"), Content::text("b")], Dir::TTB, None);
        assert_ne!(mk(), other_spacing);
        // Children diferentes → diferente.
        let other_children = Content::stack(
            vec![Content::text("a"), Content::text("c")],
            Dir::TTB,
            Some(Length::pt(3.0)),
        );
        assert_ne!(mk(), other_children);
    }

    #[test]
    fn stack_map_text_recurse_em_cada_child() {
        use crate::entities::dir::Dir;
        let s = Content::stack(
            vec![Content::text("hello"), Content::text("world")],
            Dir::TTB,
            None,
        );
        let upper = s.map_text(&mut |t| t.to_uppercase());
        assert_eq!(upper.plain_text(), "HELLOWORLD");
        // Atributos preservados.
        if let Content::Stack(e) = upper {
            assert_eq!(e.dir, Dir::TTB);
        } else {
            panic!("esperado Content::Stack após map_text");
        }
    }

    // ── Passo 156J (ADR-0061 Fase 3 sub-passo 1) — Repeat ────────────────

    #[test]
    fn repeat_constructor_default_gap_justify() {
        let r = Content::repeat(Content::text("."), None, true);
        if let Content::Repeat(e) = &r {
            assert_eq!(e.body.plain_text(), ".");
            assert_eq!(e.gap, None);
            assert!(e.justify);
        } else {
            panic!("esperado Content::Repeat");
        }
    }

    #[test]
    fn repeat_constructor_explicit_gap_justify_false() {
        use crate::entities::layout_types::Length;
        let r = Content::repeat(Content::text("a"), Some(Length::pt(2.0)), false);
        if let Content::Repeat(e) = &r {
            assert_eq!(e.gap, Some(Length::pt(2.0)));
            assert!(!e.justify);
        } else {
            panic!("esperado Content::Repeat");
        }
    }

    #[test]
    fn repeat_is_empty_proxy_via_body() {
        // Body Empty → repeat é vazio (atributos não tornam não-vazio).
        let r_empty = Content::repeat(Content::Empty, None, true);
        assert!(r_empty.is_empty());
        // Body com texto → não vazio.
        let r_dot = Content::repeat(Content::text("."), None, true);
        assert!(!r_dot.is_empty());
    }

    #[test]
    fn repeat_plain_text_recurse_no_body() {
        // Plain text recurse sem multiplicar — paridade não visível em
        // texto plano (semântica de repetição é runtime-only).
        let r = Content::repeat(Content::text("xy"), None, true);
        assert_eq!(r.plain_text(), "xy");
    }

    #[test]
    fn repeat_partial_eq_cobre_todos_os_fields() {
        use crate::entities::layout_types::Length;
        let mk = || Content::repeat(Content::text("."), Some(Length::pt(3.0)), true);
        assert_eq!(mk(), mk());
        // Body diferente → diferente.
        let other_body = Content::repeat(Content::text("o"), Some(Length::pt(3.0)), true);
        assert_ne!(mk(), other_body);
        // Gap diferente → diferente.
        let other_gap = Content::repeat(Content::text("."), None, true);
        assert_ne!(mk(), other_gap);
        // Justify diferente → diferente.
        let other_justify =
            Content::repeat(Content::text("."), Some(Length::pt(3.0)), false);
        assert_ne!(mk(), other_justify);
    }

    #[test]
    fn repeat_map_text_recurse_no_body() {
        use crate::entities::layout_types::Length;
        let r = Content::repeat(Content::text("hello"), Some(Length::pt(2.0)), false);
        let upper = r.map_text(&mut |t| t.to_uppercase());
        assert_eq!(upper.plain_text(), "HELLO");
        // Atributos preservados.
        if let Content::Repeat(e) = upper {
            assert_eq!(e.gap, Some(Length::pt(2.0)));
            assert!(!e.justify);
        } else {
            panic!("esperado Content::Repeat após map_text");
        }
    }

    // ── P217 (DEBT-56 sub-fase b primeiro sub-passo) — Columns ──────

    #[test]
    fn p217_columns_variant_existe() {
        use crate::entities::layout_types::Length;
        let c = Content::columns(Content::text("hello"), 2, Some(Length::pt(10.0)));
        if let Content::Columns(e) = &c {
            assert_eq!(e.count, 2);
            assert_eq!(e.gutter, Some(Length::pt(10.0)));
            assert_eq!(e.body.plain_text(), "hello");
        } else {
            panic!("esperado Content::Columns");
        }
    }

    #[test]
    fn p217_columns_plain_text_recurse() {
        // Stub transparente — plain_text recurse no body.
        let c = Content::columns(Content::text("multi"), 3, None);
        assert_eq!(c.plain_text(), "multi");
    }

    #[test]
    fn p217_columns_is_empty_proxy() {
        // Body Empty → columns vazio (atributos não tornam não-vazio).
        let c_empty = Content::columns(Content::Empty, 2, None);
        assert!(c_empty.is_empty());
        // Body com texto → não vazio.
        let c_text = Content::columns(Content::text("x"), 2, None);
        assert!(!c_text.is_empty());
    }

    #[test]
    fn p217_columns_map_content_recurse() {
        // map_content recurse no body preservando count/gutter.
        use crate::entities::layout_types::Length;
        let c = Content::columns(Content::text("a"), 4, Some(Length::pt(5.0)));
        let mapped = c.map_content(&mut |x| Ok(Some(x.clone()))).unwrap();
        if let Content::Columns(e) = &mapped {
            assert_eq!(e.count, 4);
            assert_eq!(e.gutter, Some(Length::pt(5.0)));
            assert_eq!(e.body.plain_text(), "a");
        } else {
            panic!("esperado Content::Columns após map_content");
        }
    }

    #[test]
    fn p217_columns_partial_eq_3_fields() {
        use crate::entities::layout_types::Length;
        let mk = || Content::columns(Content::text("."), 2, Some(Length::pt(8.0)));
        assert_eq!(mk(), mk());
        // Count diferente → diferente.
        let other_count = Content::columns(Content::text("."), 3, Some(Length::pt(8.0)));
        assert_ne!(mk(), other_count);
        // Gutter diferente → diferente.
        let other_gutter =
            Content::columns(Content::text("."), 2, Some(Length::pt(12.0)));
        assert_ne!(mk(), other_gutter);
        // Body diferente → diferente.
        let other_body = Content::columns(Content::text("o"), 2, Some(Length::pt(8.0)));
        assert_ne!(mk(), other_body);
    }

    // ── Passo 220 (ADR-0078 PROPOSTO sub-fase b 4/4) — Colbreak ──────────

    #[test]
    fn p220_colbreak_variant_existe() {
        let c = Content::colbreak(false);
        if let Content::Colbreak(e) = &c {
            assert_eq!(e.weak, false);
        } else {
            panic!("esperado Content::Colbreak");
        }
        let c2 = Content::colbreak(true);
        if let Content::Colbreak(e) = &c2 {
            assert_eq!(e.weak, true);
        } else {
            panic!("esperado Content::Colbreak");
        }
    }

    #[test]
    fn p220_colbreak_is_empty_sempre_false() {
        // Colbreak é event observável (downgrade graded a pagebreak)
        // mesmo "vazio" — paridade Pagebreak/Divider.
        assert!(
            !Content::colbreak(false).is_empty(),
            "Content::Colbreak nunca é considerado vazio (event com efeito)"
        );
        assert!(!Content::colbreak(true).is_empty());
    }

    #[test]
    fn p220_colbreak_plain_text_vazio() {
        // Colbreak é leaf — sem texto plain.
        assert_eq!(Content::colbreak(false).plain_text(), "");
        assert_eq!(Content::colbreak(true).plain_text(), "");
    }

    #[test]
    fn p220_colbreak_partial_eq_1_field() {
        // Eq compara `weak` (1 field — paridade Pagebreak sem to).
        assert_eq!(Content::colbreak(false), Content::colbreak(false));
        assert_eq!(Content::colbreak(true), Content::colbreak(true));
        assert_ne!(Content::colbreak(false), Content::colbreak(true));
    }

    #[test]
    fn p220_colbreak_map_content_terminal() {
        // Colbreak é leaf — map_content clone directo.
        let c = Content::colbreak(true);
        let mapped = c.map_content(&mut |x| Ok(Some(x.clone()))).unwrap();
        assert_eq!(c, mapped);
    }

    // ── Passo 223 (ADR-0061 Fase 4 Layout candidata sub-passo 2) — Place refino ──

    #[test]
    fn p223_place_variant_aceita_float_clearance() {
        // P223 refino: variant aceita 2 fields novos float + clearance.
        use crate::entities::layout_types::{
            Align2D, HAlign, Length, PlaceScope, VAlign,
        };
        let p = Content::place(
            Align2D { h: Some(HAlign::Left), v: Some(VAlign::Top) },
            0.0,
            0.0,
            PlaceScope::Column,
            true,
            Some(Length::pt(10.0)),
            Content::text("body"),
        );
        if let Content::Place(e) = &p {
            assert_eq!(e.float, true);
            assert_eq!(e.clearance, Some(Length::pt(10.0)));
        } else {
            panic!("esperado Content::Place");
        }
    }

    #[test]
    fn p223_place_default_float_false_clearance_none() {
        // Defaults stdlib: float=false, clearance=None.
        use crate::entities::layout_types::{Align2D, HAlign, PlaceScope, VAlign};
        let p = Content::place(
            Align2D { h: Some(HAlign::Left), v: Some(VAlign::Top) },
            0.0,
            0.0,
            PlaceScope::Column,
            false,
            None,
            Content::text("body"),
        );
        if let Content::Place(e) = &p {
            assert_eq!(e.float, false, "default float == false");
            assert!(e.clearance.is_none(), "default clearance == None");
        } else {
            panic!("esperado Content::Place");
        }
    }

    #[test]
    fn p223_place_partial_eq_inclui_float_clearance() {
        // Eq compara 7 fields agora (P223 +2 fields).
        use crate::entities::layout_types::{
            Align2D, HAlign, Length, PlaceScope, VAlign,
        };
        let mk = |float: bool, clearance: Option<Length>| {
            Content::place(
                Align2D { h: Some(HAlign::Left), v: Some(VAlign::Top) },
                0.0,
                0.0,
                PlaceScope::Column,
                float,
                clearance,
                Content::text("."),
            )
        };
        assert_eq!(mk(false, None), mk(false, None));
        // Float diferente → diferente.
        assert_ne!(mk(false, None), mk(true, None));
        // Clearance diferente → diferente.
        assert_ne!(mk(false, None), mk(false, Some(Length::pt(5.0))));
    }

    #[test]
    fn p223_place_map_content_preserva_atributos() {
        // map_content recurse no body preservando float + clearance.
        use crate::entities::layout_types::{
            Align2D, HAlign, Length, PlaceScope, VAlign,
        };
        let p = Content::place(
            Align2D { h: Some(HAlign::Left), v: Some(VAlign::Top) },
            0.0,
            0.0,
            PlaceScope::Parent,
            true,
            Some(Length::pt(8.0)),
            Content::text("X"),
        );
        let mapped = p.map_content(&mut |x| Ok(Some(x.clone()))).unwrap();
        if let Content::Place(e) = &mapped {
            assert_eq!(e.float, true, "map_content preserva float");
            assert_eq!(
                e.clearance,
                Some(Length::pt(8.0)),
                "map_content preserva clearance"
            );
        } else {
            panic!("esperado Content::Place após map_content");
        }
    }

    // ── Passo 224 (ADR-0061 Fase 4 candidata sub-3) — Grid refino + 3 variants ──

    #[test]
    fn p224_grid_variant_aceita_5_fields_aditivos() {
        // Grid variant aceita gutter/align/inset/header/footer.
        use crate::entities::layout_types::{
            Align2D, HAlign, Length, TrackSizing, VAlign,
        };
        use crate::entities::sides::Sides;
        let g = Content::Grid(std::sync::Arc::new(
            crate::entities::elements::grid::GridElem {
                columns: vec![TrackSizing::Auto],
                rows: vec![],
                cells: vec![Content::text("A")],
                hlines: vec![],
                vlines: vec![],
                gutter: Some(Length::pt(5.0)),
                align: Some(Align2D { h: Some(HAlign::Left), v: Some(VAlign::Top) }),
                inset: Sides::uniform(Length::pt(2.0)),
                header: Some(Content::text("H")),
                footer: Some(Content::text("F")),
                stroke: None,
                fill: None,
            },
        ));
        if let Content::Grid(e) = &g {
            assert_eq!(e.gutter, Some(Length::pt(5.0)));
            assert!(e.header.is_some());
            assert!(e.footer.is_some());
        } else {
            panic!("esperado Content::Grid");
        }
    }

    #[test]
    fn p224_grid_partial_eq_inclui_5_fields_novos() {
        use crate::entities::layout_types::{Length, TrackSizing};
        use crate::entities::sides::Sides;
        let mk = |gutter: Option<Length>| {
            Content::Grid(std::sync::Arc::new(
                crate::entities::elements::grid::GridElem {
                    columns: vec![TrackSizing::Auto],
                    rows: vec![],
                    cells: vec![],
                    hlines: vec![],
                    vlines: vec![],
                    gutter,
                    align: None,
                    inset: Sides::uniform(Length::pt(0.0)),
                    header: None,
                    footer: None,
                    stroke: None,
                    fill: None,
                },
            ))
        };
        assert_eq!(mk(None), mk(None));
        assert_ne!(mk(None), mk(Some(Length::pt(5.0))));
    }

    #[test]
    fn p224_gridheader_variant_aceita() {
        let h = Content::grid_header(Content::text("hdr"), true);
        if let Content::GridHeader(e) = &h {
            assert_eq!(e.body.plain_text(), "hdr");
            assert_eq!(e.repeat, true);
        } else {
            panic!("esperado GridHeader");
        }
    }

    #[test]
    fn p224_gridfooter_variant_aceita() {
        let f = Content::grid_footer(Content::text("ftr"), false);
        if let Content::GridFooter(e) = &f {
            assert_eq!(e.body.plain_text(), "ftr");
            assert_eq!(e.repeat, false);
        } else {
            panic!("esperado GridFooter");
        }
    }

    #[test]
    fn p224_gridheader_is_empty_proxy_body() {
        assert!(Content::grid_header(Content::Empty, true).is_empty());
        assert!(!Content::grid_header(Content::text("x"), true).is_empty());
    }

    #[test]
    fn p224_gridcell_variant_aceita_5_fields() {
        let c = Content::GridCell(std::sync::Arc::new(
            crate::entities::elements::grid_cell::GridCellElem {
                body: Content::text("cell"),
                x: Some(1),
                y: Some(2),
                colspan: Some(3),
                rowspan: Some(4),
                stroke: None,
                fill: None,
                align: None,
                inset: None,
                breakable: None,
            },
        ));
        if let Content::GridCell(e) = &c {
            assert_eq!(e.x, Some(1));
            assert_eq!(e.y, Some(2));
            assert_eq!(e.colspan, Some(3));
            assert_eq!(e.rowspan, Some(4));
        } else {
            panic!("esperado GridCell");
        }
    }

    #[test]
    fn p224_gridcell_partial_eq_5_fields() {
        // Eq compara 5 fields (paridade P157B TableCell literal).
        let mk = |x: Option<usize>| {
            Content::GridCell(std::sync::Arc::new(
                crate::entities::elements::grid_cell::GridCellElem {
                    body: Content::text("."),
                    x,
                    y: None,
                    colspan: None,
                    rowspan: None,
                    stroke: None,
                    fill: None,
                    align: None,
                    inset: None,
                    breakable: None,
                },
            ))
        };
        assert_eq!(mk(None), mk(None));
        assert_ne!(mk(None), mk(Some(1)));
    }

    #[test]
    fn p224_gridcell_map_content_preserva_fields() {
        let c = Content::GridCell(std::sync::Arc::new(
            crate::entities::elements::grid_cell::GridCellElem {
                body: Content::text("x"),
                x: Some(0),
                y: Some(1),
                colspan: Some(2),
                rowspan: None,
                stroke: None,
                fill: None,
                align: None,
                inset: None,
                breakable: None,
            },
        ));
        let mapped = c.map_content(&mut |x| Ok(Some(x.clone()))).unwrap();
        if let Content::GridCell(e) = &mapped {
            assert_eq!(e.x, Some(0));
            assert_eq!(e.y, Some(1));
            assert_eq!(e.colspan, Some(2));
            assert!(e.rowspan.is_none());
        } else {
            panic!("esperado GridCell após map_content");
        }
    }

    // ── Passo 227 (ADR-0079 PROPOSTO Fase 5 Layout Categoria A.1) — stroke ──

    #[test]
    fn p227_grid_variant_aceita_stroke() {
        // Grid variant aceita stroke (+1 field P227; total 9 fields).
        use crate::entities::geometry::Stroke;
        use crate::entities::layout_types::{Color, Length, TrackSizing};
        use crate::entities::sides::Sides;
        let g = Content::Grid(std::sync::Arc::new(
            crate::entities::elements::grid::GridElem {
                columns: vec![TrackSizing::Auto],
                rows: vec![],
                cells: vec![Content::text("A")],
                hlines: vec![],
                vlines: vec![],
                gutter: None,
                align: None,
                inset: Sides::uniform(Length::pt(0.0)),
                header: None,
                footer: None,
                stroke: Some(Stroke {
                    paint: Paint::Solid(Color::rgb(255, 0, 0)),
                    thickness: 2.0,
                    overhang: false,
                }),
                fill: None,
            },
        ));
        if let Content::Grid(e) = &g {
            assert!(e.stroke.is_some());
            let s = e.stroke.as_ref().unwrap();
            assert_eq!(s.thickness, 2.0);
        } else {
            panic!("esperado Content::Grid");
        }
    }

    #[test]
    fn p227_table_variant_aceita_stroke() {
        // Paridade Grid para Table (refino paralelo).
        use crate::entities::geometry::Stroke;
        use crate::entities::layout_types::{Color, TrackSizing};
        let t = Content::Table(std::sync::Arc::new(
            crate::entities::elements::table::TableElem {
                columns: vec![TrackSizing::Auto],
                rows: vec![],
                children: vec![Content::text("X")],
                hlines: vec![],
                vlines: vec![],
                header: None,
                footer: None,
                stroke: Some(Stroke {
                    paint: Paint::Solid(Color::rgb(0, 0, 255)),
                    thickness: 1.5,
                    overhang: false,
                }),
                fill: None,
                caption: None,
            },
        ));
        if let Content::Table(e) = &t {
            assert!(e.stroke.is_some());
        } else {
            panic!("esperado Content::Table");
        }
    }

    #[test]
    fn p227_grid_partial_eq_inclui_stroke() {
        // Eq compara 9 fields agora (P227 +1 stroke).
        use crate::entities::geometry::Stroke;
        use crate::entities::layout_types::{Color, Length, TrackSizing};
        use crate::entities::sides::Sides;
        let mk = |stroke: Option<Stroke>| {
            Content::Grid(std::sync::Arc::new(
                crate::entities::elements::grid::GridElem {
                    columns: vec![TrackSizing::Auto],
                    rows: vec![],
                    cells: vec![],
                    hlines: vec![],
                    vlines: vec![],
                    gutter: None,
                    align: None,
                    inset: Sides::uniform(Length::pt(0.0)),
                    header: None,
                    footer: None,
                    stroke,
                    fill: None,
                },
            ))
        };
        assert_eq!(mk(None), mk(None));
        let s = Stroke {
            paint: Paint::Solid(Color::rgb(0, 0, 0)),
            thickness: 1.0,
            overhang: false,
        };
        assert_ne!(mk(None), mk(Some(s)));
    }

    // ── Passo 228 (Fase 5 Layout Categoria A.2) — fill ──

    #[test]
    fn p228_grid_variant_aceita_fill() {
        // Grid variant aceita fill (+1 field P228; total 10 fields).
        use crate::entities::layout_types::{Color, Length, TrackSizing};
        use crate::entities::sides::Sides;
        let g = Content::Grid(std::sync::Arc::new(
            crate::entities::elements::grid::GridElem {
                columns: vec![TrackSizing::Auto],
                rows: vec![],
                cells: vec![Content::text("A")],
                hlines: vec![],
                vlines: vec![],
                gutter: None,
                align: None,
                inset: Sides::uniform(Length::pt(0.0)),
                header: None,
                footer: None,
                stroke: None,
                fill: Some(Color::rgb(255, 255, 0)),
            },
        ));
        if let Content::Grid(e) = &g {
            assert!(e.fill.is_some());
        } else {
            panic!("esperado Content::Grid");
        }
    }

    #[test]
    fn p228_table_variant_aceita_fill() {
        use crate::entities::layout_types::{Color, TrackSizing};
        let t = Content::Table(std::sync::Arc::new(
            crate::entities::elements::table::TableElem {
                columns: vec![TrackSizing::Auto],
                rows: vec![],
                children: vec![Content::text("X")],
                hlines: vec![],
                vlines: vec![],
                header: None,
                footer: None,
                stroke: None,
                fill: Some(Color::rgb(0, 255, 0)),
                caption: None,
            },
        ));
        if let Content::Table(e) = &t {
            assert!(e.fill.is_some());
        } else {
            panic!("esperado Content::Table");
        }
    }

    #[test]
    fn p228_grid_partial_eq_inclui_fill() {
        use crate::entities::layout_types::{Color, Length, TrackSizing};
        use crate::entities::sides::Sides;
        let mk = |fill: Option<Color>| {
            Content::Grid(std::sync::Arc::new(
                crate::entities::elements::grid::GridElem {
                    columns: vec![TrackSizing::Auto],
                    rows: vec![],
                    cells: vec![],
                    hlines: vec![],
                    vlines: vec![],
                    gutter: None,
                    align: None,
                    inset: Sides::uniform(Length::pt(0.0)),
                    header: None,
                    footer: None,
                    stroke: None,
                    fill,
                },
            ))
        };
        assert_eq!(mk(None), mk(None));
        assert_ne!(mk(None), mk(Some(Color::rgb(255, 0, 0))));
    }

    #[test]
    fn p228_grid_map_content_preserva_fill() {
        use crate::entities::layout_types::{Color, Length, TrackSizing};
        use crate::entities::sides::Sides;
        let fill_orig = Color::rgb(50, 50, 50);
        let g = Content::Grid(std::sync::Arc::new(
            crate::entities::elements::grid::GridElem {
                columns: vec![TrackSizing::Auto],
                rows: vec![],
                cells: vec![Content::text("a")],
                hlines: vec![],
                vlines: vec![],
                gutter: None,
                align: None,
                inset: Sides::uniform(Length::pt(0.0)),
                header: None,
                footer: None,
                stroke: None,
                fill: Some(fill_orig),
            },
        ));
        let mapped = g.map_content(&mut |x| Ok(Some(x.clone()))).unwrap();
        if let Content::Grid(e) = &mapped {
            assert_eq!(e.fill, Some(fill_orig));
        } else {
            panic!("esperado Content::Grid após map_content");
        }
    }

    #[test]
    fn p227_grid_map_content_preserva_stroke() {
        // map_content preserva stroke (Option<Stroke> Clone).
        use crate::entities::geometry::Stroke;
        use crate::entities::layout_types::{Color, Length, TrackSizing};
        use crate::entities::sides::Sides;
        let stroke_orig = Stroke {
            paint: Paint::Solid(Color::rgb(100, 100, 100)),
            thickness: 3.0,
            overhang: false,
        };
        let g = Content::Grid(std::sync::Arc::new(
            crate::entities::elements::grid::GridElem {
                columns: vec![TrackSizing::Auto],
                rows: vec![],
                cells: vec![Content::text("a")],
                hlines: vec![],
                vlines: vec![],
                gutter: None,
                align: None,
                inset: Sides::uniform(Length::pt(0.0)),
                header: None,
                footer: None,
                stroke: Some(stroke_orig.clone()),
                fill: None,
            },
        ));
        let mapped = g.map_content(&mut |x| Ok(Some(x.clone()))).unwrap();
        if let Content::Grid(e) = &mapped {
            assert_eq!(e.stroke, Some(stroke_orig));
        } else {
            panic!("esperado Content::Grid após map_content");
        }
    }

    // ── Passo 230 (Fase 5 Layout Categoria A.3) — stroke/fill per-cell ──

    #[test]
    fn p230_gridcell_variant_aceita_stroke_fill() {
        use crate::entities::geometry::Stroke;
        use crate::entities::layout_types::Color;
        let c = Content::GridCell(std::sync::Arc::new(
            crate::entities::elements::grid_cell::GridCellElem {
                body: Content::text("cell"),
                x: None,
                y: None,
                colspan: None,
                rowspan: None,
                stroke: Some(Stroke {
                    paint: Paint::Solid(Color::rgb(0, 0, 0)),
                    thickness: 1.0,
                    overhang: false,
                }),
                fill: Some(Color::rgb(255, 255, 0)),
                align: None,
                inset: None,
                breakable: None,
            },
        ));
        if let Content::GridCell(e) = &c {
            assert!(e.stroke.is_some() && e.fill.is_some());
        } else {
            panic!("esperado GridCell");
        }
    }

    #[test]
    fn p230_tablecell_variant_aceita_stroke_fill() {
        use crate::entities::layout_types::Color;
        let c = Content::TableCell(std::sync::Arc::new(
            crate::entities::elements::table_cell::TableCellElem {
                body: Content::text("cell"),
                x: None,
                y: None,
                colspan: None,
                rowspan: None,
                stroke: None,
                fill: Some(Color::rgb(0, 255, 0)),
                align: None,
                inset: None,
                breakable: None,
            },
        ));
        if let Content::TableCell(e) = &c {
            assert!(e.fill.is_some());
        } else {
            panic!("esperado TableCell");
        }
    }

    #[test]
    fn p230_gridcell_partial_eq_inclui_stroke_fill() {
        use crate::entities::layout_types::Color;
        let mk = |fill: Option<Color>| {
            Content::GridCell(std::sync::Arc::new(
                crate::entities::elements::grid_cell::GridCellElem {
                    body: Content::text("."),
                    x: None,
                    y: None,
                    colspan: None,
                    rowspan: None,
                    stroke: None,
                    fill,
                    align: None,
                    inset: None,
                    breakable: None,
                },
            ))
        };
        assert_eq!(mk(None), mk(None));
        assert_ne!(mk(None), mk(Some(Color::rgb(255, 0, 0))));
    }

    #[test]
    fn p230_gridcell_map_content_preserva_stroke_fill() {
        use crate::entities::geometry::Stroke;
        use crate::entities::layout_types::Color;
        let stroke_orig = Stroke {
            paint: Paint::Solid(Color::rgb(50, 50, 50)),
            thickness: 2.0,
            overhang: false,
        };
        let fill_orig = Color::rgb(200, 200, 200);
        let c = Content::GridCell(std::sync::Arc::new(
            crate::entities::elements::grid_cell::GridCellElem {
                body: Content::text("c"),
                x: None,
                y: None,
                colspan: None,
                rowspan: None,
                stroke: Some(stroke_orig.clone()),
                fill: Some(fill_orig),
                align: None,
                inset: None,
                breakable: None,
            },
        ));
        let mapped = c.map_content(&mut |x| Ok(Some(x.clone()))).unwrap();
        if let Content::GridCell(e) = &mapped {
            assert_eq!(e.stroke, Some(stroke_orig));
            assert_eq!(e.fill, Some(fill_orig));
        } else {
            panic!("esperado GridCell após map_content");
        }
    }

    // ── Passo 235 (Fase 5 Layout Categoria B.3) — GridCell/TableCell algorítmicos ──

    #[test]
    fn p235_gridcell_variant_aceita_align_inset_breakable() {
        use crate::entities::layout_types::Align2D;
        use crate::entities::sides::Sides;
        let c = Content::GridCell(std::sync::Arc::new(
            crate::entities::elements::grid_cell::GridCellElem {
                body: Content::text("p235"),
                x: None,
                y: None,
                colspan: None,
                rowspan: None,
                stroke: None,
                fill: None,
                align: Some(Align2D::from_string("center")),
                inset: Some(Sides::uniform(Length::pt(7.0))),
                breakable: Some(false),
            },
        ));
        if let Content::GridCell(e) = &c {
            assert!(e.align.is_some());
            assert!(e.inset.is_some());
            assert_eq!(e.breakable, Some(false));
        } else {
            panic!("esperado GridCell");
        }
    }

    #[test]
    fn p235_tablecell_variant_aceita_align_inset_breakable() {
        use crate::entities::layout_types::Align2D;
        use crate::entities::sides::Sides;
        let c = Content::TableCell(std::sync::Arc::new(
            crate::entities::elements::table_cell::TableCellElem {
                body: Content::text("p235t"),
                x: None,
                y: None,
                colspan: None,
                rowspan: None,
                stroke: None,
                fill: None,
                align: Some(Align2D::from_string("right")),
                inset: Some(Sides::uniform(Length::pt(3.0))),
                breakable: Some(true),
            },
        ));
        if let Content::TableCell(e) = &c {
            assert!(e.align.is_some());
            assert!(e.inset.is_some());
            assert_eq!(e.breakable, Some(true));
        } else {
            panic!("esperado TableCell");
        }
    }

    #[test]
    fn p235_gridcell_partial_eq_inclui_3_fields() {
        let mk = |breakable: Option<bool>| {
            Content::GridCell(std::sync::Arc::new(
                crate::entities::elements::grid_cell::GridCellElem {
                    body: Content::text("."),
                    x: None,
                    y: None,
                    colspan: None,
                    rowspan: None,
                    stroke: None,
                    fill: None,
                    align: None,
                    inset: None,
                    breakable,
                },
            ))
        };
        assert_eq!(mk(None), mk(None));
        assert_ne!(mk(None), mk(Some(false)));
        assert_ne!(mk(Some(true)), mk(Some(false)));
    }

    #[test]
    fn p235_gridcell_map_content_preserva_3_fields() {
        use crate::entities::layout_types::Align2D;
        use crate::entities::sides::Sides;
        let c = Content::GridCell(std::sync::Arc::new(
            crate::entities::elements::grid_cell::GridCellElem {
                body: Content::text("x"),
                x: None,
                y: None,
                colspan: None,
                rowspan: None,
                stroke: None,
                fill: None,
                align: Some(Align2D::from_string("top")),
                inset: Some(Sides::uniform(Length::pt(2.0))),
                breakable: Some(true),
            },
        ));
        let mapped = c.map_content(&mut |x| Ok(Some(x.clone()))).unwrap();
        if let Content::GridCell(e) = &mapped {
            assert!(e.align.is_some());
            assert!(e.inset.is_some());
            assert_eq!(e.breakable, Some(true));
        } else {
            panic!("esperado GridCell após map_content");
        }
    }

    // ── Passo 231 (Fase 5 Layout Categoria A.4) — Block/Boxed outset/radius/clip ──

    #[test]
    fn p231_block_variant_aceita_outset_radius_clip() {
        // P242 adapta: radius `Option<Length>` → `Corners<Length>`.
        use crate::entities::corners::Corners;
        use crate::entities::sides::Sides;
        let b = Content::Block(std::sync::Arc::new(
            crate::entities::elements::block::BlockElem {
                body: Content::text("body"),
                width: None,
                height: None,
                inset: Sides::uniform(Length::pt(0.0)),
                breakable: true,
                outset: Sides::uniform(Length::pt(5.0)),
                radius: Corners::uniform(Length::pt(3.0)),
                clip: true,
                fill: None,
                stroke: None,
                spacing: None,
                above: None,
                below: None,
                sticky: false,
            },
        ));
        if let Content::Block(e) = &b {
            assert_eq!(e.radius.top_left, Length::pt(3.0));
            assert_eq!(e.radius.top_right, Length::pt(3.0));
            assert_eq!(e.radius.bottom_right, Length::pt(3.0));
            assert_eq!(e.radius.bottom_left, Length::pt(3.0));
            assert_eq!(e.clip, true);
            assert_eq!(e.outset.left, Length::pt(5.0));
        } else {
            panic!("esperado Block");
        }
    }

    #[test]
    fn p231_boxed_variant_aceita_outset_radius_clip() {
        // P242 adapta: radius `Option<Length>` → `Corners<Length>`.
        use crate::entities::corners::Corners;
        use crate::entities::sides::Sides;
        let b = Content::Boxed(std::sync::Arc::new(
            crate::entities::elements::boxed::BoxedElem {
                body: Content::text("body"),
                width: None,
                height: None,
                inset: Sides::uniform(Length::pt(0.0)),
                baseline: Length::pt(0.0),
                outset: Sides::uniform(Length::pt(2.0)),
                radius: Corners::uniform(Length::pt(4.0)),
                clip: false,
                fill: None,
                stroke: None,
            },
        ));
        if let Content::Boxed(e) = &b {
            assert_eq!(e.radius.top_left, Length::pt(4.0));
            assert_eq!(e.clip, false);
            assert_eq!(e.outset.left, Length::pt(2.0));
        } else {
            panic!("esperado Boxed");
        }
    }

    #[test]
    fn p231_block_partial_eq_inclui_3_fields() {
        // P242 adapta: radius `Option<Length>` → `Corners<Length>`.
        use crate::entities::corners::Corners;
        use crate::entities::sides::Sides;
        let mk = |clip: bool| {
            Content::Block(std::sync::Arc::new(
                crate::entities::elements::block::BlockElem {
                    body: Content::text("."),
                    width: None,
                    height: None,
                    inset: Sides::uniform(Length::pt(0.0)),
                    breakable: true,
                    outset: Sides::uniform(Length::pt(0.0)),
                    radius: Corners::uniform(Length::ZERO),
                    clip,
                    fill: None,
                    stroke: None,
                    spacing: None,
                    above: None,
                    below: None,
                    sticky: false,
                },
            ))
        };
        assert_eq!(mk(false), mk(false));
        assert_ne!(mk(false), mk(true));
    }

    #[test]
    fn p231_block_map_content_preserva_3_fields() {
        // P242 adapta: radius `Option<Length>` → `Corners<Length>`.
        use crate::entities::corners::Corners;
        use crate::entities::sides::Sides;
        let outset_orig = Sides::uniform(Length::pt(7.0));
        let radius_orig = Corners::uniform(Length::pt(2.0));
        let b = Content::Block(std::sync::Arc::new(
            crate::entities::elements::block::BlockElem {
                body: Content::text("b"),
                width: None,
                height: None,
                inset: Sides::uniform(Length::pt(0.0)),
                breakable: true,
                outset: outset_orig,
                radius: radius_orig,
                clip: true,
                fill: None,
                stroke: None,
                spacing: None,
                above: None,
                below: None,
                sticky: false,
            },
        ));
        let mapped = b.map_content(&mut |x| Ok(Some(x.clone()))).unwrap();
        if let Content::Block(e) = &mapped {
            assert_eq!(e.outset, outset_orig);
            assert_eq!(e.radius, radius_orig);
            assert_eq!(e.clip, true);
        } else {
            panic!("esperado Block após map_content");
        }
    }

    // ── Passo 247 (M9d / M7+5; ADR-0079 Categoria A.4) ──────────────────
    //     Block + Boxed +2 fields (fill, stroke); promoção real cumulativa
    //     scope-outs ADR-0054 graded N=3 P247 (outset semantic real +
    //     fill + stroke).

    #[test]
    fn p247_block_variant_aceita_fill_stroke() {
        use crate::entities::corners::Corners;
        use crate::entities::geometry::Stroke;
        use crate::entities::layout_types::Color;
        use crate::entities::sides::Sides;
        let b = Content::Block(std::sync::Arc::new(
            crate::entities::elements::block::BlockElem {
                body: Content::text("p247"),
                width: None,
                height: None,
                inset: Sides::uniform(Length::pt(0.0)),
                breakable: true,
                outset: Sides::uniform(Length::pt(0.0)),
                radius: Corners::uniform(Length::ZERO),
                clip: false,
                fill: Some(Color::rgb(200, 0, 0)),
                stroke: Some(Stroke {
                    paint: Paint::Solid(Color::rgb(0, 0, 0)),
                    thickness: 2.0,
                    overhang: false,
                }),
                spacing: None,
                above: None,
                below: None,
                sticky: false,
            },
        ));
        if let Content::Block(e) = &b {
            assert_eq!(e.fill, Some(Color::rgb(200, 0, 0)));
            assert_eq!(e.stroke.as_ref().unwrap().thickness, 2.0);
        } else {
            panic!("esperado Block");
        }
    }

    #[test]
    fn p247_boxed_variant_aceita_fill_stroke() {
        use crate::entities::corners::Corners;
        use crate::entities::geometry::Stroke;
        use crate::entities::layout_types::Color;
        use crate::entities::sides::Sides;
        let b = Content::Boxed(std::sync::Arc::new(
            crate::entities::elements::boxed::BoxedElem {
                body: Content::text("p247"),
                width: None,
                height: None,
                inset: Sides::uniform(Length::pt(0.0)),
                baseline: Length::pt(0.0),
                outset: Sides::uniform(Length::pt(0.0)),
                radius: Corners::uniform(Length::ZERO),
                clip: false,
                fill: Some(Color::rgb(0, 200, 0)),
                stroke: Some(Stroke {
                    paint: Paint::Solid(Color::rgb(0, 0, 0)),
                    thickness: 1.5,
                    overhang: false,
                }),
            },
        ));
        if let Content::Boxed(e) = &b {
            assert_eq!(e.fill, Some(Color::rgb(0, 200, 0)));
            assert_eq!(e.stroke.as_ref().unwrap().thickness, 1.5);
        } else {
            panic!("esperado Boxed");
        }
    }

    #[test]
    fn p247_block_partial_eq_inclui_fill_stroke() {
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::Color;
        use crate::entities::sides::Sides;
        let mk = |fill: Option<Color>| {
            Content::Block(std::sync::Arc::new(
                crate::entities::elements::block::BlockElem {
                    body: Content::text("p247eq"),
                    width: None,
                    height: None,
                    inset: Sides::uniform(Length::pt(0.0)),
                    breakable: true,
                    outset: Sides::uniform(Length::pt(0.0)),
                    radius: Corners::uniform(Length::ZERO),
                    clip: false,
                    fill,
                    stroke: None,
                    spacing: None,
                    above: None,
                    below: None,
                    sticky: false,
                },
            ))
        };
        assert_eq!(mk(None), mk(None));
        assert_ne!(mk(None), mk(Some(Color::rgb(255, 0, 0))));
        assert_eq!(mk(Some(Color::rgb(1, 2, 3))), mk(Some(Color::rgb(1, 2, 3))));
    }

    #[test]
    fn p247_block_map_content_preserva_fill_stroke() {
        use crate::entities::corners::Corners;
        use crate::entities::geometry::Stroke;
        use crate::entities::layout_types::Color;
        use crate::entities::sides::Sides;
        let fill_orig = Some(Color::rgb(50, 100, 150));
        let stroke_orig = Some(Stroke {
            paint: Paint::Solid(Color::rgb(0, 0, 0)),
            thickness: 3.0,
            overhang: false,
        });
        let b = Content::Block(std::sync::Arc::new(
            crate::entities::elements::block::BlockElem {
                body: Content::text("p247map"),
                width: None,
                height: None,
                inset: Sides::uniform(Length::pt(0.0)),
                breakable: true,
                outset: Sides::uniform(Length::pt(0.0)),
                radius: Corners::uniform(Length::ZERO),
                clip: false,
                fill: fill_orig,
                stroke: stroke_orig.clone(),
                spacing: None,
                above: None,
                below: None,
                sticky: false,
            },
        ));
        let mapped = b.map_content(&mut |x| Ok(Some(x.clone()))).unwrap();
        if let Content::Block(e) = &mapped {
            assert_eq!(e.fill, fill_orig);
            assert_eq!(e.stroke, stroke_orig);
        } else {
            panic!("esperado Block após map_content");
        }
    }

    #[test]
    fn p247_block_construtor_defaults_fill_stroke_none() {
        use crate::entities::sides::Sides;
        let b = Content::block(
            Content::text("p247def"),
            None,
            None,
            Sides::uniform(Length::pt(0.0)),
            true,
        );
        if let Content::Block(e) = &b {
            assert_eq!(e.fill, None, "construtor default fill = None");
            assert!(e.stroke.is_none(), "construtor default stroke = None");
        } else {
            panic!("esperado Block");
        }
    }

    #[test]
    fn p247_boxed_construtor_defaults_fill_stroke_none() {
        use crate::entities::sides::Sides;
        let b = Content::boxed(
            Content::text("p247def"),
            None,
            None,
            Sides::uniform(Length::pt(0.0)),
            Length::pt(0.0),
        );
        if let Content::Boxed(e) = &b {
            assert_eq!(e.fill, None);
            assert!(e.stroke.is_none());
        } else {
            panic!("esperado Boxed");
        }
    }

    // ── Passo 157A (ADR-0060 Fase 2 sub-passo 1) — Table ─────────────────

    #[test]
    fn table_constructor_default() {
        let t = Content::table(vec![], vec![], vec![]);
        if let Content::Table(e) = &t {
            assert!(e.columns.is_empty());
            assert!(e.rows.is_empty());
            assert!(e.children.is_empty());
        } else {
            panic!("esperado Content::Table");
        }
    }

    #[test]
    fn table_constructor_com_tracks_e_children() {
        use crate::entities::layout_types::TrackSizing;
        let t = Content::table(
            vec![TrackSizing::Auto, TrackSizing::Auto],
            vec![TrackSizing::Auto],
            vec![Content::text("a"), Content::text("b")],
        );
        if let Content::Table(e) = &t {
            assert_eq!(e.columns.len(), 2);
            assert_eq!(e.rows.len(), 1);
            assert_eq!(e.children.len(), 2);
        } else {
            panic!("esperado Content::Table");
        }
    }

    #[test]
    fn table_is_empty_proxy_via_children() {
        use crate::entities::layout_types::TrackSizing;
        // Children vazios → table vazio (mesmo com tracks declaradas).
        let t_empty =
            Content::table(vec![TrackSizing::Auto], vec![TrackSizing::Auto], vec![]);
        assert!(t_empty.is_empty());
        // Children com texto → não vazio.
        let t_full = Content::table(
            vec![TrackSizing::Auto],
            vec![TrackSizing::Auto],
            vec![Content::text("x")],
        );
        assert!(!t_full.is_empty());
    }

    #[test]
    fn table_plain_text_concatena_children_com_space() {
        use crate::entities::layout_types::TrackSizing;
        let t = Content::table(
            vec![TrackSizing::Auto, TrackSizing::Auto],
            vec![],
            vec![Content::text("hello"), Content::text("world")],
        );
        // Paridade Grid: join(" ").
        assert_eq!(t.plain_text(), "hello world");
    }

    #[test]
    fn table_partial_eq() {
        use crate::entities::layout_types::TrackSizing;
        let mk = || {
            Content::table(
                vec![TrackSizing::Auto],
                vec![TrackSizing::Auto],
                vec![Content::text("a")],
            )
        };
        assert_eq!(mk(), mk());
        // Children diferentes → diferente.
        let other_children = Content::table(
            vec![TrackSizing::Auto],
            vec![TrackSizing::Auto],
            vec![Content::text("b")],
        );
        assert_ne!(mk(), other_children);
        // Columns diferentes → diferente.
        let other_columns = Content::table(
            vec![TrackSizing::Auto, TrackSizing::Auto],
            vec![TrackSizing::Auto],
            vec![Content::text("a")],
        );
        assert_ne!(mk(), other_columns);
    }

    #[test]
    fn table_map_text_recurse_em_children() {
        use crate::entities::layout_types::TrackSizing;
        let t = Content::table(
            vec![TrackSizing::Auto],
            vec![TrackSizing::Auto],
            vec![Content::text("hello"), Content::text("world")],
        );
        let upper = t.map_text(&mut |s| s.to_uppercase());
        assert_eq!(upper.plain_text(), "HELLO WORLD");
    }

    // ── Passo 157B (ADR-0060 Fase 2 sub-passo 2) — TableCell ─────────────

    #[test]
    fn table_cell_constructor_default_todos_none() {
        // P157B: defaults — todos os fields x/y/colspan/rowspan None.
        let c = Content::table_cell(Content::text("body"), None, None, None, None);
        if let Content::TableCell(e) = &c {
            assert_eq!(e.body.plain_text(), "body");
            assert_eq!(e.x, None);
            assert_eq!(e.y, None);
            assert_eq!(e.colspan, None);
            assert_eq!(e.rowspan, None);
        } else {
            panic!("esperado Content::TableCell");
        }
    }

    #[test]
    fn table_cell_constructor_com_x_y() {
        // P157B: ADR-0064 Caso A — Some(n) ↔ posição explícita.
        let c = Content::table_cell(Content::text("x"), Some(2), Some(3), None, None);
        if let Content::TableCell(e) = &c {
            assert_eq!(e.x, Some(2));
            assert_eq!(e.y, Some(3));
        } else {
            panic!("esperado Content::TableCell");
        }
    }

    #[test]
    fn table_cell_constructor_com_colspan_rowspan() {
        // P157B: ADR-0064 Caso C — Some(n) ↔ span explícito.
        let c = Content::table_cell(Content::text("x"), None, None, Some(2), Some(3));
        if let Content::TableCell(e) = &c {
            assert_eq!(e.colspan, Some(2));
            assert_eq!(e.rowspan, Some(3));
        } else {
            panic!("esperado Content::TableCell");
        }
    }

    #[test]
    fn table_cell_is_empty_proxy_via_body() {
        // Body Empty → cell vazio (atributos não tornam não-vazio).
        let c_empty = Content::table_cell(Content::Empty, Some(2), Some(3), None, None);
        assert!(c_empty.is_empty());
        // Body com texto → não vazio.
        let c_full = Content::table_cell(Content::text("a"), None, None, None, None);
        assert!(!c_full.is_empty());
    }

    #[test]
    fn table_cell_plain_text_recurse_no_body() {
        // Plain text recurse sem multiplicar por colspan/rowspan
        // (paridade não visível em texto plano; spans são runtime
        // diferidos em DEBT-34e).
        let c = Content::table_cell(Content::text("xy"), None, None, Some(3), Some(2));
        assert_eq!(c.plain_text(), "xy");
    }

    #[test]
    fn table_cell_partial_eq_cobre_todos_os_5_fields() {
        let mk = || {
            Content::table_cell(Content::text("a"), Some(1), Some(2), Some(3), Some(4))
        };
        assert_eq!(mk(), mk());
        // x diferente → diferente.
        let other_x =
            Content::table_cell(Content::text("a"), Some(99), Some(2), Some(3), Some(4));
        assert_ne!(mk(), other_x);
        // y diferente → diferente.
        let other_y =
            Content::table_cell(Content::text("a"), Some(1), Some(99), Some(3), Some(4));
        assert_ne!(mk(), other_y);
        // colspan diferente → diferente.
        let other_cs =
            Content::table_cell(Content::text("a"), Some(1), Some(2), Some(99), Some(4));
        assert_ne!(mk(), other_cs);
        // rowspan diferente → diferente.
        let other_rs =
            Content::table_cell(Content::text("a"), Some(1), Some(2), Some(3), Some(99));
        assert_ne!(mk(), other_rs);
        // body diferente → diferente.
        let other_body =
            Content::table_cell(Content::text("b"), Some(1), Some(2), Some(3), Some(4));
        assert_ne!(mk(), other_body);
    }

    #[test]
    fn table_cell_map_text_recurse_no_body_preserva_fields() {
        let c = Content::table_cell(
            Content::text("hello"),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
        );
        let upper = c.map_text(&mut |s| s.to_uppercase());
        assert_eq!(upper.plain_text(), "HELLO");
        // Atributos preservados após map_text.
        if let Content::TableCell(e) = upper {
            assert_eq!(e.x, Some(2));
            assert_eq!(e.y, Some(3));
            assert_eq!(e.colspan, Some(4));
            assert_eq!(e.rowspan, Some(5));
        } else {
            panic!("esperado Content::TableCell após map_text");
        }
    }

    // ── Passo 157C (ADR-0060 Fase 2 sub-passo 3 — fecha table foundations) ──
    // Par simétrico TableHeader/TableFooter — tests imediatamente
    // adjacentes para tornar paridade visualmente óbvia.

    #[test]
    fn table_header_constructor_default_repeat_true() {
        // P157C ADR-0064 Caso D: default vanilla `repeat=true`.
        let h = Content::table_header(Content::text("body"), true);
        if let Content::TableHeader(e) = &h {
            assert_eq!(e.body.plain_text(), "body");
            assert!(e.repeat, "default vanilla repeat=true (Caso D)");
        } else {
            panic!("esperado Content::TableHeader");
        }
    }

    #[test]
    fn table_footer_constructor_default_repeat_true() {
        // Par simétrico — paridade absoluta com TableHeader.
        let f = Content::table_footer(Content::text("body"), true);
        if let Content::TableFooter(e) = &f {
            assert_eq!(e.body.plain_text(), "body");
            assert!(e.repeat);
        } else {
            panic!("esperado Content::TableFooter");
        }
    }

    #[test]
    fn table_header_repeat_false_explicito() {
        let h = Content::table_header(Content::text("x"), false);
        if let Content::TableHeader(e) = h {
            assert!(!e.repeat);
        } else {
            panic!("esperado Content::TableHeader");
        }
    }

    #[test]
    fn table_footer_repeat_false_explicito() {
        let f = Content::table_footer(Content::text("x"), false);
        if let Content::TableFooter(e) = f {
            assert!(!e.repeat);
        } else {
            panic!("esperado Content::TableFooter");
        }
    }

    #[test]
    fn table_header_is_empty_proxy_via_body() {
        let h_empty = Content::table_header(Content::Empty, true);
        let h_full = Content::table_header(Content::text("a"), true);
        assert!(h_empty.is_empty());
        assert!(!h_full.is_empty());
    }

    #[test]
    fn table_footer_is_empty_proxy_via_body() {
        let f_empty = Content::table_footer(Content::Empty, true);
        let f_full = Content::table_footer(Content::text("a"), true);
        assert!(f_empty.is_empty());
        assert!(!f_full.is_empty());
    }

    #[test]
    fn table_header_plain_text_recurse_no_body() {
        let h = Content::table_header(Content::text("hi"), true);
        assert_eq!(h.plain_text(), "hi");
    }

    #[test]
    fn table_footer_plain_text_recurse_no_body() {
        let f = Content::table_footer(Content::text("hi"), true);
        assert_eq!(f.plain_text(), "hi");
    }

    #[test]
    fn table_header_partial_eq() {
        let mk = || Content::table_header(Content::text("a"), true);
        assert_eq!(mk(), mk());
        // body diferente → diferente.
        let other_body = Content::table_header(Content::text("b"), true);
        assert_ne!(mk(), other_body);
        // repeat diferente → diferente.
        let other_repeat = Content::table_header(Content::text("a"), false);
        assert_ne!(mk(), other_repeat);
    }

    #[test]
    fn table_footer_partial_eq() {
        let mk = || Content::table_footer(Content::text("a"), true);
        assert_eq!(mk(), mk());
        let other_body = Content::table_footer(Content::text("b"), true);
        assert_ne!(mk(), other_body);
        let other_repeat = Content::table_footer(Content::text("a"), false);
        assert_ne!(mk(), other_repeat);
    }

    #[test]
    fn table_header_e_footer_sao_variants_distintos() {
        // Paridade interna absoluta no contrato; mas variants
        // distintos ao nível do enum (não confundir Header com
        // Footer mesmo com mesmos fields).
        let h = Content::table_header(Content::text("a"), true);
        let f = Content::table_footer(Content::text("a"), true);
        assert_ne!(h, f, "TableHeader e TableFooter são variants distintos");
    }

    #[test]
    fn table_header_map_text_recurse_e_preserva_repeat() {
        let h = Content::table_header(Content::text("hello"), false);
        let upper = h.map_text(&mut |s| s.to_uppercase());
        assert_eq!(upper.plain_text(), "HELLO");
        if let Content::TableHeader(e) = upper {
            assert!(!e.repeat, "repeat preservado após map_text");
        } else {
            panic!("esperado Content::TableHeader");
        }
    }

    #[test]
    fn table_footer_map_text_recurse_e_preserva_repeat() {
        let f = Content::table_footer(Content::text("hello"), false);
        let upper = f.map_text(&mut |s| s.to_uppercase());
        assert_eq!(upper.plain_text(), "HELLO");
        if let Content::TableFooter(e) = upper {
            assert!(!e.repeat);
        } else {
            panic!("esperado Content::TableFooter");
        }
    }

    // ── Passo 159A (ADR-0060 Fase 2 — Bibliography + Cite par acoplado) ──

    #[test]
    fn bibliography_constructor_default_vazia() {
        let b = Content::bibliography(vec![], None);
        if let Content::Bibliography(e) = &b {
            assert!(e.entries.is_empty());
            assert!(e.title.is_none());
        } else {
            panic!("esperado Content::Bibliography");
        }
    }

    #[test]
    fn bibliography_constructor_com_entries_e_title() {
        use crate::entities::bib_entry::BibEntry;
        let b = Content::bibliography(
            vec![BibEntry::new("k1", "A1", "T1", 2024)],
            Some(Content::text("Referências")),
        );
        if let Content::Bibliography(e) = &b {
            assert_eq!(e.entries.len(), 1);
            assert_eq!(e.entries[0].key, "k1");
            assert_eq!(
                e.title.as_ref().map(|t| t.plain_text()).as_deref(),
                Some("Referências")
            );
        } else {
            panic!("esperado Content::Bibliography");
        }
    }

    #[test]
    fn bibliography_is_empty_proxy_via_entries_e_title() {
        use crate::entities::bib_entry::BibEntry;
        // Vazia (sem entries, sem title) → empty.
        let b_empty = Content::bibliography(vec![], None);
        assert!(b_empty.is_empty());
        // Só com title → não empty.
        let b_title = Content::bibliography(vec![], Some(Content::text("R")));
        assert!(!b_title.is_empty());
        // Só com entries → não empty.
        let b_entries =
            Content::bibliography(vec![BibEntry::new("k", "A", "T", 2024)], None);
        assert!(!b_entries.is_empty());
    }

    #[test]
    fn bibliography_plain_text_concatena_title_e_entries() {
        use crate::entities::bib_entry::BibEntry;
        let b = Content::bibliography(
            vec![BibEntry::new("smith2024", "Smith, J.", "On Crystal Math", 2024)],
            Some(Content::text("Referências")),
        );
        let txt = b.plain_text();
        assert!(txt.contains("Referências"), "title presente no plain_text");
        assert!(txt.contains("[smith2024]"), "key formatada como [key]");
        assert!(txt.contains("Smith, J."), "author presente");
        assert!(txt.contains("On Crystal Math"), "title presente");
        assert!(txt.contains("2024"), "year presente");
    }

    #[test]
    fn bibliography_partial_eq_cobre_2_fields() {
        use crate::entities::bib_entry::BibEntry;
        let mk = || {
            Content::bibliography(
                vec![BibEntry::new("k", "A", "T", 2024)],
                Some(Content::text("R")),
            )
        };
        assert_eq!(mk(), mk());
        // entries diferentes → diferente.
        let other_entries = Content::bibliography(
            vec![BibEntry::new("k", "A", "T", 2025)], // year diferente
            Some(Content::text("R")),
        );
        assert_ne!(mk(), other_entries);
        // title diferente → diferente.
        let other_title = Content::bibliography(
            vec![BibEntry::new("k", "A", "T", 2024)],
            Some(Content::text("Bibliografia")),
        );
        assert_ne!(mk(), other_title);
    }

    #[test]
    fn bibliography_with_style_preserva_style_e_locale() {
        use crate::entities::bib_entry::BibEntry;
        let b = Content::bibliography_with_style(
            vec![BibEntry::new("k", "A", "T", 2024)],
            Some(Content::text("R")),
            Some("ieee".into()),
            Some("en-US".into()),
        );
        if let Content::Bibliography(e) = &b {
            assert_eq!(e.style.as_deref(), Some("ieee"));
            assert_eq!(e.locale.as_deref(), Some("en-US"));
        } else {
            panic!("esperado Bibliography");
        }
    }

    #[test]
    fn bibliography_with_style_distingue_por_style() {
        use crate::entities::bib_entry::BibEntry;
        let a = Content::bibliography_with_style(
            vec![BibEntry::new("k", "A", "T", 2024)],
            None,
            Some("ieee".into()),
            None,
        );
        let b = Content::bibliography_with_style(
            vec![BibEntry::new("k", "A", "T", 2024)],
            None,
            Some("apa".into()),
            None,
        );
        assert_ne!(a, b);
    }

    #[test]
    fn bibliography_from_path_preserva_path_e_style() {
        let b = Content::bibliography_from_path(
            "refs.bib",
            Some(Content::text("R")),
            Some("ieee".into()),
            Some("en-US".into()),
        );
        if let Content::Bibliography(e) = &b {
            assert_eq!(e.path.as_deref(), Some("refs.bib"));
            assert!(e.entries.is_empty());
            assert_eq!(e.style.as_deref(), Some("ieee"));
            assert_eq!(e.locale.as_deref(), Some("en-US"));
        } else {
            panic!("esperado Bibliography");
        }
    }

    #[test]
    fn cite_constructor_so_key() {
        let c = Content::cite("smith2024", None, None);
        if let Content::Cite(e) = &c {
            assert_eq!(e.key, "smith2024");
            assert!(e.supplement.is_none());
            assert!(e.form.is_none());
        } else {
            panic!("esperado Content::Cite");
        }
    }

    #[test]
    fn cite_constructor_com_supplement() {
        let c = Content::cite("smith2024", Some(Content::text("p. 42")), None);
        if let Content::Cite(e) = &c {
            assert_eq!(e.key, "smith2024");
            assert_eq!(
                e.supplement.as_ref().map(|s| s.plain_text()).as_deref(),
                Some("p. 42")
            );
            assert!(e.form.is_none());
        } else {
            panic!("esperado Content::Cite");
        }
    }

    #[test]
    fn cite_is_empty_sempre_false() {
        // Cite nunca vazio — placeholder [key] sempre observable.
        let c1 = Content::cite("k", None, None);
        let c2 = Content::cite("k", Some(Content::text("p. 1")), None);
        assert!(!c1.is_empty());
        assert!(!c2.is_empty());
    }

    #[test]
    fn cite_plain_text_emite_placeholder_com_key() {
        // Sem supplement.
        let c1 = Content::cite("smith2024", None, None);
        assert_eq!(c1.plain_text(), "[smith2024]");
        // Com supplement.
        let c2 = Content::cite("smith2024", Some(Content::text("p. 42")), None);
        assert_eq!(c2.plain_text(), "[smith2024]p. 42");
    }

    #[test]
    fn cite_partial_eq_cobre_3_fields() {
        let mk = || Content::cite("k", Some(Content::text("p. 1")), None);
        assert_eq!(mk(), mk());
        // key diferente → diferente.
        let other_key = Content::cite("k2", Some(Content::text("p. 1")), None);
        assert_ne!(mk(), other_key);
        // supplement diferente → diferente.
        let other_sup = Content::cite("k", Some(Content::text("p. 99")), None);
        assert_ne!(mk(), other_sup);
        // supplement None vs Some → diferente.
        let other_none = Content::cite("k", None, None);
        assert_ne!(mk(), other_none);
    }

    // ── Passo 159C: Cite.form ──────────────────────────────────────────────
    use crate::entities::citation_form::CitationForm;

    #[test]
    fn cite_constructor_com_form() {
        let c = Content::cite("smith2024", None, Some(CitationForm::Prose));
        if let Content::Cite(e) = &c {
            assert_eq!(e.key, "smith2024");
            assert!(e.supplement.is_none());
            assert_eq!(e.form, Some(CitationForm::Prose));
        } else {
            panic!("esperado Content::Cite");
        }
    }

    #[test]
    fn cite_partial_eq_cobre_form() {
        let mk = || Content::cite("k", None, Some(CitationForm::Prose));
        assert_eq!(mk(), mk());
        // form diferente → diferente.
        let other_form = Content::cite("k", None, Some(CitationForm::Author));
        assert_ne!(mk(), other_form);
        // form None vs Some → diferente.
        let other_none = Content::cite("k", None, None);
        assert_ne!(mk(), other_none);
    }

    // ── Passo 240 (M9d/M7+1; ADR-0081 PROPOSTO P239 Opção γ) — Content::StateDisplay ──

    #[test]
    fn p240_content_statedisplay_partial_eq_sem_callback() {
        // Sem callback → comparação por key.
        let a = Content::state_display("k".to_string(), None);
        let b = Content::state_display("k".to_string(), None);
        assert_eq!(a, b);
        let c = Content::state_display("other".to_string(), None);
        assert_ne!(a, c);
    }

    #[test]
    fn p240_content_statedisplay_partial_eq_com_callback_ptr_eq() {
        // Com callback → comparação via Func::PartialEq (identidade de Arc;
        // **P742**: nativas com mesmo nome são a mesma função — medido no
        // vanilla `color.rgb == rgb` → true — logo o caso "distinto" usa
        // nome diferente).
        use crate::entities::func::Func;
        let f1 = Func::native("identity", |_, args, _, _| {
            Ok(args
                .items
                .first()
                .cloned()
                .unwrap_or(crate::entities::value::Value::None))
        });
        let a = Content::state_display("k".to_string(), Some(f1.clone()));
        let b = Content::state_display("k".to_string(), Some(f1.clone()));
        // Mesmo Arc partilhado → equal.
        assert_eq!(a, b);
        // Func distinta (nome diferente → função diferente) → not equal.
        let f2 = Func::native("identity2", |_, args, _, _| {
            Ok(args
                .items
                .first()
                .cloned()
                .unwrap_or(crate::entities::value::Value::None))
        });
        let c = Content::state_display("k".to_string(), Some(f2));
        assert_ne!(a, c);
    }

    #[test]
    fn p240_content_statedisplay_plain_text_vazio() {
        // P240 — StateDisplay sem texto direct (resolução pós-fixpoint).
        let c = Content::state_display("k".to_string(), None);
        assert_eq!(c.plain_text(), "");
    }

    // ── Passo 241 (M9d/M7+2; ADR-0081 IMPLEMENTADO parcial paralelo
    //     absoluto P240) — Content::CounterDisplayCallback ──

    #[test]
    fn p241_content_counter_display_callback_partial_eq_sem_callback() {
        // Sem callback → comparação por key.
        let a = Content::counter_display_callback("heading".to_string(), None);
        let b = Content::counter_display_callback("heading".to_string(), None);
        assert_eq!(a, b);
        let c = Content::counter_display_callback("figure".to_string(), None);
        assert_ne!(a, c);
    }

    #[test]
    fn p241_content_counter_display_callback_partial_eq_com_callback_ptr_eq() {
        // Com callback → comparação via Func::PartialEq (identidade de Arc;
        // **P742**: nativas com mesmo nome são a mesma função — medido no
        // vanilla `color.rgb == rgb` → true — logo o caso "distinto" usa
        // nome diferente).
        use crate::entities::func::Func;
        let f1 = Func::native("identity", |_, args, _, _| {
            Ok(args
                .items
                .first()
                .cloned()
                .unwrap_or(crate::entities::value::Value::None))
        });
        let a = Content::counter_display_callback("k".to_string(), Some(f1.clone()));
        let b = Content::counter_display_callback("k".to_string(), Some(f1.clone()));
        assert_eq!(a, b);
        let f2 = Func::native("identity2", |_, args, _, _| {
            Ok(args
                .items
                .first()
                .cloned()
                .unwrap_or(crate::entities::value::Value::None))
        });
        let c = Content::counter_display_callback("k".to_string(), Some(f2));
        assert_ne!(a, c);
    }

    #[test]
    fn p241_content_counter_display_callback_plain_text_vazio() {
        // P241 — CounterDisplayCallback sem texto direct (resolução
        // pós-fixpoint via apply_counter_displays).
        let c = Content::counter_display_callback("heading".to_string(), None);
        assert_eq!(c.plain_text(), "");
    }

    #[test]
    fn p241_content_counter_display_callback_distinto_de_legacy_counter_display() {
        // P241 — variant nova paralela coexiste com Content::CounterDisplay
        // legacy { kind } sem conflito (Decisão 1 Opção α).
        let new = Content::counter_display_callback("heading".to_string(), None);
        let legacy = Content::counter_display("heading".to_string());
        assert_ne!(new, legacy);
    }
}
