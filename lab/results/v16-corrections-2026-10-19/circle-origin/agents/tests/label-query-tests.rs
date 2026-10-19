
#[cfg(test)]
mod circle_origin_label_tests {
    use super::*;
    use crate::entities::content::Content;
    use crate::entities::value::IntrospectedContent;

    #[test]
    fn circle_origin_label_selector_returns_all_distinct_matching_occurrences() {
        let mut locator = Locator::new();
        let first = locator.next();
        let other = locator.next();
        let second = locator.next();
        let matching = Label("matching".into());
        let unrelated = Label("other".into());
        let mut intr = TagIntrospector::empty();
        let equal_content = Content::heading(1, Content::text("same"));
        for location in [first, other, second] {
            intr.counters.observe_location(location);
            intr.elements.insert(location, IntrospectedContent::new(equal_content.clone(), None));
        }
        intr.labels.add(matching.clone(), first);
        intr.labels.add(unrelated.clone(), other);
        intr.labels.add(matching.clone(), second);
        assert_eq!(intr.query(&Selector::Label(matching)), vec![first, second]);
        assert_eq!(intr.query(&Selector::Label(unrelated)), vec![other]);
        assert!(intr.query(&Selector::Label(Label("unknown".into()))).is_empty());
    }

    #[test]
    fn circle_origin_label_selector_preserves_causal_order_and_deduplicates_aliases() {
        let mut locator = Locator::new();
        let last = locator.next();
        let first = locator.next();
        let middle = locator.next();
        let label = Label("shared".into());
        let mut intr = TagIntrospector::empty();
        for location in [first, middle, last] {
            intr.counters.observe_location(location);
        }
        // Registry aliases retain one identity. Numeric IDs and registration
        // order are deliberately different from recorded realization order.
        for location in [last, first, last, middle, first] {
            intr.labels.add(label.clone(), location);
        }
        assert_eq!(intr.query(&Selector::Label(label)), vec![first, middle, last]);
    }

    #[test]
    fn circle_origin_label_single_lookup_keeps_first_registered_location() {
        let mut locator = Locator::new();
        let registered_first = locator.next();
        let causal_first = locator.next();
        let label = Label("shared".into());
        let mut intr = TagIntrospector::empty();
        intr.counters.observe_location(causal_first);
        intr.counters.observe_location(registered_first);
        intr.labels.add(label.clone(), registered_first);
        intr.labels.add(label.clone(), causal_first);
        intr.labels.add(label.clone(), registered_first);
        assert_eq!(intr.query_by_label(&label), Some(registered_first));
        assert_eq!(intr.label_count(&label), 3, "raw registry API preserves aliases");
        assert_eq!(intr.query_by_label(&Label("unknown".into())), None);
    }
}
