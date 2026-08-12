//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/list_item.md
//! @prompt-hash d39ac527
//! @layer L1
//! @updated 2026-07-23
//!
//! Atomização (ADR-0109, P380): o layout de `ListItem` movido do monólito
//! `layout_content` para o arquivo da feature (forma B). Campo `marker`
//! suportado em P470; `indent`/`body-indent`/`tight` suportados em P505.
#![allow(deprecated)] // P483 — FrameItem::Text fallback path legítimo
use crate::entities::elements::list_item::ListItemElem;
use crate::entities::layout_types::{FrameItem, Length, Point, Pt};

use super::{FontMetrics, ImageSizer, Layouter};

/// Layout de um item de lista não ordenada (`- ...`): emite o marcador na
/// margem deslocado por `indent`, indenta o cursor e o início de linha para o
/// corpo, e renderiza o body. `tight: false` adiciona espaçamento vertical
/// entre itens.
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e: &ListItemElem,
) {
    // **P751** — fixar a baseline inicial com o estilo activo antes de
    // posicionar o primeiro marcador/texto real.
    layouter.ensure_initial_baseline();
    if layouter.regions.current.cursor_x.0 > layouter.regions.current.line_start_x.0 {
        layouter.flush_line();
    }

    let margin_pt = Pt(layouter.page_config.margin);
    let font_size = layouter.style.size;

    // **P762** — espaçamento de parágrafo *entre* itens soltos usa o mesmo
    // modelo de avanço de linha: top-edge + |bottom-edge| + leading default.
    let is_loose = e.tight == Some(false);
    if is_loose && layouter.last_was_loose_item {
        let (top, bottom) = layouter.metrics.text_edges(font_size, &layouter.style);
        let leading = layouter
            .style
            .leading
            .map(|l| l.resolve_pt(font_size.val()))
            .unwrap_or_else(|| font_size.val() * 0.65);
        layouter.regions.current.cursor_y += top + Pt(-bottom.0) + Pt(leading);
    }
    layouter.last_was_loose_item = is_loose;

    // Resolve indentação com defaults.
    let indent_pt = Pt(e.indent.unwrap_or(Length::pt(0.0)).resolve_pt(font_size.val()));
    let body_indent_pt =
        Pt(e.body_indent.unwrap_or(Length::pt(0.0)).resolve_pt(font_size.val()));

    let marker_str = e.marker.as_ref().map(|m| m.render()).unwrap_or("•");
    let marker_width = layouter.metrics.advance(marker_str, font_size, &layouter.style);

    let marker_x = margin_pt + indent_pt;
    let body_x = marker_x + marker_width + body_indent_pt;

    // Salva o início de linha original para restaurar após o body.
    let saved_line_start_x = layouter.regions.current.line_start_x;

    // Emite o marcador na linha actual.
    layouter.regions.current.current_line.push(FrameItem::Text {
        pos: Point { x: marker_x, y: layouter.regions.current.cursor_y },
        text: marker_str.into(),
        style: layouter.style.clone(),
    });

    // Prepara o cursor para o corpo e força quebras de linha subsequentes a
    // manterem a indentação do corpo.
    layouter.regions.current.line_start_x = body_x;
    layouter.regions.current.cursor_x = body_x;

    layouter.layout_content(&e.body);
    layouter.flush_line();

    // Restaura o início de linha original.
    layouter.regions.current.line_start_x = saved_line_start_x;
    layouter.regions.current.cursor_x = margin_pt;
}
