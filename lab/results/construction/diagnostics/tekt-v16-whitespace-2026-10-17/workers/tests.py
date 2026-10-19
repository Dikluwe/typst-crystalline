from pathlib import Path
import json,gzip,hashlib,subprocess,datetime
out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));assert json.load(open(out/'l0-before-code.json'))['productive_sources_unchanged'];sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();assert all(sha(p)==h for p,h in b['hashes'].items() if p not in b['owners']);additions={}
additions[b['sources'][0]]=r'''

#[cfg(test)]
mod v16_whitespace_tests {
    use super::*;
    use crate::entities::content::{LocatedContent, RealizedContent, SourceContent};
    use crate::entities::{file_id::FileId, location::Location, span::Span, style::Styles};
    use std::sync::Arc;

    fn span(at: usize) -> Span {
        Span::from_range(FileId::from_raw(std::num::NonZeroU16::new(1).unwrap()), at..at + 1)
    }
    fn carriers(body: Content) -> Vec<Content> {
        vec![
            body.clone().with_source_span(span(1)),
            Content::Located(Arc::new(LocatedContent { body: body.clone(), location: Location::from_raw(9) })),
            Content::Realized(Arc::new(RealizedContent { original: Content::text("must-not-observe"), visual: body.clone() })),
            Content::styled(Box::new(body), Styles::default()),
        ]
    }
    fn view(content: &Content) -> String {
        match content.source_body() {
            Content::Located(e) => view(&e.body),
            Content::Realized(e) => view(&e.visual),
            Content::Styled(e) => view(&e.body),
            Content::Empty => "_".into(),
            Content::Space => "S".into(),
            Content::SpaceFromNewline => "N".into(),
            Content::Linebreak(_) => "L".into(),
            Content::Parbreak => "P".into(),
            Content::Text(t) => format!("T:{t}"),
            Content::MathIdent(t) => format!("M:{t}"),
            other => panic!("unsupported observation {other:?}"),
        }
    }
    fn assert_run(input: Vec<Content>, expected: &[&str]) {
        let mut inplace = input.clone();
        normalize_markup_spaces(&mut inplace);
        assert_eq!(inplace.iter().map(view).collect::<Vec<_>>(), expected);
        let lazy = normalize_markup_spaces_if_changed(&input).unwrap_or(input);
        assert_eq!(lazy.iter().map(view).collect::<Vec<_>>(), expected);
        assert!(normalize_markup_spaces_if_changed(&inplace).is_none());
        assert!(normalize_markup_spaces_if_changed(&lazy).is_none());
    }
    fn right_break(middle: Content) {
        assert_run(vec![Content::text("a"), Content::Space, middle, Content::text("b")], &["T:a", "_", "L", "T:b"]);
    }
    #[test]
    fn v16_whitespace_break_right() {
        for middle in carriers(Content::linebreak()) { right_break(middle); }
    }
    #[test]
    fn v16_whitespace_break_left() {
        for middle in carriers(Content::linebreak()) {
            assert_run(vec![Content::text("a"), middle, Content::SpaceFromNewline, Content::text("b")], &["T:a", "L", "_", "T:b"]);
        }
    }
    #[test]
    fn v16_whitespace_empty_run() {
        for middle in carriers(Content::Empty) {
            assert_run(vec![Content::text("a"), Content::Space, middle, Content::SpaceFromNewline, Content::text("b")], &["T:a", "N", "_", "_", "T:b"]);
        }
    }
    #[test]
    fn v16_whitespace_empty_run_reverse() {
        for middle in carriers(Content::Empty) {
            assert_run(vec![Content::text("a"), Content::SpaceFromNewline, middle, Content::Space, Content::text("b")], &["T:a", "N", "_", "_", "T:b"]);
        }
    }
    #[test]
    fn v16_whitespace_explicit_run() {
        for middle in carriers(Content::Empty) {
            assert_run(vec![Content::text("a"), Content::Space, middle, Content::Space, Content::text("b")], &["T:a", "S", "_", "_", "T:b"]);
        }
    }
    #[test]
    fn v16_whitespace_nested_linebreak() {
        for inner in carriers(Content::linebreak().with_source_span(span(1))) {
            right_break(Content::Source(Arc::new(SourceContent { body: inner, span: span(3) })));
        }
    }
    #[test]
    fn v16_whitespace_paragraph_boundary() {
        for middle in carriers(Content::Parbreak) {
            assert_run(vec![Content::text("a"), Content::Space, middle, Content::SpaceFromNewline, Content::text("b")], &["T:a", "S", "P", "N", "T:b"]);
        }
    }
    #[test]
    fn v16_whitespace_material_text() {
        for middle in carriers(Content::text("x")) {
            assert_run(vec![Content::text("a"), Content::Space, middle, Content::SpaceFromNewline, Content::text("b")], &["T:a", "S", "T:x", "N", "T:b"]);
        }
    }
    #[test]
    fn v16_whitespace_nontext_math_boundary() {
        for middle in carriers(Content::MathIdent("x".into())) {
            assert_run(vec![Content::text("a"), Content::Space, middle, Content::SpaceFromNewline, Content::text("b")], &["T:a", "S", "M:x", "N", "T:b"]);
        }
    }
    #[test]
    fn v16_whitespace_canonical_origin_and_style() {
        let original = Content::styled(Box::new(Content::Space), Styles::default()).with_source_span(span(1));
        let input = vec![Content::text("a"), original, Content::text("b")];
        assert!(normalize_markup_spaces_if_changed(&input).is_none());
        let mut output = input.clone();
        normalize_markup_spaces(&mut output);
        assert_eq!(output[1].source_span(), span(1));
        assert!(matches!(output[1].source_body(), Content::Styled(e) if matches!(e.body, Content::Space)));
        assert_run(input, &["T:a", "S", "T:b"]);
    }
    #[test]
    fn v16_whitespace_realized_uses_visual() {
        let text = Content::Realized(Arc::new(RealizedContent { original: Content::linebreak(), visual: Content::text("x") }));
        assert_run(vec![Content::text("a"), Content::Space, text, Content::SpaceFromNewline, Content::text("b")], &["T:a", "S", "T:x", "N", "T:b"]);
        let line = Content::Realized(Arc::new(RealizedContent { original: Content::text("x"), visual: Content::linebreak() }));
        assert_run(vec![Content::text("a"), Content::Space, line, Content::SpaceFromNewline, Content::text("b")], &["T:a", "_", "L", "_", "T:b"]);
    }
}
'''
additions[b['sources'][1]]=r'''

#[cfg(test)]
mod v16_whitespace_tests {
    use super::*;
    use crate::entities::content::{LocatedContent, RealizedContent};
    use crate::entities::{file_id::FileId, location::Location};
    fn carriers(body: Content) -> Vec<Content> {
        let span = Span::from_range(FileId::from_raw(std::num::NonZeroU16::new(1).unwrap()), 1..2);
        vec![body.clone().with_source_span(span), Content::Located(Arc::new(LocatedContent {body:body.clone(),location:Location::from_raw(9)})), Content::Realized(Arc::new(RealizedContent {original:Content::linebreak(),visual:body}))]
    }
    #[test]
    fn v16_whitespace_matching_preserves_pieces() {
        for (body, expected) in [(Content::text("alpha"), "alpha"), (Content::Space, " "), (Content::SpaceFromNewline, " "), (Content::Empty, ""), (Content::sequence(vec![Content::text("a"), Content::Space, Content::text("b")]), "a b")] {
            for content in carriers(body) { assert_eq!(textual_piece(&content).as_deref(), Some(expected)); }
        }
    }
    #[test]
    fn v16_whitespace_matching_keeps_boundaries() {
        for body in [Content::linebreak(), Content::Parbreak, Content::styled(Box::new(Content::text("x")), Styles::default()), Content::MathIdent("x".into()), Content::sequence(vec![Content::text("a"), Content::Parbreak, Content::text("b")])] {
            for content in carriers(body) { assert!(textual_piece(&content).is_none()); }
        }
    }
}
'''
additions[b['sources'][2]]=r'''

#[cfg(test)]
mod v16_whitespace_tests {
    use super::*;
    use std::sync::Arc;
    use typst_core::entities::html::{HtmlBody, HtmlElem};
    fn element(tag: &str) -> Content {
        Content::HtmlElem(Arc::new(HtmlElem::new(tag.into(), None, HtmlBody::Content(Box::new(Content::Empty)))))
    }
    fn verify(left: Content, row: usize) {
        let expected = [["", "", ""], ["", PROTECTED_SPACE, PROTECTED_SPACE], ["", PROTECTED_SPACE, " "]];
        for (column, right) in [element("div"), element("span"), Content::text("A")].into_iter().enumerate() {
            let input = [left.clone(), Content::Space, right];
            assert_eq!(space_between(&input, 1), expected[row][column]);
            assert_eq!(top_level_space_between(&input, 1), expected[row][column]);
        }
    }
    #[test]
    fn v16_whitespace_html_boundary_row() { verify(element("div"), 0); }
    #[test]
    fn v16_whitespace_html_empty_inline_row() { verify(element("span"), 1); }
    #[test]
    fn v16_whitespace_html_supportive_row() { verify(Content::text("A"), 2); }
}
'''
assert sum(v.count('#[test]') for v in additions.values())==16
record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),additions=additions,expected_files={})
for p,s in additions.items():
 assert Path(p).read_text()==b['files'][p];Path(p).write_text(b['files'][p]+s);subprocess.run(['rustfmt','--edition','2021','--config-path',str(Path('rustfmt.toml').resolve()),'--config','skip_children=true',p],check=True);record['expected_files'][p]=Path(p).read_text()
(out/'tests-before-code.json').write_text(json.dumps(record,indent=2)+'\n');print('16 controls written before implementation',flush=True)
