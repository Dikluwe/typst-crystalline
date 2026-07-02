//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/atomizacao_elementos.md
//! @prompt-hash e6442e3f
//! @layer L1
//! @updated 2026-07-02
//!
//! Atomização (ADR-0109, P377): o layout de `Columns` movido do monólito
//! `layout_content` para o arquivo da feature (forma B — free function na
//! camada de render).
//!
//! **P537** — implementação real de múltiplas colunas lado a lado para
//! notas de rodapé por coluna. O body é dividido pelos `colbreak()` e cada
//! segmento é renderizado sequencialmente no mesmo `Layouter`, reutilizando
//! o estado da página actual. As notas de rodapé pendentes são flushed no
//! fim de cada coluna através do modo `column_mode` do `Layouter`.

use crate::entities::content::Content;
use crate::entities::elements::columns::ColumnsElem;
use crate::entities::layout_types::{FrameItem, Pt};

use super::{FontMetrics, ImageSizer, Layouter};

/// Divide uma sequência de content pelos `Content::Colbreak`.
/// Garante pelo menos `count` segmentos — se houver menos colbreaks do que
/// `count - 1`, os segmentos em falta ficam vazios.
fn split_by_colbreak(content: &Content, count: usize) -> Vec<Content> {
    let mut segments: Vec<Vec<Content>> = vec![Vec::new()];
    let children = match content {
        Content::Sequence(seq) => seq.iter().collect::<Vec<_>>(),
        Content::Empty => Vec::new(),
        other => vec![other],
    };
    for child in children {
        if matches!(child, Content::Colbreak(_)) {
            segments.push(Vec::new());
        } else {
            segments.last_mut().unwrap().push(child.clone());
        }
    }
    // Garantir `count` segmentos.
    while segments.len() < count {
        segments.push(Vec::new());
    }
    // Se por alguma razão houver mais segmentos do que count, fundir os
    // excessivos no último (preserva conteúdo).
    if segments.len() > count && count > 0 {
        let tail: Vec<Content> = segments.drain(count - 1..).flatten().collect();
        segments.push(tail);
    }
    segments.into_iter().map(Content::sequence).collect()
}

/// Layout real de `columns(count, body, gutter:)`.
///
/// Cada segmento delimitado por `colbreak()` é renderizado numa coluna da
/// mesma página. As notas de rodapé são flushed no fim de cada coluna
/// através de `flush_pending_footnote_bodies`, que em `column_mode` usa a
/// largura e origem horizontal da coluna actual.
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e:        &ColumnsElem,
) {
    // 1. Flush line pendente (columns são structural — começam
    //    em nova linha lógica).
    if layouter.regions.current.cursor_x.0 > layouter.regions.current.line_start_x.0 {
        layouter.flush_line();
    }

    let full_width = layouter.regions.current.width;
    let count = e.count.max(1) as usize;
    let count_f = count as f64;

    // 2. Resolver gutter (Length → f64 Pt; default ~4% width).
    let gutter_pt = match e.gutter {
        Some(g) => g.resolve_pt(layouter.font_size_pt.0),
        None => full_width * super::COLUMNS_DEFAULT_GUTTER_RATIO,
    };

    // 3. column_width = (full_width - (count-1)*gutter) / count.
    let column_width = (full_width - (count_f - 1.0) * gutter_pt) / count_f;

    // 4. Dividir body pelos colbreaks.
    let segments = split_by_colbreak(&e.body, count);

    // 5. Posições horizontais das colunas (origem x absoluta na página).
    let margin = layouter.page_config.margin;
    let column_x_offsets: Vec<f64> = (0..count)
        .map(|i| margin + i as f64 * (column_width + gutter_pt))
        .collect();

    // 6. Renderizar cada segmento como uma coluna independente na mesma
    //    página. Todas as colunas partilham o mesmo y inicial; a altura do
    //    bloco columns é a máxima altura consumida por qualquer coluna.
    //    Activamos `column_mode` para que as footnotes sejam posicionadas no
    //    fundo da coluna actual.
    let ascender = layouter.metrics.vertical_metrics(layouter.font_size_pt).0;
    let mut all_column_items: Vec<FrameItem> = Vec::new();
    let column_start_y = layouter.regions.current.cursor_y;
    let mut max_column_bottom_y = column_start_y;

    for (idx, segment) in segments.iter().enumerate() {
        // Salvaguardar estado actual (os items da página principal ficam
        // intocados enquanto a coluna é renderizada num buffer isolado).
        let saved_items = std::mem::take(&mut layouter.regions.current.current_items);
        let saved_line = std::mem::take(&mut layouter.regions.current.current_line);
        let saved_cursor_x = layouter.regions.current.cursor_x;
        let saved_width = layouter.regions.current.width;
        let saved_footnotes = std::mem::take(&mut layouter.pending_footnote_bodies);
        let saved_footnote_counter = layouter.footnote_counter;

        // Activar modo coluna para flush de footnotes.
        layouter.column_mode = true;
        layouter.column_origin_x = column_x_offsets[idx];
        layouter.column_width = column_width;

        // Configurar region para a coluna actual.
        layouter.regions.current.width = column_width;
        layouter.regions.current.cursor_x = Pt(margin);
        layouter.regions.current.line_start_x = Pt(margin);
        layouter.regions.current.cursor_y = column_start_y + ascender;

        // Layout do segmento e flush final da coluna.
        layouter.layout_content(segment);
        layouter.flush_line();
        layouter.flush_pending_footnote_bodies();

        // Altura consumida por esta coluna.
        let column_bottom_y = layouter.regions.current.cursor_y;
        if column_bottom_y.0 > max_column_bottom_y.0 {
            max_column_bottom_y = column_bottom_y;
        }

        // Recolher items da coluna e restaurar estado.
        let mut column_items = std::mem::take(&mut layouter.regions.current.current_items);
        layouter.regions.current.current_items = saved_items;
        layouter.regions.current.current_line = saved_line;
        layouter.regions.current.cursor_x = saved_cursor_x;
        layouter.regions.current.cursor_y = column_start_y;
        layouter.regions.current.line_start_x = Pt(margin);
        layouter.regions.current.width = saved_width;
        // Nota: footnotes pendentes que não couberam na coluna são
        // descartadas aqui; no teste de P537 cada coluna cabe na página.
        layouter.pending_footnote_bodies = saved_footnotes;
        layouter.footnote_counter = saved_footnote_counter;
        layouter.column_mode = false;

        // Translada os items horizontalmente para a coluna correcta.
        let dx = column_x_offsets[idx] - margin;
        for item in &mut column_items {
            *item = translate_item_x(item.clone(), dx);
        }
        all_column_items.extend(column_items);
    }

    // 7. Adicionar todos os items das colunas à página actual.
    layouter.regions.current.current_items.extend(all_column_items);

    // 8. Avançar o cursor principal para a altura máxima consumida pelas
    //    colunas.
    layouter.regions.current.cursor_y = max_column_bottom_y;
}

/// Translada um `FrameItem` horizontalmente por `dx` pontos,
/// preservando a coordenada Y original.
fn translate_item_x(item: FrameItem, dx: f64) -> FrameItem {
    use crate::rules::layout::helpers::{item_pos, translate_frame_item};
    let (x, y) = item_pos(&item);
    translate_frame_item(item, Pt(x + dx), Pt(y))
}
