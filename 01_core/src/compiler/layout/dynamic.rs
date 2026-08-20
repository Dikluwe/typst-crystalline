//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout.md
//! @prompt-hash 0054a989
//! @layer L1
//! @updated 2026-06-23
//!
//! Layout de `Content::Dynamic` — elemento dinâmico de utilizador
//! (fronteira E1). Extraído de `layout/mod.rs` no P425 (ADR-0109 forma B).

use std::sync::Arc;

use crate::entities::{
    content::Content, elements::dynamic::DynElement, image_sizer::ImageSizer,
    value::Value,
};

use super::metrics::FontMetrics;
use super::Layouter;

/// Layout de um elemento dinâmico: resolve campos setáveis da chain e
/// renderiza o `body` (ou `plain_text` em fallback).
///
/// F-item3 (P368, §3a.11): `#set <kind>(prop:)` chega como
/// `custom("<kind>.<prop>")`; precedência construído > chain no próprio
/// elemento. O custom é transparente à morfologia (P366).
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<'_, M, S>,
    e: &Arc<dyn DynElement>,
) {
    let kind = e.dyn_kind();
    let resolved = e.dyn_resolve_settable(&|prop| {
        layouter.chain.custom(&format!("{kind}.{prop}")).cloned()
    });
    let re: &dyn DynElement = match &resolved {
        Content::Dynamic(re) => re.as_ref(),
        _ => e.as_ref(),
    };
    match re.dyn_get_field("body") {
        Some(Value::Content(body)) => {
            layouter.layout_content(&body);
        }
        _ => {
            let t = re.dyn_plain_text();
            if !t.is_empty() {
                layouter.layout_content(&Content::text(t));
            }
        }
    }
}
