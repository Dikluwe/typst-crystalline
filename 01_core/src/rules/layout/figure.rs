//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/layout_figure.md
//! @prompt-hash 9e5ba4d3
//! @layer L1
//! @updated 2026-04-20

use crate::entities::content::Content;

use super::{FontMetrics, ImageSizer, Layouter};

/// Renderiza uma figura com legenda opcional (Passo 62/75, DEBT-14/15).
///
/// `caption_prefix`: prefixo de numeração pré-calculado pelo layouter
/// (ex: "Figura 1: "), ou `None` se a figura não tiver numeração activa.
/// O cálculo acontece em `layout_content` usando `figure_progress` e
/// `counter_state.figure_numbers` — a introspecção pré-computou os números.
pub(super) fn layout_figure<M: FontMetrics, S: ImageSizer>(
    layouter:       &mut Layouter<M, S>,
    body:           &Content,
    caption:        &Option<Box<Content>>,
    caption_prefix: Option<String>,
) {
    // 1. Desenhar o corpo da figura.
    layouter.layout_content(body);

    // 2. Desenhar a legenda, se existir.
    if let Some(cap) = caption {
        layouter.layout_content(&Content::Linebreak);

        if let Some(prefix) = caption_prefix {
            let caption_block = Content::Sequence(
                vec![
                    Content::text(prefix),
                    *cap.clone(),
                ]
                .into(),
            );
            layouter.layout_content(&caption_block);
        } else {
            layouter.layout_content(cap);
        }
    }
}
