//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/selector_matching.md
//! @prompt-hash 3b286537
//! @layer L1
//! @updated 2026-08-12
//!
//! Matching de selectores de show rule contra conteúdo; conversão de query
//! selector para show selector. Extraído de `compiler/eval/rules.rs` no Passo
//! 1011 conforme ADR-0109 (atomização — forma B, free function no arquivo da
//! unidade).

use crate::entities::content::Content;
use crate::entities::element_kind::ElementKind;
use crate::entities::selector::Selector as QuerySelector;
use crate::entities::show::{NodeKind, Selector};
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

/// **P417 (M)** — Converte um `entities::selector::Selector` (query)
/// para um `entities::show::Selector` (show rule). Apenas `Kind` e
/// `Where` sobre `Kind` de elementos nativos suportados são convertidos;
/// outros selectors são scope-out com erro claro.
pub(crate) fn query_selector_to_show_selector(
    sel: QuerySelector,
    span: Span,
) -> SourceResult<Selector> {
    fn kind_to_node(kind: ElementKind) -> Option<NodeKind> {
        match kind {
            ElementKind::Heading => Some(NodeKind::Heading),
            ElementKind::Figure => Some(NodeKind::Figure),
            ElementKind::Link => Some(NodeKind::Link),
            ElementKind::Raw => Some(NodeKind::Raw),
            ElementKind::Quote => Some(NodeKind::Quote),
            ElementKind::Footnote => Some(NodeKind::Footnote),
            ElementKind::List => Some(NodeKind::List),
            ElementKind::Enum => Some(NodeKind::Enum),
            _ => None,
        }
    }

    match sel {
        QuerySelector::Kind(kind) => match kind_to_node(kind) {
            Some(node) => Ok(Selector::NodeKind(node)),
            None => Err(vec![SourceDiagnostic::error(
                span,
                format!(
                    "selector de kind '{}' não é suportado em show rule",
                    kind.as_str()
                ),
            )]),
        },
        // P493c — Where encadeado: converte recursivamente a base.
        QuerySelector::Where { base, field, value } => {
            let base_sel = query_selector_to_show_selector(*base, span)?;
            if is_node_rule(&base_sel) {
                Ok(Selector::Where { base: Box::new(base_sel), field, value })
            } else {
                Err(vec![SourceDiagnostic::error(
                    span,
                    "selector where com base não-node não suportado em show rule"
                        .to_string(),
                )])
            }
        }
        // **P423 (S-M)** — combinadores And/Or convertidos recursivamente.
        QuerySelector::And(sels) => {
            let mut converted = Vec::with_capacity(sels.len());
            for s in sels.iter() {
                converted.push(query_selector_to_show_selector(s.clone(), span)?);
            }
            Ok(Selector::And(converted))
        }
        QuerySelector::Or(sels) => {
            let mut converted = Vec::with_capacity(sels.len());
            for s in sels.iter() {
                converted.push(query_selector_to_show_selector(s.clone(), span)?);
            }
            Ok(Selector::Or(converted))
        }
        _ => Err(vec![SourceDiagnostic::error(
            span,
            "selector não suportado em show rule".to_string(),
        )]),
    }
}

/// Casa um nó contra um selector de element rule (`NodeKind`/`DynKind`).
/// **Partilhado** (P352) pelo loop α (transformação func/content) e pela passagem
/// de show-set: ambos usam exatamente o mesmo critério de match. `Selector::Text`
/// nunca casa aqui (tratado por `map_text`).
pub(crate) fn selector_matches(work: &Content, selector: &Selector) -> bool {
    // F-5b fatia 1 (P371): `show strong/emph` casam as **variantes próprias**
    // `Content::Strong`/`Emph` (S1, por tipo). P431 (DEBT-50) acrescenta a
    // distinção de origem no `Style` para o cenário pós-bake-in (wrapping):
    // um `Content::Styled` com `Bold { from_strong: true }` casa `show strong`,
    // mas `Bold { from_strong: false }` (de `#set text(bold)`) não casa.
    match selector {
        Selector::NodeKind(kind) => {
            matches!(
                (work, kind),
                (Content::Heading(_), NodeKind::Heading)
                | (Content::Figure(_), NodeKind::Figure)
                | (Content::Raw { .. }, NodeKind::Raw)
                | (Content::Equation { .. }, NodeKind::Equation)
                | (Content::Par { .. }, NodeKind::Par)
                | (Content::ListItem(_), NodeKind::ListItem)
                | (Content::Strong(_), NodeKind::Strong)
                | (Content::Emph(_), NodeKind::Emph)
                | (Content::Underline(_), NodeKind::Underline)
                | (Content::Strike(_), NodeKind::Strike)
                | (Content::Overline(_), NodeKind::Overline)
                | (Content::SmallCaps { .. }, NodeKind::Smallcaps)
                // P494
                | (Content::Link(_), NodeKind::Link)
                | (Content::Quote(_), NodeKind::Quote)
                | (Content::Footnote(_), NodeKind::Footnote)
            ) || matches!(
                (work, kind),
                (_, NodeKind::List)
                    if matches!(work, Content::Sequence(seq) if seq.iter().all(|c| matches!(c, Content::ListItem(_))))
                    || matches!(work, Content::ListItem(_))
            ) || matches!(
                (work, kind),
                (_, NodeKind::Enum)
                    if matches!(work, Content::Sequence(seq) if seq.iter().all(|c| matches!(c, Content::EnumItem(_))))
                    || matches!(work, Content::EnumItem(_))
            ) || matches!(
                (work, kind),
                (_, NodeKind::Strong) if is_styled_origin(work, true, false, false, false, false)
            ) || matches!(
                (work, kind),
                (_, NodeKind::Emph) if is_styled_origin(work, false, true, false, false, false)
            ) || matches!(
                (work, kind),
                (_, NodeKind::Subscript) if is_styled_origin(work, false, false, true, false, false)
            ) || matches!(
                (work, kind),
                (_, NodeKind::Superscript) if is_styled_origin(work, false, false, false, true, false)
            ) || matches!(
                (work, kind),
                (_, NodeKind::Highlight) if is_styled_origin(work, false, false, false, false, true)
            )
        }
        Selector::DynKind(name) => {
            matches!(work, Content::Dynamic(e) if e.dyn_kind() == name)
        }
        Selector::Text(_) => false,
        Selector::Regex(_) => false,
        // P791 — regras de label NÃO casam na travessia principal: são
        // aplicadas por `intercept_labelled` no ponto de associação
        // retroactiva (evita dupla aplicação das outras regras e garante
        // `it` = corpo, não o wrapper).
        Selector::Label(_) => false,
        Selector::Where { base, field, value } => {
            selector_matches(work, base)
                && work
                    .get_field(field.as_str())
                    .map(|actual| values_eq_semantic(&actual, value.as_ref()))
                    .unwrap_or(false)
        }
        // **P423 (S-M)** — combinadores And/Or com curto-circuito.
        // And/Or vazios retornam `false` (Opção A fixada em P209C/P423).
        Selector::And(sels) => {
            !sels.is_empty() && sels.iter().all(|s| selector_matches(work, s))
        }
        Selector::Or(sels) => {
            !sels.is_empty() && sels.iter().any(|s| selector_matches(work, s))
        }
    }
}

/// **P417 (M)** — Igualdade semântica de `Value` para matching de `Where`.
/// Replica ADR-0025 (coerção Int↔Float em comparações) e ADR-0107
/// (paridade comportamental, não mecânica).
fn values_eq_semantic(actual: &Value, expected: &Value) -> bool {
    match (actual, expected) {
        (Value::Int(a), Value::Float(b)) => (*a as f64) == *b,
        (Value::Float(a), Value::Int(b)) => *a == (*b as f64),
        (a, b) => a == b,
    }
}

/// **P417/P423** — Verifica se um selector de show rule deve viajar pela
/// travessia de nós (`map_content`). Recursivo para `Where` com base
/// `NodeKind`/`DynKind`; P423 estende a `And`/`Or` (todos os sub-selectors
/// devem ser node-like).
pub(crate) fn is_node_rule(selector: &Selector) -> bool {
    match selector {
        Selector::NodeKind(_) | Selector::DynKind(_) => true,
        Selector::Where { base, .. } => is_node_rule(base.as_ref()),
        // **P423 (S-M)** — combinadores viajam pela travessia de nós sse
        // todos os sub-selectors forem node-like.
        Selector::And(sels) | Selector::Or(sels) => sels.iter().all(|s| is_node_rule(s)),
        Selector::Text(_) | Selector::Regex(_) => false,
        // P791 — regras de label não viajam pela travessia de nós
        // (aplicação dedicada em `intercept_labelled`).
        Selector::Label(_) => false,
    }
}

/// **P790** — Fatia `text` nas ocorrências de `pattern` e emenda o
/// replacement produzido para cada match (paridade vanilla
/// `visit_regex_match`, `typst-realize/src/lib.rs:1391`): o texto antes e
/// depois do match é preservado como `Content::Text` e o output da regra
/// entra no lugar do match. `Ok(None)` quando não há match (nó inalterado).
/// Fatias vazias são omitidas; `Content::sequence` colapsa o Vec se só
/// restar um nó (match de texto integral). Guarda defensiva: padrão vazio
/// nunca casa (o eval já rejeita com "text selector is empty").
pub(crate) fn splice_text_rule_matches(
    text: &str,
    pattern: &str,
    mut replacement: impl FnMut(&str) -> SourceResult<Content>,
) -> SourceResult<Option<Content>> {
    if pattern.is_empty() || !text.contains(pattern) {
        return Ok(None);
    }
    let mut parts: Vec<Content> = Vec::new();
    let mut rest = text;
    while let Some(idx) = rest.find(pattern) {
        let (before, with_match) = rest.split_at(idx);
        if !before.is_empty() {
            parts.push(Content::text(before));
        }
        let (matched, after) = with_match.split_at(pattern.len());
        parts.push(replacement(matched)?);
        rest = after;
    }
    if !rest.is_empty() {
        parts.push(Content::text(rest));
    }
    Ok(Some(Content::sequence(parts)))
}

/// Helper partilhado para casamento de origem sintática de estilos.
/// Um `Content::Styled` produzido por `*strong*`/`_emph_`/etc. marca a
/// origem no delta de estilos; um `Content::Styled` produzido por `#set`
/// não casa os selectors de elemento correspondentes.
fn is_styled_origin(
    work: &Content,
    bold_from_strong: bool,
    italic_from_emph: bool,
    subscript: bool,
    superscript: bool,
    highlight: bool,
) -> bool {
    match work {
        Content::Styled(_, styles) => {
            let d = styles.delta();
            let mut ok = false;
            if bold_from_strong {
                ok |= d.bold == Some(true) && d.bold_from_strong == Some(true);
            }
            if italic_from_emph {
                ok |= d.italic == Some(true) && d.italic_from_emph == Some(true);
            }
            if subscript {
                ok |= d.subscript == Some(true);
            }
            if superscript {
                ok |= d.superscript == Some(true);
            }
            if highlight {
                ok |= d.highlight.is_some();
            }
            ok
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::content::Content;
    use crate::entities::show::Selector;
    use crate::entities::value::Value;
    use ecow::EcoString;

    fn where_selector(field: &str, value: Value) -> Selector {
        Selector::Where {
            base: Box::new(Selector::NodeKind(NodeKind::Heading)),
            field: EcoString::from(field),
            value: Box::new(value),
        }
    }

    #[test]
    fn p417_matches_where_positivo() {
        let content = Content::heading(1, Content::text("Intro"));
        let sel = where_selector("level", Value::Int(1));
        assert!(selector_matches(&content, &sel));
    }

    #[test]
    fn p417_matches_where_valor_errado() {
        let content = Content::heading(1, Content::text("Intro"));
        let sel = where_selector("level", Value::Int(2));
        assert!(!selector_matches(&content, &sel));
    }

    #[test]
    fn p417_matches_where_campo_inexistente() {
        let content = Content::heading(1, Content::text("Intro"));
        let sel = where_selector("inexistente", Value::Int(1));
        assert!(!selector_matches(&content, &sel));
    }

    #[test]
    fn p417_matches_where_base_nao_casa() {
        let content =
            Content::figure(Content::text("Fig"), None, Some("image".to_string()), None);
        let sel = where_selector("level", Value::Int(1));
        assert!(!selector_matches(&content, &sel));
    }

    #[test]
    fn p417_matches_where_int_float_coerce() {
        let content = Content::heading(1, Content::text("Intro"));
        let sel = where_selector("level", Value::Float(1.0));
        assert!(selector_matches(&content, &sel));
    }

    #[test]
    fn p417_matches_where_body_content() {
        let content = Content::heading(1, Content::text("Intro"));
        let sel = where_selector("body", Value::Content(Content::text("Intro")));
        assert!(selector_matches(&content, &sel));
    }

    #[test]
    fn p417_extract_field_heading_level() {
        let content = Content::heading(2, Content::text("X"));
        assert_eq!(content.get_field("level"), Some(Value::Int(2)));
    }

    #[test]
    fn p417_extract_field_heading_body() {
        let content = Content::heading(1, Content::text("Intro"));
        assert_eq!(
            content.get_field("body"),
            Some(Value::Content(Content::text("Intro")))
        );
    }

    #[test]
    fn p417_extract_field_heading_inexistente() {
        let content = Content::heading(1, Content::text("X"));
        assert_eq!(content.get_field("inexistente"), None);
    }

    // ── P423 (S-M) — combinadores And/Or em show rules ──────────────────────

    fn or_selector(a: Selector, b: Selector) -> Selector {
        Selector::Or(vec![a, b])
    }

    fn and_selector(a: Selector, b: Selector) -> Selector {
        Selector::And(vec![a, b])
    }

    #[test]
    fn p423_matches_or_positivo_heading() {
        let content = Content::heading(1, Content::text("T"));
        let sel = or_selector(
            Selector::NodeKind(NodeKind::Heading),
            Selector::NodeKind(NodeKind::Figure),
        );
        assert!(selector_matches(&content, &sel));
    }

    #[test]
    fn p423_matches_or_positivo_figure() {
        let content =
            Content::figure(Content::text("F"), None, Some("image".to_string()), None);
        let sel = or_selector(
            Selector::NodeKind(NodeKind::Heading),
            Selector::NodeKind(NodeKind::Figure),
        );
        assert!(selector_matches(&content, &sel));
    }

    #[test]
    fn p423_matches_or_negativo_paragraph() {
        let content = Content::text("par");
        let sel = or_selector(
            Selector::NodeKind(NodeKind::Heading),
            Selector::NodeKind(NodeKind::Figure),
        );
        assert!(!selector_matches(&content, &sel));
    }

    #[test]
    fn p423_matches_and_positivo_where_plus_kind() {
        let content = Content::heading(1, Content::text("T"));
        let sel = and_selector(
            Selector::Where {
                base: Box::new(Selector::NodeKind(NodeKind::Heading)),
                field: "level".into(),
                value: Box::new(Value::Int(1)),
            },
            Selector::NodeKind(NodeKind::Heading),
        );
        assert!(selector_matches(&content, &sel));
    }

    #[test]
    fn p423_matches_and_negativo_contraditorio() {
        let content = Content::heading(1, Content::text("T"));
        let sel = and_selector(
            Selector::NodeKind(NodeKind::Heading),
            Selector::NodeKind(NodeKind::Figure),
        );
        assert!(!selector_matches(&content, &sel));
    }

    #[test]
    fn p423_matches_and_vazio_false() {
        let content = Content::heading(1, Content::text("T"));
        let sel = Selector::And(vec![]);
        assert!(!selector_matches(&content, &sel));
    }

    #[test]
    fn p423_matches_or_vazio_false() {
        let content = Content::heading(1, Content::text("T"));
        let sel = Selector::Or(vec![]);
        assert!(!selector_matches(&content, &sel));
    }

    #[test]
    fn p423_is_node_rule_and_or_com_node_kinds() {
        let sel = Selector::And(vec![
            Selector::NodeKind(NodeKind::Heading),
            Selector::NodeKind(NodeKind::Figure),
        ]);
        assert!(is_node_rule(&sel));
        let sel = Selector::Or(vec![
            Selector::NodeKind(NodeKind::Heading),
            Selector::NodeKind(NodeKind::Figure),
        ]);
        assert!(is_node_rule(&sel));
    }

    #[test]
    fn p423_is_node_rule_and_nao_node_rejeita() {
        let sel = Selector::And(vec![
            Selector::NodeKind(NodeKind::Heading),
            Selector::Text("x".to_string()),
        ]);
        assert!(!is_node_rule(&sel));
    }

    // ── Passo 444 (P284) — selectors para text decoration ───────────────────

    #[test]
    fn p444_selector_underline_casa_content_underline() {
        let content = Content::underline(Content::text("x"), None, None, None);
        assert!(selector_matches(&content, &Selector::NodeKind(NodeKind::Underline)));
        assert!(!selector_matches(&content, &Selector::NodeKind(NodeKind::Strike)));
        assert!(!selector_matches(&content, &Selector::NodeKind(NodeKind::Overline)));
    }

    #[test]
    fn p444_selector_strike_casa_content_strike() {
        let content = Content::strike(Content::text("x"), None, None, None);
        assert!(selector_matches(&content, &Selector::NodeKind(NodeKind::Strike)));
        assert!(!selector_matches(&content, &Selector::NodeKind(NodeKind::Underline)));
    }

    #[test]
    fn p444_selector_overline_casa_content_overline() {
        let content = Content::overline(Content::text("x"), None, None, None);
        assert!(selector_matches(&content, &Selector::NodeKind(NodeKind::Overline)));
        assert!(!selector_matches(&content, &Selector::NodeKind(NodeKind::Underline)));
    }

    #[test]
    fn p444_selector_decoration_nao_casa_texto_plano() {
        let content = Content::text("x");
        assert!(!selector_matches(&content, &Selector::NodeKind(NodeKind::Underline)));
        assert!(!selector_matches(&content, &Selector::NodeKind(NodeKind::Strike)));
        assert!(!selector_matches(&content, &Selector::NodeKind(NodeKind::Overline)));
    }

    // ── Passo 446 (P408) — selector para smallcaps ──────────────────────────

    #[test]
    fn p446_selector_smallcaps_casa_content_smallcaps() {
        let content = Content::smallcaps(Content::text("x"));
        assert!(selector_matches(&content, &Selector::NodeKind(NodeKind::Smallcaps)));
        assert!(!selector_matches(&content, &Selector::NodeKind(NodeKind::Strong)));
    }

    #[test]
    fn p446_selector_smallcaps_nao_casa_texto_plano() {
        let content = Content::text("x");
        assert!(!selector_matches(&content, &Selector::NodeKind(NodeKind::Smallcaps)));
    }

    // ── Passo 449 (P449) — selector para highlight ────────────────────────

    #[test]
    fn p449_selector_highlight_casa_styled_highlight() {
        let content = Content::highlight(Content::text("x"), None);
        assert!(selector_matches(&content, &Selector::NodeKind(NodeKind::Highlight)));
        assert!(!selector_matches(&content, &Selector::NodeKind(NodeKind::Strong)));
    }

    #[test]
    fn p449_selector_highlight_nao_casa_texto_plano() {
        let content = Content::text("x");
        assert!(!selector_matches(&content, &Selector::NodeKind(NodeKind::Highlight)));
    }

    #[test]
    fn p449_selector_highlight_fill_none_ainda_casa() {
        // `fill: none` é um highlight "desactivado"; continua a casar o selector
        // de tipo, exactamente como um `set` vazio não remove a origem sintática.
        let content = Content::highlight(Content::text("x"), None);
        assert!(selector_matches(&content, &Selector::NodeKind(NodeKind::Highlight)));
    }
}
