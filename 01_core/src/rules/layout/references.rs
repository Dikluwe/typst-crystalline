//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/layout_references.md
//! @prompt-hash 0226beef
//! @layer L1
//! @updated 2026-04-20

use crate::entities::{content::Content, label::Label};

use super::{FontMetrics, ImageSizer, Layouter};

/// Braço `Labelled` — layout transparente do target com registo de página.
///
/// O layout do target ocorre primeiro porque o target pode forçar uma quebra
/// de página. O registo da página acontece **depois** — o elemento já aterrou
/// na sua página final (Passo 63, DEBT-12).
pub(super) fn layout_labelled<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    target:   &Content,
    label:    &Label,
) {
    // Layout primeiro — o target pode forçar uma quebra de página.
    layouter.layout_content(target);

    // Registar a página DEPOIS do layout: o elemento já aterrou na sua página
    // final. Registar antes resultaria no número da página anterior quando
    // o target força uma quebra.
    let page = layouter.current_page_number();
    // P190C (M6 categoria Page tracking): label_pages movido para
    // LayouterRuntimeState (Layouter-runtime — não derivado de Content).
    layouter.runtime.label_pages.insert(label.clone(), page);
}

/// Braço `Ref` — consulta contadores de figura e `resolved_labels` populados pela introspecção.
/// Forward e backward refs resolvem. Fallback `@nome` se a label não existir.
///
/// **P168 (M5 sub-passo 2)**: figure-ref consulta `Introspector::figure_number_for_label`
/// PRIMEIRO; se vazio (introspector não populado pelo caller), fallback a
/// `state.figure_label_numbers` legacy. Migração gradual — quando todos os
/// callers migrarem para `layout_with_introspector`, fallback torna-se
/// dead code (M6 elimina). Caso section-ref permanece em legacy
/// (lacuna #4-#7 documentadas em `m1-lacunas-captura.md`).
pub(super) fn layout_ref<M: FontMetrics, S: ImageSizer>(layouter: &mut Layouter<M, S>, target: &Label) {
    use crate::entities::introspector::Introspector;

    // P190H (M6 categoria Figures): fallback legacy
    // `counter.figure_label_numbers` ELIMINADO — field eliminado de
    // CounterStateLegacy. Caminho Introspector activo desde P168
    // (populate_intr arms Figure + Labelled populate
    // `intr.figure_label_numbers`). Substitution-with-fallback
    // colapsa em Introspector path puro.
    if let Some(fig_num) = layouter.introspector.figure_number_for_label(target) {
        layouter.layout_content(&Content::text(format!("Figura {}", fig_num)));
        return;
    }
    // P190G (M6 categoria Labels & TOC): fallback legacy
    // `counter.resolved_labels` ELIMINADO — field eliminado de
    // CounterStateLegacy. Caminho Introspector activo desde
    // P195D (Tag::Labelled pós-recursão; populate_intr arm Labelled
    // popula intr.resolved_labels). P194B substitution-with-fallback
    // colapsa em Introspector path puro.
    let display_text = match layouter.introspector.resolved_label_for(target) {
        Some(text) => text.to_string(),
        None       => format!("@{}", target.0),
    };
    layouter.layout_content(&Content::text(display_text));
}
