//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/layout_references.md
//! @prompt-hash 905b7ae5
//! @layer L1
//! @updated 2026-04-13

use crate::entities::{content::Content, label::Label};

use super::{FontMetrics, Layouter};

/// Braço `Labelled` — layout transparente do target.
/// A label em si não tem presença visual; foi registada na introspecção.
pub(super) fn layout_labelled<M: FontMetrics>(layouter: &mut Layouter<M>, target: &Content) {
    layouter.layout_content(target);
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
