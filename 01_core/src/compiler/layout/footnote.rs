//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/atomizacao_elementos.md
//! @prompt-hash bf8f0b19
//! @layer L1
//! @updated 2026-06-19
//!
//! Atomização (ADR-0109, P381): o layout de `Footnote` movido do monólito
//! `layout_content` para o arquivo da feature (forma B). Content-preserving.
//! Estado dedicado (`pending_footnote_bodies`) acedido por descendência de
//! módulo — sem extracção.
//!
//! **P1016** — o número deixou de vir de `Layouter::footnote_counter` (campo
//! local, removido) e passa a vir do introspector, como no P461 para `Table`.
//! É o que torna `counter(footnote)` visível à língua.

use crate::entities::content::Content;
use crate::entities::elements::footnote::FootnoteElem;
use crate::entities::introspector::Introspector;

use super::{FontMetrics, ImageSizer, Layouter};

/// Layout de `footnote(...)`: emite o marcador `[N]` inline e difere o body
/// para o rodapé via `pending_footnote_bodies` (flush em new_page/finish).
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e: &FootnoteElem,
) {
    // **P1016** — número via introspector (counter flat `"footnote"`), fonte
    // única. `current_location` foi actualizado no topo de `layout_content`
    // porque Footnote é locatable desde este passo. O `unwrap_or` cobre a
    // primeira iteração do fixpoint, mesma forma que `layout/table.rs:37-40`.
    let n = layouter
        .current_location
        .and_then(|loc| layouter.introspector.flat_counter_at("footnote", loc))
        .unwrap_or(1) as u32;
    let marker = format!("[{}]", n);
    layouter.layout_content(&Content::text(marker));
    layouter.pending_footnote_bodies.push((n, Box::new(e.body.clone())));
}
