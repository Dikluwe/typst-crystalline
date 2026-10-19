from pathlib import Path
import json,hashlib,datetime
out=Path('00_nucleo/diagnosticos/tekt-v16-introspect-walk-2026-10-16');scope=json.load(open(out/'scope.json'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
entity=Path('01_core/src/entities/elements/outline_entry.rs');s=entity.read_text();at=s.rfind('\n}')
s=s[:at]+'''
    fn walk_source(body: Content) -> Content {
        use crate::entities::{file_id::FileId, span::Span};
        body.with_source_span(Span::from_range(
            FileId::from_raw(std::num::NonZeroU16::new(1).unwrap()), 2..5,
        ))
    }

    #[test]
    fn v16_walk_entry_body_preserves_source_heading_morphology() {
        let body = Content::strong(Content::text("semantic"));
        let element = walk_source(Content::heading(2, body.clone()));
        let entry = OutlineEntryElem::new(2, element.clone(), None, false);
        assert_eq!(entry.body().unwrap(), body);
        assert_eq!(entry.element, element);
        assert_eq!(entry.prefix(), None);
        assert_eq!(entry.element_location(), None);
    }

    #[test]
    fn v16_walk_entry_body_rejects_source_strong_without_unwrapping_it() {
        let entry = OutlineEntryElem::new(
            1, walk_source(Content::strong(Content::text("invalid"))), None, false,
        );
        assert_eq!(entry.body().unwrap_err(), "cannot outline strong");
    }
'''+s[at:];entity.write_text(s)
p=Path('01_core/src/compiler/introspect/extract_payload.rs');s=p.read_text();s=s.replace('fn v16_origins_outline_entry_alignment_remains_explicitly_pending()', 'fn v16_walk_entry_payload_alignment_matches_native_delegate()');old='''        assert!(extract_payload(&entry).is_none());
        assert!(extract_payload(&entry.with_source_span(origin_span())).is_none());''';new='''        assert_eq!(extract_payload(&entry), Some(ElementPayload::NativeElement));
        assert_eq!(extract_payload(&entry.with_source_span(origin_span())), Some(ElementPayload::NativeElement));''';assert s.count(old)==1;p.write_text(s.replace(old,new))
p=Path('01_core/src/compiler/introspect/walk.rs');p.write_text(p.read_text()+'''
#[cfg(test)]
mod v16_walk_tests {
    use super::*;
    use crate::entities::{content::LocatedContent, file_id::FileId, func::Func, span::Span};

    fn entry_selector() -> Selector {
        Selector::NativeElement(Func::native("outline.entry", crate::compiler::stdlib::native_outline_entry))
    }

    fn source_entry() -> Content {
        Content::outline_entry(1, Content::heading(1, Content::text("reference")), None)
            .with_source_span(Span::from_range(
                FileId::from_raw(std::num::NonZeroU16::new(1).unwrap()), 2..5,
            ))
    }

    #[test]
    fn v16_walk_entry_emits_own_labeled_tags_without_indexing_element_field() {
        let mut locator = Locator::new();
        let mut tags = Vec::new();
        let mut intr = TagIntrospector::empty();
        let mut auto = 0;
        let label = Label("entry".into());
        walk(&source_entry(), &mut locator, &mut tags, &mut intr, &mut auto,
            None, &StyleChain::default_chain(), Some(&label));
        assert_eq!(tags.len(), 2);
        let Tag::Start(location, info) = &tags[0] else { panic!("missing Start") };
        assert_eq!(info.payload, ElementPayload::NativeElement);
        assert_eq!(info.label.as_ref(), Some(&label));
        assert!(matches!(&tags[1], Tag::End(end, _) if end == location));
        assert_eq!(intr.query(&entry_selector()), vec![*location]);
        assert_eq!(intr.query(&Selector::Label(label)), vec![*location]);
        assert!(intr.kind_index.get(&ElementKind::Heading).is_none());
        assert_eq!(auto, 0);
    }

    #[test]
    fn v16_walk_entry_direct_and_synthesized_are_distinct_without_heading_ghosts() {
        let tree = Content::sequence(vec![
            Content::outline(), Content::heading(1, Content::text("real")), source_entry(),
        ]);
        let intr = crate::compiler::introspect::introspect_with_introspector(&tree);
        let entries = intr.query(&entry_selector());
        assert_eq!(entries.len(), 2);
        assert_ne!(entries[0], entries[1]);
        let heading = Selector::NativeElement(Func::native("heading", crate::compiler::stdlib::native_heading));
        assert_eq!(intr.query(&heading).len(), 1);
        let bodies: Vec<_> = entries.iter().map(|location| {
            let Content::OutlineEntry(entry) = intr.elements[location].content().source_body() else { panic!("entry lost") };
            entry.body().unwrap().plain_text()
        }).collect();
        assert!(bodies.contains(&"reference".to_string()));
        assert!(bodies.contains(&"real".to_string()));
    }

    #[test]
    fn v16_walk_entry_located_alias_is_indexed_once() {
        let location = Locator::new().next();
        let entry = Content::Located(Arc::new(LocatedContent { location, body: source_entry() }));
        let intr = crate::compiler::introspect::introspect_with_introspector(
            &Content::sequence(vec![entry.clone(), entry]),
        );
        assert_eq!(intr.query(&entry_selector()), vec![location]);
        assert_eq!(intr.elements.len(), 1);
    }
}
''')
p=Path('01_core/src/compiler/layout/realized.rs');p.write_text(p.read_text()+'''
#[cfg(test)]
mod v16_walk_prepared_tests {
    use super::*;
    use crate::entities::{content::{LocatedContent, RealizedContent}, locator::Locator};
    use std::sync::Arc;

    fn referenced_entry() -> (Content, Location) {
        let location = Locator::new().next();
        let heading = Content::Located(Arc::new(LocatedContent {
            location, body: Content::heading(1, Content::text("reference")),
        }));
        (Content::outline_entry(1, heading, None), location)
    }

    #[test]
    fn v16_walk_entry_reference_does_not_certify_prepared_origin() {
        let (entry, _) = referenced_entry();
        let mut allocator = Locator::new();
        let _ = allocator.next();
        let located = Content::Located(Arc::new(LocatedContent {
            location: allocator.next(), body: entry.clone(),
        }));
        let realized = Content::Realized(Arc::new(RealizedContent {
            original: located.clone(), visual: Content::text("visual"),
        }));
        for content in [entry, located, realized] {
            assert!(!prepared_locations(&content, &mut Vec::new()));
        }
    }

    #[test]
    fn v16_walk_entry_fallback_indexes_entry_without_its_referenced_heading() {
        let (entry, reference) = referenced_entry();
        let intr = crate::compiler::introspect::introspect_with_introspector(&entry);
        assert_eq!(intr.elements.len(), 1);
        assert!(!intr.elements.contains_key(&reference));
        assert!(intr.elements.values().all(|item| matches!(item.content().source_body(), Content::OutlineEntry(_))));
    }
}
''')
(out/'tests-before-code.json').write_text(json.dumps({'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'new_tests':7,'migrated_pending_test':1,'sources':{s:sha(s) for s in scope['sources']}},indent=2)+'\n')
print('Seven new tests and one explicit pending-test migration written before production changes')
