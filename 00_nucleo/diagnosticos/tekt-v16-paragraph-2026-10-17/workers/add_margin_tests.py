from pathlib import Path
import json,hashlib,datetime
out=Path(__file__).resolve().parents[1];p=Path('01_core/src/compiler/layout/paragraph.rs');s=p.read_text();assert 'mod v16_paragraph_tests' in s and 'v16_paragraph_empty_sizes_collapse_both_sides' not in s
s=s.replace('    fn audit_content(src: &str) -> Content {','    pub(super) fn audit_content(src: &str) -> Content {');assert s.endswith('}\n');s=s[:-2]+'''
    fn language(source: &str) -> PagedDocument {
        render(vec![super::tests::audit_content(source)])
    }

    #[test]
    fn v16_paragraph_empty_sizes_collapse_both_sides() {
        for (size, delta) in [(8, 37.2), (18, 51.6)] {
            let doc = language(&format!("A#text(size:{size}pt)[#par[]]C"));
            let p = positions(&doc);
            assert_eq!(p.iter().map(|p| p.0.as_str()).collect::<Vec<_>>(), ["A", "C"]);
            assert!((p[1].2 - p[0].2 - delta).abs() < 1e-8, "size {size}: {p:?}");
        }
    }

    #[test]
    fn v16_paragraph_empty_initial_sizes_collapse_with_following_text() {
        for (size, baseline) in [(8, 22.8), (18, 30.0)] {
            let doc = language(&format!("#text(size:{size}pt)[#par[]]C"));
            assert!((positions(&doc)[0].2 - baseline).abs() < 1e-8);
        }
    }

    #[test]
    fn v16_paragraph_two_empty_nodes_keep_internal_gap_without_ink() {
        for text in ["#text(size:8pt)[#par[]]#text(size:18pt)[#par[]]", "#text(size:18pt)[#par[]]#text(size:8pt)[#par[]]"] {
            let doc = language(text);
            assert!(positions(&doc).is_empty());
            assert!((doc.pages[0].height - 21.6).abs() < 1e-8);
        }
    }

    #[test]
    fn v16_paragraph_two_empty_text_nodes_keep_virtual_tops() {
        let doc = language("#text(size:8pt)[#par[#text(\"\")]]#text(size:18pt)[#par[#text(\"\")]]");
        assert!(positions(&doc).is_empty());
        assert!((doc.pages[0].height - (5.6 + 21.6 + 12.6)).abs() < 1e-8);
    }

    #[test]
    fn v16_paragraph_spacing_constructor_controls() {
        for (text, delta) in [("A#par(spacing:30pt)[]C", 68.4), ("A#text(size:18pt)[#par(spacing:2em)[]]C", 80.4)] {
            let doc = language(text);
            let p = positions(&doc);
            assert!((p[1].2 - p[0].2 - delta).abs() < 1e-8, "{p:?}");
        }
    }

    #[test]
    fn v16_paragraph_empty_terminals_reserve_only_above_gap() {
        for (text, height) in [("A#text(size:8pt)[#par[]]", 22.8), ("A#text(size:18pt)[#par[]]", 30.0), ("A#par(spacing:30pt)[]", 38.4)] {
            let doc = language(text);
            assert!((doc.pages[0].height - height).abs() < 1e-8);
        }
    }

    #[test]
    fn v16_paragraph_empty_between_box_and_text_preserves_geometry() {
        let source = language("#box(width:8pt,height:7pt)[]#par[]C");
        assert!(positions(&source)[0].2 > 28.8);
        assert_eq!(positions(&source)[0].0, "C");
    }

    #[test]
    fn v16_paragraph_empty_gap_zero_negative_length() {
        let expected = language("A#par[]C");
        for text in ["A#par(spacing:0pt)[]C", "A#par(spacing:-5pt)[]C"] {
            same_geometry(&language(text), &expected);
        }
    }
}
''';p.write_text(s)
p=Path('01_core/src/compiler/stdlib/structural/par.rs');s=p.read_text();s+='''
#[cfg(test)]
mod v16_paragraph_spacing_tests {
    use super::*;
    use crate::contracts::world::World;
    use crate::entities::{
        args::ArgOccurrence,
        font_book::FontBook,
        source::Source,
        world_types::{Bytes, Datetime, FileError, FileResult, Font, Library},
    };
    use std::num::NonZeroU16;

    #[derive(Default)]
    struct TestWorld { library: Library, book: FontBook }
    impl World for TestWorld {
        fn library(&self) -> &Library { &self.library }
        fn book(&self) -> &FontBook { &self.book }
        fn main(&self) -> FileId { FileId::from_raw(NonZeroU16::new(1).unwrap()) }
        fn source(&self, _: FileId) -> FileResult<Source> { Err(FileError::NotFound) }
        fn file(&self, _: FileId) -> FileResult<Bytes> { Err(FileError::NotFound) }
        fn font(&self, _: usize) -> Option<Font> { None }
        fn today(&self, _: Option<crate::entities::duration::Duration>) -> Option<Datetime> { None }
    }
    fn invoke(args: &Args) -> SourceResult<Value> {
        let world = TestWorld::default();
        native_par(&mut EvalContext::new(), args, &world, world.main())
    }

    #[test]
    fn v16_paragraph_spacing_rejects_invalid_types() {
        for value in [Value::None, Value::Auto, Value::Bool(true), Value::Int(1), Value::Float(1.5), Value::Str("x".into()), Value::Ratio(crate::entities::layout_types::Ratio(0.5))] {
            let mut args = Args::positional(vec![Value::Content(Content::Empty)]);
            args.named.insert("spacing".into(), value);
            assert!(invoke(&args).unwrap_err()[0].message.starts_with("expected length, found"));
        }
    }

    #[test]
    fn v16_paragraph_spacing_integer_hint_and_last_origin() {
        let file = TestWorld::default().main();
        let whole = Span::from_range(file, 0..30);
        let first = Span::from_range(file, 5..9);
        let last = Span::from_range(file, 25..26);
        let args = Args::from_occurrences(whole, vec![
            ArgOccurrence { name: None, value: Value::Content(Content::Empty), span: whole, value_span: whole },
            ArgOccurrence { name: Some("spacing".into()), value: Value::Length(Length::pt(30.0)), span: first, value_span: first },
            ArgOccurrence { name: Some("spacing".into()), value: Value::Int(1), span: last, value_span: last },
        ]);
        let diagnostics = invoke(&args).unwrap_err();
        assert_eq!(diagnostics[0].message, "expected length, found integer");
        assert_eq!(diagnostics[0].span, last);
        assert_eq!(diagnostics[0].hints, ["a length needs a unit - did you mean 1pt?"]);
    }

    #[test]
    fn v16_paragraph_spacing_synthetic_arguments_use_aggregate_span() {
        let mut args = Args::positional(vec![Value::Content(Content::Empty)]);
        args.span = Span::from_range(TestWorld::default().main(), 10..20);
        args.named.insert("spacing".into(), Value::Auto);
        assert_eq!(invoke(&args).unwrap_err()[0].span, args.span);
    }

    #[test]
    fn v16_paragraph_spacing_body_error_precedes_option_error() {
        let mut args = Args::positional(vec![Value::Bool(true)]);
        args.named.insert("spacing".into(), Value::None);
        assert!(invoke(&args).unwrap_err()[0].message.starts_with("expected content"));
    }
}
''';p.write_text(s)
files={str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in [Path('01_core/src/compiler/layout/paragraph.rs'),Path('01_core/src/compiler/stdlib/structural/par.rs')]};(out/'margin-tests-before-code.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),files=files),indent=2)+'\n')
