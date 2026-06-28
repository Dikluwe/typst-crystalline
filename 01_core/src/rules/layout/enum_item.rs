//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/atomizacao_elementos.md
//! @prompt-hash e6442e3f
//! @layer L1
//! @updated 2026-06-26
//!
//! Atomização (ADR-0109, P380): o layout de `EnumItem` movido do monólito
//! `layout_content` para o arquivo da feature (forma B). Campo `numbering`
//! suportado em P470.
#![allow(deprecated)] // P483 — FrameItem::Text fallback path legítimo
use crate::entities::elements::enum_item::EnumItemElem;
use crate::entities::enum_numbering::EnumNumbering;
use crate::entities::layout_types::{FrameItem, Point, Pt};

use super::{FontMetrics, ImageSizer, Layouter};

/// Layout de um item de lista ordenada (`+ ...` / `1. ...`): emite o rótulo
/// numérico na margem (default Decimal `"N."` ou esquema configurado via P470),
/// indenta o cursor e renderiza o body.
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e:        &EnumItemElem,
) {
    if layouter.regions.current.cursor_x.0 > layouter.page_config.margin { layouter.flush_line(); }
    let margin_pt = Pt(layouter.page_config.margin);
    let label: ecow::EcoString = match e.number {
        Some(n) => {
            let scheme = e.numbering.as_ref().unwrap_or(&EnumNumbering::Decimal);
            scheme.format(n).into()
        }
        None => "-".into(),
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
