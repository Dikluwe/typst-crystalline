//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/selector_matching/textual.md
//! @prompt-hash 7b8f6be1
//! @layer L1
//! @updated 2026-09-30
//!
//! Casamento e substituição textuais de regras.

use super::*;

/// O caminho textual dedicado aceita somente estes dois selectors; os de
/// elemento e de parágrafo conservam os seus pontos de intercepção.
pub(crate) fn is_text_rule(selector: &Selector) -> bool {
    match selector {
        Selector::Text(_) | Selector::Regex(_) => true,
        Selector::NodeKind(_)
        | Selector::NativeElement(_)
        | Selector::DynKind(_)
        | Selector::Where { .. }
        | Selector::And(_)
        | Selector::Or(_)
        | Selector::Label(_) => false,
    }
}

/// Testa a representação textual normalizada do grupo antes do splice.
/// Matches vazios não disparam recipes nem substituições.
pub(crate) fn textual_selector_matches(selector: &Selector, text: &str) -> bool {
    match selector {
        Selector::Text(pattern) => !pattern.is_empty() && text.contains(pattern),
        Selector::Regex(regex) => regex
            .captures_all(text)
            .iter()
            .any(|matched| matched.start < matched.end),
        Selector::NodeKind(_)
        | Selector::NativeElement(_)
        | Selector::DynKind(_)
        | Selector::Where { .. }
        | Selector::And(_)
        | Selector::Or(_)
        | Selector::Label(_) => false,
    }
}

/// Offsets UTF-8 das ocorrências não vazias e não sobrepostas no grupo já
/// normalizado; a projeção para nós originais pertence ao owner rules.
pub(crate) fn textual_match_ranges(
    selector: &Selector,
    text: &str,
) -> Vec<(usize, usize)> {
    match selector {
        Selector::Text(pattern) if !pattern.is_empty() => {
            let mut ranges = Vec::new();
            let mut cursor = 0;
            while let Some(offset) = text[cursor..].find(pattern) {
                let start = cursor + offset;
                let end = start + pattern.len();
                ranges.push((start, end));
                cursor = end;
            }
            ranges
        }
        Selector::Regex(regex) => regex
            .captures_all(text)
            .into_iter()
            .filter(|matched| matched.start < matched.end)
            .map(|matched| (matched.start, matched.end))
            .collect(),
        Selector::Text(_)
        | Selector::NodeKind(_)
        | Selector::NativeElement(_)
        | Selector::DynKind(_)
        | Selector::Where { .. }
        | Selector::And(_)
        | Selector::Or(_)
        | Selector::Label(_) => Vec::new(),
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

/// Fatia `text` em todos os matches não vazios de `regex`, preservando as
/// partes não casadas e entregando à transformação somente cada ocorrência.
pub(crate) fn splice_regex_rule_matches(
    text: &str,
    regex: &crate::entities::regex::Regex,
    mut replacement: impl FnMut(&str) -> SourceResult<Content>,
) -> SourceResult<Option<Content>> {
    let matches = regex.captures_all(text);
    if !matches.iter().any(|matched| matched.start < matched.end) {
        return Ok(None);
    }

    let mut parts = Vec::new();
    let mut cursor = 0;
    for matched in matches {
        if matched.start == matched.end {
            continue;
        }
        if cursor < matched.start {
            parts.push(Content::text(&text[cursor..matched.start]));
        }
        parts.push(replacement(&text[matched.start..matched.end])?);
        cursor = matched.end;
    }
    if cursor < text.len() {
        parts.push(Content::text(&text[cursor..]));
    }

    Ok(Some(Content::sequence(parts)))
}

#[cfg(test)]
mod smoke {
    use super::*;

    #[test]
    fn v16_show_selector_text_routes_keep_utf8_empty_and_nontext_boundaries() {
        use crate::entities::label::Label;
        use crate::entities::regex::Regex;

        let text = "éa éa";
        for selector in
            [Selector::Text("éa".into()), Selector::Regex(Regex::new("éa").unwrap())]
        {
            assert!(is_text_rule(&selector));
            assert!(textual_selector_matches(&selector, text));
            assert_eq!(textual_match_ranges(&selector, text), [(0, 3), (4, 7)]);
            assert!(!textual_selector_matches(&selector, "miss"));
            assert!(textual_match_ranges(&selector, "miss").is_empty());
        }
        for selector in
            [Selector::Text(String::new()), Selector::Regex(Regex::new("").unwrap())]
        {
            assert!(is_text_rule(&selector));
            assert!(!textual_selector_matches(&selector, text));
            assert!(textual_match_ranges(&selector, text).is_empty());
        }
        for selector in [
            Selector::NodeKind(NodeKind::Heading),
            Selector::NativeElement(Func::native(
                "text",
                crate::compiler::stdlib::native_text,
            )),
            Selector::DynKind("text".into()),
            Selector::Where {
                base: Box::new(Selector::Text("éa".into())),
                field: "body".into(),
                value: Box::new(Value::None),
            },
            Selector::And(vec![Selector::Text("éa".into())]),
            Selector::Or(vec![Selector::Regex(Regex::new("éa").unwrap())]),
            Selector::Label(Label("éa".into())),
        ] {
            assert!(!is_text_rule(&selector), "{selector:?}");
            assert!(!textual_selector_matches(&selector, text), "{selector:?}");
            assert!(textual_match_ranges(&selector, text).is_empty(), "{selector:?}");
        }
    }

    #[test]
    fn modulo_compila_e_carrega() {}
}
