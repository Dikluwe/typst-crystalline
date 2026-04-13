//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/layout_outline.md
//! @prompt-hash 0680a55d
//! @layer L1
//! @updated 2026-04-13

use crate::entities::content::Content;

use super::{FontMetrics, Layouter};

/// Gera a sequência visual da Tabela de Conteúdos.
/// Lê `headings_for_toc` do estado injectado pela introspecção.
/// Usa Content clonado (não String) para preservar formatação dos títulos.
pub(super) fn layout_outline<M: FontMetrics>(layouter: &mut Layouter<M>) {
    // Clonar o vector antes do loop para evitar borrow duplo de `layouter`:
    // `layouter.counter` (borrow imutável) e `layouter.layout_content` (borrow mutável).
    let entries: Vec<_> = layouter.counter.headings_for_toc.clone();

    let mut seq = Vec::new();

    // Título da TOC
    seq.push(Content::heading(1, Content::text("Índice")));

    for (label, body_content, level) in entries {
        // Indentação proporcional ao nível.
        let indent = "  ".repeat(level.saturating_sub(1));

        // O Ref usa a label automática. Se a numeração estava activa,
        // resolved_labels contém "Secção X"; se não estava, contém "".
        // Em ambos os casos, o braço Ref não usa o fallback "@auto-toc-N".
        let line = Content::Sequence(
            vec![
                Content::text(indent),
                Content::Ref { target: label },
                Content::text(" "),
                body_content, // Content clonado — preserva formatação original
                              // ATENÇÃO: se o título contiver CounterUpdate ou CounterDisplay,
                              // esses nós serão avaliados pelo Layouter novamente aqui,
                              // causando duplicação de efeitos (ex: contador avançado duas vezes).
                              // Registado como DEBT-13 — ver Tarefa 6.
                Content::Linebreak,
            ]
            .into(),
        );
        seq.push(line);
    }

    let toc_content = Content::Sequence(seq.into());
    layouter.layout_content(&toc_content);
}
