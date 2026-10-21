// Draft ONLY: append inside existing batch_g_html_bundle_tests after L0/RED authorization.
// Uses current public Content constructors/export; no private resolved fields.
#[test]
fn batch_g_html_raw_footer_projection_has_no_unemitted_warning() {
    use typst_core::entities::content::{RealizedContent, SourceContent};
    let deep = || Content::heading(6, Content::text("DORMANT-FOOT"));
    let nested = Content::table(vec![], vec![], vec![
        Content::table_cell(deep(), None, None, None, None),
    ]);
    let transported = Content::Source(Arc::new(SourceContent {
        span: Span::detached(),
        body: Content::styled(Content::Realized(Arc::new(RealizedContent {
            original: Content::text("UNEMITTED-ORIGINAL"), visual: deep(),
        })), typst_core::entities::style::Styles::new()),
    }));
    for (name, body) in [("direct", deep()), ("source-styled-realized", transported), ("nested-table", nested)] {
        let Content::Table(table) = Content::table(
            vec![], vec![], vec![Content::table_cell(Content::text("BODY"), None, None, None, None)],
        ) else { panic!("table constructor"); };
        let mut table = table.as_ref().clone();
        table.footer = Some(Content::table_footer(body, false));
        let mut warnings = vec![];
        let projected = bundle_html_semantic_content(&Content::Table(Arc::new(table)), &mut warnings);
        let Content::Table(table) = &projected else { panic!("table identity retained"); };
        assert!(table.resolved.is_none(), "exercise actual raw renderer branch");
        let Some(Content::TableFooter(footer)) = &table.footer else { panic!("footer field retained"); };
        assert!(!footer.repeat);
        if name == "direct" {
            assert!(matches!(&footer.body, Content::HtmlElem(e) if e.tag=="div" && e.attrs.as_ref().unwrap().get("aria-level").unwrap()=="7"));
        }
        let field_html = crate::export::html::export_html(&footer.body).expect("retained field remains projected");
        assert!(field_html.contains("aria-level=\"7\">DORMANT-FOOT"), "{name}: {field_html}");
        let html = crate::export::html::export_html(&projected).expect("raw Table body is supported");
        assert!(html.contains("<td>BODY</td>"));
        assert!(!html.contains("DORMANT-FOOT"), "preserve current raw-footer rendering limit");
        assert!(warnings.is_empty(), "{name}: unrendered footer contributes no deep-heading warning");
    }
}

#[test]
fn batch_g_html_emitted_standalone_footer_still_warns() {
    let footer = Content::table_footer(Content::heading(6, Content::text("EMITTED-FOOT")), false);
    let mut warnings = vec![];
    let projected = bundle_html_semantic_content(&footer, &mut warnings);
    let html = crate::export::html::export_html(&projected).expect("standalone Footer body is supported");
    assert!(html.contains("EMITTED-FOOT"));
    assert_eq!(warnings.len(), 1);
    assert_eq!(warnings[0].message, "heading of level 6 was transformed to <div role=\"heading\" aria-level=\"7\">, which is not supported by all assistive technology");
    assert_eq!(warnings[0].hints, vec![
        "HTML only supports <h1> to <h6>, not <h7>",
        "you may want to restructure your document so that it doesn't contain deep headings",
    ]);
}

#[test]
fn batch_g_html_public_resolved_footer_keeps_deep_warning_once() {
    let mut features = Features::empty();
    features.enable(Feature::Bundle);
    features.enable(Feature::Html);
    let world = crate::world::SystemWorld::for_stdin(
        std::path::PathBuf::from("/tmp"),
        "#document(\"one.html\")[#table(columns:1,[BODY],table.footer([#heading(level:6)[FOOT]]))]".into(),
    );
    let source = world.source(world.main()).unwrap();
    let (result, warnings, _) = compile_to_bundle_files(&world, &source, BundleCompileOptions {
        full_error:false, features, document_id:None, pdf_stream_mode:StreamMode::Verbose,
        pdf_serialization:PdfSerializationMode::Compact, pdf_tagged_override:None,
        pdf_pages_override:None, pdf_standard_override:None, png_ppi_override:None,
        html_serialization:crate::export::HtmlSerializationMode::Vanilla, pretty_override:None,
    });
    let files = result.expect("resolved footer remains supported");
    let html = String::from_utf8(files[0].data.clone()).unwrap();
    assert!(html.contains("<tfoot><tr><td><div role=\"heading\" aria-level=\"7\">FOOT</div></td></tr></tfoot>"), "{html}");
    let deep = warnings.iter().filter(|w| w.message.contains("heading of level 6")).collect::<Vec<_>>();
    assert_eq!(deep.len(), 1);
    assert_eq!(deep[0].message, "heading of level 6 was transformed to <div role=\"heading\" aria-level=\"7\">, which is not supported by all assistive technology");
    assert_eq!(deep[0].hints, vec![
        "HTML only supports <h1> to <h6>, not <h7>",
        "you may want to restructure your document so that it doesn't contain deep headings",
    ]);
}
