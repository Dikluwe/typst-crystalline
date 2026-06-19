//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/atomizacao_elementos.md
//! @prompt-hash 3331d6ba
//! @layer L1
//! @updated 2026-06-19
//!
//! Atomização (ADR-0109, P377): o layout de `Columns` movido do monólito
//! `layout_content` para o arquivo da feature (forma B — free function na
//! camada de render). Content-preserving — a lógica é idêntica.

use crate::entities::elements::columns::ColumnsElem;

use super::{FontMetrics, ImageSizer, Layouter};

/// Layout de `columns(...)`: reduz a width útil para `column_width` durante o
/// body e restaura no fim (structural — começa em nova linha lógica).
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
    let count_f = if e.count == 0 { 1.0 } else { e.count as f64 };

    // 2. Resolver gutter (Length → f64 Pt; default ~4% width).
    let gutter_pt = match e.gutter {
        Some(g) => g.resolve_pt(layouter.font_size_pt.0),
        None => full_width * super::COLUMNS_DEFAULT_GUTTER_RATIO,
    };

    // 3. column_width = (full_width - (count-1)*gutter) / count.
    let column_width = if count_f >= 1.0 {
        (full_width - (count_f - 1.0) * gutter_pt) / count_f
    } else {
        full_width
    };

    // 4. Saved/restore pattern (paridade P156C Pad cursor_x).
    let saved_width = full_width;
    layouter.regions.current.width = column_width;

    // 5. Layout body com width reduzida.
    layouter.layout_content(&e.body);

    // 6. Flush line pendente do body antes de restaurar.
    if layouter.regions.current.cursor_x.0 > layouter.regions.current.line_start_x.0 {
        layouter.flush_line();
    }

    // 7. Restaurar width original (invariante crucial — conteúdo
    //    subsequente fora do columns block volta a width original).
    layouter.regions.current.width = saved_width;
}
