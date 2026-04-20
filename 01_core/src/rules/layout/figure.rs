//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/layout_figure.md
//! @prompt-hash 9e5ba4d3
//! @layer L1
//! @updated 2026-04-13

use crate::entities::content::Content;

use super::{FontMetrics, ImageSizer, Layouter};

/// Renderiza uma figura com legenda opcional.
///
/// A dupla contagem é intencional: a introspecção conta na Passagem 1 para
/// resolver labels; o Layouter conta aqui na Passagem 2 para gerar os prefixos
/// visuais iterativamente na ordem correcta.
pub(super) fn layout_figure<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    body:     &Content,
    caption:  &Option<Box<Content>>,
) {
    // 1. Desenhar o corpo da figura.
    layouter.layout_content(body);

    // 2. Avançar o contador visual apenas se a figura tiver legenda —
    // mesma regra da introspecção (Passagem 1) para manter sincronização.
    let is_numbered = layouter.counter.is_numbering_active("figure") && caption.is_some();
    if is_numbered {
        layouter.counter.step_flat("figure");
    }

    // 3. Desenhar a legenda, se existir.
    if let Some(cap) = caption {
        layouter.layout_content(&Content::Linebreak);

        if is_numbered {
            let n = layouter.counter.get_flat("figure");
            // Agrupar prefixo e legenda num único Sequence para garantir que ficam
            // na mesma linha. DEBT-13: clonar `cap` aqui pode duplicar side-effects
            // se a legenda contiver CounterUpdate — ver DEBT-13 em DEBT.md.
            let caption_block = Content::Sequence(
                vec![
                    Content::text(format!("Figura {}: ", n)),
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
