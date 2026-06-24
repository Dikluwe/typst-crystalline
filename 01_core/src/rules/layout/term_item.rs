//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/atomizacao_elementos.md
//! @prompt-hash 3331d6ba
//! @layer L1
//! @updated 2026-06-19
//!
//! Atomização (ADR-0109, P380): o layout de `TermItem` movido do monólito
//! `layout_content` para o arquivo da feature (forma B). Content-preserving.

use crate::entities::content::Content;
use crate::entities::elements::term_item::TermItemElem;
use crate::entities::layout_types::{Pt, TextStyle};
use crate::entities::style::{Style, Styles};

use super::{FontMetrics, ImageSizer, Layouter};

/// Layout de um item de definição (`/ termo: descrição`): o termo em negrito
/// (push de `Bold` na chain), `": "`, e a descrição.
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e:        &TermItemElem,
) {
    if layouter.regions.current.cursor_x.0 > layouter.page_config.margin { layouter.flush_line(); }
    let margin_pt = Pt(layouter.page_config.margin);
    layouter.regions.current.cursor_x = margin_pt + layouter.font_size_pt * 1.5;
    // O termo aparece em negrito — convenção de listas de definições.
    let prev_chain = layouter.chain.clone();
    let prev_style = layouter.style.clone();
    layouter.chain = layouter.chain.push_styles(&Styles::from_iter([Style::bold(true)]));
    layouter.style = TextStyle::from(&layouter.chain);
    layouter.layout_content(&e.term);
    layouter.chain = prev_chain;
    layouter.style = prev_style;
    layouter.layout_content(&Content::text(": "));
    layouter.layout_content(&e.description);
    layouter.flush_line();
    layouter.regions.current.cursor_x = margin_pt;
}
