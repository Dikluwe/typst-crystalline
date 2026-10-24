//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/introspect/bibliography.md
//! @prompt-hash 43b87e4d
//! @layer L1
//! @updated 2026-10-07

use super::*;

/// **P533** — Converte referências `@key` que correspondam a entradas
/// bibliográficas em `Content::Cite`, de modo que o walk de introspecção
/// as conte corretamente para `citation_order`/`back_refs` e o layout as
/// renderize como citações resolvidas em vez de referências cruzadas.
pub fn convert_bib_refs_to_cites(content: Content) -> Content {
    let keys = collect_bib_keys(&content);
    if keys.is_empty() {
        return content;
    }
    convert_refs(content, &keys)
}

pub(super) fn collect_bib_keys(content: &Content) -> std::collections::HashSet<String> {
    let mut keys = std::collections::HashSet::new();
    content.visit_content_static(&mut |node| {
        if matches!(node, Content::Document { .. } | Content::Asset { .. }) {
            return false;
        }
        if let Content::Bibliography(b) = node {
            keys.extend(b.entries.iter().map(|entry| entry.key.clone()));
        }
        true
    });
    keys
}

pub(super) fn project_reference_root(
    content: &Content,
    keys: &std::collections::HashSet<String>,
) -> Option<Content> {
    match content {
        Content::Ref(reference) => {
            let key = reference.name.to_string();
            keys.contains(&key)
                .then(|| Content::cite(key, reference.supplement.clone(), None))
        }
        Content::Source(..)
        | Content::ShowRecipe(..)
        | Content::Realized(..)
        | Content::Located(..)
        | Content::Empty
        | Content::Text(..)
        | Content::Space
        | Content::SpaceFromNewline
        | Content::Parbreak
        | Content::Sequence(..)
        | Content::HtmlElem(..)
        | Content::HtmlFrame(..)
        | Content::Par { .. }
        | Content::Heading(..)
        | Content::Title(..)
        | Content::Strong(..)
        | Content::Emph(..)
        | Content::Raw(..)
        | Content::ListItem(..)
        | Content::EnumItem(..)
        | Content::Link(..)
        | Content::Equation(..)
        | Content::MathSequence(..)
        | Content::Symbol(..)
        | Content::MathIdent(..)
        | Content::MathText(..)
        | Content::MathFrac(..)
        | Content::MathAttach(..)
        | Content::MathPrimes(..)
        | Content::MathRoot(..)
        | Content::MathStretch(..)
        | Content::MathDelimited(..)
        | Content::MathLr(..)
        | Content::MathMid(..)
        | Content::MathOverbrace(..)
        | Content::MathUnderbrace(..)
        | Content::MathUnderbracket(..)
        | Content::MathUnderparen(..)
        | Content::MathUndershell(..)
        | Content::MathOverbracket(..)
        | Content::MathOverparen(..)
        | Content::MathOvershell(..)
        | Content::MathAlignPoint(..)
        | Content::Linebreak(..)
        | Content::MathMatrix(..)
        | Content::MathCases(..)
        | Content::MathAccent(..)
        | Content::MathCancel(..)
        | Content::MathUnderline(..)
        | Content::MathOverline(..)
        | Content::MathVec(..)
        | Content::MathClassOverride(..)
        | Content::MathLimitsOverride(..)
        | Content::MathUnderover(..)
        | Content::MathOp(..)
        | Content::MathStyled(..)
        | Content::Label(..)
        | Content::CounterDisplay(..)
        | Content::CounterUpdate(..)
        | Content::Outline(..)
        | Content::OutlineEntry(..)
        | Content::Figure(..)
        | Content::FigureCaption(..)
        | Content::Image(..)
        | Content::Shape(..)
        | Content::Curve(..)
        | Content::Transform(..)
        | Content::Grid(..)
        | Content::GridHeader(..)
        | Content::GridFooter(..)
        | Content::GridCell(..)
        | Content::SetPage(..)
        | Content::PageRun(..)
        | Content::Align(..)
        | Content::Place(..)
        | Content::Flush(..)
        | Content::Styled(..)
        | Content::Divider(..)
        | Content::Terms(..)
        | Content::TermItem(..)
        | Content::Quote(..)
        | Content::Document { .. }
        | Content::Asset { .. }
        | Content::SmartQuote(..)
        | Content::PdfAttach(..)
        | Content::PdfArtifact(..)
        | Content::Underline(..)
        | Content::Strike(..)
        | Content::Overline(..)
        | Content::SmallCaps { .. }
        | Content::Pad(..)
        | Content::Hide(..)
        | Content::HSpace(..)
        | Content::VSpace(..)
        | Content::Pagebreak(..)
        | Content::Colbreak(..)
        | Content::Stack(..)
        | Content::Boxed(..)
        | Content::Block(..)
        | Content::TableCell(..)
        | Content::Bibliography(..)
        | Content::Cite(..)
        | Content::Footnote(..)
        | Content::FootnoteEntry(..)
        | Content::TableHeader(..)
        | Content::TableFooter(..)
        | Content::GridHLine(..)
        | Content::GridVLine(..)
        | Content::TableHLine(..)
        | Content::TableVLine(..)
        | Content::Table(..)
        | Content::Repeat(..)
        | Content::Columns(..)
        | Content::Metadata(..)
        | Content::State(..)
        | Content::StateUpdate(..)
        | Content::StateDisplay(..)
        | Content::CounterDisplayCallback(..)
        | Content::ContextBlock(..)
        | Content::Dynamic(..) => None,
    }
}

pub(super) fn convert_refs(
    content: Content,
    keys: &std::collections::HashSet<String>,
) -> Content {
    content
        .map_content_pre(&mut |node| {
            // A bundled document owns its own bibliography; do not cross its boundary.
            if matches!(node, Content::Document { .. } | Content::Asset { .. }) {
                return Ok(Some(node.clone()));
            }
            if let Content::Ref(r) = node {
                let key = r.name.to_string();
                if keys.contains(&key) {
                    return Ok(Some(Content::cite(key, r.supplement.clone(), None)));
                }
            }
            // The private pre-order helper intentionally leaves Dynamic opaque.
            // Preserve the bibliographic converter's established dynamic delegation.
            if let Content::Dynamic(e) = node {
                let mut f = |c: &Content| Ok(Some(convert_refs(c.clone(), keys)));
                return e.dyn_map_content(&mut f).map(Some);
            }
            // Follow the same active view as TableCell::visual and its finalizer.
            // A coexisting standalone carrier may only hold observed properties.
            if let Content::TableCell(e) = node {
                let mut cell = e.as_ref().clone();
                if let Some(r) = &e.resolved {
                    let mut plan = r.as_ref().clone();
                    plan.visual = convert_refs(plan.visual, keys);
                    cell.resolved = Some(std::sync::Arc::new(plan));
                } else if let Some(r) = &e.standalone {
                    let mut plan = r.as_ref().clone();
                    plan.visual = convert_refs(plan.visual, keys);
                    cell.standalone = Some(std::sync::Arc::new(plan));
                } else {
                    cell.body = convert_refs(cell.body, keys);
                }
                return Ok(Some(Content::TableCell(std::sync::Arc::new(cell))));
            }
            // Preserve the realized outline body, which is private to its element.
            if let Content::Outline(e) = node {
                let mut mapped = e.as_ref().clone();
                mapped.title = e.title.as_ref().map(|c| convert_refs(c.clone(), keys));
                if let Some(body) = e.realized_body() {
                    mapped = mapped.with_realized_body(convert_refs(body.clone(), keys));
                }
                return Ok(Some(Content::Outline(std::sync::Arc::new(mapped))));
            }
            Ok(None)
        })
        .expect("convert_bib_refs_to_cites: pure traversal must not fail")
}

#[cfg(test)]
mod carrier_regressions {
    use crate::entities::{
        content::{
            Content, LocatedContent, RealizedContent, ShowRecipeContent, SourceContent,
        },
        file_id::FileId,
        locator::Locator,
        span::Span,
        style_chain::StyleChain,
    };
    use std::sync::Arc;
    fn wrap(kind: usize, body: Content, locator: &mut Locator) -> Content {
        match kind {
            0 => Content::Source(Arc::new(SourceContent {
                body,
                span: Span::from_range(
                    FileId::from_raw(std::num::NonZeroU16::new(7).unwrap()),
                    0..1,
                ),
            })),
            1 => Content::Located(Arc::new(LocatedContent {
                body,
                location: locator.next(),
            })),
            2 => Content::Realized(Arc::new(RealizedContent {
                original: Content::text("causal original"),
                visual: body,
            })),
            3 => Content::ShowRecipe(Arc::new(ShowRecipeContent {
                body,
                textual: false,
                context_id_floor: 0,
                rules: Arc::from([]),
                styles: StyleChain::default_chain(),
                guards: vec![],
                file: FileId::from_raw(std::num::NonZeroU16::new(7).unwrap()),
            })),
            _ => panic!("probe kind"),
        }
    }

    use crate::compiler::introspect::introspect_with_introspector;
    use crate::entities::{bib_entry::BibEntry, introspector::Introspector};
    fn bib() -> Content {
        Content::bibliography(
            vec![
                BibEntry::new("k1", "A1", "T1", 2024),
                BibEntry::new("k2", "A2", "T2", 2023),
            ],
            None,
        )
    }
    fn refs(b: Content) -> Content {
        Content::Sequence(
            vec![Content::reference("k2"), Content::reference("k1"), b].into(),
        )
    }

    #[test]
    fn introspection_carriers_citations_keep_order() {
        let expected = vec!["k2".to_string(), "k1".to_string()];
        assert_eq!(
            introspect_with_introspector(&refs(bib())).citation_order(),
            expected.as_slice()
        );
        let mut differences = Vec::new();
        for kind in 0..4 {
            let mut locator = Locator::new();
            for _ in 0..100 {
                locator.next();
            }
            for position in 0..3 {
                let input = match position {
                    0 => wrap(kind, refs(bib()), &mut locator),
                    1 => refs(wrap(kind, bib(), &mut locator)),
                    2 => Content::Sequence(
                        vec![
                            wrap(kind, Content::reference("k2"), &mut locator),
                            wrap(kind, Content::reference("k1"), &mut locator),
                            bib(),
                        ]
                        .into(),
                    ),
                    _ => unreachable!(),
                };
                let intr = introspect_with_introspector(&input);
                if intr.citation_order() != expected.as_slice() {
                    differences
                        .push(format!("{kind}/{position}: {:?}", intr.citation_order()));
                }
            }
        }
        assert!(differences.is_empty(), "{}", differences.join("\n"));
    }
    #[test]
    fn introspection_carriers_original_and_unknown_are_not_citations() {
        let hidden = Content::Realized(Arc::new(RealizedContent {
            original: refs(bib()),
            visual: Content::Empty,
        }));
        assert!(introspect_with_introspector(&hidden).citation_order().is_empty());
        for kind in 0..4 {
            let mut locator = Locator::new();
            for _ in 0..100 {
                locator.next();
            }
            let unknown = wrap(kind, Content::reference("unknown"), &mut locator);
            let output = super::convert_bib_refs_to_cites(unknown.clone());
            assert_eq!(output, unknown);
            assert!(introspect_with_introspector(&output).citation_order().is_empty());
            let before = wrap(kind, refs(bib()), &mut locator);
            let after = super::convert_bib_refs_to_cites(before.clone());
            match (&before, &after) {
                (Content::Source(a), Content::Source(z)) => assert_eq!(a.span, z.span),
                (Content::Located(a), Content::Located(z)) => {
                    assert_eq!(a.location, z.location)
                }
                (Content::Realized(a), Content::Realized(z)) => {
                    assert_eq!(a.original, z.original)
                }
                (Content::ShowRecipe(a), Content::ShowRecipe(z)) => {
                    assert_eq!(a.textual, z.textual);
                    assert_eq!(a.context_id_floor, z.context_id_floor);
                    assert_eq!(a.file, z.file);
                    assert!(Arc::ptr_eq(&a.rules, &z.rules));
                    assert_eq!(a.guards, z.guards);
                }
                _ => panic!("carrier shape changed"),
            }
        }
    }
}

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
        content
            .map_content(&mut |c| {
                if let Content::Cite(e) = c {
                    keys.push(e.key.clone());
                }
                Ok(None)
            })
            .unwrap();
        keys
    }
    #[test]
    fn bibliography_static_collects_nested_keys() {
        let mut missing = Vec::new();
        for kind in 0..18 {
            let keys = super::collect_bib_keys(&wrap(kind, bib()));
            if !keys.contains("i2") {
                missing.push(kind);
            }
        }
        assert!(missing.is_empty(), "missing key in static fields: {missing:?}");
    }
    #[test]
    fn bibliography_static_converts_nested_known_refs() {
        let mut missing = Vec::new();
        for kind in 0..18 {
            let input = Content::Sequence(
                vec![wrap(kind, Content::reference("i2")), bib()].into(),
            );
            let output = super::convert_bib_refs_to_cites(input);
            let keys = cite_keys(&output);
            if keys.iter().filter(|k| k.as_str() == "i2").count() != 1 {
                missing.push((kind, keys));
            }
        }
        assert!(missing.is_empty(), "missing citation in static fields: {missing:?}");
    }
}

#[cfg(test)]
mod static_boundaries_regressions {
    use super::*;
    use crate::entities::{
        bib_entry::BibEntry,
        content::RealizedContent,
        elements::{
            table_cell::{
                TableCellResolved, TableCellStage, TableCellStandaloneResolved,
            },
            test_callout::CalloutElem,
        },
        value::Value,
    };
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
        let input = Content::Sequence(
            vec![document(Content::reference("host")), bib("host")].into(),
        );
        let Content::Sequence(parts) = super::convert_bib_refs_to_cites(input) else {
            panic!("sequence");
        };
        let Content::Document { body, .. } = &parts[0] else {
            panic!("document");
        };
        assert!(matches!(body.as_ref(), Content::Ref(e) if e.name.as_str() == "host"));
        let dynamic =
            Content::dynamic(CalloutElem::new(Content::reference("host"), "T", "note"));
        let keys = std::collections::HashSet::from(["host".to_string()]);
        let Content::Dynamic(mapped) = super::convert_refs(dynamic, &keys) else {
            panic!("dynamic");
        };
        let Some(Value::Content(body)) = mapped.dyn_get_field("body") else {
            panic!("body");
        };
        assert!(matches!(body, Content::Cite(e) if e.key == "host"));
    }
    #[test]
    fn bibliography_static_prepared_cell_uses_only_selected_visual() {
        let Content::TableCell(raw) =
            Content::table_cell(bib("raw"), None, None, None, None)
        else {
            panic!("cell");
        };
        for mode in 0..3 {
            let mut cell = raw.as_ref().clone();
            cell.resolved = Some(Arc::new(TableCellResolved {
                x: 2,
                y: 3,
                colspan: 2,
                rowspan: 1,
                fill: None,
                stroke: None,
                align: None,
                inset: Default::default(),
                breakable: false,
                visual: Content::Sequence(
                    vec![Content::reference("resolved"), bib("resolved")].into(),
                ),
                stage: TableCellStage::Complete,
                guards: vec![11],
            }));
            if mode != 0 {
                cell.standalone = Some(Arc::new(TableCellStandaloneResolved {
                    properties: Some(raw.properties()),
                    visual: Content::Sequence(
                        vec![Content::reference("standalone"), bib("standalone")].into(),
                    ),
                    stage: TableCellStage::RecipeApplied,
                    guards: vec![13],
                }));
            }
            if mode == 1 {
                cell.resolved = None;
            }
            let input = Content::TableCell(Arc::new(cell));
            let expected = if mode == 1 { "standalone" } else { "resolved" };
            assert_eq!(
                super::collect_bib_keys(&input),
                std::collections::HashSet::from([expected.to_string()])
            );
            let Content::TableCell(mapped) =
                super::convert_bib_refs_to_cites(input.clone())
            else {
                panic!("cell");
            };
            let Content::Sequence(parts) = mapped.visual() else {
                panic!("visual");
            };
            assert!(matches!(&parts[0], Content::Cite(e) if e.key == expected));
            let Content::TableCell(before) = input else {
                panic!("cell");
            };
            assert_eq!(mapped.body, before.body);
            if let Some(r) = &mapped.resolved {
                assert_eq!((r.x, r.y, r.colspan, r.rowspan), (2, 3, 2, 1));
                assert_eq!(r.guards, vec![11]);
            }
            if mode != 0 {
                assert_eq!(mapped.standalone.as_ref().unwrap().guards, vec![13]);
            }
            if mode == 2 {
                assert!(Arc::ptr_eq(
                    mapped.standalone.as_ref().unwrap(),
                    before.standalone.as_ref().unwrap()
                ));
            }
        }
        let hidden = Content::Realized(Arc::new(RealizedContent {
            original: bib("hidden"),
            visual: Content::Empty,
        }));
        assert!(super::collect_bib_keys(&hidden).is_empty());
    }
}
