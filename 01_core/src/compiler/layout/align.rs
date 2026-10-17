//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/align.md
//! @prompt-hash 0977a51a
//! @layer L1
//! @updated 2026-09-26

use super::{FontMetrics, ImageSizer, Layouter};
use crate::entities::elements::align::AlignElem;

pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e: &AlignElem,
) {
    layouter.layout_align(e.alignment, &e.body);
}

#[cfg(test)]
mod tests {
    use crate::compiler::layout::{FixedMetrics, Layouter};
    use crate::entities::{
        content::Content,
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
    };
    use comemo::Track;

    fn make(intr: &dyn Introspector) -> Layouter<'_, FixedMetrics, NullImageSizer> {
        Layouter::new(FixedMetrics, NullImageSizer, 11.0, intr.track())
    }

    #[test]
    fn atomized_align_delegates_without_losing_body_or_leaking_alignment() {
        use crate::entities::{elements::align::AlignElem, layout_types::Align2D};
        let intr = TagIntrospector::empty();
        let mut layouter = make(&intr);
        let original = layouter.style.clone();
        layouter.layout_content(&Content::Align(std::sync::Arc::new(AlignElem {
            alignment: Align2D::from_string("center"),
            body: Content::text("Inside"),
        })));
        assert_eq!(layouter.style, original);
        layouter.layout_content(&Content::text("Outside"));
        let text = layouter.finish().plain_text();
        assert!(text.contains("Inside"));
        assert!(text.contains("Outside"));
    }
}
