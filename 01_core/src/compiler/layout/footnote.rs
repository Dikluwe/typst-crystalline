//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/footnote.md
//! @prompt-hash d49c3c3d
//! @layer L1
//! @updated 2026-08-12
//!
//! Atomização (ADR-0109, P381): o layout de `Footnote` movido do monólito
//! `layout_content` para o arquivo da feature (forma B). Content-preserving.
//! Estado dedicado (`pending_footnote_bodies`) acedido por descendência de
//! módulo — sem extracção.
//! **P1020** — marcador inline formatado por `format_pattern` e envolvido em
//! `Content::superscript`, sem colchetes literais.
//!
//! **P1016** — o número deixou de vir de `Layouter::footnote_counter` (campo
//! local, removido) e passa a vir do introspector, como no P461 para `Table`.
//! É o que torna `counter(footnote)` visível à língua.

use crate::entities::content::Content;
use crate::entities::counter::CounterKey;
use crate::entities::counter_format::format_counter;
use crate::entities::element_kind::ElementKind;
use crate::entities::elements::footnote::FootnoteElem;
use crate::entities::introspector::Introspector;
use crate::entities::selector::Selector;

use super::{FontMetrics, ImageSizer, Layouter};

/// Layout de `footnote(...)`: emite o marcador `N` em superscript inline e
/// difere o body para o rodapé via `pending_footnote_bodies`
/// (flush em new_page/finish).
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e: &FootnoteElem,
) {
    // **P1016** — número via introspector (counter `Selector(Kind(Footnote))`),
    // fonte única. `current_location` foi actualizado no topo de
    // `layout_content` porque Footnote é locatable desde este passo.
    let footnote_key = CounterKey::Selector(Selector::Kind(ElementKind::Footnote));
    let n = layouter
        .current_location
        .and_then(|loc| layouter.introspector.flat_counter_at(&footnote_key, loc))
        .unwrap_or(1) as usize;
    // **P1020** — marcador formatado pelo pattern `e.numbering` (default "1")
    // e envolvido em superscript, em vez do literal `[N]`.
    let pattern = e.numbering.as_deref().unwrap_or("1");
    let marker_text = format_counter(&[n], pattern).unwrap_or_else(|| n.to_string());
    let marker = Content::superscript(Content::text(marker_text));
    layouter.layout_content(&marker);
    layouter.pending_footnote_bodies.push((n as u32, Box::new(e.body.clone())));
}
