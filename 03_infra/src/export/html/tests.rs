//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/html/tests.md
//! @prompt-hash 258dd346
//! @layer L3
//! @updated 2026-10-07

use super::document::PREFIX;
use super::*;
use std::sync::Arc;
use typst_core::entities::html::{HtmlAttrs, HtmlElem};

#[test]
fn p1293_c_modos_separam_escape_e_default_preserva_crystalline() {
    let mut attrs = HtmlAttrs::default();
    attrs.insert("value".into(), "x&\"<>".into());
    let elem = Content::HtmlElem(Arc::new(HtmlElem::new(
        "button".into(),
        Some(attrs),
        typst_core::entities::html::HtmlBody::Content(Box::new(Content::text("<&>"))),
    )));
    let default = export_html(&elem).unwrap();
    let crystalline =
        export_html_with_serialization(&elem, HtmlSerializationMode::Crystalline)
            .unwrap();
    let vanilla =
        export_html_with_serialization(&elem, HtmlSerializationMode::Vanilla).unwrap();

    assert_eq!(default, crystalline);
    assert!(crystalline.contains("value=\"x&amp;&quot;&lt;&gt;\""));
    assert!(crystalline.contains("&lt;&amp;&gt;"));
    assert!(vanilla.contains("value=\"x&amp;&quot;<>\""));
    assert!(vanilla.contains("&lt;&amp;>"));
}

#[test]
fn exports_plain_paragraph_like_vanilla() {
    let html = export_html(&Content::text("Hello, parity. ")).unwrap();
    assert_eq!(html, format!("{PREFIX}<p>Hello, parity.</p></body></html>"));
}

#[test]
fn escapes_html_metacharacters() {
    let html = export_html(&Content::text("<&\"> ")).unwrap();
    assert!(html.contains("&lt;&amp;&quot;&gt;"));
}

#[test]
fn exports_explicit_html_element_without_paragraph_wrapper() {
    let mut attrs = HtmlAttrs::default();
    attrs.insert("lang".into(), "pt".into());
    attrs.insert("title".into(), "a&b".into());
    let elem = Content::HtmlElem(Arc::new(HtmlElem::new(
        "article".into(),
        Some(attrs),
        typst_core::entities::html::HtmlBody::Content(Box::new(Content::text(
            "Olá & mundo",
        ))),
    )));
    let html = export_html(&Content::sequence(vec![elem])).unwrap();
    assert_eq!(
        html,
        format!(
            "{PREFIX}<article lang=\"pt\" title=\"a&amp;b\">Olá &amp; mundo</article></body></html>"
        )
    );
}

#[test]
fn p1168_exports_empty_attribute_without_equals_or_quotes() {
    let mut attrs = HtmlAttrs::default();
    attrs.insert("hidden".into(), "".into());
    let elem = Content::HtmlElem(Arc::new(HtmlElem::new(
        "div".into(),
        Some(attrs),
        typst_core::entities::html::HtmlBody::None,
    )));
    let html = export_html(&elem).unwrap();
    assert!(html.contains("<div hidden></div>"));
    assert!(!html.contains("hidden=\"\""));
}

#[test]
fn p1171_1_void_nao_fecha_e_rejeita_body() {
    let br = Content::HtmlElem(Arc::new(HtmlElem::new(
        "br".into(),
        None,
        typst_core::entities::html::HtmlBody::Unset,
    )));
    assert!(export_html(&br).unwrap().contains("<body><p><br></p></body>"));

    let with_body = Content::HtmlElem(Arc::new(HtmlElem::new(
        "br".into(),
        None,
        typst_core::entities::html::HtmlBody::Content(Box::new(Content::text("X"))),
    )));
    assert_eq!(
        export_html(&with_body).unwrap_err().message,
        "HTML void elements must not have children"
    );
}

#[test]
fn p1171_1_remove_bordas_e_preserva_espaco_inline() {
    let spans = Content::HtmlElem(Arc::new(HtmlElem::new(
        "div".into(),
        None,
        typst_core::entities::html::HtmlBody::Content(Box::new(Content::sequence(vec![
            Content::Space,
            Content::HtmlElem(Arc::new(HtmlElem::new(
                "span".into(),
                None,
                typst_core::entities::html::HtmlBody::Content(Box::new(Content::text(
                    "A",
                ))),
            ))),
            Content::Space,
            Content::HtmlElem(Arc::new(HtmlElem::new(
                "span".into(),
                None,
                typst_core::entities::html::HtmlBody::Content(Box::new(Content::text(
                    "B",
                ))),
            ))),
            Content::Space,
        ]))),
    )));
    assert!(export_html(&spans)
        .unwrap()
        .contains("<div><span>A</span> <span>B</span></div>"));
}

#[test]
fn p1172_1_agrupa_phrasing_e_respeita_boundaries() {
    let elem = |tag: &str, text: &str| {
        Content::HtmlElem(Arc::new(HtmlElem::new(
            tag.into(),
            None,
            typst_core::entities::html::HtmlBody::Content(Box::new(Content::text(text))),
        )))
    };
    let content =
        Content::sequence(vec![elem("span", "A"), elem("div", "B"), elem("a", "C")]);
    let html = export_html(&content).unwrap();
    assert!(html.contains("<p><span>A</span></p><div>B</div><p><a>C</a></p>"));

    let custom = export_html(&elem("x-widget", "X")).unwrap();
    assert!(custom.contains("<body><x-widget>X</x-widget></body>"));
    assert!(!custom.contains("<p><x-widget>"));
}

#[test]
fn p1173_1_remove_espaco_adjacente_a_block_siblings() {
    let elem = |tag: &str, text: &str| {
        Content::HtmlElem(Arc::new(HtmlElem::new(
            tag.into(),
            None,
            typst_core::entities::html::HtmlBody::Content(Box::new(Content::text(text))),
        )))
    };
    let article =
        Content::HtmlElem(Arc::new(HtmlElem::new(
            "article".into(),
            None,
            typst_core::entities::html::HtmlBody::Content(Box::new(Content::sequence(
                vec![elem("address", "A"), Content::Space, elem("aside", "B")],
            ))),
        )));
    assert!(export_html(&article)
        .unwrap()
        .contains("<article><address>A</address><aside>B</aside></article>"));
}

#[test]
fn p1175_1_protege_espaco_entre_inline_vazios() {
    let empty = |tag: &str| {
        Content::HtmlElem(Arc::new(HtmlElem::new(
            tag.into(),
            None,
            typst_core::entities::html::HtmlBody::None,
        )))
    };
    let content = Content::sequence(vec![empty("span"), Content::Space, empty("span")]);
    assert!(export_html(&content).unwrap().contains(
        "<p><span></span><span style=\"white-space: pre-wrap\">&#x20;</span><span></span></p>"
    ));
}

#[test]
fn p1176_1_summary_block_e_boundary_top_level() {
    let empty = |tag: &str| {
        Content::HtmlElem(Arc::new(HtmlElem::new(
            tag.into(),
            None,
            typst_core::entities::html::HtmlBody::None,
        )))
    };
    let top = Content::sequence(vec![
        empty("datalist"),
        Content::Space,
        empty("datalist"),
        Content::Space,
        empty("summary"),
    ]);
    let html = export_html(&top).unwrap();
    assert!(
        html.contains("<datalist></datalist><datalist></datalist><summary></summary>")
    );
    assert!(!html.contains("white-space: pre-wrap"));

    let details =
        Content::HtmlElem(Arc::new(HtmlElem::new(
            "details".into(),
            None,
            typst_core::entities::html::HtmlBody::Content(Box::new(Content::sequence(
                vec![empty("summary"), Content::Space, Content::text("Body")],
            ))),
        )));
    assert!(export_html(&details)
        .unwrap()
        .contains("<details><summary></summary>Body</details>"));
}

#[test]
fn p1177_1_ruby_descarta_so_espaco_estrutural_rp_rt() {
    let elem = |tag: &str, body: Content| {
        Content::HtmlElem(Arc::new(HtmlElem::new(
            tag.into(),
            None,
            typst_core::entities::html::HtmlBody::Content(Box::new(body)),
        )))
    };
    let ruby = elem(
        "ruby",
        Content::sequence(vec![
            elem("rp", Content::text("(")),
            Content::Space,
            elem("rt", Content::text("T")),
            Content::Space,
            elem("rp", Content::text(")")),
        ]),
    );
    assert!(export_html(&ruby)
        .unwrap()
        .contains("<p><ruby><rp>(</rp><rt>T</rt><rp>)</rp></ruby></p>"));

    let counterexample = elem(
        "ruby",
        Content::sequence(vec![
            Content::text("A"),
            Content::Space,
            elem("rt", Content::text("T")),
            Content::Space,
            elem("span", Content::text("X")),
            Content::Space,
            Content::text("B"),
        ]),
    );
    assert!(export_html(&counterexample)
        .unwrap()
        .contains("<ruby>A <rt>T</rt> <span>X</span> B</ruby>"));
}

#[test]
fn p1178_1_adota_html_e_body_unicos() {
    let elem = |tag: &str, body: Content| {
        Content::HtmlElem(Arc::new(HtmlElem::new(
            tag.into(),
            None,
            typst_core::entities::html::HtmlBody::Content(Box::new(body)),
        )))
    };
    let explicit_html = elem(
        "html",
        Content::sequence(vec![
            elem("head", elem("title", Content::text("T"))),
            elem("body", Content::text("B")),
        ]),
    );
    assert_eq!(
        export_html(&explicit_html).unwrap(),
        "<!DOCTYPE html><html><head><title>T</title></head><body>B</body></html>"
    );

    let body = elem("body", Content::text("B"));
    assert_eq!(
        export_html(&body).unwrap(),
        format!("<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\"></head><body>B</body></html>")
    );
}

#[test]
fn p1178_1_rejeita_html_e_body_com_siblings() {
    let empty = |tag: &str| {
        Content::HtmlElem(Arc::new(HtmlElem::new(
            tag.into(),
            None,
            typst_core::entities::html::HtmlBody::None,
        )))
    };
    let html = Content::sequence(vec![Content::text("A"), empty("html")]);
    assert_eq!(
        export_html(&html).unwrap_err().message,
        "`<html>` element must be the only element in the document"
    );
    let bodies = Content::sequence(vec![empty("body"), empty("body")]);
    assert_eq!(
        export_html(&bodies).unwrap_err().message,
        "`<body>` element must be the only element in the document"
    );
}

#[test]
fn p1178_1_title_usa_texto_pre_escapable_raw() {
    let title = Content::HtmlElem(Arc::new(HtmlElem::new(
        "title".into(),
        None,
        typst_core::entities::html::HtmlBody::Content(Box::new(Content::sequence(vec![
            Content::text("A  B"),
            Content::linebreak(),
            Content::text("C & < > \""),
        ]))),
    )));
    assert!(export_html(&title)
        .unwrap()
        .contains("<title>A  B\nC &amp; &lt; > \"</title>"));

    let invalid = Content::HtmlElem(Arc::new(HtmlElem::new(
        "title".into(),
        None,
        typst_core::entities::html::HtmlBody::Content(Box::new(Content::HtmlElem(
            Arc::new(HtmlElem::new(
                "span".into(),
                None,
                typst_core::entities::html::HtmlBody::None,
            )),
        ))),
    )));
    assert_eq!(
        export_html(&invalid).unwrap_err().message,
        "HTML raw text element cannot have non-text children"
    );
}

#[test]
fn p1423_script_e_style_preservam_raw_text() {
    for (tag, text) in
        [("script", "if (a < b) x &= 1;"), ("style", "a > b { content: \"<&\"; }")]
    {
        let elem = Content::HtmlElem(Arc::new(HtmlElem::new(
            tag.into(),
            None,
            typst_core::entities::html::HtmlBody::Content(Box::new(Content::text(text))),
        )));
        let html = export_html_with_serialization(&elem, HtmlSerializationMode::Vanilla)
            .expect("raw text HTML");
        assert!(html.contains(&format!("<{tag}>{text}</{tag}>")));
    }
}
#[test]
fn styled_text_preserves_contextual_raw_and_title_encoding() {
    use typst_core::entities::html::HtmlBody;
    use typst_core::entities::style::Styles;
    let text = Content::styled(Content::text("A  <&>\" B"), Styles::new());
    for (tag, expected) in [("script", "A  <&>\" B"), ("title", "A  &lt;&amp;>\" B")] {
        let content = Content::HtmlElem(Arc::new(HtmlElem::new(
            tag.into(),
            None,
            HtmlBody::Content(Box::new(text.clone())),
        )));
        for mode in [HtmlSerializationMode::Crystalline, HtmlSerializationMode::Vanilla] {
            let compact = export_html_with_serialization(&content, mode).unwrap();
            assert!(compact.contains(&format!("<{tag}>{expected}</{tag}>")));
        }
    }
}

#[test]
fn resolved_frame_is_inserted_once_without_text_escape() {
    use std::cell::Cell;
    use typst_core::entities::html::HtmlFrameElem;
    let frame =
        Content::HtmlFrame(Arc::new(HtmlFrameElem { body: Content::text("frame") }));
    assert_eq!(
        export_html(&frame).unwrap_err().message,
        "html.frame must be resolved by the HTML compilation pipeline"
    );
    let calls = Cell::new(0);
    let resolver =
        |_: &HtmlFrameElem, _: &typst_core::entities::style_chain::StyleChain| {
            calls.set(calls.get() + 1);
            Ok("<svg><text>A &amp; B</text></svg>".to_string())
        };
    let html = export_html_with_resolved_frames(
        &frame,
        HtmlSerializationMode::Crystalline,
        false,
        &resolver,
    )
    .unwrap();
    assert_eq!(calls.get(), 1);
    assert!(html.contains("<body><svg><text>A &amp; B</text></svg></body>"));
    assert!(!html.contains("&lt;svg"));
}

#[test]
fn semantic_table_preserves_header_spans_and_cell_escaping() {
    use typst_core::entities::elements::table::TableElem;
    use typst_core::entities::elements::table_cell::{TableCellElem, TableCellKind};
    use typst_core::entities::layout_types::{Length, TrackSizing};
    use typst_core::entities::sides::Sides;
    let cell = Content::TableCell(Arc::new(TableCellElem {
        source_span: typst_core::entities::span::Span::detached(),
        resolved: None,
        standalone: None,
        presence: typst_core::entities::elements::table_cell::TableCellPresence {
            colspan: true,
            rowspan: true,
            ..Default::default()
        },
        body: Content::text("<&"),
        x: None,
        y: None,
        colspan: Some(2),
        rowspan: Some(3),
        stroke: None,
        fill: None,
        align: None,
        inset: None,
        breakable: None,
        kind: TableCellKind::Auto,
    }));
    let table = Content::Table(Arc::new(TableElem {
        source_span: typst_core::entities::span::Span::detached(),
        resolved: None,
        rows_explicit: false,
        columns: vec![TrackSizing::Auto, TrackSizing::Auto],
        rows: vec![],
        children: vec![cell],
        hlines: vec![],
        vlines: vec![],
        header: Some(Content::sequence(vec![Content::text("H1"), Content::text("H2")])),
        footer: None,
        stroke: None,
        fill: None,
        caption: None,
        summary: None,
        inset: Sides::uniform(Length::pt(0.0)),
        align: None,
    }));
    for mode in [HtmlSerializationMode::Crystalline, HtmlSerializationMode::Vanilla] {
        let compact = export_html_with_serialization(&table, mode).unwrap();
        assert!(compact.contains("<table><thead><tr><th>H1</th><th>H2</th></tr></thead><tbody><tr><td colspan=\"2\" rowspan=\"3\">&lt;&amp;</td></tr></tbody></table>"));
    }
}

#[test]
fn p1494_espaco_com_newline_tem_projecao_html_local_identica() {
    fn element(tag: &str, body: Content) -> Content {
        Content::HtmlElem(Arc::new(HtmlElem::new(
            tag.into(),
            None,
            typst_core::entities::html::HtmlBody::Content(Box::new(body)),
        )))
    }

    fn render_cases(space: Content) -> Vec<String> {
        let br = Content::HtmlElem(Arc::new(HtmlElem::new(
            "br".into(),
            None,
            typst_core::entities::html::HtmlBody::None,
        )));
        let cases = [
            // Top-level paragraph and its boundary whitespace.
            Content::sequence(vec![
                space.clone(),
                Content::text("A"),
                space.clone(),
                Content::text("B"),
                space.clone(),
            ]),
            // Inline, raw and title use separate exporter paths.
            element(
                "span",
                Content::sequence(vec![
                    Content::text("A"),
                    space.clone(),
                    Content::text("B"),
                ]),
            ),
            element(
                "script",
                Content::sequence(vec![
                    Content::text("A"),
                    space.clone(),
                    Content::text("B"),
                ]),
            ),
            element(
                "title",
                Content::sequence(vec![
                    Content::text("A"),
                    space.clone(),
                    Content::text("B"),
                ]),
            ),
            // Block and void neighbors still use structural whitespace.
            Content::sequence(vec![
                Content::text("A"),
                space.clone(),
                element("div", Content::text("B")),
                space.clone(),
                br,
                space.clone(),
                Content::text("C"),
            ]),
            // Explicit document root must ignore both edge-space kinds.
            Content::sequence(vec![
                space.clone(),
                element("html", element("body", Content::text("Body"))),
                space,
            ]),
        ];
        cases.iter().map(|content| export_html(content).unwrap()).collect()
    }

    assert_eq!(render_cases(Content::Space), render_cases(Content::SpaceFromNewline));
}

#[test]
fn smartquote_semantic_html_context_crosses_styles_and_resets_paragraph() {
    use typst_core::entities::style::Styles;
    let content = Content::sequence(vec![
        Content::smartquote(true),
        Content::styled(Box::new(Content::text("hello")), Styles::default()),
        Content::smartquote(true),
        Content::Parbreak,
        Content::text("6"),
        Content::smartquote(false),
        Content::Space,
        Content::text("2"),
        Content::smartquote(true),
    ]);
    let html =
        export_html_with_serialization(&content, HtmlSerializationMode::Vanilla).unwrap();
    assert!(html.contains("<p>“hello”</p><p>6′ 2″</p>"), "{html}");
}

#[test]
fn smartquote_semantic_html_title_custom_escape_and_raw_guard() {
    use typst_core::entities::{html::HtmlBody, style::Styles, value::Value};
    let quoted = Content::sequence(vec![
        Content::smartquote(true),
        Content::text("X&"),
        Content::smartquote(true),
    ]);
    let body = Content::styled(
        Box::new(quoted),
        Styles::default().push_custom("smartquote.quotes", Value::Str("<>".into())),
    );
    let title = Content::HtmlElem(Arc::new(HtmlElem::new(
        "title".into(),
        None,
        HtmlBody::Content(Box::new(body)),
    )));
    let html =
        export_html_with_serialization(&title, HtmlSerializationMode::Vanilla).unwrap();
    assert!(html.contains("<title>&lt;X&amp;></title>"), "{html}");
    let script = Content::HtmlElem(Arc::new(HtmlElem::new(
        "script".into(),
        None,
        HtmlBody::Content(Box::new(Content::smartquote(true))),
    )));
    assert_eq!(
        export_html(&script).unwrap_err().message,
        "HTML raw text element cannot have non-text children"
    );
}

#[test]
fn d08b3b_typc_atom_preserves_semantic_code_and_constant_style() {
    for atom in ["auto", "true", "false"] {
        let content = Content::raw(atom, Some("typc".into()), false);
        let html =
            export_html_with_serialization(&content, HtmlSerializationMode::Vanilla)
                .unwrap();
        assert!(html.contains(&format!("<code data-lang=\"typc\"><span style=\"color: #d73948\">{atom}</span></code>")),"{html}");
    }
}
