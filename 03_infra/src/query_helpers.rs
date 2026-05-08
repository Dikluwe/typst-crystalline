//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/query-helpers.md
//! @prompt-hash 51294329
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
    if trimmed.contains('.') || trimmed.contains('(') {
        return Err(QueryError::InvalidSelector(format!(
            "complex selectors (e.g. `kind.where(...)`) not supported in P206C; got `{}`",
            trimmed
        )));
    }
    match ElementKind::from_name(trimmed) {
        Some(kind) => Ok(ParsedSelector::Kind(kind)),
        None       => Err(QueryError::InvalidSelector(format!(
            "unknown kind name `{}`; expected one of: heading, figure, citation, metadata, state, state_update, outline, bibliography, equation, counter_update",
            trimmed
        ))),
    }
}

/// Sumariza o resultado de aplicar `parsed` a `intr`.
///
/// Pure function — sem I/O. Útil para callers que já
/// têm um `TagIntrospector` construído.
pub fn summarize_query(
    intr: &TagIntrospector,
    parsed: &ParsedSelector,
    raw_selector: &str,
) -> QuerySummary {
    match parsed {
        ParsedSelector::Kind(kind) => {
            let locations = intr.query_by_kind(*kind);
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
                count:           locations.len(),
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
    let intr = introspect(content);
    Ok(summarize_query(&intr, &parsed, selector))
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
}
