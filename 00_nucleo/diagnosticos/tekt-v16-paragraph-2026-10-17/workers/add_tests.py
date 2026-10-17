from pathlib import Path
import gzip,json,hashlib,datetime
out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));p=Path(b['source']);assert p.read_text()==b['files'][str(p)]
p.write_text(p.read_text()+'''
#[cfg(test)]
mod v16_paragraph_tests {
    use super::*;
    use super::tests::{make, positions};
    use crate::entities::{
        content::SourceContent,
        introspector::TagIntrospector,
        layout_types::{PagedDocument, Pt},
        span::Span,
        style::{Style, Styles},
    };
    use std::sync::Arc;

    fn source(body: Content) -> Content {
        Content::Source(Arc::new(SourceContent { body, span: Span::detached() }))
    }

    fn render(parts: Vec<Content>) -> PagedDocument {
        let intr = TagIntrospector::empty();
        let mut l = make(&intr, false);
        l.page_config.height = f64::INFINITY;
        l.regions.current.height = f64::INFINITY;
        for part in parts { l.layout_content(&part); }
        l.finish()
    }

    fn same_geometry(actual: &PagedDocument, expected: &PagedDocument) {
        assert_eq!(actual.pages.len(), expected.pages.len());
        for (a, e) in actual.pages.iter().zip(&expected.pages) {
            assert!((a.height - e.height).abs() < 1e-8, "height {} versus {}", a.height, e.height);
        }
        let a = positions(actual);
        let e = positions(expected);
        assert_eq!(a.len(), e.len());
        for (a, e) in a.iter().zip(e.iter()) {
            assert_eq!(a.0, e.0);
            assert!((a.1 - e.1).abs() < 1e-8);
            assert!((a.2 - e.2).abs() < 1e-8, "baseline {} versus {}", a.2, e.2);
        }
    }

    #[test]
    fn v16_paragraph_source_empty_before_text_keeps_spacing() {
        let doc = render(vec![Content::par(source(Content::Empty)), Content::text("A")]);
        assert!((positions(&doc)[0].2 - 22.8).abs() < 1e-8);
    }

    #[test]
    fn v16_paragraph_source_empty_after_text_keeps_auto_height() {
        let doc = render(vec![Content::text("A"), Content::par(source(Content::Empty))]);
        assert!((doc.pages[0].height - 22.8).abs() < 1e-8);
    }

    #[test]
    fn v16_paragraph_source_space_sequence_keeps_spacing() {
        let body = Content::Sequence(Arc::from([source(Content::Space), source(Content::SpaceFromNewline)]));
        let doc = render(vec![Content::par(body), Content::text("A")]);
        assert!((positions(&doc)[0].2 - 22.8).abs() < 1e-8);
    }

    #[test]
    fn v16_paragraph_nested_source_empty_text_keeps_virtual_top_once() {
        let body = source(source(Content::styled(source(Content::text("")), Styles::new())));
        let actual = render(vec![Content::par(body), Content::text("A")]);
        assert!((positions(&actual)[0].2 - 31.2).abs() < 1e-8);
    }

    #[test]
    fn v16_paragraph_source_empty_only_collapses_weak_spacing() {
        let doc = render(vec![Content::par(source(Content::Empty))]);
        assert!((doc.pages[0].height).abs() < 1e-8);
        assert!(positions(&doc).is_empty());
    }

    #[test]
    fn v16_paragraph_source_empty_text_only_keeps_top_edge() {
        let doc = render(vec![Content::par(source(Content::text("")))]);
        assert!((doc.pages[0].height - 8.4).abs() < 1e-8);
        assert!(positions(&doc).is_empty());
    }

    #[test]
    fn v16_paragraph_source_styled_boundary_closes_before_size() {
        let par = Content::par(Content::text("B"));
        let styles = Styles::from_iter([Style::Size(Pt(18.0))]);
        let expected = render(vec![Content::text("A"), Content::styled(par.clone(), styles.clone()), Content::text("C")]);
        let actual = render(vec![Content::text("A"), Content::styled(source(par), styles), Content::text("C")]);
        same_geometry(&actual, &expected);
    }

    #[test]
    fn v16_paragraph_source_styled_empty_boundary_restores_scope() {
        let par = Content::par(Content::Empty);
        let styles = Styles::from_iter([Style::Size(Pt(18.0))]);
        let expected = render(vec![Content::text("A"), Content::styled(par.clone(), styles.clone()), Content::text("C")]);
        let actual = render(vec![Content::text("A"), Content::styled(source(par), styles), Content::text("C")]);
        same_geometry(&actual, &expected);
    }

    #[test]
    fn v16_paragraph_source_material_and_inline_styles_remain_material() {
        let body = Content::styled(Content::text("B"), Styles::from_iter([Style::Size(Pt(18.0))]));
        let expected = render(vec![Content::text("A"), Content::par(body.clone()), Content::text("C")]);
        let actual = render(vec![Content::text("A"), Content::par(source(body)), Content::text("C")]);
        same_geometry(&actual, &expected);
    }
}
''')
(out/'tests-before-code.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),source=str(p),sha256=hashlib.sha256(p.read_bytes()).hexdigest(),implementation_unchanged=p.read_text().startswith(b['files'][str(p)])),indent=2)+'\n')
