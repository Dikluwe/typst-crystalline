//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/layout_outline.md
//! @prompt-hash 64f585a1
//! @layer L1
//! @updated 2026-04-13

use crate::entities::content::Content;

use super::{FontMetrics, Layouter};

/// Gera a Tabela de Conteúdos visual.
///
/// Lê `headings_for_toc` e `label_pages` do estado injectado pela introspecção.
/// Usa Content clonado para preservar formatação dos títulos.
///
/// Motor de congelamento (DEBT-13): `is_readonly = true` durante o layout de
/// cada linha impede que CounterUpdate embebido no clone avance contadores.
///
/// Números de página (DEBT-12): lidos de `label_pages` se disponíveis.
/// Na Passagem 2 (draft) estará vazio — TOC sem números.
/// Na Passagem 3 (final) terá os dados reais — TOC com páginas correctas.
pub(super) fn layout_outline<M: FontMetrics>(layouter: &mut Layouter<M>) {
    // Clonar o vector antes do loop para evitar borrow duplo de `layouter`.
    let entries: Vec<_> = layouter.counter.headings_for_toc.clone();

    // Título da TOC — fora do modo read-only (não contém efeitos colaterais).
    layouter.layout_content(&Content::heading(1, Content::text("Índice")));

    for (label, body_content, level) in entries {
        let indent = "  ".repeat(level.saturating_sub(1));

        // Ler página ANTES de activar is_readonly — evita borrow duplo.
        // Na Passagem 2 (draft), label_pages está vazio → string vazia.
        // Na Passagem 3 (final), label_pages tem os dados → "  N".
        let page_num = layouter.counter.label_pages.get(&label)
            .map(|p| format!("  {}", p))
            .unwrap_or_default();

        // O Ref usa a label automática. Se a numeração estava activa,
        // resolved_labels contém "Secção X"; se não estava, contém "".
        let line = Content::Sequence(
            vec![
                Content::text(indent),
                Content::Ref { target: label },
                Content::text(" "),
                body_content, // Content clonado — preserva formatação original
                Content::text(page_num),
                Content::Linebreak,
            ]
            .into(),
        );

        // Activar is_readonly antes do layout para bloquear CounterUpdate/step
        // durante a renderização do clone (DEBT-13).
        layouter.counter.is_readonly = true;
        layouter.layout_content(&line);
        // Restaurar DEPOIS do layout — a protecção deve cobrir toda a execução.
        layouter.counter.is_readonly = false;
    }
}
