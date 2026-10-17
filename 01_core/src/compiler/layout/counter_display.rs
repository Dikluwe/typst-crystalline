//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/counter_display.md
//! @prompt-hash 49991ac5
//! @layer L1
//! @updated 2026-09-26

use super::{FontMetrics, ImageSizer, Layouter};
use crate::entities::content::Content;
use crate::entities::elements::counter_display::CounterDisplayElem;

pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e: &CounterDisplayElem,
) {
    // P190I (M6 fechado): Layouter consome via Introspector
    // path location-aware. `current_location` set por
    // walk-content para locatable contents (P185C). Para
    // CounterDisplay (não-locatable), usa última location
    // emitida (snapshot até este ponto).
    use crate::entities::introspector::Introspector;
    let kind = &e.kind;
    let text = layouter
        .current_location
        .and_then(|loc| layouter.introspector.formatted_counter_at(kind, loc))
        .unwrap_or_else(|| {
            layouter
                .introspector
                .formatted_counter(kind)
                .unwrap_or_else(|| "0".to_string())
        });
    let display = Content::text(text);
    layouter.layout_content(&display);
}

#[cfg(test)]
mod tests {
    use crate::entities::content::Content;

    #[test]
    fn atomized_legacy_counter_keeps_zero_fallback() {
        let doc = crate::compiler::layout::layout(&Content::counter_display(
            "absent".to_string(),
        ));
        assert_eq!(doc.plain_text(), "0");
    }

    #[test]
    fn atomized_legacy_counter_reads_updated_value() {
        use crate::entities::counter_update::CounterUpdate;
        let content = Content::sequence(vec![
            Content::counter_update("named".to_string(), CounterUpdate::Set(vec![5])),
            Content::counter_display("named".to_string()),
        ]);
        assert_eq!(crate::compiler::layout::layout(&content).plain_text(), "5");
    }
}
