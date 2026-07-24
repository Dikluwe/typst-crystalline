//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/atomizacao_elementos.md
//! @prompt-hash 2be1ad5d
//! @layer L1
//! @updated 2026-06-19
//!
//! Atomização (ADR-0109, P381): o layout de `Footnote` movido do monólito
//! `layout_content` para o arquivo da feature (forma B). Content-preserving.
//! Estado dedicado (`footnote_counter`/`pending_footnote_bodies`) acedido por
//! descendência de módulo — sem extracção.

use crate::entities::content::Content;
use crate::entities::elements::footnote::FootnoteElem;

use super::{FontMetrics, ImageSizer, Layouter};

/// Layout de `footnote(...)`: emite o marcador `[N]` inline e difere o body
/// para o rodapé via `pending_footnote_bodies` (flush em new_page/finish).
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e: &FootnoteElem,
) {
    layouter.footnote_counter += 1;
    let n = layouter.footnote_counter;
    let marker = format!("[{}]", n);
    layouter.layout_content(&Content::text(marker));
    layouter.pending_footnote_bodies.push((n, Box::new(e.body.clone())));
}
