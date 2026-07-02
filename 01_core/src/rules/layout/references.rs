//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/layout_references.md
//! @layer L1
//! @updated 2026-06-25

use crate::entities::{
    content::Content, counter_format::format_counter, elements::cite::CiteElem,
    elements::r#ref::RefElem, introspector::Introspector, label::Label,
    layout_types::{FrameItem, LinkTarget, Point},
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
    let text = resolve_ref_text(layouter, elem, &target_label);

    // P463: todo ref é clicável, mesmo que o destino não exista (PDF reader
    // simplesmente não navega). O label inexistente renderiza "?".
    let items_before = layouter.regions.current.current_items.len();
    let line_before = layouter.regions.current.current_line.len();

    layouter.layout_content(&Content::text(text));

    // O layout pode fazer flush (move itens de current_line para current_items),
    // pelo que line_before pode ficar maior que o length actual.
    let new_line: Vec<FrameItem> = if line_before <= layouter.regions.current.current_line.len() {
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
            let formatted = if key == "heading" {
                layouter
                    .introspector
                    .formatted_counter_at(key, loc)
                    .unwrap_or_default()
            } else if key == "equation" {
                layouter
                    .introspector
                    .flat_counter_at(key, loc)
                    .map(|n| format_counter(&[n], "(1)").unwrap_or_else(|| n.to_string()))
                    .unwrap_or_default()
            } else {
                // figure:* e table — número simples.
                layouter
                    .introspector
                    .flat_counter_at(key, loc)
                    .map(|n| n.to_string())
                    .unwrap_or_default()
            };

            let supplement =
                elem.supplement.clone().or_else(|| default_supplement_for_key(key));
            return match supplement {
                Some(sup) => format!("{}{}", sup.plain_text(), formatted),
                None => formatted,
            };
        }
    }

    // 2. Fallback legacy: figure Labelled.
    if let Some(fig_num) = layouter.introspector.figure_number_for_label(target_label) {
        let prefix = elem
            .supplement
            .clone()
            .map(|s| s.plain_text())
            .unwrap_or_else(|| "Fig. ".to_string());
        return format!("{}{}", prefix, fig_num);
    }

    // 3. Fallback legacy: texto resolvido (ex: "Secção 1" para Labelled heading).
    if let Some(text) = layouter.introspector.resolved_label_for(target_label) {
        return text.to_string();
    }

    // 4. Label não encontrada.
    "?".to_string()
}

/// Supplement default por chave de counter (P462).
fn default_supplement_for_key(key: &str) -> Option<Content> {
    match key {
        k if k.starts_with("figure:") => Some(Content::text("Fig. ")),
        "table" => Some(Content::text("Table ")),
        _ => None,
    }
}
