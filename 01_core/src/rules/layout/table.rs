//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/atomizacao_elementos.md
//! @prompt 00_nucleo/prompts/rules/layout/table.md
//! @prompt-hash d36ca01b
//! @layer L1
//! @updated 2026-06-25
//!
//! Atomização (ADR-0109, P380): o layout de `Table` movido do monólito
//! `layout_content` para o arquivo da feature (forma B). Content-preserving.
//! Cluster `layout_grid` (P379): delega ao mesmo motor que `Grid`.
//! P459: caption numerado posicionado acima da table.

use crate::entities::content::Content;
use crate::entities::counter_format::format_counter;
use crate::entities::elements::table::TableElem;
use crate::entities::introspector::Introspector;
use crate::entities::value::Value;

use super::{FontMetrics, ImageSizer, Layouter};

/// Layout de `table(...)` (P157A, ADR-0060 Fase 2): caption numerado acima
/// (P459) e delegação do grid para `layout_grid`.
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e:        &TableElem,
) {
    // P459 — ler `table.numbering` da chain e computar prefixo se houver caption.
    let numbering_pattern = layouter
        .chain
        .custom("table.numbering")
        .and_then(|v| match v {
            Value::Str(s) => Some(s.as_str()),
            _ => None,
        });
    let caption_prefix: Option<String> = if e.caption.is_some() {
        if let Some(pattern) = numbering_pattern {
            // P461 — número via Introspector (`"table"` counter), não campo
            // local do Layouter. `current_location` foi actualizado no topo
            // de `layout_content` porque Table é locatable.
            let table_number = layouter
                .current_location
                .and_then(|loc| layouter.introspector.flat_counter_at("table", loc))
                .unwrap_or(1);
            let formatted = format_counter(&[table_number], pattern)
                .unwrap_or_else(|| table_number.to_string());
            Some(format!("Table {}: ", formatted))
        } else {
            None
        }
    } else {
        None
    };

    // P459 — caption posicionada ACIMA da table (vanilla default).
    if let Some(cap) = &e.caption {
        if let Some(prefix) = caption_prefix {
            let captioned = Content::Sequence(
                vec![Content::text(prefix), cap.clone()].into(),
            );
            layouter.layout_content(&captioned);
        } else {
            layouter.layout_content(cap);
        }
        layouter.layout_content(&Content::linebreak());
    }

    // P224+P227+P228 — Table delegate; herda stroke + fill.
    layouter.layout_grid(&e.columns, &e.rows, &e.children,
                         None, None,
                         crate::entities::sides::Sides::uniform(
                             crate::entities::layout_types::Length::pt(0.0)),
                         None, None,
                         e.stroke.as_ref(), e.fill.as_ref());
}
