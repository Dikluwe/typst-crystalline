//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/query_helpers.md
//! @prompt-hash b2b5d960
//! @layer L3
//! @updated 2026-10-07
//!
//! Approximate query summaries for callers with evaluated content or an
//! existing introspector. Indexed kinds use the introspector; document-group
//! projections retain the bounded legacy policy specified by their L0 owner.
//! Compiled CLI queries use the compiled index through their own pipeline.

use typst_core::compiler::introspect::introspect;
use typst_core::contracts::world::World;
use typst_core::entities::content::Content;
use typst_core::entities::element_kind::ElementKind;
use typst_core::entities::introspector::{Introspector, TagIntrospector};
use typst_core::entities::label::Label;
use typst_core::entities::source::Source;
use typst_core::entities::source_result::SourceDiagnostic;
use typst_core::entities::span::Span;
use typst_core::entities::value::Value;

use crate::pipeline::eval_to_module_with_sink;

/// Selector parseado: discriminação Kind vs Label.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParsedSelector {
    /// Selector por kind (ex: `"heading"`, `"figure"`,
    /// `"metadata"`).
    Kind(ElementKind),
    /// Selector por label (ex: `"<fig-alfa>"` → `"fig-alfa"`).
    Label(String),
}

/// Resultado domain-level de uma query.
///
/// Não implementa `Serialize` — caller (lab/parity) faz
/// a conversão JSON com sua própria dependência
/// `serde_json`. Per L0 §"Restrições".
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuerySummary {
    /// Selector input literal (ex: `"heading"` ou `"<fig-alfa>"`).
    pub selector: String,
    /// Tipo de selector aplicado.
    pub kind: SelectorKind,
    /// Count de matches.
    pub count: usize,
    /// Nome textual do kind se Kind selector (ex: `"heading"`).
    pub kind_name: Option<String>,
    /// Label encontrado se Label selector com match (sem `<>`).
    pub label_found: Option<String>,
    /// Plain text de cada metadata value se selector
    /// `"metadata"`. Vazio caso contrário.
    pub metadata_values: Vec<String>,
}

/// Discriminador de tipo de selector aplicado.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectorKind {
    /// Selector por kind (`heading`, `figure`, etc.).
    Kind,
    /// Selector por label (`<my-label>`).
    Label,
}

/// Erros possíveis ao executar uma query.
#[derive(Debug)]
pub enum QueryError {
    /// Eval produziu erros de compilação.
    EvalFailed(String),
    /// Eval ok mas Module não tem content.
    NoContent,
    /// Selector não parseável (sintaxe ou kind desconhecido).
    InvalidSelector(String),
}

impl std::fmt::Display for QueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QueryError::EvalFailed(s) => write!(f, "eval failed: {}", s),
            QueryError::NoContent => write!(f, "module has no content"),
            QueryError::InvalidSelector(s) => write!(f, "invalid selector: {}", s),
        }
    }
}

impl std::error::Error for QueryError {}

/// Parsing minimal de selector input.
///
/// Aceita:
/// - `"<label-name>"` → `ParsedSelector::Label("label-name")`.
/// - `"heading"`, `"figure"`, etc. → `ParsedSelector::Kind`
///   via `ElementKind::from_str`.
///
/// Rejeita formas vanilla complexas (`heading.where(...)`,
/// etc.) com `InvalidSelector`. Documentado em L0
/// §"Restrições".
pub fn parse_selector(s: &str) -> Result<ParsedSelector, QueryError> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return Err(QueryError::InvalidSelector("empty".to_string()));
    }
    if trimmed.starts_with('<') && trimmed.ends_with('>') {
        let label = &trimmed[1..trimmed.len() - 1];
        if label.is_empty() {
            return Err(QueryError::InvalidSelector("empty label".to_string()));
        }
        return Ok(ParsedSelector::Label(label.to_string()));
    }
    // Text and regex selectors are meaningful for show rules, but cannot be
    // resolved to a location by the introspector. Keep this diagnostic
    // distinct from the generic complex-selector error.
    if (trimmed.starts_with('"') && trimmed.ends_with('"'))
        || trimmed.starts_with("regex(")
    {
        return Err(QueryError::InvalidSelector("text is not locatable".to_string()));
    }
    // P480 — alias vanilla: `math.equation` → ElementKind::Equation.
    // Vanilla rejeita `equation` standalone; aceita `math.equation`.
    if trimmed == "math.equation" {
        return Ok(ParsedSelector::Kind(ElementKind::Equation));
    }
    if trimmed.contains('.') || trimmed.contains('(') {
        return Err(QueryError::InvalidSelector(format!(
            "complex selectors (e.g. `kind.where(...)`) not supported in P206C; got `{}`",
            trimmed
        )));
    }
    match ElementKind::from_name(trimmed) {
        Some(kind) => Ok(ParsedSelector::Kind(kind)),
        None       => Err(QueryError::InvalidSelector(format!(
            "unknown kind name `{}`; expected one of: heading, figure, citation, metadata, state, state_update, outline, bibliography, equation, counter_update, table, list, enum, par, link, raw, quote, footnote",
            trimmed
        ))),
    }
}

/// Sumariza o resultado de aplicar `parsed` a `intr` e `content`.
///
/// Pure function — sem I/O. Útil para callers que já
/// têm um `TagIntrospector` construído.
pub fn summarize_query(
    intr: &TagIntrospector,
    content: &Content,
    parsed: &ParsedSelector,
    raw_selector: &str,
) -> QuerySummary {
    match parsed {
        ParsedSelector::Kind(kind) => {
            // P494: kinds de elementos de documento sem container
            // locatable em L1 são contados por análise do Content.
            let count = if is_document_element_kind(*kind) {
                count_element_in_content(content, *kind)
            } else {
                intr.query_by_kind(*kind).len()
            };
            let metadata_values = if matches!(kind, ElementKind::Metadata) {
                intr.query_metadata().iter().map(value_plain_text).collect()
            } else {
                Vec::new()
            };
            QuerySummary {
                selector: raw_selector.to_string(),
                kind: SelectorKind::Kind,
                count,
                kind_name: Some(kind.as_str().to_string()),
                label_found: None,
                metadata_values,
            }
        }
        ParsedSelector::Label(label_str) => {
            let label = Label(label_str.clone());
            let location = intr.query_by_label(&label);
            QuerySummary {
                selector: raw_selector.to_string(),
                kind: SelectorKind::Label,
                count: if location.is_some() { 1 } else { 0 },
                kind_name: None,
                label_found: location.map(|_| label_str.clone()),
                metadata_values: Vec::new(),
            }
        }
    }
}

/// **P494** — Kinds que devem ser contados por análise directa do
/// `Content` em vez de `Introspector::query_by_kind`.
fn is_document_element_kind(kind: ElementKind) -> bool {
    matches!(
        kind,
        ElementKind::List
            | ElementKind::Enum
            | ElementKind::Par
            | ElementKind::Link
            | ElementKind::Raw
            | ElementKind::Quote
            | ElementKind::Footnote
    )
}

/// **P494** — Conta ocorrências de um elemento de documento no
/// `Content` resultante do eval. Usado para kinds que não têm
/// container locatable em L1 (`list`, `enum`, `par`) ou que ainda
/// não foram promovidos a locatable (`link`, `raw`, `quote`,
/// `footnote`).
///
/// - `List`: `Sequence` cujos filhos são todos `ListItem`, ou um
///   um item isolado recorre ao seu corpo. Conta 1 por grupo reconhecido.
/// - `Enum`: análogo a `List` para `EnumItem`.
/// - `Par`: retorna 1 se o documento contiver qualquer `Content::Text`
///   (aproximação — cristalino não materializa `ParElem`).
/// - `Link`/`Raw`/`Quote`/`Footnote`: conta variantes directas.
pub fn count_element_in_content(content: &Content, kind: ElementKind) -> usize {
    match kind {
        ElementKind::List => count_list_groups(content),
        ElementKind::Enum => count_enum_groups(content),
        ElementKind::Par => {
            if has_any_text(content) {
                1
            } else {
                0
            }
        }
        ElementKind::Link => count_variant(content, |c| matches!(c, Content::Link(_))),
        ElementKind::Raw => count_variant(content, |c| matches!(c, Content::Raw(_))),
        ElementKind::Quote => count_variant(content, |c| matches!(c, Content::Quote(_))),
        ElementKind::Footnote => {
            count_variant(content, |c| matches!(c, Content::Footnote(_)))
        }
        ElementKind::Heading
        | ElementKind::Figure
        | ElementKind::Citation
        | ElementKind::Metadata
        | ElementKind::State
        | ElementKind::StateUpdate
        | ElementKind::Outline
        | ElementKind::OutlineEntry
        | ElementKind::Bibliography
        | ElementKind::Equation
        | ElementKind::CounterUpdate
        | ElementKind::Table
        | ElementKind::StateDisplay
        | ElementKind::CounterDisplay
        | ElementKind::ContextBlock => 0,
    }
}

fn count_list_groups(content: &Content) -> usize {
    match content {
        Content::Sequence(seq) => {
            if seq.iter().all(|c| matches!(c, Content::ListItem(_))) {
                1
            } else {
                seq.iter().map(count_list_groups).sum()
            }
        }
        Content::ListItem(item) => count_list_groups(&item.body),
        Content::Source(_)
        | Content::ShowRecipe(_)
        | Content::Realized(_)
        | Content::Located(_)
        | Content::Empty
        | Content::Text(_)
        | Content::Space
        | Content::SpaceFromNewline
        | Content::Parbreak
        | Content::HtmlElem(_)
        | Content::HtmlFrame(_)
        | Content::Par { .. }
        | Content::Heading(_)
        | Content::Title(_)
        | Content::Strong(_)
        | Content::Emph(_)
        | Content::Raw(_)
        | Content::EnumItem(_)
        | Content::Link(_)
        | Content::Equation(_)
        | Content::MathSequence(_)
        | Content::Symbol(_)
        | Content::MathIdent(_)
        | Content::MathText(_)
        | Content::MathFrac(_)
        | Content::MathAttach(_)
        | Content::MathPrimes(_)
        | Content::MathRoot(_)
        | Content::MathStretch(_)
        | Content::MathDelimited(_)
        | Content::MathLr(_)
        | Content::MathMid(_)
        | Content::MathOverbrace(_)
        | Content::MathUnderbrace(_)
        | Content::MathUnderbracket(_)
        | Content::MathUnderparen(_)
        | Content::MathUndershell(_)
        | Content::MathOverbracket(_)
        | Content::MathOverparen(_)
        | Content::MathOvershell(_)
        | Content::MathAlignPoint(_)
        | Content::Linebreak(_)
        | Content::MathMatrix(_)
        | Content::MathCases(_)
        | Content::MathAccent(_)
        | Content::MathCancel(_)
        | Content::MathUnderline(_)
        | Content::MathOverline(_)
        | Content::MathVec(_)
        | Content::MathClassOverride(_)
        | Content::MathLimitsOverride(_)
        | Content::MathUnderover(_)
        | Content::MathOp(_)
        | Content::MathStyled(_)
        | Content::Label(_)
        | Content::Ref(_)
        | Content::CounterDisplay(_)
        | Content::CounterUpdate(_)
        | Content::Outline(_)
        | Content::OutlineEntry(_)
        | Content::Figure(_)
        | Content::FigureCaption(_)
        | Content::Image(_)
        | Content::Shape(_)
        | Content::Curve(_)
        | Content::Transform(_)
        | Content::Grid(_)
        | Content::GridHeader(_)
        | Content::GridFooter(_)
        | Content::GridCell(_)
        | Content::SetPage(_)
        | Content::PageRun(_)
        | Content::Align(_)
        | Content::Place(_)
        | Content::Flush(_)
        | Content::Styled(_)
        | Content::Divider(_)
        | Content::Terms(_)
        | Content::TermItem(_)
        | Content::Quote(_)
        | Content::Document { .. }
        | Content::Asset { .. }
        | Content::SmartQuote(_)
        | Content::PdfAttach(_)
        | Content::PdfArtifact(_)
        | Content::Underline(_)
        | Content::Strike(_)
        | Content::Overline(_)
        | Content::SmallCaps { .. }
        | Content::Pad(_)
        | Content::Hide(_)
        | Content::HSpace(_)
        | Content::VSpace(_)
        | Content::Pagebreak(_)
        | Content::Colbreak(_)
        | Content::Stack(_)
        | Content::Boxed(_)
        | Content::Block(_)
        | Content::TableCell(_)
        | Content::Bibliography(_)
        | Content::Cite(_)
        | Content::Footnote(_)
        | Content::FootnoteEntry(_)
        | Content::TableHeader(_)
        | Content::TableFooter(_)
        | Content::GridHLine(_)
        | Content::GridVLine(_)
        | Content::TableHLine(_)
        | Content::TableVLine(_)
        | Content::Table(_)
        | Content::Repeat(_)
        | Content::Columns(_)
        | Content::Metadata(_)
        | Content::State(_)
        | Content::StateUpdate(_)
        | Content::StateDisplay(_)
        | Content::CounterDisplayCallback(_)
        | Content::ContextBlock(_)
        | Content::Dynamic(_) => 0,
    }
}

fn count_enum_groups(content: &Content) -> usize {
    match content {
        Content::Sequence(seq) => {
            if seq.iter().all(|c| matches!(c, Content::EnumItem(_))) {
                1
            } else {
                seq.iter().map(count_enum_groups).sum()
            }
        }
        Content::EnumItem(item) => count_enum_groups(&item.body),
        Content::Source(_)
        | Content::ShowRecipe(_)
        | Content::Realized(_)
        | Content::Located(_)
        | Content::Empty
        | Content::Text(_)
        | Content::Space
        | Content::SpaceFromNewline
        | Content::Parbreak
        | Content::HtmlElem(_)
        | Content::HtmlFrame(_)
        | Content::Par { .. }
        | Content::Heading(_)
        | Content::Title(_)
        | Content::Strong(_)
        | Content::Emph(_)
        | Content::Raw(_)
        | Content::ListItem(_)
        | Content::Link(_)
        | Content::Equation(_)
        | Content::MathSequence(_)
        | Content::Symbol(_)
        | Content::MathIdent(_)
        | Content::MathText(_)
        | Content::MathFrac(_)
        | Content::MathAttach(_)
        | Content::MathPrimes(_)
        | Content::MathRoot(_)
        | Content::MathStretch(_)
        | Content::MathDelimited(_)
        | Content::MathLr(_)
        | Content::MathMid(_)
        | Content::MathOverbrace(_)
        | Content::MathUnderbrace(_)
        | Content::MathUnderbracket(_)
        | Content::MathUnderparen(_)
        | Content::MathUndershell(_)
        | Content::MathOverbracket(_)
        | Content::MathOverparen(_)
        | Content::MathOvershell(_)
        | Content::MathAlignPoint(_)
        | Content::Linebreak(_)
        | Content::MathMatrix(_)
        | Content::MathCases(_)
        | Content::MathAccent(_)
        | Content::MathCancel(_)
        | Content::MathUnderline(_)
        | Content::MathOverline(_)
        | Content::MathVec(_)
        | Content::MathClassOverride(_)
        | Content::MathLimitsOverride(_)
        | Content::MathUnderover(_)
        | Content::MathOp(_)
        | Content::MathStyled(_)
        | Content::Label(_)
        | Content::Ref(_)
        | Content::CounterDisplay(_)
        | Content::CounterUpdate(_)
        | Content::Outline(_)
        | Content::OutlineEntry(_)
        | Content::Figure(_)
        | Content::FigureCaption(_)
        | Content::Image(_)
        | Content::Shape(_)
        | Content::Curve(_)
        | Content::Transform(_)
        | Content::Grid(_)
        | Content::GridHeader(_)
        | Content::GridFooter(_)
        | Content::GridCell(_)
        | Content::SetPage(_)
        | Content::PageRun(_)
        | Content::Align(_)
        | Content::Place(_)
        | Content::Flush(_)
        | Content::Styled(_)
        | Content::Divider(_)
        | Content::Terms(_)
        | Content::TermItem(_)
        | Content::Quote(_)
        | Content::Document { .. }
        | Content::Asset { .. }
        | Content::SmartQuote(_)
        | Content::PdfAttach(_)
        | Content::PdfArtifact(_)
        | Content::Underline(_)
        | Content::Strike(_)
        | Content::Overline(_)
        | Content::SmallCaps { .. }
        | Content::Pad(_)
        | Content::Hide(_)
        | Content::HSpace(_)
        | Content::VSpace(_)
        | Content::Pagebreak(_)
        | Content::Colbreak(_)
        | Content::Stack(_)
        | Content::Boxed(_)
        | Content::Block(_)
        | Content::TableCell(_)
        | Content::Bibliography(_)
        | Content::Cite(_)
        | Content::Footnote(_)
        | Content::FootnoteEntry(_)
        | Content::TableHeader(_)
        | Content::TableFooter(_)
        | Content::GridHLine(_)
        | Content::GridVLine(_)
        | Content::TableHLine(_)
        | Content::TableVLine(_)
        | Content::Table(_)
        | Content::Repeat(_)
        | Content::Columns(_)
        | Content::Metadata(_)
        | Content::State(_)
        | Content::StateUpdate(_)
        | Content::StateDisplay(_)
        | Content::CounterDisplayCallback(_)
        | Content::ContextBlock(_)
        | Content::Dynamic(_) => 0,
    }
}

fn has_any_text(content: &Content) -> bool {
    match content {
        Content::Source(e) => has_any_text(&e.body),
        Content::ShowRecipe(e) => has_any_text(&e.body),
        Content::Located(e) => has_any_text(&e.body),
        Content::Realized(e) => has_any_text(&e.visual),
        Content::Text(_) => true,
        Content::Sequence(seq) => seq.iter().any(has_any_text),
        Content::ListItem(item) => has_any_text(&item.body),
        Content::EnumItem(item) => has_any_text(&item.body),
        Content::Link(link) => has_any_text(&link.body),
        Content::Quote(quote) => has_any_text(&quote.body),
        Content::Footnote(footnote) => has_any_text(&footnote.body),
        // P1473 — figure.caption/footnote.entry, escopo reduzido.
        Content::FigureCaption(caption) => has_any_text(&caption.body),
        Content::FootnoteEntry(entry) => has_any_text(&entry.note),
        // Contentores comuns que podem embrulhar texto.
        Content::Styled(styled) => has_any_text(&styled.body),
        Content::Strong(strong) => has_any_text(&strong.body),
        Content::Emph(emph) => has_any_text(&emph.body),
        Content::Title(title) => has_any_text(&title.body),
        Content::Par { body } => has_any_text(body),
        Content::PageRun(run) => has_any_text(&run.body),
        Content::Align(align) => has_any_text(&align.body),
        Content::Pad(pad) => has_any_text(&pad.body),
        Content::Hide(hide) => has_any_text(&hide.body),
        Content::Block(block) => has_any_text(&block.body),
        Content::Boxed(boxed) => has_any_text(&boxed.body),
        Content::Stack(stack) => stack.children.iter().any(has_any_text),
        Content::Grid(grid) => {
            let header_text = grid.header.as_ref().is_some_and(has_any_text);
            let footer_text = grid.footer.as_ref().is_some_and(has_any_text);
            header_text || grid.cells.iter().any(has_any_text) || footer_text
        }
        Content::GridCell(cell) => has_any_text(&cell.body),
        Content::Table(table) => {
            let caption_text = table.caption.as_ref().is_some_and(has_any_text);
            caption_text || table.resolved.as_ref().map_or(table.items.as_ref().unwrap_or(&table.children),|r|&r.cells).iter().any(has_any_text)
        }
        Content::TableHeader(header) => has_any_text(&header.body),
        Content::TableFooter(footer) => has_any_text(&footer.body),
        Content::TableCell(cell) => has_any_text(cell.resolved.as_ref().map_or(&cell.body,|r|&r.visual)),
        Content::Terms(terms) => terms.items.iter().any(has_any_text),
        Content::TermItem(item) => has_any_text(&item.term) || has_any_text(&item.description),
        Content::Repeat(repeat) => has_any_text(&repeat.body),
        Content::Columns(columns) => has_any_text(&columns.body),
        Content::Place(place) => has_any_text(&place.body),
        Content::Transform(transform) => has_any_text(&transform.body),
        Content::Underline(u) => has_any_text(&u.body),
        Content::Strike(s) => has_any_text(&s.body),
        Content::Overline(o) => has_any_text(&o.body),
        Content::SmallCaps { body } => has_any_text(body),
        Content::Figure(figure) => has_any_text(&figure.body),
        Content::Equation(equation) => has_any_text(&equation.body),
        Content::Bibliography(bib) => bib.title.as_ref().is_some_and(has_any_text),
        Content::Label(label) => has_any_text(&label.body),
        Content::PdfArtifact(artifact) => has_any_text(&artifact.body),
        Content::HtmlFrame(frame) => has_any_text(&frame.body),
        Content::OutlineEntry(entry) => has_any_text(&entry.element),
        Content::Heading(_)
        | Content::Metadata(_) | Content::State(_) | Content::StateUpdate(_)
        | Content::StateDisplay(_) | Content::CounterDisplayCallback(_) | Content::CounterDisplay(_)
        | Content::CounterUpdate(_) | Content::Outline(_) | Content::Cite(_)
        | Content::Raw(_) | Content::Ref(_) | Content::Image(_)
        | Content::Shape { .. } | Content::Curve(_)
        | Content::Divider(_)
        | Content::Linebreak(_) | Content::HSpace(_) | Content::VSpace(_)
        | Content::Pagebreak(_) | Content::Colbreak(_)
        | Content::MathSequence(_) | Content::Symbol(_) | Content::MathIdent(_) | Content::MathText(_)
        | Content::MathFrac(_) | Content::MathAttach(_) | Content::MathPrimes(_) | Content::MathRoot(_) | Content::MathStretch(_)
        | Content::MathDelimited(_) | Content::MathLr(_) | Content::MathMid(_) | Content::MathOverbrace(_) | Content::MathUnderbrace(_) | Content::MathUnderbracket(_) | Content::MathUnderparen(_) | Content::MathUndershell(_) | Content::MathOverbracket(_) | Content::MathOverparen(_) | Content::MathOvershell(_) | Content::MathAlignPoint(_)
        | Content::MathMatrix(_) | Content::MathCases(_)
        | Content::MathAccent(_) | Content::MathCancel(_) | Content::MathUnderline(_) | Content::MathOverline(_)
        | Content::MathVec(_)
        | Content::Flush(_)
        | Content::MathClassOverride(_)
        | Content::MathLimitsOverride(_)
        | Content::MathUnderover(_) | Content::MathOp(_) | Content::MathStyled(_)
        | Content::SmartQuote(_) | Content::Dynamic(_) | Content::PdfAttach(_)
        | Content::GridHeader(_) | Content::GridFooter(_) | Content::SetPage { .. }
        | Content::ContextBlock(_)
        | Content::GridHLine(_) | Content::GridVLine(_) | Content::TableHLine(_) | Content::TableVLine(_)
        // P622: Parbreak não contém texto.
        | Content::Empty | Content::Space | Content::SpaceFromNewline | Content::Parbreak | Content::Document { .. } | Content::Asset { .. } | Content::HtmlElem(_) => false,
    }
}

fn count_variant<F>(content: &Content, predicate: F) -> usize
where
    F: Fn(&Content) -> bool + Copy,
{
    if predicate(content) {
        return 1;
    }
    match content {
        Content::Source(e) => count_variant(&e.body, predicate),
        Content::ShowRecipe(e) => count_variant(&e.body, predicate),
        Content::Located(e) => count_variant(&e.body, predicate),
        Content::Realized(e) => count_variant(&e.visual, predicate),
        Content::Sequence(seq) => seq.iter().map(|c| count_variant(c, predicate)).sum(),
        Content::ListItem(item) => count_variant(&item.body, predicate),
        Content::EnumItem(item) => count_variant(&item.body, predicate),
        Content::Quote(quote) => count_variant(&quote.body, predicate),
        Content::Footnote(footnote) => count_variant(&footnote.body, predicate),
        // P1473 — figure.caption/footnote.entry, escopo reduzido.
        Content::FigureCaption(caption) => count_variant(&caption.body, predicate),
        Content::FootnoteEntry(entry) => count_variant(&entry.note, predicate),
        Content::Styled(styled) => count_variant(&styled.body, predicate),
        Content::Strong(strong) => count_variant(&strong.body, predicate),
        Content::Emph(emph) => count_variant(&emph.body, predicate),
        Content::Title(title) => count_variant(&title.body, predicate),
        Content::Par { body } => count_variant(body, predicate),
        Content::PageRun(run) => count_variant(&run.body, predicate),
        Content::Align(align) => count_variant(&align.body, predicate),
        Content::Pad(pad) => count_variant(&pad.body, predicate),
        Content::Hide(hide) => count_variant(&hide.body, predicate),
        Content::Block(block) => count_variant(&block.body, predicate),
        Content::Boxed(boxed) => count_variant(&boxed.body, predicate),
        Content::Stack(stack) => stack.children.iter().map(|c| count_variant(c, predicate)).sum(),
        Content::Grid(grid) => {
            grid.header.as_ref().map_or(0, |h| count_variant(h, predicate))
                + grid.cells.iter().map(|c| count_variant(c, predicate)).sum::<usize>()
                + grid.footer.as_ref().map_or(0, |f| count_variant(f, predicate))
        }
        Content::GridCell(cell) => count_variant(&cell.body, predicate),
        Content::Table(table) => {
            table.caption.as_ref().map_or(0, |c| count_variant(c, predicate))
                + table.resolved.as_ref().map_or(table.items.as_ref().unwrap_or(&table.children),|r|&r.cells).iter().map(|c| count_variant(c, predicate)).sum::<usize>()
        }
        Content::TableHeader(header) => count_variant(&header.body, predicate),
        Content::TableFooter(footer) => count_variant(&footer.body, predicate),
        Content::TableCell(cell) => count_variant(cell.resolved.as_ref().map_or(&cell.body,|r|&r.visual), predicate),
        Content::Terms(terms) => terms.items.iter().map(|c| count_variant(c, predicate)).sum(),
        Content::TermItem(item) => {
            count_variant(&item.term, predicate) + count_variant(&item.description, predicate)
        }
        Content::Repeat(repeat) => count_variant(&repeat.body, predicate),
        Content::Columns(columns) => count_variant(&columns.body, predicate),
        Content::Place(place) => count_variant(&place.body, predicate),
        Content::Transform(transform) => count_variant(&transform.body, predicate),
        Content::Underline(u) => count_variant(&u.body, predicate),
        Content::Strike(s) => count_variant(&s.body, predicate),
        Content::Overline(o) => count_variant(&o.body, predicate),
        Content::SmallCaps { body } => count_variant(body, predicate),
        Content::Figure(figure) => count_variant(&figure.body, predicate),
        Content::Equation(equation) => count_variant(&equation.body, predicate),
        Content::Bibliography(bib) => bib.title.as_ref().map_or(0, |t| count_variant(t, predicate)),
        Content::Label(label) => count_variant(&label.body, predicate),
        Content::PdfArtifact(artifact) => count_variant(&artifact.body, predicate),
        Content::HtmlFrame(frame) => count_variant(&frame.body, predicate),
        Content::OutlineEntry(entry) => {
            count_variant(&entry.element, predicate)
                + entry
                    .fill
                    .as_ref()
                    .map_or(0, |fill| count_variant(fill, predicate))
        }
        Content::Heading(_) | Content::Link(_) | Content::Raw(_) | Content::Metadata(_)
        | Content::State(_) | Content::StateUpdate(_) | Content::StateDisplay(_)
        | Content::CounterDisplayCallback(_) | Content::CounterDisplay(_)
        | Content::CounterUpdate(_) | Content::Outline(_) | Content::Cite(_)
        | Content::Ref(_) | Content::Image(_) | Content::Shape { .. } | Content::Curve(_)
        | Content::Divider(_) | Content::Linebreak(_) | Content::HSpace(_)
        | Content::VSpace(_) | Content::Pagebreak(_) | Content::Colbreak(_)
        | Content::MathSequence(_) | Content::Symbol(_) | Content::MathIdent(_) | Content::MathText(_)
        | Content::MathFrac(_) | Content::MathAttach(_) | Content::MathPrimes(_) | Content::MathRoot(_) | Content::MathStretch(_)
        | Content::MathDelimited(_) | Content::MathLr(_) | Content::MathMid(_) | Content::MathOverbrace(_) | Content::MathUnderbrace(_) | Content::MathUnderbracket(_) | Content::MathUnderparen(_) | Content::MathUndershell(_) | Content::MathOverbracket(_) | Content::MathOverparen(_) | Content::MathOvershell(_) | Content::MathAlignPoint(_)
        | Content::MathMatrix(_) | Content::MathCases(_)
        | Content::MathAccent(_) | Content::MathCancel(_) | Content::MathUnderline(_) | Content::MathOverline(_)
        | Content::MathVec(_)
        | Content::Flush(_)
        | Content::MathClassOverride(_)
        | Content::MathLimitsOverride(_)
        | Content::MathUnderover(_) | Content::MathOp(_) | Content::MathStyled(_)
        | Content::SmartQuote(_) | Content::Dynamic(_) | Content::PdfAttach(_)
        | Content::GridHeader(_) | Content::GridFooter(_) | Content::SetPage { .. }
        | Content::ContextBlock(_)
        | Content::GridHLine(_) | Content::GridVLine(_) | Content::TableHLine(_) | Content::TableVLine(_)
        // P622: Parbreak é leaf estrutural — não conta para a variante.
        | Content::Empty | Content::Space | Content::SpaceFromNewline | Content::Parbreak | Content::Text(_)
        | Content::Document { .. } | Content::Asset { .. } | Content::HtmlElem(_) => 0,
    }
}

/// Função pública principal — pipeline completo source
/// → query summary.
///
/// Reusa `eval_to_module_with_sink` (L3 pipeline) +
/// `introspect` (L1) + `summarize_query` (esta lib).
pub fn query_to_summary(
    world: &dyn World,
    source: &Source,
    selector: &str,
) -> Result<QuerySummary, QueryError> {
    let parsed = parse_selector(selector)?;
    let (eval_result, _warnings) = eval_to_module_with_sink(world, source);
    let module = eval_result.map_err(|errors| {
        QueryError::EvalFailed(format!("{} diagnostic(s)", errors.len()))
    })?;
    let content = module.content().ok_or(QueryError::NoContent)?;
    // P498 — usar conteúdo original (pré-show-rules) para introspecção,
    // garantindo que elementos locatable consumidos por show-rules ainda
    // são visíveis a `query`.
    let intr_content = module.introspection_content().unwrap_or(content);
    let fallback;
    let intr = match module.realization_introspector() {
        Some(intr) => intr,
        None => {
            fallback = introspect(intr_content);
            &fallback
        }
    };
    Ok(summarize_query(intr, content, &parsed, selector))
}

/// Compilation diagnostics and retrieval failures have distinct CLI envelopes.
#[derive(Debug)]
pub enum CompiledQueryError {
    Compilation(Vec<SourceDiagnostic>),
    Retrieval(String),
}

/// Queries the complete paged context, preserving captured language values.
pub fn query_compiled_elements(
    world: &dyn World,
    source: &Source,
    selector: &str,
) -> (Result<Vec<Value>, CompiledQueryError>, Vec<SourceDiagnostic>) {
    let (compiled, warnings) = crate::pipeline::compile_query_context(world, source);
    let introspector = match compiled {
        Ok(introspector) => introspector,
        Err(errors) => return (Err(CompiledQueryError::Compilation(errors)), warnings),
    };
    let expression = match Source::new_with_parser(
        source.id(),
        selector.to_string(),
        typst_core::compiler::parse::parse_code,
    ) {
        Ok(expression) => expression,
        Err(errors) => {
            return (
                Err(CompiledQueryError::Retrieval(format!(
                    "failed to evaluate selector: {}",
                    errors
                        .into_iter()
                        .map(|error| error.message)
                        .collect::<Vec<_>>()
                        .join(", "),
                ))),
                warnings,
            )
        }
    };
    let (evaluated, _) =
        typst_core::compiler::eval::eval_source_with_features_and_introspector(
            world,
            &expression,
            typst_core::entities::compiler_features::Features::default(),
            &TagIntrospector::empty(),
        );
    let messages = |errors: Vec<SourceDiagnostic>| {
        errors
            .into_iter()
            .map(|error| error.message)
            .collect::<Vec<_>>()
            .join(", ")
    };
    let value = match evaluated {
        Ok(value) => value,
        Err(errors) => {
            return (
                Err(CompiledQueryError::Retrieval(format!(
                    "failed to evaluate selector: {}",
                    messages(errors),
                ))),
                warnings,
            )
        }
    };
    if matches!(&value, Value::Str(_) | Value::Regex(_)) {
        return (
            Err(CompiledQueryError::Retrieval("text is not locatable".into())),
            warnings,
        );
    }
    let mut context = typst_core::compiler::eval::EvalContext::new();
    context.introspector = introspector;
    let args = typst_core::entities::args::Args::positional(vec![value]);
    let queried = typst_core::compiler::stdlib::native_query(
        &mut context,
        &args,
        world,
        source.id(),
    );
    let result = match queried {
        Ok(Value::Array(mut values)) => {
            let labelled = context.introspector.query_labelled();
            for value in &mut values {
                if let Value::LocatedContent(content, location) = value {
                    if content.fields().is_none()
                        && !matches!(content.content(), Content::Label(_))
                    {
                        if let Some((label, _)) =
                            labelled.iter().find(|(_, found)| found == location)
                        {
                            *content =
                                typst_core::entities::value::IntrospectedContent::new(
                                    Content::label_auto(
                                        label.0.as_str(),
                                        content.content().clone(),
                                    ),
                                    None,
                                );
                        }
                    }
                }
            }
            Ok(values)
        }
        Ok(_) => {
            Err(CompiledQueryError::Retrieval("query result is not an array".into()))
        }
        Err(errors) => Err(CompiledQueryError::Retrieval(messages(errors))),
    };
    (result, warnings)
}

/// Executa uma query locatável e devolve os elementos reais em ordem.
pub fn query_elements(
    world: &dyn World,
    source: &Source,
    selector: &str,
) -> (Result<Vec<Value>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>) {
    let parsed = match parse_selector(selector) {
        Ok(parsed) => parsed,
        Err(error) => {
            let message = match error {
                QueryError::InvalidSelector(message)
                    if message == "text is not locatable" =>
                {
                    message
                }
                other => other.to_string(),
            };
            return (
                Err(vec![SourceDiagnostic::error(Span::detached(), message)]),
                Vec::new(),
            );
        }
    };
    let (evaluated, warnings) = eval_to_module_with_sink(world, source);
    let module = match evaluated {
        Ok(module) => module,
        Err(errors) => return (Err(errors), warnings),
    };
    let Some(content) = module.content() else {
        return (Ok(Vec::new()), warnings);
    };
    let locations_for = |intr: &TagIntrospector| match &parsed {
        ParsedSelector::Kind(kind) => intr.query_by_kind(*kind),
        ParsedSelector::Label(label) => {
            intr.query_by_label(&Label(label.clone())).into_iter().collect()
        }
    };
    let fallback;
    let intr = match module.realization_introspector() {
        Some(intr) => intr,
        None => {
            fallback = introspect(module.introspection_content().unwrap_or(content));
            &fallback
        }
    };
    let locations = locations_for(intr);
    let labelled = intr.query_labelled();
    let mut elements = Vec::with_capacity(locations.len());
    for location in locations {
        match intr.elements.get(&location) {
            Some(element) => {
                if element.fields().is_some() {
                    elements.push(Value::LocatedContent(element.clone(), location));
                    continue;
                }
                let label = match &parsed {
                    ParsedSelector::Label(label) => Some(label.as_str()),
                    ParsedSelector::Kind(_) => labelled
                        .iter()
                        .find(|(_, labelled_location)| *labelled_location == location)
                        .map(|(label, _)| label.0.as_str()),
                };
                let content = match label {
                    Some(label) => Content::label_auto(label, element.content().clone()),
                    None => element.content().clone(),
                };
                elements.push(Value::LocatedContent(
                    typst_core::entities::value::IntrospectedContent::new(content, None),
                    location,
                ));
            }
            None => {
                return (
                    Err(vec![SourceDiagnostic::error(
                        Span::detached(),
                        "query result has no locatable element",
                    )]),
                    warnings,
                )
            }
        }
    }
    (Ok(elements), warnings)
}

/// Plain text representation of a `Value` for metadata
/// summarization. Para `Content`, usa `plain_text`; para
/// `Dict`/`Array`, usa `Debug`. Estável o suficiente para
/// comparação cristalino vs vanilla (vanilla output é
/// nested JSON; cristalino aqui produz string).
fn value_plain_text(v: &Value) -> String {
    match v {
        Value::None => "none".to_string(),
        Value::Auto => "auto".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Int(i) => i.to_string(),
        Value::Float(f) => f.to_string(),
        Value::Str(s) => s.to_string(),
        Value::Content(c) => c.plain_text(),
        // Variants opacos: Debug é estável o suficiente
        // para parity comparison (vanilla produz JSON
        // estruturado distinto; cristalino faz match por
        // contagem e ordem, não por estrutura literal).
        other => format!("{:?}", other),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── Tests de parse_selector ────────────────────────

    #[test]
    fn p206c_parse_selector_kind_basico() {
        let parsed = parse_selector("heading").unwrap();
        assert_eq!(parsed, ParsedSelector::Kind(ElementKind::Heading));
    }

    #[test]
    fn p206c_parse_selector_kind_metadata() {
        let parsed = parse_selector("metadata").unwrap();
        assert_eq!(parsed, ParsedSelector::Kind(ElementKind::Metadata));
    }

    #[test]
    fn p206c_parse_selector_kind_figure() {
        let parsed = parse_selector("figure").unwrap();
        assert_eq!(parsed, ParsedSelector::Kind(ElementKind::Figure));
    }

    #[test]
    fn p206c_parse_selector_label_basico() {
        let parsed = parse_selector("<fig-alfa>").unwrap();
        assert_eq!(parsed, ParsedSelector::Label("fig-alfa".to_string()));
    }

    #[test]
    fn p206c_parse_selector_label_com_hifen() {
        let parsed = parse_selector("<eq-pitagoras>").unwrap();
        assert_eq!(parsed, ParsedSelector::Label("eq-pitagoras".to_string()));
    }

    #[test]
    fn p206c_parse_selector_complexo_rejeitado() {
        // Vanilla aceita `heading.where(level: 1)`; cristalino rejeita.
        let err = parse_selector("heading.where(level: 1)").unwrap_err();
        match err {
            QueryError::InvalidSelector(_) => (),
            other => panic!("expected InvalidSelector, got {:?}", other),
        }
    }

    #[test]
    fn p206c_parse_selector_kind_desconhecido_rejeitado() {
        let err = parse_selector("unknown_kind").unwrap_err();
        match err {
            QueryError::InvalidSelector(msg) => assert!(msg.contains("unknown kind")),
            other => panic!("expected InvalidSelector, got {:?}", other),
        }
    }

    #[test]
    fn p206c_parse_selector_vazio_rejeitado() {
        let err = parse_selector("").unwrap_err();
        match err {
            QueryError::InvalidSelector(_) => (),
            other => panic!("expected InvalidSelector, got {:?}", other),
        }
    }

    #[test]
    fn p206c_parse_selector_label_vazio_rejeitado() {
        let err = parse_selector("<>").unwrap_err();
        match err {
            QueryError::InvalidSelector(msg) => assert!(msg.contains("empty")),
            other => panic!("expected InvalidSelector, got {:?}", other),
        }
    }

    // ── Tests de summarize_query (end-to-end via query_to_summary) ──
    //
    // Usa source minimal + SystemWorld; sub-stores L1 são populados
    // por `introspect()` real. Evita dependência de APIs `pub(crate)`
    // (LabelRegistry::add, MetadataStore::add, Location::from_raw).

    use crate::world::SystemWorld;
    use std::path::{Path, PathBuf};
    use typst_core::contracts::world::World;

    struct TempDir(PathBuf);

    impl TempDir {
        fn path(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    fn tempdir() -> TempDir {
        let path = std::env::temp_dir().join(format!(
            "typst-p206c-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.subsec_nanos())
                .unwrap_or(0)
        ));
        std::fs::create_dir_all(&path).unwrap();
        TempDir(path)
    }

    fn world_from_str(src: &str) -> (SystemWorld, TempDir) {
        let dir = tempdir();
        std::fs::write(dir.path().join("main.typ"), src).unwrap();
        let world = SystemWorld::new(dir.path(), "main.typ").unwrap();
        (world, dir)
    }

    fn run_query(src: &str, selector: &str) -> Result<QuerySummary, QueryError> {
        let (world, _dir) = world_from_str(src);
        let source = world.source(world.main()).unwrap();
        query_to_summary(&world, &source, selector)
    }

    #[test]
    fn p206c_query_kind_heading_count() {
        let src = "= Primeiro\n\n= Segundo\n\n= Terceiro\n";
        let summary = run_query(src, "heading").unwrap();
        assert_eq!(summary.count, 3);
        assert_eq!(summary.kind, SelectorKind::Kind);
        assert_eq!(summary.kind_name.as_deref(), Some("heading"));
        assert!(summary.label_found.is_none());
        assert!(summary.metadata_values.is_empty());
    }

    #[test]
    fn p206c_query_kind_metadata_values() {
        let src = "#metadata(\"primeiro\")\n\n#metadata(\"segundo\")\n";
        let summary = run_query(src, "metadata").unwrap();
        assert_eq!(summary.count, 2);
        assert_eq!(summary.metadata_values.len(), 2);
        assert!(summary.metadata_values.iter().any(|v| v == "primeiro"));
        assert!(summary.metadata_values.iter().any(|v| v == "segundo"));
    }

    #[test]
    fn p206c_query_kind_vazio() {
        // Source sem headings; query devolve count 0.
        let src = "Apenas texto plano.\n";
        let summary = run_query(src, "heading").unwrap();
        assert_eq!(summary.count, 0);
        assert_eq!(summary.kind_name.as_deref(), Some("heading"));
    }

    #[test]
    fn p206c_query_invalid_selector_propaga_erro() {
        let src = "= Heading\n";
        let err = run_query(src, "weird.where(level: 1)").unwrap_err();
        match err {
            QueryError::InvalidSelector(_) => (),
            other => panic!("expected InvalidSelector, got {:?}", other),
        }
    }

    // ── P480 — math.equation alias ────────────────────────────────────────────

    #[test]
    fn p480_parse_selector_math_equation_resolve_equation_kind() {
        // P480 — `math.equation` é o namespace vanilla para o selector de
        // equações. parse_selector aceita este alias e mapeia para
        // ElementKind::Equation (vanilla rejeita `equation` standalone).
        let parsed = parse_selector("math.equation").unwrap();
        assert_eq!(
            parsed,
            ParsedSelector::Kind(ElementKind::Equation),
            "P480: math.equation deve resolver para ElementKind::Equation",
        );
    }

    #[test]
    fn p480_parse_selector_equation_standalone_ainda_aceito() {
        // P480 — `equation` standalone continua a funcionar em cristalino
        // (compatibilidade interna). Não é alterado.
        let parsed = parse_selector("equation").unwrap();
        assert_eq!(parsed, ParsedSelector::Kind(ElementKind::Equation));
    }

    // ── P494 — Selectors de elementos de documento ───────────────────────

    #[test]
    fn p494_parse_selector_list() {
        assert_eq!(
            parse_selector("list").unwrap(),
            ParsedSelector::Kind(ElementKind::List)
        );
    }

    #[test]
    fn p494_parse_selector_enum() {
        assert_eq!(
            parse_selector("enum").unwrap(),
            ParsedSelector::Kind(ElementKind::Enum)
        );
    }

    #[test]
    fn p494_parse_selector_par() {
        assert_eq!(
            parse_selector("par").unwrap(),
            ParsedSelector::Kind(ElementKind::Par)
        );
    }

    #[test]
    fn p494_parse_selector_link_raw_quote_footnote() {
        assert_eq!(
            parse_selector("link").unwrap(),
            ParsedSelector::Kind(ElementKind::Link)
        );
        assert_eq!(
            parse_selector("raw").unwrap(),
            ParsedSelector::Kind(ElementKind::Raw)
        );
        assert_eq!(
            parse_selector("quote").unwrap(),
            ParsedSelector::Kind(ElementKind::Quote)
        );
        assert_eq!(
            parse_selector("footnote").unwrap(),
            ParsedSelector::Kind(ElementKind::Footnote)
        );
    }

    #[test]
    fn p494_query_list_count() {
        let summary = run_query("#list([A], [B])", "list").unwrap();
        assert_eq!(summary.count, 1);
        assert_eq!(summary.kind_name.as_deref(), Some("list"));
    }

    #[test]
    fn p494_query_enum_count() {
        let summary = run_query("#enum([A], [B])", "enum").unwrap();
        assert_eq!(summary.count, 1);
        assert_eq!(summary.kind_name.as_deref(), Some("enum"));
    }

    #[test]
    fn p494_query_par_count() {
        let summary = run_query("#set par(leading: 1.5em)\nParágrafo.", "par").unwrap();
        assert_eq!(summary.count, 1);
        assert_eq!(summary.kind_name.as_deref(), Some("par"));
    }

    #[test]
    fn p494_query_link_count() {
        let summary = run_query("#link(\"https://x.pt\")[sítio]", "link").unwrap();
        assert_eq!(summary.count, 1);
        assert_eq!(summary.kind_name.as_deref(), Some("link"));
    }

    #[test]
    fn p494_query_raw_count() {
        let summary = run_query("```rust\nfn main() {}\n```", "raw").unwrap();
        assert_eq!(summary.count, 1);
        assert_eq!(summary.kind_name.as_deref(), Some("raw"));
    }

    #[test]
    fn p494_query_quote_count() {
        let summary = run_query("#quote[Texto]", "quote").unwrap();
        assert_eq!(summary.count, 1);
        assert_eq!(summary.kind_name.as_deref(), Some("quote"));
    }

    #[test]
    fn p494_query_footnote_count() {
        let summary = run_query("Texto#footnote[Nota]", "footnote").unwrap();
        assert_eq!(summary.count, 1);
        assert_eq!(summary.kind_name.as_deref(), Some("footnote"));
    }

    fn p1285_query_elements(src: &str, selector: &str) -> Vec<Content> {
        let (world, _dir) = world_from_str(src);
        let source = world.source(world.main()).unwrap();
        let (result, warnings) = query_elements(&world, &source, selector);
        assert!(warnings.is_empty(), "warnings inesperados: {warnings:?}");
        result
            .unwrap_or_else(|diagnostics| panic!("query falhou: {diagnostics:?}"))
            .into_iter()
            .map(|value| match value {
                Value::LocatedContent(content, _) => content.into_content(),
                other => panic!("expected located query content, got {other:?}"),
            })
            .collect()
    }

    #[test]
    fn p1285_query_elements_recupera_metadata_e_figure_na_ordem() {
        let src = "#metadata((name: \"x\", n: 2))\n#figure[one]\n";

        let metadata = p1285_query_elements(src, "metadata");
        assert_eq!(metadata.len(), 1);
        assert!(matches!(metadata[0], Content::Metadata(_)));

        let figures = p1285_query_elements(src, "figure");
        assert_eq!(figures.len(), 1);
        assert!(matches!(figures[0], Content::Figure(_)));
    }

    #[test]
    fn p1285_query_elements_transporta_label_sem_criar_match_extra() {
        let elements = p1285_query_elements("#figure[one] <fig-one>\n", "<fig-one>");
        assert_eq!(elements.len(), 1);
        let Content::Label(label) = &elements[0] else {
            panic!("label deve envolver o elemento original: {:?}", elements[0]);
        };
        assert_eq!(label.name.as_str(), "fig-one");
        assert!(matches!(label.body, Content::Figure(_)));
    }

    #[test]
    fn p1436_math_lr_permanece_terminal_para_query_textual_l3() {
        let lr = Content::math_lr(
            Content::text("hidden"),
            typst_core::entities::rel::Rel::from_percent(100.0),
            false,
        );
        assert!(!has_any_text(&lr));
    }
    #[test]
    fn p1494_espacos_nao_criam_texto_ou_ocorrencia_queryable() {
        for space in [Content::Space, Content::SpaceFromNewline] {
            assert!(!has_any_text(&space));
            assert_eq!(count_variant(&space, |node| matches!(node, Content::Text(_))), 0);
            let neighbors = Content::sequence(vec![
                Content::text("before"),
                space,
                Content::text("after"),
            ]);
            assert!(has_any_text(&neighbors));
            assert_eq!(
                count_variant(&neighbors, |node| matches!(node, Content::Text(_))),
                2
            );
        }
    }
}

#[cfg(test)]
mod compiled_query_tests {
    use super::*;
    use crate::world::SystemWorld;
    #[test]
    fn compiled_query_distinguishes_document_and_retrieval_and_retains_context_values() {
        let root = std::env::current_dir().unwrap();
        let world = SystemWorld::for_stdin(root.clone(), "#unknown".into());
        let source = world.source(world.main()).unwrap();
        let (result, _) = query_compiled_elements(&world, &source, "unknown_selector");
        assert!(matches!(result, Err(CompiledQueryError::Compilation(_))));
        let world =
            SystemWorld::for_stdin(root.clone(), "#context metadata(\"x\")".into());
        let source = world.source(world.main()).unwrap();
        let (result, _) = query_compiled_elements(&world, &source, "metadata");
        let values = result.unwrap();
        assert_eq!(values.len(), 1);
        assert!(matches!(&values[0], Value::LocatedContent(_, _)));
        let (result, _) = query_compiled_elements(&world, &source, "unknown_selector");
        assert!(
            matches!(result, Err(CompiledQueryError::Retrieval(message)) if message == "failed to evaluate selector: unknown variable `unknown_selector`")
        );
        let (result, _) = query_compiled_elements(&world, &source, "\"x\"");
        assert!(
            matches!(result, Err(CompiledQueryError::Retrieval(message)) if message == "text is not locatable")
        );
    }
}

#[cfg(test)]
mod batch_g_projection_tests {
    use super::*;
    use std::sync::Arc;
    use typst_core::entities::style::Styles;

    #[test]
    fn batch_g_raw_empty_group_is_distinct_from_canonical_empty() {
        let raw = Content::Sequence(Arc::from([]));
        let canonical = Content::sequence(Vec::new());
        for kind in [ElementKind::List, ElementKind::Enum] {
            assert_eq!(count_element_in_content(&raw, kind), 1);
            assert_eq!(count_element_in_content(&canonical, kind), 0);
        }
    }

    #[test]
    fn batch_g_approximate_group_boundaries_remain_explicit() {
        let list = Content::list_item(Content::text("item"));
        let enumeration = Content::enum_item(None, Content::text("item"));
        for (kind, item) in [(ElementKind::List, list), (ElementKind::Enum, enumeration)]
        {
            let group = Content::Sequence(vec![item.clone()].into());
            assert_eq!(count_element_in_content(&item, kind), 0);
            assert_eq!(count_element_in_content(&group, kind), 1);
            assert_eq!(
                count_element_in_content(
                    &Content::styled(group.clone(), Styles::default()),
                    kind,
                ),
                0,
            );
            let mixed = Content::sequence(vec![
                Content::text("before"),
                group.clone(),
                Content::text("after"),
            ]);
            assert_eq!(count_element_in_content(&mixed, kind), 1);
        }
    }

    #[test]
    fn batch_g_nested_group_bodies_keep_their_legacy_projection() {
        let list_group =
            Content::Sequence(vec![Content::list_item(Content::text("nested"))].into());
        let enum_group = Content::Sequence(
            vec![Content::enum_item(None, Content::text("nested"))].into(),
        );
        assert_eq!(
            count_element_in_content(&Content::list_item(list_group), ElementKind::List,),
            1,
        );
        assert_eq!(
            count_element_in_content(
                &Content::enum_item(None, enum_group),
                ElementKind::Enum,
            ),
            1,
        );
    }

    #[test]
    fn batch_g_real_introspector_summary_does_not_use_direct_zero_projection() {
        let content = Content::sequence(vec![
            Content::heading(1, Content::text("First")),
            Content::heading(1, Content::text("Second")),
            Content::metadata(Value::Str("value".into())),
        ]);
        let intr = introspect(&content);
        for (kind, count) in [(ElementKind::Heading, 2), (ElementKind::Metadata, 1)] {
            assert_eq!(count_element_in_content(&content, kind), 0);
            let summary = summarize_query(
                &intr,
                &content,
                &ParsedSelector::Kind(kind),
                kind.as_str(),
            );
            assert_eq!(summary.count, count);
            assert_eq!(summary.selector, kind.as_str());
            if kind == ElementKind::Metadata {
                assert_eq!(summary.metadata_values, ["value"]);
            }
        }
    }
}
