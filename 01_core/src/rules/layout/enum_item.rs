//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/layout/enum_item.md
//! @prompt-hash 5ec37264
//! @layer L1
//! @updated 2026-06-29
//!
//! Atomização (ADR-0109, P380): o layout de `EnumItem` movido do monólito
//! `layout_content` para o arquivo da feature (forma B). Campo `numbering`
//! suportado em P470; `indent`/`body-indent`/`tight` suportados em P505.
#![allow(deprecated)] // P483 — FrameItem::Text fallback path legítimo
use crate::entities::elements::enum_item::EnumItemElem;
use crate::entities::enum_numbering::EnumNumbering;
use crate::entities::layout_types::{FrameItem, Length, Point, Pt};

use super::{FontMetrics, ImageSizer, Layouter};

/// Layout de um item de lista ordenada (`+ ...` / `1. ...`): emite o rótulo
/// numérico na margem deslocado por `indent`, indenta o cursor e o início de
/// linha para o corpo, e renderiza o body. `tight: false` adiciona espaçamento
/// vertical entre itens.
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e:        &EnumItemElem,
) {
    // **P751** — fixar a baseline inicial com o estilo activo antes de
    // posicionar o primeiro rótulo/texto real.
    layouter.ensure_initial_baseline();
    if layouter.regions.current.cursor_x.0 > layouter.regions.current.line_start_x.0 {
        layouter.flush_line();
    }

    let margin_pt = Pt(layouter.page_config.margin);
    let font_size = layouter.style.size;

    // P505 — espaçamento de parágrafo *entre* itens soltos.
    let is_loose = e.tight == Some(false);
    if is_loose && layouter.last_was_loose_item {
        let (_, line_height) = layouter.metrics.vertical_metrics(font_size);
        layouter.regions.current.cursor_y += line_height;
    }
    layouter.last_was_loose_item = is_loose;

    // Resolve indentação com defaults.
    let indent_pt = Pt(e.indent.unwrap_or(Length::pt(0.0)).resolve_pt(font_size.val()));
    let body_indent_pt =
        Pt(e.body_indent.unwrap_or(Length::pt(0.0)).resolve_pt(font_size.val()));

    let label: ecow::EcoString = match e.number {
        Some(n) => {
            let scheme = e.numbering.as_ref().unwrap_or(&EnumNumbering::Decimal);
            scheme.format(n).into()
        }
        None => "-".into(),
    };
    let label_width = layouter.metrics.advance(&label, font_size, &layouter.style);

    let label_x = margin_pt + indent_pt;
    let body_x = label_x + label_width + body_indent_pt;

    // Salva o início de linha original para restaurar após o body.
    let saved_line_start_x = layouter.regions.current.line_start_x;

    // Emite o rótulo na linha actual.
    layouter.regions.current.current_line.push(FrameItem::Text {
        pos:   Point { x: label_x, y: layouter.regions.current.cursor_y },
        text:  label,
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
