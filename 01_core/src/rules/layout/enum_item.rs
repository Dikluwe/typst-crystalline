//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/atomizacao_elementos.md
//! @prompt-hash 03f97ccc
//! @layer L1
//! @updated 2026-06-19
//!
//! Atomização (ADR-0109, P380): o layout de `EnumItem` movido do monólito
//! `layout_content` para o arquivo da feature (forma B). Content-preserving.

use ecow::EcoString;

use crate::entities::elements::enum_item::EnumItemElem;
use crate::entities::layout_types::{FrameItem, Point, Pt};

use super::{FontMetrics, ImageSizer, Layouter};

/// Layout de um item de lista ordenada (`+ ...` / `1. ...`): emite o rótulo
/// numérico na margem, indenta o cursor e renderiza o body.
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e:        &EnumItemElem,
) {
    if layouter.regions.current.cursor_x.0 > layouter.page_config.margin { layouter.flush_line(); }
    let margin_pt = Pt(layouter.page_config.margin);
    let label: EcoString = match e.number {
        Some(n) => format!("{}.", n).into(),
        None    => "-".into(),
    };
    layouter.regions.current.current_line.push(FrameItem::Text {
        pos:   Point { x: margin_pt, y: layouter.regions.current.cursor_y },
        text:  label,
        style: layouter.style.clone(),
    });
    layouter.regions.current.cursor_x = margin_pt + layouter.font_size_pt * 2.0;
    layouter.layout_content(&e.body);
    layouter.flush_line();
    layouter.regions.current.cursor_x = margin_pt;
}
