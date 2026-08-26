//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/title.md
//! @prompt-hash f46f097b
//! @layer L1
//! @updated 2026-07-15
//!
//! Atomização (ADR-0109): layout de `Title` no arquivo da feature.
//! P765a — paridade com vanilla CLI 0.15.0: tamanho 1.7em, negrito.

use crate::entities::content::Content;
use crate::entities::elements::title::TitleElem;
use crate::entities::layout_types::TextStyle;

use super::{FontMetrics, ImageSizer, Layouter};

/// Layout de um título de documento (`#title(...)`): negrito, tamanho
/// 1.7× o tamanho actual, com quebra de linha antes e depois.
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    t: &TitleElem,
) {
    const SCALE: f64 = 1.7;

    let body = &t.body;
    let title_size = layouter.style.size * SCALE;
    let prev = layouter.style.clone();

    layouter.style = TextStyle {
        bold: true,
        italic: false,
        size: title_size,
        ..TextStyle::default()
    };
    if layouter.regions.current.cursor_x.0 > layouter.page_config.margin.left {
        layouter.flush_line();
    }

    layouter.layout_content(body);
    layouter.flush_line();
    layouter.style = prev;
}
