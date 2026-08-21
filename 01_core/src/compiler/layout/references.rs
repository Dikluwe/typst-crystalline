//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout_references.md
//! @layer L1
//! @updated 2026-08-18

use crate::entities::{
    content::Content,
    counter::CounterKey,
    counter_format::format_counter,
    element_kind::ElementKind,
    elements::cite::CiteElem,
    elements::r#ref::RefElem,
    introspector::Introspector,
    label::Label,
    layout_types::{FrameItem, LinkTarget, Point},
    selector::Selector,
    source_result::SourceDiagnostic,
    span::Span,
};

use super::{link::link_bbox, FontMetrics, ImageSizer, Layouter};

/// Layout transparente do body com registo de página e posição (P460/P464).
pub(super) fn layout_label<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    body: &Content,
    label: Label,
) {
    let pos = Point {
        x: layouter.regions.current.cursor_x,
        y: layouter.regions.current.cursor_y,
    };
    layouter.layout_content(body);
    let page = layouter.current_page_number();
    layouter.runtime.label_pages.insert(label.clone(), page);
    layouter.runtime.label_positions.insert(label, pos);
}

/// Braço `Ref` (P462/P463) — resolve o número do elemento associado ao label
/// e envolve o texto num `FrameItem::Link` clicável para o destino interno.
///
/// Ordem de resolução:
/// 1. `Content::Label` numérico (heading/figure/equation/table) via
///    `Introspector::counter_key_for_label` + `counter_values_at`.
/// 2. Figure `Content::Labelled` via `figure_number_for_label` (fallback legacy).
/// 3. Texto resolvido `Content::Labelled` via `resolved_label_for` (fallback legacy).
/// 4. Label não encontrada → renderiza "?".
pub(super) fn layout_ref<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    elem: &RefElem,
) {
    // **P533** — `@key` é uma referência bibliográfica se a key existir no
    // BibStore. Neste caso, delegamos ao layout de `Cite` em vez de tratar
    // como referência cruzada genérica.
    if layouter.introspector.bib_entry_for_key(elem.name.as_str()).is_some() {
        let cite = CiteElem {
            key: elem.name.to_string(),
            supplement: elem.supplement.clone(),
            form: None,
            style: None,
        };
        super::cite::layout(layouter, &cite);
        return;
    }

    let target_label = Label(elem.name.to_string());

    // **P856** — label existe mas não é referenciável (texto, raw, equation
    // sem numbering, etc.). Verificação em primeiro lugar porque alguns
    // destes labels também são conhecidos por `query_by_label` (equations
    // emitem Tag) e ficariam escondidos pela verificação de `known`.
    match layouter.introspector.unreferencable_label_kind(&target_label) {
        Some(crate::entities::label_kind::UnreferencableKind::Raw) => {
            layouter.layout_errors.push(SourceDiagnostic::error(
                Span::detached(),
                "cannot reference raw directly, try putting it into a figure",
            ));
            return;
        }
        Some(crate::entities::label_kind::UnreferencableKind::EquationWithoutNumbering) => {
            layouter.layout_errors.push(
                SourceDiagnostic::error(
                    Span::detached(),
                    "cannot reference equation without numbering",
                )
                .with_hint(
                    "you can enable equation numbering with `#set math.equation(numbering: \"1.\")`",
                ),
            );
            return;
        }
        Some(crate::entities::label_kind::UnreferencableKind::Text)
        | Some(crate::entities::label_kind::UnreferencableKind::Other) => {
            layouter.layout_errors.push(SourceDiagnostic::error(
                Span::detached(),
                "cannot reference text",
            ));
            return;
        }
        None => {}
    }

    // **P788** — validações do vanilla (antes: "?" / vazio em silêncio,
    // achados A8/A10 de P786). Mensagens medidas no vanilla 0.15.0.
    // `RefElem` não carrega span → `Span::detached()` (limitação registada).
    let known = layouter.introspector.counter_key_for_label(&target_label).is_some()
        || layouter.introspector.figure_number_for_label(&target_label).is_some()
        || layouter.introspector.resolved_label_for(&target_label).is_some()
        || layouter.introspector.query_by_label(&target_label).is_some();
    if !known {
        layouter.layout_errors.push(SourceDiagnostic::error(
            Span::detached(),
            format!("label `<{}>` does not exist in the document", elem.name),
        ));
        return;
    }
    let heading_key = CounterKey::Selector(Selector::Kind(ElementKind::Heading));
    if layouter.introspector.counter_key_for_label(&target_label) == Some(&heading_key) {
        if let Some(loc) = layouter.introspector.query_by_label(&target_label) {
            if layouter.introspector.heading_has_numbering(loc) == Some(false) {
                layouter.layout_errors.push(
                    SourceDiagnostic::error(
                        Span::detached(),
                        "cannot reference heading without numbering",
                    )
                    .with_hint(
                        "you can enable heading numbering with `#set heading(numbering: \"1.\")`",
                    ),
                );
                return;
            }
        }
    }
    let equation_key = CounterKey::Selector(Selector::Kind(ElementKind::Equation));
    if layouter.introspector.counter_key_for_label(&target_label) == Some(&equation_key) {
        if let Some(loc) = layouter.introspector.query_by_label(&target_label) {
            if layouter.introspector.equation_has_numbering(loc) == Some(false) {
                layouter.layout_errors.push(
                    SourceDiagnostic::error(
                        Span::detached(),
                        "cannot reference equation without numbering",
                    )
                    .with_hint(
                        "you can enable equation numbering with `#set math.equation(numbering: \"1.\")`",
                    ),
                );
                return;
            }
        }
    }

    let text = resolve_ref_text(layouter, elem, &target_label);

    // P463: todo ref é clicável, mesmo que o destino não exista (PDF reader
    // simplesmente não navega). O label inexistente renderiza "?".
    let items_before = layouter.regions.current.current_items.len();
    let line_before = layouter.regions.current.current_line.len();

    layouter.layout_content(&Content::text(text));

    // O layout pode fazer flush (move itens de current_line para current_items),
    // pelo que line_before pode ficar maior que o length actual.
    let new_line: Vec<FrameItem> =
        if line_before <= layouter.regions.current.current_line.len() {
            layouter.regions.current.current_line.drain(line_before..).collect()
        } else {
            Vec::new()
        };
    let new_items: Vec<FrameItem> =
        layouter.regions.current.current_items.drain(items_before..).collect();

    let mut link_items = Vec::with_capacity(new_line.len() + new_items.len());
    link_items.extend(new_line);
    link_items.extend(new_items);

    let (pos, size) = link_bbox(&link_items, &layouter.metrics);

    layouter.regions.current.current_line.push(FrameItem::Link {
        target: LinkTarget::Destination(target_label),
        items: link_items,
        pos,
        size,
    });
}

/// Resolve o texto a renderizar para um `RefElem`.
fn resolve_ref_text<M: FontMetrics, S: ImageSizer>(
    layouter: &Layouter<M, S>,
    elem: &RefElem,
    target_label: &Label,
) -> String {
    // 1. Caminho P462: label associada a um elemento numerado via Content::Label.
    if let Some(key) = layouter.introspector.counter_key_for_label(target_label) {
        if let Some(loc) = layouter.introspector.query_by_label(target_label) {
            let heading_key = CounterKey::Selector(Selector::Kind(ElementKind::Heading));
            let equation_key = CounterKey::Selector(Selector::Kind(ElementKind::Equation));
            let formatted = if *key == heading_key {
                layouter
                    .introspector
                    .formatted_counter_at(key, loc)
                    .unwrap_or_default()
            } else if *key == equation_key {
                if let Some(pat) = layouter.introspector.equation_numbering_pattern(loc) {
                    let raw = layouter
                        .introspector
                        .counter_values_at(key, loc)
                        .and_then(|vals| crate::entities::counter_format::format_counter(vals, pat))
                        .unwrap_or_else(|| {
                            layouter
                                .introspector
                                .flat_counter_at(key, loc)
                                .map(|n| n.to_string())
                                .unwrap_or_default()
                        });
                    raw.trim_matches(|c| c == '(' || c == ')').to_string()
                } else {
                    layouter
                        .introspector
                        .flat_counter_at(key, loc)
                        .map(|n| n.to_string())
                        .unwrap_or_default()
                }
            } else {
                // figure:*, table — número simples (P1073: paridade vanilla Equation 1, não (1)).
                layouter
                    .introspector
                    .flat_counter_at(key, loc)
                    .map(|n| n.to_string())
                    .unwrap_or_default()
            };

            let supplement = elem.supplement.clone().or_else(|| {
                default_supplement_for_key(key, layouter.style.lang.as_ref())
            });
            return match supplement {
                // P788 — join do vanilla (`realize_reference`): NBSP (U+A0)
                // entre suplemento não-vazio e número.
                Some(sup) => {
                    let sup = sup.plain_text();
                    if sup.is_empty() {
                        formatted
                    } else {
                        format!("{sup}\u{a0}{formatted}")
                    }
                }
                None => formatted,
            };
        }
    }

    // 2. Fallback legacy: figure Labelled (P1073: Figure/Figura com NBSP).
    if let Some(fig_num) = layouter.introspector.figure_number_for_label(target_label) {
        let is_pt = layouter.style.lang.as_ref().map(|l| l.as_str() == "pt").unwrap_or(false);
        let prefix = elem
            .supplement
            .clone()
            .map(|s| format!("{}\u{a0}", s.plain_text()))
            .unwrap_or_else(|| if is_pt { "Figura\u{a0}".to_string() } else { "Figure\u{a0}".to_string() });
        return format!("{}{}", prefix, fig_num);
    }

    // 3. Fallback legacy: texto resolvido (ex: "Secção 1" para Labelled heading).
    if let Some(text) = layouter.introspector.resolved_label_for(target_label) {
        return text.to_string();
    }

    // 4. Label não encontrada (inalcançável desde P788 — a validação em
    // `layout_ref` erra antes; mantido por segurança).
    "?".to_string()
}

/// Supplement default por chave de counter (P462).
/// **P788** — heading ganha suplemento por língua (vanilla, medido: doc
/// `en` → "Section 1"; `pt` → "Seção 1"). Outras línguas → fallback `en`
/// (limitação registada no L0).
fn default_supplement_for_key(
    key: &CounterKey,
    lang: Option<&crate::entities::lang::Lang>,
) -> Option<Content> {
    let is_pt = lang.map(|l| l.as_str() == "pt").unwrap_or(false);
    match key {
        CounterKey::Selector(Selector::Kind(ElementKind::Heading)) => {
            Some(Content::text(if is_pt { "Seção" } else { "Section" }))
        }
        CounterKey::Str(s) if s.starts_with("figure:") => {
            let kind = s.strip_prefix("figure:").unwrap_or("image");
            match kind {
                "table" => Some(Content::text(if is_pt { "Tabela" } else { "Table" })),
                "raw" => Some(Content::text(if is_pt { "Listagem" } else { "Listing" })),
                _ => Some(Content::text(if is_pt { "Figura" } else { "Figure" })),
            }
        }
        CounterKey::Selector(Selector::Kind(ElementKind::Equation)) => {
            Some(Content::text(if is_pt { "Equação" } else { "Equation" }))
        }
        CounterKey::Selector(Selector::Kind(ElementKind::Table)) => {
            Some(Content::text(if is_pt { "Tabela" } else { "Table" }))
        }
        _ => None, // neutro: N16[β] — Content sem referência cruzada retorna None
    }
}
