//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/query-helpers.md
//! @prompt-hash d8de80fb
//! @layer L3
//! @updated 2026-05-08
//!
//! **P206C (Vanilla integration)** — helper L3 para
//! comparação estrutural cristalino vs vanilla via JSON
//! shape compatível com `typst query`.
//!
//! Per ADR-0075 PROPOSTO + P206C C2 = Caminho B
//! (helper em workspace cristalino, não subcomando CLI).
//!
//! Pipeline: source → eval → content → introspect →
//! query → `QuerySummary`. Selector parsing aceita Kind
//! names (10 ElementKind variants) + label syntax
//! `<label>`.
//!
//! Caminho A (subcomando CLI em `04_wiring/`) deferred
//! para sub-passo dedicado pós-P206 — refactor
//! cross-modular era L magnitude.

use typst_core::contracts::world::World;
use typst_core::entities::content::Content;
use typst_core::entities::element_kind::ElementKind;
use typst_core::entities::introspector::{Introspector, TagIntrospector};
use typst_core::entities::label::Label;
use typst_core::entities::source::Source;
use typst_core::entities::value::Value;
use typst_core::rules::introspect::introspect;

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
            QueryError::NoContent     => write!(f, "module has no content"),
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
                intr.query_metadata()
                    .iter()
                    .map(value_plain_text)
                    .collect()
            } else {
                Vec::new()
            };
            QuerySummary {
                selector:        raw_selector.to_string(),
                kind:            SelectorKind::Kind,
                count,
                kind_name:       Some(kind.as_str().to_string()),
                label_found:     None,
                metadata_values,
            }
        }
        ParsedSelector::Label(label_str) => {
            let label = Label(label_str.clone());
            let location = intr.query_by_label(&label);
            QuerySummary {
                selector:        raw_selector.to_string(),
                kind:            SelectorKind::Label,
                count:           if location.is_some() { 1 } else { 0 },
                kind_name:       None,
                label_found:     location.map(|_| label_str.clone()),
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
///   único `ListItem` na raiz. Conta 1 por lista.
/// - `Enum`: análogo a `List` para `EnumItem`.
/// - `Par`: retorna 1 se o documento contiver qualquer `Content::Text`
///   (aproximação — cristalino não materializa `ParElem`).
/// - `Link`/`Raw`/`Quote`/`Footnote`: conta variantes directas.
pub fn count_element_in_content(content: &Content, kind: ElementKind) -> usize {
    match kind {
        ElementKind::List => count_list_groups(content),
        ElementKind::Enum => count_enum_groups(content),
        ElementKind::Par => if has_any_text(content) { 1 } else { 0 },
        ElementKind::Link => count_variant(content, |c| matches!(c, Content::Link(_))),
        ElementKind::Raw => count_variant(content, |c| matches!(c, Content::Raw(_))),
        ElementKind::Quote => count_variant(content, |c| matches!(c, Content::Quote(_))),
        ElementKind::Footnote => count_variant(content, |c| matches!(c, Content::Footnote(_))),
        _ => 0,
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
        _ => 0,
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
        _ => 0,
    }
}

fn has_any_text(content: &Content) -> bool {
    match content {
        Content::Text(_) => true,
        Content::Sequence(seq) => seq.iter().any(has_any_text),
        Content::ListItem(item) => has_any_text(&item.body),
        Content::EnumItem(item) => has_any_text(&item.body),
        Content::Link(link) => has_any_text(&link.body),
        Content::Quote(quote) => has_any_text(&quote.body),
        Content::Footnote(footnote) => has_any_text(&footnote.body),
        // Contentores comuns que podem embrulhar texto.
        Content::Styled(body, _) => has_any_text(body),
        Content::Strong(strong) => has_any_text(&strong.body),
        Content::Emph(emph) => has_any_text(&emph.body),
        Content::Title(title) => has_any_text(&title.body),
        Content::Align(align) => has_any_text(&align.body),
        Content::Pad(pad) => has_any_text(&pad.body),
        Content::Hide(hide) => has_any_text(&hide.body),
        Content::Block(block) => has_any_text(&block.body),
        Content::Boxed(boxed) => has_any_text(&boxed.body),
        Content::Stack(stack) => stack.children.iter().any(has_any_text),
        Content::Grid(grid) => {
            let header_text = grid.header.as_ref().map_or(false, has_any_text);
            let footer_text = grid.footer.as_ref().map_or(false, has_any_text);
            header_text || grid.cells.iter().any(has_any_text) || footer_text
        }
        Content::GridCell(cell) => has_any_text(&cell.body),
        Content::Table(table) => {
            let caption_text = table.caption.as_ref().map_or(false, |c| has_any_text(c));
            caption_text || table.children.iter().any(has_any_text)
        }
        Content::TableHeader(header) => has_any_text(&header.body),
        Content::TableFooter(footer) => has_any_text(&footer.body),
        Content::TableCell(cell) => has_any_text(&cell.body),
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
        Content::Bibliography(bib) => bib.title.as_ref().map_or(false, has_any_text),
        Content::Label(label) => has_any_text(&label.body),
        Content::Heading(_)
        | Content::Metadata(_) | Content::State(_) | Content::StateUpdate(_)
        | Content::StateDisplay(_) | Content::CounterDisplayCallback(_) | Content::CounterDisplay(_)
        | Content::CounterUpdate(_) | Content::Outline(_) | Content::Cite(_)
        | Content::Raw(_) | Content::Ref(_) | Content::Image(_)
        | Content::Shape { .. } | Content::Curve(_)
        | Content::Divider(_)
        | Content::Linebreak(_) | Content::HSpace(_) | Content::VSpace(_)
        | Content::Pagebreak(_) | Content::Colbreak(_)
        | Content::MathSequence(_) | Content::MathIdent(_) | Content::MathText(_)
        | Content::MathFrac(_) | Content::MathAttach(_) | Content::MathRoot(_)
        | Content::MathDelimited(_) | Content::MathAlignPoint(_)
        | Content::MathMatrix(_) | Content::MathCases(_)
        | Content::MathAccent(_) | Content::MathCancel(_) | Content::MathClassOverride(_)
        | Content::MathUnderover(_) | Content::MathOp(_) | Content::MathStyled(_)
        | Content::SmartQuote(_) | Content::Dynamic(_)
        | Content::GridHeader(_) | Content::GridFooter(_) | Content::SetPage { .. }
        | Content::ContextBlock(_)
        | Content::GridHLine(_) | Content::GridVLine(_) | Content::TableHLine(_) | Content::TableVLine(_)
        // P622: Parbreak não contém texto.
        | Content::Empty | Content::Space | Content::Parbreak | Content::Document { .. } | Content::Asset { .. } => false,
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
        Content::Sequence(seq) => seq.iter().map(|c| count_variant(c, predicate)).sum(),
        Content::ListItem(item) => count_variant(&item.body, predicate),
        Content::EnumItem(item) => count_variant(&item.body, predicate),
        Content::Quote(quote) => count_variant(&quote.body, predicate),
        Content::Footnote(footnote) => count_variant(&footnote.body, predicate),
        Content::Styled(body, _) => count_variant(body, predicate),
        Content::Strong(strong) => count_variant(&strong.body, predicate),
        Content::Emph(emph) => count_variant(&emph.body, predicate),
        Content::Title(title) => count_variant(&title.body, predicate),
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
                + table.children.iter().map(|c| count_variant(c, predicate)).sum::<usize>()
        }
        Content::TableHeader(header) => count_variant(&header.body, predicate),
        Content::TableFooter(footer) => count_variant(&footer.body, predicate),
        Content::TableCell(cell) => count_variant(&cell.body, predicate),
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
        Content::Heading(_) | Content::Link(_) | Content::Raw(_) | Content::Metadata(_)
        | Content::State(_) | Content::StateUpdate(_) | Content::StateDisplay(_)
        | Content::CounterDisplayCallback(_) | Content::CounterDisplay(_)
        | Content::CounterUpdate(_) | Content::Outline(_) | Content::Cite(_)
        | Content::Ref(_) | Content::Image(_) | Content::Shape { .. } | Content::Curve(_)
        | Content::Divider(_) | Content::Linebreak(_) | Content::HSpace(_)
        | Content::VSpace(_) | Content::Pagebreak(_) | Content::Colbreak(_)
        | Content::MathSequence(_) | Content::MathIdent(_) | Content::MathText(_)
        | Content::MathFrac(_) | Content::MathAttach(_) | Content::MathRoot(_)
        | Content::MathDelimited(_) | Content::MathAlignPoint(_)
        | Content::MathMatrix(_) | Content::MathCases(_)
        | Content::MathAccent(_) | Content::MathCancel(_) | Content::MathClassOverride(_)
        | Content::MathUnderover(_) | Content::MathOp(_) | Content::MathStyled(_)
        | Content::SmartQuote(_) | Content::Dynamic(_)
        | Content::GridHeader(_) | Content::GridFooter(_) | Content::SetPage { .. }
        | Content::ContextBlock(_)
        | Content::GridHLine(_) | Content::GridVLine(_) | Content::TableHLine(_) | Content::TableVLine(_)
        // P622: Parbreak é leaf estrutural — não conta para a variante.
        | Content::Empty | Content::Space | Content::Parbreak | Content::Text(_)
        | Content::Document { .. } | Content::Asset { .. } => 0,
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
    let intr = introspect(intr_content);
    Ok(summarize_query(&intr, content, &parsed, selector))
}

/// Plain text representation of a `Value` for metadata
/// summarization. Para `Content`, usa `plain_text`; para
/// `Dict`/`Array`, usa `Debug`. Estável o suficiente para
/// comparação cristalino vs vanilla (vanilla output é
/// nested JSON; cristalino aqui produz string).
fn value_plain_text(v: &Value) -> String {
    match v {
        Value::None       => "none".to_string(),
        Value::Auto       => "auto".to_string(),
        Value::Bool(b)    => b.to_string(),
        Value::Int(i)     => i.to_string(),
        Value::Float(f)   => f.to_string(),
        Value::Str(s)     => s.to_string(),
        Value::Content(c) => c.plain_text(),
        // Variants opacos: Debug é estável o suficiente
        // para parity comparison (vanilla produz JSON
        // estruturado distinto; cristalino faz match por
        // contagem e ordem, não por estrutura literal).
        other             => format!("{:?}", other),
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

    use std::path::{Path, PathBuf};
    use typst_core::contracts::world::World;
    use crate::world::SystemWorld;

    struct TempDir(PathBuf);

    impl TempDir {
        fn path(&self) -> &Path { &self.0 }
    }

    impl Drop for TempDir {
        fn drop(&mut self) { let _ = std::fs::remove_dir_all(&self.0); }
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
        assert_eq!(parse_selector("list").unwrap(), ParsedSelector::Kind(ElementKind::List));
    }

    #[test]
    fn p494_parse_selector_enum() {
        assert_eq!(parse_selector("enum").unwrap(), ParsedSelector::Kind(ElementKind::Enum));
    }

    #[test]
    fn p494_parse_selector_par() {
        assert_eq!(parse_selector("par").unwrap(), ParsedSelector::Kind(ElementKind::Par));
    }

    #[test]
    fn p494_parse_selector_link_raw_quote_footnote() {
        assert_eq!(parse_selector("link").unwrap(), ParsedSelector::Kind(ElementKind::Link));
        assert_eq!(parse_selector("raw").unwrap(), ParsedSelector::Kind(ElementKind::Raw));
        assert_eq!(parse_selector("quote").unwrap(), ParsedSelector::Kind(ElementKind::Quote));
        assert_eq!(parse_selector("footnote").unwrap(), ParsedSelector::Kind(ElementKind::Footnote));
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
}
