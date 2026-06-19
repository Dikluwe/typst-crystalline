//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/atomizacao_elementos.md
//! @prompt-hash cedb58ca
//! @layer L1
//! @updated 2026-06-19
//!
//! Atomização (ADR-0109, P380): o layout de `ListItem` movido do monólito
//! `layout_content` para o arquivo da feature (forma B). Content-preserving.

use crate::entities::elements::list_item::ListItemElem;
use crate::entities::layout_types::{FrameItem, Point, Pt};

use super::{FontMetrics, ImageSizer, Layouter};

/// Layout de um item de lista não ordenada (`- ...`): emite o marcador `•` na
/// margem, indenta o cursor e renderiza o body.
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e:        &ListItemElem,
) {
    if layouter.regions.current.cursor_x.0 > layouter.page_config.margin { layouter.flush_line(); }
    let margin_pt = Pt(layouter.page_config.margin);
    layouter.regions.current.current_line.push(FrameItem::Text {
        pos:   Point { x: margin_pt, y: layouter.regions.current.cursor_y },
        text:  "•".into(),  // U+2022 — suportado com CIDFont (DEBT-5 pago)
        style: layouter.style.clone(),
    });
    layouter.regions.current.cursor_x = margin_pt + layouter.font_size_pt * 1.5;
    layouter.layout_content(&e.body);
    layouter.flush_line();
    layouter.regions.current.cursor_x = margin_pt;
}
