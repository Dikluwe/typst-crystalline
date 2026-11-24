
#[cfg(test)]
mod borrowed_mapper_reservation_tests {
    use super::*;
    use crate::entities::content::{LocatedContent, RealizedContent};
    use crate::entities::elements::heading::HeadingElem;
    use std::sync::Arc;

    fn located(location: u128, body: Content) -> Content {
        Content::Located(Arc::new(LocatedContent {
            body,
            location: Location::from_raw(location),
        }))
    }

    fn heading(body: Content) -> Content {
        Content::Heading(Arc::new(HeadingElem::new(1, body)))
    }

    #[test]
    fn borrows_heading_and_preserves_visual_and_original_locations() {
        for (visual, original) in [(83, 17), (17, 83)] {
            let tree = heading(Content::Realized(Arc::new(RealizedContent {
                visual: located(visual, Content::Space),
                original: located(original, Content::Space),
            })));
            let mut locator = Locator::new();
            assert!(try_reserve_mapped(&tree, &mut locator, true).unwrap());
            assert_eq!(locator.next(), Location::from_raw(84));

            let mut locator = Locator::new();
            reserve_locations(&tree, &mut locator);
            assert_eq!(locator.next(), Location::from_raw(84));
        }
    }

    #[test]
    fn mapped_located_suppresses_body_root_through_source() {
        use crate::entities::content::SourceContent;
        use crate::entities::span::Span;
        let body = Content::Source(Arc::new(SourceContent {
            span: Span::detached(),
            body: located(900, Content::Sequence(Arc::from([
                located(47, Content::Space),
                Content::Space,
            ]))),
        }));
        let tree = heading(located(11, body));
        let mut locator = Locator::new();
        assert!(try_reserve_mapped(&tree, &mut locator, true).unwrap());
        assert_eq!(locator.next(), Location::from_raw(48));

        let mut legacy = Locator::new();
        tree.map_content(&mut |node| {
            if let Content::Located(e) = node {
                legacy.observe(e.location);
            }
            Ok(None)
        }).unwrap();
        assert_eq!(legacy.next(), Location::from_raw(48));
    }
}
