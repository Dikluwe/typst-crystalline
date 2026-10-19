//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/state_display.md
//! @prompt-hash 4d4a4831
//! @layer L1
//! @updated 2026-09-26

use super::{FontMetrics, ImageSizer, Layouter};
use crate::entities::elements::state_display::StateDisplayElem;

pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e: &StateDisplayElem,
) {
    use crate::entities::introspector::Introspector;
    if let Some(loc) = layouter.current_location {
        let pre_rendered_opt =
            layouter.introspector.state_display_value(e.key.clone(), loc);
        if let Some(pre_rendered) = pre_rendered_opt {
            layouter.layout_content(&pre_rendered);
        }
        // Sem pre_rendered: defensive ignore (fixpoint pre-walk
        // ainda não convergiu OR Func errored OR key inexistente).
    }
    // Sem current_location: defensive ignore (walk pre-Locator).
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
    fn atomized_display_uses_only_the_causal_pre_rendered_value() {
        use crate::entities::{
            elements::state_display::StateDisplayElem, func::Func, locator::Locator,
        };
        let mut locator = Locator::new();
        let loc = locator.next();
        let other = locator.next();
        let key = "key".to_string();
        let mut intr = TagIntrospector::empty();
        intr.state_displays.insert((key.clone(), loc), Content::text("Ready"));
        intr.state_displays
            .insert((key.clone(), other), Content::text("Other"));
        let elem = StateDisplayElem {
            key,
            callback: Some(Func::native("must-not-run", |_, _, _, _| {
                panic!("layout executed callback")
            })),
        };
        for (location, expected) in [
            (None, ""),
            (Some(loc), "Ready"),
            (Some(other), "Other"),
            (Some(locator.next()), ""),
        ] {
            let mut layouter = make(&intr);
            layouter.current_location = location;
            super::layout(&mut layouter, &elem);
            assert_eq!(layouter.finish().plain_text(), expected);
        }
    }
}
