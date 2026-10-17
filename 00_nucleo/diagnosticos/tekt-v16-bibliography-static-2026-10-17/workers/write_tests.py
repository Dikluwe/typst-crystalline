from pathlib import Path
out=Path('00_nucleo/diagnosticos/tekt-v16-bibliography-static-2026-10-17');src=Path('01_core/src/compiler/introspect/bibliography.rs')
s=r'''

#[cfg(test)]
mod static_coverage_regressions {
    use super::*;
    use crate::entities::{bib_entry::BibEntry, layout_types::TransformMatrix};
    fn bib() -> Content {
        Content::bibliography(vec![BibEntry::new("i2", "Author", "Title", 2026)], None)
    }
    fn wrap(kind: usize, body: Content) -> Content {
        match kind {
            0 => Content::Par { body: Box::new(body) },
            1 => Content::heading(1, body),
            2 => Content::repeat(body, None, false),
            3 => Content::columns(body, 2, None),
            4 => Content::table_cell(body, None, None, None, None),
            5 => Content::table_header(body, true),
            6 => Content::table_footer(body, true),
            7 => Content::grid_header(body, true),
            8 => Content::grid_footer(body, true),
            9 => Content::label("i2-label", body),
            10 => Content::equation(body, false),
            11 => Content::terms(vec![body]),
            12 => Content::quote(Content::Empty, Some(body), false, true),
            13 => Content::figure_caption(Content::Empty, None, Some(body)),
            14 => Content::footnote_entry(Content::Empty, Some(body), None, None, None),
            15 => Content::bibliography(vec![], Some(body)),
            16 => Content::cite("outer", Some(body), None),
            17 => Content::transform(TransformMatrix::identity(), body),
            _ => unreachable!("fixture kind"),
        }
    }
    fn cite_keys(content: &Content) -> Vec<String> {
        let mut keys = Vec::new();
        content.map_content(&mut |c| {
            if let Content::Cite(e) = c { keys.push(e.key.clone()); }
            Ok(None)
        }).unwrap();
        keys
    }
    #[test]
    fn bibliography_static_collects_nested_keys() {
        let mut missing = Vec::new();
        for kind in 0..18 {
            let keys = super::collect_bib_keys(&wrap(kind, bib()));
            if !keys.contains("i2") { missing.push(kind); }
        }
        assert!(missing.is_empty(), "missing key in static fields: {missing:?}");
    }
    #[test]
    fn bibliography_static_converts_nested_known_refs() {
        let mut missing = Vec::new();
        for kind in 0..18 {
            let input = Content::Sequence(vec![wrap(kind, Content::reference("i2")), bib()].into());
            let output = super::convert_bib_refs_to_cites(input);
            let keys = cite_keys(&output);
            if keys.iter().filter(|k| k.as_str() == "i2").count() != 1 { missing.push((kind,keys)); }
        }
        assert!(missing.is_empty(), "missing citation in static fields: {missing:?}");
    }
}
'''
src.write_text(src.read_text()+s);(out/'workers/write_tests.py').write_text(Path('/tmp/bib-i2-tests.py').read_text());print('Two functional tests written before production code')
