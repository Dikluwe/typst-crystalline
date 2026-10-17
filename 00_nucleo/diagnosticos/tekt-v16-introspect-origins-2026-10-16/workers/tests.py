from pathlib import Path
import json,datetime,hashlib
out=Path('00_nucleo/diagnosticos/tekt-v16-introspect-origins-2026-10-16')
tests={
'01_core/src/compiler/introspect/extract_payload.rs':r'''

#[cfg(test)]
mod v16_origins_tests {
    use super::*;
    use crate::entities::file_id::FileId;
    use crate::entities::span::Span;
    use crate::entities::value::Value;
    use std::num::NonZeroU16;
    use std::sync::Arc;

    fn origin_span() -> Span {
        Span::from_range(FileId::from_raw(NonZeroU16::new(1).unwrap()), 2..5)
    }

    #[test]
    fn v16_origins_source_heading_keeps_body_payload() {
        let body = Content::heading(2, Content::text("section"));
        let expected = extract_payload(&body);
        assert!(expected.is_some());
        let sourced = body.with_source_span(origin_span());
        assert_eq!(extract_payload(&sourced), expected);
    }

    #[test]
    fn v16_origins_source_emphasis_and_metadata_keep_own_payloads() {
        for body in [Content::strong(Content::Empty), Content::emph(Content::Empty), Content::metadata(Value::Int(42))] {
            let expected = extract_payload(&body);
            assert!(expected.is_some());
            assert_eq!(extract_payload(&body.with_source_span(origin_span())), expected);
        }
    }

    #[test]
    fn v16_origins_source_non_producers_do_not_gain_payloads() {
        for body in [Content::Empty, Content::Space, Content::SpaceFromNewline, Content::text("text"), Content::Sequence(Arc::from(vec![Content::text("child")]))] {
            assert!(extract_payload(&body.with_source_span(origin_span())).is_none());
        }
    }

    #[test]
    fn v16_origins_outline_entry_alignment_remains_explicitly_pending() {
        let entry = Content::outline_entry(1, Content::heading(1, Content::Empty), None);
        assert!(crate::compiler::introspect::locatable::is_locatable(&entry));
        assert!(extract_payload(&entry).is_none());
        assert!(extract_payload(&entry.with_source_span(origin_span())).is_none());
    }
}
''',
'01_core/src/compiler/layout/realized.rs':r'''

#[cfg(test)]
mod v16_origins_tests {
    use super::*;
    use crate::entities::content::{LocatedContent, RealizedContent};
    use crate::entities::file_id::FileId;
    use crate::entities::locator::Locator;
    use crate::entities::span::Span;
    use std::num::NonZeroU16;
    use std::sync::Arc;

    fn origin_span() -> Span {
        Span::from_range(FileId::from_raw(NonZeroU16::new(1).unwrap()), 2..5)
    }

    #[test]
    fn v16_origins_located_source_heading_is_a_prepared_occurrence() {
        let location = Locator::new().next();
        let original = Content::Located(Arc::new(LocatedContent { location, body: Content::heading(1, Content::text("h")).with_source_span(origin_span()) }));
        let mut locations = Vec::new();
        assert!(prepared_locations(&original, &mut locations));
        assert_eq!(locations, vec![location]);
    }

    #[test]
    fn v16_origins_source_around_located_does_not_duplicate_location() {
        let location = Locator::new().next();
        let original = Content::Located(Arc::new(LocatedContent { location, body: Content::heading(1, Content::Empty) })).with_source_span(origin_span());
        let mut locations = Vec::new();
        assert!(prepared_locations(&original, &mut locations));
        assert_eq!(locations, vec![location]);
    }

    #[test]
    fn v16_origins_realized_source_original_is_collected_once() {
        let location = Locator::new().next();
        let original = Content::Located(Arc::new(LocatedContent { location, body: Content::heading(1, Content::text("original")).with_source_span(origin_span()) }));
        let realized = Content::Realized(Arc::new(RealizedContent { original, visual: Content::text("visual") }));
        let mut locations = Vec::new();
        assert!(prepared_locations(&realized, &mut locations));
        assert_eq!(locations, vec![location]);
    }

    #[test]
    fn v16_origins_unprepared_or_special_origins_keep_fallback() {
        for body in [Content::text("not a payload"), Content::reference("target").with_source_span(origin_span()), Content::outline().with_source_span(origin_span())] {
            let original = Content::Located(Arc::new(LocatedContent { location: Locator::new().next(), body }));
            let mut locations = Vec::new();
            assert!(!prepared_locations(&original, &mut locations));
            assert!(locations.is_empty());
        }
        let mut locations = Vec::new();
        assert!(!prepared_locations(&Content::heading(1, Content::Empty), &mut locations));
    }

    #[test]
    fn v16_origins_special_source_classes_still_require_fresh_index() {
        for content in [Content::outline(), Content::bibliography(vec![], None), Content::reference("target")] {
            assert!(needs_fresh_index(&content));
            assert!(needs_fresh_index(&content.with_source_span(origin_span())));
        }
        for content in [Content::Empty, Content::text("plain"), Content::heading(1, Content::Empty)] {
            assert!(!needs_fresh_index(&content));
        }
    }

    #[test]
    fn v16_origins_block_equation_certificate_preserves_wrapper_boundary() {
        let equation = Content::equation(Content::Empty, true);
        assert!(block_equation_origin(&equation));
        assert!(!block_equation_origin(&Content::equation(Content::Empty, false)));
        let located = Content::Located(Arc::new(LocatedContent { location: Locator::new().next(), body: equation.clone() })).with_source_span(origin_span());
        let realized = Content::Realized(Arc::new(RealizedContent { original: located, visual: Content::text("visual") }));
        assert!(block_equation_origin(&realized));
        assert!(!block_equation_origin(&Content::strong(equation)));
        assert!(!block_equation_origin(&Content::text("plain")));
    }

    #[test]
    fn v16_origins_textual_certificate_keeps_realized_outside_its_scope() {
        for content in [Content::Empty, Content::Space, Content::SpaceFromNewline, Content::linebreak(), Content::strong(Content::text("bold")), Content::emph(Content::text("italic")), Content::Sequence(Arc::from(vec![Content::text("a"), Content::Space]))] {
            assert!(textual_product(&content.with_source_span(origin_span())));
        }
        let realized = Content::Realized(Arc::new(RealizedContent { original: Content::text("original"), visual: Content::text("visual") }));
        assert!(!textual_product(&realized));
        assert!(!textual_product(&Content::heading(1, Content::Empty)));
        assert!(!textual_product(&Content::equation(Content::Empty, true)));
    }
}
'''}
record={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'tests':11,'expected_red':5,'preservation_controls':6,'production_implementation_written':False,'sources':{}}
for source,addition in tests.items():
 p=Path(source);before=p.read_text();assert 'mod v16_origins_tests' not in before;p.write_text(before+addition)
 record['sources'][source]={'before':before,'addition':addition,'sha256':hashlib.sha256(p.read_bytes()).hexdigest()}
(out/'tests-before-code.json').write_text(json.dumps(record,ensure_ascii=False,indent=2)+'\n')
