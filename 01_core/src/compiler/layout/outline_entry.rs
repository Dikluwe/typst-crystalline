//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/outline_entry.md
//! @prompt-hash 80b009a0
//! @layer L1
//! @updated 2026-09-26

use super::{FontMetrics, ImageSizer, Layouter};
use crate::entities::elements::outline_entry::OutlineEntryElem;

pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e: &OutlineEntryElem,
) {
    if let Ok(body) = e.body() {
        layouter.layout_content(&body);
    }
}

#[cfg(test)]
mod tests {
    use crate::entities::content::Content;

    #[test]
    fn atomized_outline_entry_descends_only_into_valid_body() {
        use crate::entities::elements::outline_entry::OutlineEntryElem;
        for (body, expected) in [
            (Content::heading(1, Content::text("Title")), "Title"),
            (Content::text("Invalid"), ""),
        ] {
            let content = Content::OutlineEntry(std::sync::Arc::new(
                OutlineEntryElem::new(1, body, Some(Content::text("Fill")), true),
            ));
            assert_eq!(crate::compiler::layout::layout(&content).plain_text(), expected);
        }
    }
}
