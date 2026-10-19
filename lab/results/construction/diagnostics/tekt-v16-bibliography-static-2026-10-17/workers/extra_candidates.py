from pathlib import Path
out=Path('00_nucleo/diagnosticos/tekt-v16-bibliography-static-2026-10-17')
s=r'''

#[cfg(test)]
mod static_boundaries_regressions {
    use super::*;
    use crate::entities::{bib_entry::BibEntry, content::RealizedContent, elements::{test_callout::CalloutElem, table_cell::{TableCellResolved, TableCellStage, TableCellStandaloneResolved}}, value::Value};
    use std::sync::Arc;
    fn bib(key: &str) -> Content {
        Content::bibliography(vec![BibEntry::new(key, "Author", "Title", 2026)], None)
    }
    fn document(body: Content) -> Content {
        Content::document("other.typ".into(), body, None, None, vec![], None, vec![])
    }
    #[test]
    fn bibliography_static_resource_and_dynamic_boundaries() {
        let separate = document(bib("other"));
        assert!(!super::collect_bib_keys(&separate).contains("other"));
        let dynamic = Content::dynamic(CalloutElem::new(bib("dynamic"), "T", "note"));
        assert!(!super::collect_bib_keys(&dynamic).contains("dynamic"));
        let input = Content::Sequence(vec![document(Content::reference("host")), bib("host")].into());
        let Content::Sequence(parts) = super::convert_bib_refs_to_cites(input) else { panic!("sequence"); };
        let Content::Document { body, .. } = &parts[0] else { panic!("document"); };
        assert!(matches!(body.as_ref(), Content::Ref(e) if e.name.as_str() == "host"));
        let dynamic = Content::dynamic(CalloutElem::new(Content::reference("host"), "T", "note"));
        let keys = std::collections::HashSet::from(["host".to_string()]);
        let Content::Dynamic(mapped) = super::convert_refs(dynamic, &keys) else { panic!("dynamic"); };
        let Some(Value::Content(body)) = mapped.dyn_get_field("body") else { panic!("body"); };
        assert!(matches!(body, Content::Cite(e) if e.key == "host"));
    }
    #[test]
    fn bibliography_static_prepared_cell_uses_only_selected_visual() {
        let Content::TableCell(raw) = Content::table_cell(bib("raw"), None, None, None, None) else { panic!("cell"); };
        for standalone in [false, true] {
            let mut cell = raw.as_ref().clone();
            cell.resolved = Some(Arc::new(TableCellResolved {
                x: 2, y: 3, colspan: 2, rowspan: 1, fill: None, stroke: None,
                align: None, inset: Default::default(), breakable: false,
                visual: Content::Sequence(vec![Content::reference("resolved"), bib("resolved")].into()),
                stage: TableCellStage::Complete, guards: vec![11],
            }));
            if standalone { cell.standalone = Some(Arc::new(TableCellStandaloneResolved {
                properties: Some(raw.properties()),
                visual: Content::Sequence(vec![Content::reference("standalone"), bib("standalone")].into()),
                stage: TableCellStage::RecipeApplied, guards: vec![13],
            })); }
            let input = Content::TableCell(Arc::new(cell));
            let expected = if standalone { "standalone" } else { "resolved" };
            assert_eq!(super::collect_bib_keys(&input), std::collections::HashSet::from([expected.to_string()]));
            let Content::TableCell(mapped) = super::convert_bib_refs_to_cites(input.clone()) else { panic!("cell"); };
            let Content::Sequence(parts) = mapped.visual() else { panic!("visual"); };
            assert!(matches!(&parts[0], Content::Cite(e) if e.key == expected));
            let Content::TableCell(before) = input else { panic!("cell"); };
            assert_eq!(mapped.body, before.body);
            let r = mapped.resolved.as_ref().unwrap();
            assert_eq!((r.x,r.y,r.colspan,r.rowspan), (2,3,2,1));
            assert_eq!(r.guards, vec![11]);
            if standalone { assert_eq!(mapped.standalone.as_ref().unwrap().guards, vec![13]); }
        }
        let hidden = Content::Realized(Arc::new(RealizedContent { original: bib("hidden"), visual: Content::Empty }));
        assert!(super::collect_bib_keys(&hidden).is_empty());
    }
}
'''
(out/'bibliography-extra-tests.rs').write_text(s)
(out/'traversal-extra-test.rs').write_text(r'''
    #[test]
    fn bibliography_static_visit_agrees_with_existing_container_fields() {
        for input in cases() {
            let mut expected = Vec::new();
            input.map_content(&mut |c| {
                if let Content::Text(t) = c { expected.push(t.to_string()); }
                Ok(None)
            }).unwrap();
            let mut actual = Vec::new();
            input.visit_content_static(&mut |c| {
                if let Content::Text(t) = c { actual.push(t.to_string()); }
                true
            });
            assert_eq!(actual, expected, "{}", input.elem_name());
        }
        let input = Content::Sequence(vec![
            Content::text("first"),
            Content::Par { body: Box::new(Content::text("pruned")) },
            Content::Realized(Arc::new(RealizedContent {
                original: Content::text("hidden"), visual: Content::text("visible"),
            })),
            Content::text("last"),
        ].into());
        let mut seen = Vec::new();
        input.visit_content_static(&mut |c| {
            if let Content::Text(t) = c { seen.push(t.to_string()); }
            !matches!(c, Content::Par { .. })
        });
        assert_eq!(seen, ["first", "visible", "last"]);
    }
''')
(out/'workers/extra_candidates.py').write_text(Path('/tmp/bib-i2-extras.py').read_text());print('Additional tests prepared; production files unchanged during RED')
