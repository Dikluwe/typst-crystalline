//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/layout_references.md
//! @prompt-hash 0226beef
//! @layer L1
//! @updated 2026-04-13

use crate::entities::{content::Content, label::Label};

use super::{FontMetrics, Layouter};

/// Braço `Labelled` — layout transparente do target com registo de página.
///
/// O layout do target ocorre primeiro porque o target pode forçar uma quebra
/// de página. O registo da página acontece **depois** — o elemento já aterrou
/// na sua página final (Passo 63, DEBT-12).
pub(super) fn layout_labelled<M: FontMetrics>(
    layouter: &mut Layouter<M>,
    target:   &Content,
    label:    &Label,
) {
    // Layout primeiro — o target pode forçar uma quebra de página.
    layouter.layout_content(target);

    // Registar a página DEPOIS do layout: o elemento já aterrou na sua página
    // final. Registar antes resultaria no número da página anterior quando
    // o target força uma quebra.
    let page = layouter.current_page_number();
    layouter.counter.label_pages.insert(label.clone(), page);
}

/// Braço `Ref` — consulta `resolved_labels` populado pela introspecção.
/// Forward e backward refs resolvem. Fallback `@nome` se a label não existir.
pub(super) fn layout_ref<M: FontMetrics>(layouter: &mut Layouter<M>, target: &Label) {
    let display_text = match layouter.counter.resolved_labels.get(target) {
        Some(text) => text.clone(),
        None       => format!("@{}", target.0),
    };
    layouter.layout_content(&Content::text(display_text));
}
