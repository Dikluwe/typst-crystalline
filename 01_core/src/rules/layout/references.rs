//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/layout_references.md
//! @layer L1
//! @updated 2026-06-25

use crate::entities::{
    content::Content, counter_format::format_counter, elements::r#ref::RefElem,
    label::Label, layout_types::Point,
};

use super::{FontMetrics, ImageSizer, Layouter};

/// Braço `Labelled` — layout transparente do target com registo de página
/// e posição (P460).
pub(super) fn layout_labelled<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    target: &Content,
    label: &Label,
) {
    let pos = Point {
        x: layouter.regions.current.cursor_x,
        y: layouter.regions.current.cursor_y,
    };
    layouter.layout_content(target);
    let page = layouter.current_page_number();
    layouter.runtime.label_pages.insert(label.clone(), page);
    layouter.runtime.label_positions.insert(label.clone(), pos);
}

/// Braço `Label` (P460) — wrapper transparente do body com registo de destino.
pub(super) fn layout_label<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    body: &Content,
    label: Label,
) {
    layout_labelled(layouter, body, &label);
}

/// Braço `Ref` (P462) — resolve o número do elemento associado ao label.
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
    use crate::entities::introspector::Introspector;

    let target_label = Label(elem.name.to_string());

    // 1. Caminho P462: label associada a um elemento numerado via Content::Label.
    if let Some(key) = layouter.introspector.counter_key_for_label(&target_label) {
        if let Some(loc) = layouter.introspector.query_by_label(&target_label) {
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
            let text = match supplement {
                Some(sup) => format!("{}{}", sup.plain_text(), formatted),
                None => formatted,
            };
            layouter.layout_content(&Content::text(text));
            return;
        }
    }

    // 2. Fallback legacy: figure Labelled.
    if let Some(fig_num) = layouter.introspector.figure_number_for_label(&target_label) {
        let prefix = elem
            .supplement
            .clone()
            .map(|s| s.plain_text())
            .unwrap_or_else(|| "Fig. ".to_string());
        layouter.layout_content(&Content::text(format!("{}{}", prefix, fig_num)));
        return;
    }

    // 3. Fallback legacy: texto resolvido (ex: "Secção 1" para Labelled heading).
    if let Some(text) = layouter.introspector.resolved_label_for(&target_label) {
        layouter.layout_content(&Content::text(text.to_string()));
        return;
    }

    // 4. Label não encontrada.
    layouter.layout_content(&Content::text("?"));
}

/// Supplement default por chave de counter (P462).
fn default_supplement_for_key(key: &str) -> Option<Content> {
    match key {
        k if k.starts_with("figure:") => Some(Content::text("Fig. ")),
        "table" => Some(Content::text("Table ")),
        _ => None,
    }
}
