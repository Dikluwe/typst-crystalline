//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/pipeline/tests.md
//! @prompt-hash 3ea4489a
//! @layer L3
//! @updated 2026-10-09
//!
//! Possui a suíte integrada anteriormente embutida em pipeline.rs. Conserva integralmente os oráculos e fixtures; somente a localização física muda. Exercita as fachadas públicas e os helpers internos por descendência do módulo. Não legitima alteração de código produtivo.

use super::*;
use std::num::NonZeroU16;
use typst_core::entities::file_id::FileId;
use typst_core::entities::font_book::FontBook;
use typst_core::entities::world_types::{
    Bytes, Datetime, FileError, FileResult, Font, Library,
};

#[test]
fn p1247_contexto_svg_filtra_pagina_e_posicao_homologa() {
    use typst_core::entities::label::Label;
    use typst_core::entities::layout_types::{Point, Pt};

    let mut doc = PagedDocument::new(vec![]);
    let p0 = Label("p0".to_string());
    let p1 = Label("p1".to_string());
    let sem_posicao = Label("sem-posicao".to_string());
    doc.extracted_label_pages.insert(p0.clone(), 1);
    doc.extracted_label_pages.insert(p1.clone(), 2);
    doc.extracted_label_pages.insert(sem_posicao.clone(), 1);
    doc.extracted_label_positions
        .insert(p0.clone(), Point { x: Pt(8.0), y: Pt(9.0) });
    doc.extracted_label_positions
        .insert(p1.clone(), Point { x: Pt(80.0), y: Pt(90.0) });

    let context = svg_destination_context_for_page(&doc, 1);
    assert!(context.id_for(&p0).is_some());
    assert!(context.id_for(&p1).is_none());
    assert!(context.id_for(&sem_posicao).is_none());
}

// MockWorld mínimo para smoke test — source única.
struct MockWorld {
    library: Library,
    book: FontBook,
    source: Source,
}
impl World for MockWorld {
    fn library(&self) -> &Library {
        &self.library
    }
    fn book(&self) -> &FontBook {
        &self.book
    }
    fn main(&self) -> FileId {
        self.source.id()
    }
    fn source(&self, id: FileId) -> FileResult<Source> {
        if id == self.source.id() {
            Ok(self.source.clone())
        } else {
            Err(FileError::NotFound)
        }
    }
    fn file(&self, _: FileId) -> FileResult<Bytes> {
        Err(FileError::NotFound)
    }
    fn font(&self, _: usize) -> Option<Font> {
        None
    }
    fn today(
        &self,
        _: Option<typst_core::entities::duration::Duration>,
    ) -> Option<Datetime> {
        None
    }
}

fn mock_world(src: &str) -> MockWorld {
    let id = FileId::from_raw(NonZeroU16::new(1).unwrap());
    let source = Source::new(id, src.to_string()).expect("valid Source test fixture");
    MockWorld {
        library: Library::new(),
        book: FontBook::new(),
        source,
    }
}

#[test]
fn p1462_pipeline_collects_svg_pretty_and_empty_set_is_neutral() {
    for (source, expected) in [
        ("#set format.svg(pretty: true)\nHello", true),
        ("#set format.svg()\nHello", false),
        ("#set format.svg(pretty: true)\n#set format.svg()\nHello", true),
        ("#set format.svg(pretty: true)\n#set format.svg(pretty: false)\nHello", false),
        ("#set format.svg(pretty: false)\n#set format.svg(pretty: true)\nHello", true),
    ] {
        let world = mock_world(source);
        let mut timings = Timings::default();
        let (result, _) = compile_to_paged_document_full_error(
            &world,
            &world.source,
            false,
            &mut timings,
        );

        assert_eq!(result.unwrap().format_options.svg.pretty, expected);
    }
}

#[test]
fn p1462_pipeline_rejects_nested_format_set_rule() {
    let world = mock_world("#block[\n#set format.svg(pretty: true)\nNested\n]");
    let mut timings = Timings::default();
    let (result, _) =
        compile_to_paged_document_full_error(&world, &world.source, false, &mut timings);

    assert!(result.unwrap_err().iter().any(|error| {
        error
            .message
            .contains("format set rules are not allowed inside of containers")
    }));
}

#[test]
fn p1462_set_rule_diagnostics_are_explicit() {
    for (source, expected) in [
        ("#set format.svg(true)", "unexpected argument"),
        ("#set format.svg(unknown: true)", "unexpected argument: unknown"),
        ("#set format.svg(pretty: true, pretty: false)", "duplicate argument: pretty"),
        ("#set format.svg(pretty: 1)", "expected boolean, found integer"),
    ] {
        let world = mock_world(source);
        let mut timings = Timings::default();
        let (result, _) = compile_to_paged_document_full_error(
            &world,
            &world.source,
            false,
            &mut timings,
        );

        assert!(result
            .unwrap_err()
            .iter()
            .any(|error| error.message.contains(expected)));
    }
}

#[test]
fn p1462_format_svg_cannot_be_constructed_manually() {
    let world = mock_world("#format.svg()");
    let mut timings = Timings::default();
    let (result, _) =
        compile_to_paged_document_full_error(&world, &world.source, false, &mut timings);

    assert!(result
        .unwrap_err()
        .iter()
        .any(|error| error.message.contains("cannot be constructed manually")));
}

#[test]
fn p1462_external_svg_observes_document_and_caller_precedence() {
    fn render(source: &str, pretty_override: Option<bool>) -> String {
        let world = mock_world(source);
        let (result, _, _) =
            compile_to_svg_string_with_timings_full_error_and_features_and_pretty_override(
                &world,
                &world.source,
                false,
                typst_core::entities::compiler_features::Features::default(),
                pretty_override,
            );
        result.unwrap()
    }

    let body = "#rect(width: 10pt, height: 10pt)";
    let document_pretty =
        render("#set format.svg(pretty: true)\n#rect(width: 10pt, height: 10pt)", None);
    let caller_compact = render(
        "#set format.svg(pretty: true)\n#rect(width: 10pt, height: 10pt)",
        Some(false),
    );
    let default_compact = render(body, None);
    let caller_pretty = render(body, Some(true));

    assert!(document_pretty.contains("\n  <"));
    assert!(caller_pretty.contains("\n  <"));
    assert!(!caller_compact.contains("\n  <"));
    assert!(!default_compact.contains("\n  <"));
}

#[test]
fn p1339_filtered_context_waits_for_later_contextual_update() {
    let world = mock_world(
        "#let c = counter(heading.where(level: 1))\n#context assert.eq(c.final(), (3,))\n#context c.update(3)",
    );
    let mut timings = Timings::default();
    let (result, _) =
        compile_to_paged_document_full_error(&world, &world.source, false, &mut timings);
    assert!(result.is_ok(), "{result:?}");
}

#[test]
fn p1339_filtered_context_does_not_discard_independent_error() {
    let world = mock_world(
        "#let c = counter(heading.where(level: 1))\n#context panic(\"independent\")\n#context c.final()",
    );
    let mut timings = Timings::default();
    let (result, _) =
        compile_to_paged_document_full_error(&world, &world.source, false, &mut timings);
    assert!(result
        .unwrap_err()
        .iter()
        .any(|error| error.message.contains("independent")));
}

#[test]
fn single_evaluation_recipe_context_ids_do_not_collide_with_captured_content() {
    let world = mock_world(
        "#let cached = [#context [Captured]]\n#show heading: it => [#cached #context [Generated]]\n= Original",
    );
    let mut timings = Timings::default();
    let (result, _) =
        compile_to_paged_document_full_error(&world, &world.source, false, &mut timings);
    let text = result
        .expect("distinct contextual producers must execute")
        .plain_text();
    assert!(text.contains("Captured") && text.contains("Generated"), "{text:?}");
}

#[test]
fn single_evaluation_context_query_sees_original_and_generated() {
    let world = mock_world(
        "#show heading.where(level: 1): it => heading(level: 2)[Generated]\n= Original\n#context query(heading).map(it => it.body).join([ | ])",
    );
    let mut timings = Timings::default();
    let (result, _) =
        compile_to_paged_document_full_error(&world, &world.source, false, &mut timings);
    let text = result.expect("contextual query must render").plain_text();
    assert!(text.contains("Original | Generated"), "{text:?}");
}

#[test]
fn p1406_show_outline_entry_direto_realiza_entries() {
    let world = mock_world(concat!(
        "#set heading(numbering: \"1.\")\n",
        "#show outline.entry: it => [CUSTOM #(it.level): #it.body()]\n",
        "#outline(title: none)\n\n= Alpha\n== Beta\n",
    ));
    let mut timings = Timings::default();
    let (result, _) =
        compile_to_paged_document_full_error(&world, &world.source, false, &mut timings);
    let text = result.expect("P1406 deve aceitar o seletor direto").plain_text();
    assert!(text.contains("CUSTOM 1") && text.contains("Alpha"), "{text:?}");
    assert!(text.contains("CUSTOM 2") && text.contains("Beta"), "{text:?}");
    assert!(!text.contains("Contents"), "title: none deve omitir título: {text:?}");
}

#[test]
fn p1406_where_filtra_level_da_entry_gerada() {
    let world = mock_world(concat!(
        "#set heading(numbering: \"1.\")\n",
        "#show outline.entry.where(level: 2): it => [SECOND: #it.body()]\n",
        "#outline(title: none)\n\n= Alpha\n== Beta\n",
    ));
    let mut timings = Timings::default();
    let (result, _) =
        compile_to_paged_document_full_error(&world, &world.source, false, &mut timings);
    let text = result.expect("P1406 where deve compilar").plain_text();
    assert_eq!(text.matches("SECOND").count(), 1, "{text:?}");
    assert!(text.contains("SECOND") && text.contains("Beta"), "{text:?}");
}

#[test]
fn p1406_snapshot_de_show_e_lexical_por_outline() {
    let world = mock_world(concat!(
        "#set heading(numbering: \"1.\")\n",
        "#outline(title: none)\n",
        "#show outline.entry: it => [CUSTOM: #it.body()]\n",
        "#outline(title: none)\n\n= Alpha\n",
    ));
    let mut timings = Timings::default();
    let (result, _) =
        compile_to_paged_document_full_error(&world, &world.source, false, &mut timings);
    let text = result.expect("P1406 lexical deve compilar").plain_text();
    assert_eq!(text.matches("CUSTOM").count(), 1, "{text:?}");
}

#[test]
fn p1406_parent_existe_somente_durante_realizacao() {
    let world = mock_world(concat!(
        "#set heading(numbering: \"1.\")\n",
        "#show outline.entry: it => it.indented(it.prefix(), it.body())\n",
        "#outline(title: none, indent: 2em)\n\n= Alpha\n== Beta\n",
    ));
    let mut timings = Timings::default();
    let (result, _) =
        compile_to_paged_document_full_error(&world, &world.source, false, &mut timings);
    let text = result.expect("parent P1406 deve existir durante show").plain_text();
    assert!(text.contains("Alpha") && text.contains("Beta"), "{text:?}");
}

#[test]
fn p1406_alias_preserva_identidade_do_seletor() {
    let world = mock_world(concat!(
        "#let entry-kind = outline.entry\n",
        "#show entry-kind: it => [ALIAS #it.body()]\n",
        "#outline(title: none)\n\n= Alpha\n",
    ));
    let mut timings = Timings::default();
    let (result, _) =
        compile_to_paged_document_full_error(&world, &world.source, false, &mut timings);
    let text = result.expect("alias deve conservar identidade nativa").plain_text();
    assert_eq!(text.matches("ALIAS").count(), 1, "{text:?}");
}

#[test]
fn p1406_query_nao_duplica_e_contexto_nao_pula_realizacao() {
    let world = mock_world(concat!(
        "#show outline.entry: it => [ENTRY #it.body()]\n",
        "#outline(title: none)\n\n= Alpha\n",
        "#context assert.eq(query(outline.entry).len(), 1)\n",
    ));
    let mut timings = Timings::default();
    let (result, _) =
        compile_to_paged_document_full_error(&world, &world.source, false, &mut timings);
    let text = result.expect("query deve manter uma entry").plain_text();
    assert_eq!(text.matches("ENTRY").count(), 1, "{text:?}");
}

fn readiness_outline_query_fixture(rule: &str, outlines: usize, hidden: bool) {
    let source = format!(
        "{rule}{}= Alpha\n#context assert.eq(query(outline.entry).len(), {outlines})\n",
        "#outline(title: none)\n".repeat(outlines),
    );
    let world = mock_world(&source);
    let mut timings = Timings::default();
    let (result, _) =
        compile_to_paged_document_full_error(&world, &world.source, false, &mut timings);
    let doc = result.expect("actual Entry query membership");
    let text = doc.plain_text();
    assert_eq!(
        text.matches("Alpha").count(),
        if hidden { 1 } else { outlines + 1 },
        "{text:?}"
    );
    if !hidden {
        assert!(text.contains('.') && text.contains('1'), "default fill/page: {text:?}");
    }
}

#[test]
fn readiness_outline_query_default_entry() {
    readiness_outline_query_fixture("", 1, false);
}

#[test]
fn readiness_outline_query_style_entry() {
    readiness_outline_query_fixture(
        "#show outline.entry: set text(fill: red)\n",
        1,
        false,
    );
}

#[test]
fn readiness_outline_query_identity_entry() {
    readiness_outline_query_fixture("#show outline.entry: it => it\n", 1, false);
}

#[test]
fn readiness_outline_query_hidden_entry() {
    readiness_outline_query_fixture("#show outline.entry: none\n", 1, true);
}

#[test]
fn readiness_outline_query_distinct_entries_are_preserved() {
    readiness_outline_query_fixture("", 2, false);
}

#[test]
fn p1406_default_emite_link_interno() {
    let world = mock_world("#outline(title: none)\n\n= Alpha\n");
    let mut timings = Timings::default();
    let (result, _) =
        compile_to_paged_document_full_error(&world, &world.source, false, &mut timings);
    let doc = result.expect("outline default deve compilar");
    assert!(format!("{:?}", doc.pages).contains("Destination"));
}

#[test]
fn p1406_depth_e_outlined_filtram_entries_realizadas() {
    let world = mock_world(concat!(
        "#show outline.entry: it => [ENTRY #(it.level): #it.body()]\n",
        "#outline(title: none, depth: 1)\n",
        "#heading(outlined: false)[Hidden]\n",
        "= Visible\n",
        "== Deep\n",
    ));
    let mut timings = Timings::default();
    let (result, _) =
        compile_to_paged_document_full_error(&world, &world.source, false, &mut timings);
    let text = result.expect("filtros P1406 devem compilar").plain_text();
    assert_eq!(text.matches("ENTRY").count(), 1, "{text:?}");
    assert!(text.contains("ENTRY 1") && text.contains("Visible"), "{text:?}");
}

#[test]
fn p1407_repeat_em_box_fr_multiplica_sem_duplicar_siblings() {
    let world = mock_world(concat!(
        "#set page(width: 180pt, height: auto, margin: 20pt)\n",
        "A#box(width: 100pt, repeat(\".\", gap: 2pt, justify: false))B\n",
    ));
    let mut timings = Timings::default();
    let (result, _) =
        compile_to_paged_document_full_error(&world, &world.source, false, &mut timings);
    let text = result.expect("repeat P1407 deve compilar").plain_text();
    assert_eq!(text.matches('A').count(), 1, "{text:?}");
    assert_eq!(text.matches('B').count(), 1, "{text:?}");
    assert!(text.matches('.').count() > 10, "{text:?}");
}

#[test]
fn p1407_repeat_responde_a_largura_finita() {
    let compile = |width: usize| {
        let world = mock_world(&format!(
            "#set page(width: 300pt, height: auto, margin: 20pt)\nA#box(width: {width}pt, repeat(\".\", gap: 2pt, justify: false))B\n"
        ));
        let mut timings = Timings::default();
        compile_to_paged_document_full_error(&world, &world.source, false, &mut timings)
            .0
            .expect("largura P1407 deve compilar")
            .plain_text()
            .matches('.')
            .count()
    };
    let narrow = compile(120);
    let wide = compile(240);
    assert!(narrow > 1 && wide > narrow, "narrow={narrow}, wide={wide}");
}

fn p1407_destination_dot_count(items: &[FrameItem]) -> usize {
    items
        .iter()
        .map(|item| match item {
            FrameItem::Link {
                target: typst_core::entities::layout_types::LinkTarget::Destination(_),
                items,
                ..
            } => frame_items_text(items).matches('.').count(),
            FrameItem::Group { items, .. }
            | FrameItem::Link { items, .. }
            | FrameItem::TextReplacement { items, .. }
            | FrameItem::Semantic { items, .. } => p1407_destination_dot_count(items),
            FrameItem::TextSeparator { .. }
            | FrameItem::Text { .. }
            | FrameItem::TextShaped { .. }
            | FrameItem::Line { .. }
            | FrameItem::Glyph { .. }
            | FrameItem::Image { .. }
            | FrameItem::Shape { .. } => 0,
        })
        .sum()
}

#[test]
fn p1407_outline_expande_fill_dentro_do_link() {
    let world = mock_world(concat!(
        "#set page(width: 240pt, height: auto, margin: 20pt)\n",
        "#set heading(numbering: \"1.\")\n",
        "#outline(title: none)\n\n= Alpha\n",
    ));
    let mut timings = Timings::default();
    let (result, _) =
        compile_to_paged_document_full_error(&world, &world.source, false, &mut timings);
    let doc = result.expect("outline P1407 deve compilar");
    let dots: usize = doc
        .pages
        .iter()
        .map(|page| p1407_destination_dot_count(&page.items))
        .sum();
    assert!(dots > 10, "fill dentro do link tem somente {dots} pontos");
}

#[test]
fn p1407_repeat_degenerado_e_extremo_permanecem_limitados() {
    let compile = |source: &str| {
        let world = mock_world(source);
        let mut timings = Timings::default();
        compile_to_paged_document_full_error(&world, &world.source, false, &mut timings)
            .0
            .expect("caso limitado P1407 deve compilar")
            .plain_text()
    };
    let oversized = compile(concat!(
        "#set page(width: 100pt, height: auto, margin: 10pt)\n",
        "A#box(width: 1pt, repeat(\"LONG\", justify: true))B\n",
    ));
    assert_eq!(oversized.matches("LONG").count(), 1, "{oversized:?}");

    let capped = compile(concat!(
        "#set page(width: 100100pt, height: auto, margin: 10pt)\n",
        "A#box(width: 100000pt, repeat(\".\", gap: 0pt, justify: false))B\n",
    ));
    assert_eq!(capped.matches('.').count(), 1000);
}

fn p1407_dot_positions(items: &[FrameItem], positions: &mut Vec<f64>) {
    fractional_text_world_positions(
        items,
        typst_core::entities::layout_types::TransformMatrix::identity(),
        &mut |text, x, _y| {
            if text == "." {
                positions.push(x);
            }
        },
    );
}

#[test]
fn p1407_justify_distribui_residuo_sem_criar_copia() {
    let compile = |justify: bool| {
        let world = mock_world(&format!(
            "#set page(width: 180pt, height: auto, margin: 20pt)\nA#box(width: 101pt, repeat(\".\", gap: 7pt, justify: {justify}))B\n"
        ));
        let mut timings = Timings::default();
        let doc = compile_to_paged_document_full_error(
            &world,
            &world.source,
            false,
            &mut timings,
        )
        .0
        .expect("justify P1407 deve compilar");
        let mut positions = Vec::new();
        for page in &doc.pages {
            p1407_dot_positions(&page.items, &mut positions);
        }
        positions
    };
    let fixed = compile(false);
    let justified = compile(true);
    assert_eq!(fixed.len(), justified.len());
    assert!(fixed.len() > 1);
    assert!(
        justified.last().unwrap() > fixed.last().unwrap(),
        "fixed={fixed:?}, justified={justified:?}"
    );
}

fn p1502_compile_paged(source: &str) -> PagedDocument {
    let world = mock_world(source);
    let mut timings = Timings::default();
    compile_to_paged_document_full_error(&world, &world.source, false, &mut timings)
        .0
        .expect("sonda P1502 deve compilar")
}

#[test]
fn p1502_repeat_com_prefixo_preserva_controle_sem_prefixo() {
    let page = "#set page(width: 180pt, height: auto, margin: 20pt)\n";
    let without_prefix = p1502_compile_paged(&format!(
        "{page}#box(width: 100pt, repeat(\".\", gap: 2pt, justify: false))B\n"
    ))
    .plain_text();
    let with_prefix = p1502_compile_paged(&format!(
        "{page}A#box(width: 100pt, repeat(\".\", gap: 2pt, justify: false))B\n"
    ))
    .plain_text();

    let expected_dots = without_prefix.matches('.').count();
    assert!(expected_dots > 1, "{without_prefix:?}");
    assert_eq!(without_prefix.matches('B').count(), 1, "{without_prefix:?}");
    assert_eq!(with_prefix.matches('.').count(), expected_dots, "{with_prefix:?}");
    assert_eq!(with_prefix.matches('A').count(), 1, "{with_prefix:?}");
    assert_eq!(with_prefix.matches('B').count(), 1, "{with_prefix:?}");
}

#[test]
fn p1502_duas_boxes_fr_expandem_repeats_sem_deslocar_suffix_duas_vezes() {
    let page = "#set page(width: 220pt, height: auto, margin: 10pt)\n";
    let compile = |body: &str| {
        let doc = p1502_compile_paged(&format!("{page}{body}\n"));
        let text = doc.plain_text();
        let items = &doc.pages.first().expect("página P1502").items;
        let middle_x = p1408_text_x(items, "M").expect("separador M deve existir");
        let suffix_x = p1408_text_x(items, "B").expect("suffix B deve existir");
        (text, middle_x, suffix_x)
    };
    let (_, control_middle_x, control_suffix_x) =
        compile("A#box(width: 1fr)[.]M#box(width: 1fr)[:]B");
    let (text, middle_x, suffix_x) = compile(concat!(
        "A#box(width: 1fr, repeat(\".\", gap: 2pt, justify: false))M",
        "#box(width: 1fr, repeat(\":\", gap: 2pt, justify: false))B",
    ));

    for marker in ['A', 'M', 'B'] {
        assert_eq!(text.matches(marker).count(), 1, "{marker}: {text:?}");
    }
    assert!(text.matches('.').count() > 1, "{text:?}");
    assert!(text.matches(':').count() > 1, "{text:?}");
    assert!(
        (middle_x - control_middle_x).abs() < 0.05,
        "middle={middle_x}, control={control_middle_x}"
    );
    assert!(
        (suffix_x - control_suffix_x).abs() < 0.05,
        "suffix={suffix_x}, control={control_suffix_x}"
    );
}

#[test]
fn p1502_box_de_body_bloco_nao_consume_repeat_pendente_anterior() {
    let page = "#set page(width: 180pt, height: auto, margin: 20pt)\n";
    let repeat = "#box(width: 80pt, repeat(\".\", gap: 2pt, justify: false))";
    let control = p1502_compile_paged(&format!("{page}{repeat}B\n")).plain_text();
    let with_block_box = p1502_compile_paged(&format!(
        "{page}{repeat}#box(rect(width: 1pt, height: 1pt))B\n"
    ))
    .plain_text();

    let expected_dots = control.matches('.').count();
    assert!(expected_dots > 1, "{control:?}");
    assert_eq!(with_block_box.matches('.').count(), expected_dots, "{with_block_box:?}");
    assert_eq!(with_block_box.matches('B').count(), 1, "{with_block_box:?}");
}

#[test]
fn p1502_box_aninhada_preserva_repeat_e_prefixos() {
    let control = p1502_compile_paged(concat!(
        "#set page(width: 220pt, height: auto, margin: 20pt)\n",
        "#box(width: 80pt, repeat(\".\", gap: 2pt, justify: false))B\n",
    ))
    .plain_text();
    let text = p1502_compile_paged(concat!(
        "#set page(width: 220pt, height: auto, margin: 20pt)\n",
        "A#box(width: 120pt)[X#box(width: 80pt, repeat(\".\", gap: 2pt, justify: false))Y]B\n",
    ))
    .plain_text();

    for marker in ['A', 'X', 'Y', 'B'] {
        assert_eq!(text.matches(marker).count(), 1, "{marker}: {text:?}");
    }
    let expected_dots = control.matches('.').count();
    assert!(expected_dots > 1, "{control:?}");
    assert_eq!(text.matches('.').count(), expected_dots, "{text:?}");
}

#[allow(deprecated)]
fn fractional_text_world_positions(
    items: &[FrameItem],
    matrix: typst_core::entities::layout_types::TransformMatrix,
    visit: &mut impl FnMut(&str, f64, f64),
) {
    use typst_core::entities::layout_types::TransformMatrix;
    for item in items {
        match item {
            FrameItem::Text { pos, text, .. }
            | FrameItem::TextShaped { pos, text, .. } => {
                let (x, y) = matrix.apply(pos.x.0, pos.y.0);
                visit(text.as_str(), x, y);
            }
            FrameItem::Group { pos, matrix: local, items, .. } => {
                let next = matrix
                    .concat(&TransformMatrix::translate(pos.x.0, pos.y.0))
                    .concat(local);
                fractional_text_world_positions(items, next, visit);
            }
            FrameItem::Link { items, .. }
            | FrameItem::TextReplacement { items, .. }
            | FrameItem::Semantic { items, .. } => {
                fractional_text_world_positions(items, matrix, visit);
            }
            FrameItem::TextSeparator { .. }
            | FrameItem::Line { .. }
            | FrameItem::Glyph { .. }
            | FrameItem::Image { .. }
            | FrameItem::Shape { .. } => {}
        }
    }
}

fn p1408_text_x(items: &[FrameItem], needle: &str) -> Option<f64> {
    let mut found = None;
    fractional_text_world_positions(
        items,
        typst_core::entities::layout_types::TransformMatrix::identity(),
        &mut |text, x, _y| {
            if found.is_none() && text.contains(needle) {
                found = Some(x);
            }
        },
    );
    found
}

#[test]
fn fractional_position_observers_compose_group_translation_and_matrix() {
    use typst_core::entities::layout_types::{Point, Pt, TransformMatrix};
    let doc =
        p1502_compile_paged("#set page(width: 180pt, height: auto, margin: 10pt)\n.\n");
    let original =
        p1408_text_x(&doc.pages[0].items, ".").expect("original source dot must exist");
    let wrapped = vec![FrameItem::Group {
        paint_root: false,
        pos: Point { x: Pt(10.0), y: Pt(20.0) },
        matrix: TransformMatrix::scale(2.0, 3.0),
        clip_mask: None,
        inner_width: 160.0,
        inner_height: 40.0,
        items: doc.pages[0].items.clone(),
    }];
    let expected = 10.0 + 2.0 * original;
    let actual = p1408_text_x(&wrapped, ".").expect("nested source dot must survive");
    assert!((actual - expected).abs() < 1e-8, "actual={actual}, expected={expected}");
    let mut dots = Vec::new();
    p1407_dot_positions(&wrapped, &mut dots);
    assert_eq!(dots.len(), 1);
    assert!((dots[0] - expected).abs() < 1e-8);
}

fn p1408_compile_positions(source: &str) -> (f64, Option<f64>) {
    let world = mock_world(source);
    let mut timings = Timings::default();
    let doc =
        compile_to_paged_document_full_error(&world, &world.source, false, &mut timings)
            .0
            .expect("box fracionária P1408 deve compilar");
    let items = &doc.pages[0].items;
    (p1408_text_x(items, "B").expect("suffix B deve existir"), p1408_text_x(items, "Y"))
}

#[test]
fn p1408_um_fr_e_fr_negativo_isolado_ocupam_restante() {
    let prefix = "#set page(width: 180pt, height: auto, margin: 10pt)\n";
    let (one_b, _) = p1408_compile_positions(&format!("{prefix}A#box(width: 1fr)[X]B\n"));
    let (negative_b, _) =
        p1408_compile_positions(&format!("{prefix}A#box(width: -1fr)[X]B\n"));
    assert!(one_b > 150.0, "one_b={one_b}");
    assert!((one_b - negative_b).abs() < 0.01, "one={one_b}, negative={negative_b}");
}

#[test]
fn p1408_zero_fr_nao_recebe_restante() {
    let (suffix, _) = p1408_compile_positions(concat!(
        "#set page(width: 180pt, height: auto, margin: 10pt)\n",
        "A#box(width: 0fr)[X]B\n",
    ));
    assert!(suffix < 40.0, "suffix={suffix}");
}

#[test]
fn p1408_duas_caixas_preservam_razao_um_para_dois() {
    let (suffix, middle) = p1408_compile_positions(concat!(
        "#set page(width: 180pt, height: auto, margin: 10pt)\n",
        "A#box(width: 1fr)[X]#box(width: 2fr)[Y]B\n",
    ));
    let middle = middle.expect("body Y deve existir");
    assert!(
        (middle - 65.533).abs() < 0.05,
        "middle={middle}, esperado interno=65.533 (PDF vanila=66.274)"
    );
    assert!(suffix > 150.0, "suffix={suffix}");
}

#[test]
fn p1408_soma_zero_termina_sem_distribuicao() {
    let (suffix, _) = p1408_compile_positions(concat!(
        "#set page(width: 180pt, height: auto, margin: 10pt)\n",
        "A#box(width: 1fr)[X]#box(width: -1fr)[Y]B\n",
    ));
    assert!(suffix < 50.0, "suffix={suffix}");
}

#[test]
fn p1408_height_fracionaria_permanece_rejeitada() {
    let world = mock_world("#box(height: 1fr)[X]\n");
    let mut timings = Timings::default();
    let result =
        compile_to_paged_document_full_error(&world, &world.source, false, &mut timings)
            .0;
    let errors = result.expect_err("height fracionária deve continuar fora do contrato");
    assert!(errors[0].message.contains("box(height:) espera length"));
}

#[test]
fn p1340_selected_context_preserves_page_numbering_relayout() {
    let world = mock_world(
        "#set page(numbering: (current, total) => [V#current/#total])\n#let c = counter(heading.where())\n#context c.final()\nTexto",
    );
    let (result, _) = compile_to_paged_document_full_error(
        &world,
        &world.source,
        false,
        &mut Timings::default(),
    );
    let document = result.unwrap();
    assert_eq!(frame_items_text(&document.pages[0].foreground), "V1/1");
}

#[test]
fn p1340_reused_unselected_context_keeps_legacy_result() {
    let world = mock_world("#let x = context [x]\n#x #x");
    let (result, _) = compile_to_paged_document_full_error(
        &world,
        &world.source,
        false,
        &mut Timings::default(),
    );
    assert!(result.is_ok(), "{result:?}");
}

#[test]
fn p1340_discovery_keeps_legacy_context_options() {
    let world = mock_world("#context [legacy]");
    let options = context_stabilization::discovered_options_for_test(
        &world,
        &world.source,
        typst_core::entities::compiler_features::Features::html(),
    );
    assert!(!options.is_empty());
    assert!(options.into_iter().all(|(full_error, features)| !full_error
        && features == typst_core::entities::compiler_features::Features::default()));
}

fn frame_items_text(items: &[FrameItem]) -> String {
    let mut out = String::new();
    for item in items {
        match item {
            FrameItem::Text { text, .. } | FrameItem::TextShaped { text, .. } => {
                out.push_str(text)
            }
            FrameItem::Group { items, .. }
            | FrameItem::Link { items, .. }
            | FrameItem::TextReplacement { items, .. }
            | FrameItem::Semantic { items, .. } => out.push_str(&frame_items_text(items)),
            FrameItem::TextSeparator { .. }
            | FrameItem::Line { .. }
            | FrameItem::Glyph { .. }
            | FrameItem::Image { .. }
            | FrameItem::Shape { .. } => {}
        }
    }
    out
}

fn contains_bold_text(items: &[FrameItem], expected: &str) -> bool {
    items.iter().any(|item| match item {
        FrameItem::Text { text, style, .. }
        | FrameItem::TextShaped { text, style, .. } => text == expected && style.bold,
        FrameItem::Group { items, .. }
        | FrameItem::Link { items, .. }
        | FrameItem::TextReplacement { items, .. }
        | FrameItem::Semantic { items, .. } => contains_bold_text(items, expected),
        FrameItem::TextSeparator { .. }
        | FrameItem::Line { .. }
        | FrameItem::Glyph { .. }
        | FrameItem::Image { .. }
        | FrameItem::Shape { .. } => false,
    })
}

#[test]
fn eval_to_module_retorna_modulo_e_sem_warnings() {
    let w = mock_world("Olá");
    let source = w.source.clone();
    let (result, warnings) = eval_to_module_with_sink(&w, &source);
    assert!(result.is_ok());
    assert!(warnings.is_empty());
}

#[test]
fn eval_to_module_ficheiro_vazio_emite_warning() {
    let w = mock_world("");
    let source = w.source.clone();
    let (result, warnings) = eval_to_module_with_sink(&w, &source);
    assert!(result.is_ok());
    assert_eq!(warnings.len(), 1, "pilot do Passo 106 emite warning para ficheiro vazio");
}

#[test]
fn compile_to_pdf_bytes_produz_pdf_valido() {
    let w = mock_world("Texto de teste");
    let source = w.source.clone();
    let (result, _warnings) = compile_to_pdf_bytes(
        &w,
        &source,
        StreamMode::Verbose,
        crate::export::PdfTags::Enabled,
    );
    let pdf = result.expect("compilação deve ter sucesso");
    assert!(!pdf.is_empty(), "bytes PDF devem existir");
    assert_eq!(&pdf[..5], b"%PDF-", "header PDF esperado");
}

#[test]
fn p1159_callback_de_page_numbering_chega_a_margem() {
    let w =
        mock_world("#set page(numbering: (current, total) => [V#current/#total])\nTexto");
    let source = w.source.clone();
    let mut timings = Timings::default();
    let (result, _) =
        compile_to_paged_document_full_error(&w, &source, false, &mut timings);
    let doc = result.expect("callback de numbering deve compilar");
    let margin = frame_items_text(&doc.pages[0].foreground);
    assert_eq!(margin, "V1/1");
}

#[test]
fn p1486_pipeline_transporta_numero_logico_realizado() {
    let w = mock_world(
        "#set page(numbering: \"1\")\n#counter(page).update(10)\nOne\n#pagebreak()\nTwo\n#pagebreak()\nThree\n#pagebreak()\nFour",
    );
    let source = w.source.clone();
    let mut timings = Timings::default();
    let (result, _) =
        compile_to_paged_document_full_error(&w, &source, false, &mut timings);
    let doc = result.expect("numeração lógica deve realizar");
    assert_eq!(
        doc.pages.iter().map(|page| page.logical_number).collect::<Vec<_>>(),
        [10, 11, 12, 13]
    );
    assert_eq!(
        doc.pages
            .iter()
            .map(|page| frame_items_text(&page.foreground))
            .collect::<Vec<_>>(),
        ["10", "11", "12", "13"],
        "a margem impressa deve usar o mesmo contador lógico do label PDF",
    );
}

#[test]
fn p1486_margem_composta_usa_numero_logico_e_total_logico() {
    let w = mock_world(
        "#set page(numbering: \"1 / 1\")\n#counter(page).update(10)\nOne\n#pagebreak()\nTwo\n#pagebreak()\nThree",
    );
    let mut timings = Timings::default();
    let doc = compile_to_paged_document_full_error(&w, &w.source, false, &mut timings)
        .0
        .expect("numeração composta deve compilar");
    assert_eq!(
        doc.pages
            .iter()
            .map(|page| frame_items_text(&page.foreground))
            .collect::<Vec<_>>(),
        ["10 / 12", "11 / 12", "12 / 12"],
    );
}

#[test]
fn p1486_callback_de_numbering_usa_fallback_fisico_quando_filtrado() {
    use typst_core::entities::document_format_options::PageRange;
    let w = mock_world(
        "#set page(numbering: (current, total) => [L#current])\nOne\n#pagebreak()\nTwo\n#pagebreak()\nThree",
    );
    let mut timings = Timings::default();
    let doc = compile_to_paged_document_full_error(&w, &w.source, false, &mut timings)
        .0
        .expect("callback deve compilar");
    let ranges = PageRanges::new(
        [1, 3]
            .into_iter()
            .map(|index| PageRange::at(std::num::NonZeroUsize::new(index).unwrap()))
            .collect(),
    );
    let pdf =
        export_pdf_with_document_id_and_tags_and_serialization_and_pages_and_standard(
            &doc,
            None,
            StreamMode::Verbose,
            PdfTags::Disabled,
            PdfSerializationMode::Compact,
            Some(ranges),
            PdfStandards::default(),
        );
    let raw = String::from_utf8_lossy(&pdf);
    assert!(raw.contains("/St 1") && raw.contains("/St 3"), "{raw}");
    assert!(!raw.contains("/St 2"), "página excluída não deve receber label");
}

#[test]
fn p1486_bundle_preserva_offset_apos_html_svg_png() {
    use typst_core::entities::compiler_features::{Feature, Features};

    let mut features = Features::empty();
    features.enable(Feature::Bundle);
    features.enable(Feature::Html);
    let run = |source: &str| {
        let world = mock_world(source);
        compile_to_bundle_files(
            &world,
            &world.source,
            BundleCompileOptions {
                full_error: false,
                features,
                document_id: None,
                pdf_stream_mode: StreamMode::Verbose,
                pdf_serialization: PdfSerializationMode::Compact,
                pdf_tagged_override: None,
                pdf_pages_override: None,
                pdf_standard_override: None,
                png_ppi_override: None,
                html_serialization: crate::export::HtmlSerializationMode::Crystalline,
                pretty_override: None,
            },
        )
        .0
        .expect("bundle P1486 deve compilar")
    };
    for bridge in [
        "#document(\"bridge.html\")[Bridge]",
        "#document(\"bridge.svg\")[Bridge]\n#document(\"bridge.png\")[Bridge]",
    ] {
        let source = format!(
            "#document(\"first.pdf\")[#set page(numbering: \"1\")\n\
             #counter(page).update(10) First #pagebreak() Second]\n\
             {bridge}\n\
             #document(\"last.pdf\")[#set page(numbering: \"1\")\n\
             #set format.pdf(pages: (1, 3), tagged: false)\n\
             #counter(page).update(20) Third #pagebreak() Fourth #pagebreak() Fifth]"
        );
        let files = run(&source);
        let first = files.iter().find(|file| file.path == "first.pdf").unwrap();
        let last = files.iter().find(|file| file.path == "last.pdf").unwrap();
        let first_pdf = String::from_utf8_lossy(&first.data);
        let last_pdf = String::from_utf8_lossy(&last.data);
        assert!(
            first_pdf.contains("/St 10") && first_pdf.contains("/St 11"),
            "{bridge}: primeira numeração"
        );
        assert!(last_pdf.contains("/Count 2"), "{bridge}: seleção da segunda saída");
        assert!(
            last_pdf.contains("/St 20") && last_pdf.contains("/St 22"),
            "{bridge}: offset lógico"
        );
        assert!(!last_pdf.contains("/St 21"), "{bridge}: página excluída não rotulada");
        let first_stream = crate::export::extract_page_content_streams_text(&first.data);
        let last_stream = crate::export::extract_page_content_streams_text(&last.data);
        assert!(
            first_stream.contains("(10) Tj") && first_stream.contains("(11) Tj"),
            "{bridge}: margem do primeiro PDF: {first_stream}"
        );
        assert!(
            last_stream.contains("(20) Tj") && last_stream.contains("(22) Tj"),
            "{bridge}: margem do segundo PDF: {last_stream}"
        );
        assert!(
            !last_stream.contains("(21) Tj"),
            "{bridge}: página excluída não deve ser desenhada: {last_stream}"
        );
    }
}

#[test]
fn p1486_bundle_margem_callback_usa_vista_local_realizada() {
    use typst_core::entities::compiler_features::{Feature, Features};

    let mut features = Features::empty();
    features.enable(Feature::Bundle);
    let world = mock_world(
        "#document(\"first.pdf\")[#set page(numbering: (current, total) => [V#current])\n\
         #counter(page).update(10) First #pagebreak() Second]\n\
         #document(\"last.pdf\")[#set page(numbering: (current, total) => [V#current])\n\
         #counter(page).update(20) Third #pagebreak() Fourth]",
    );
    let files = compile_to_bundle_files(
        &world,
        &world.source,
        BundleCompileOptions {
            full_error: false,
            features,
            document_id: None,
            pdf_stream_mode: StreamMode::Verbose,
            pdf_serialization: PdfSerializationMode::Compact,
            pdf_tagged_override: None,
            pdf_pages_override: None,
            pdf_standard_override: None,
            png_ppi_override: None,
            html_serialization: crate::export::HtmlSerializationMode::Crystalline,
            pretty_override: None,
        },
    )
    .0
    .expect("bundle com callback deve compilar");
    let last = files.iter().find(|file| file.path == "last.pdf").unwrap();
    let stream = crate::export::extract_page_content_streams_text(&last.data);
    assert!(stream.contains("(20) Tj") && stream.contains("(21) Tj"), "{stream}");
}

#[test]
fn p1159_location_page_numbering_ve_func_no_relayout() {
    let w = mock_world(
        "#set page(numbering: (current, total) => [V#current/#total])\n#context repr(here().page-numbering())",
    );
    let source = w.source.clone();
    let mut timings = Timings::default();
    let (result, _) =
        compile_to_paged_document_full_error(&w, &source, false, &mut timings);
    let doc = result.expect("page-numbering contextual deve compilar");
    assert!(
        matches!(
            &doc.pages[0].numbering,
            Some(typst_core::entities::numbering::Numbering::Func(_))
        ),
        "a página deve preservar o callback de numbering: {:?}",
        doc.pages[0].numbering
    );
    let text = doc.pages[0].plain_text();
    assert!(text.contains("=>"), "repr contextual inesperada: {text:?}");
}

#[test]
fn p1159_callback_usa_counter_logico_de_pagina() {
    let w = mock_world(
        "#counter(page).update(7)\n#set page(numbering: (current, total) => [N#current/#total])\nA#pagebreak()B#pagebreak()C",
    );
    let source = w.source.clone();
    let mut timings = Timings::default();
    let (result, _) =
        compile_to_paged_document_full_error(&w, &source, false, &mut timings);
    let doc = result.expect("counter(page) lógico deve compilar");
    let margins = doc
        .pages
        .iter()
        .map(|page| frame_items_text(&page.foreground))
        .collect::<Vec<_>>();
    assert_eq!(margins, ["N7/9", "N8/9", "N9/9"]);
}

#[test]
fn p1159_referencia_usa_vista_unaria_com_footer_explicito() {
    let w = mock_world(
        "#counter(page).update(7)\n#set page(numbering: (..nums) => [R#nums.pos().first()], footer: [EXPL])\n= Alvo <alvo>\n#pagebreak()\n#ref(<alvo>, form: \"page\")",
    );
    let source = w.source.clone();
    let mut timings = Timings::default();
    let (result, _) =
        compile_to_paged_document_full_error(&w, &source, false, &mut timings);
    let doc = result.expect("referência funcional deve compilar");
    assert!(doc.pages.iter().any(|page| page.plain_text().contains("R7")));
    assert!(doc.pages.iter().all(|page| {
        !page.foreground.iter().any(|item| match item {
            FrameItem::Text { text, .. } | FrameItem::TextShaped { text, .. } => {
                text.starts_with('R')
            }
            _ => false,
        })
    }));
}

#[test]
fn p1160_aridade_visivel_preserva_erros_do_callback() {
    for (source_text, expected) in [
        ("#set page(numbering: x => [#x])\nX", "unexpected argument"),
        ("#set page(numbering: (a, b, c) => [#a/#b/#c])\nX", "missing argument: c"),
    ] {
        let w = mock_world(source_text);
        let source = w.source.clone();
        let mut timings = Timings::default();
        let (result, _) =
            compile_to_paged_document_full_error(&w, &source, false, &mut timings);
        let errors = result.expect_err("aridade inválida deve falhar");
        assert!(errors.iter().any(|error| error.message.contains(expected)));
    }
}

#[test]
fn p1160_referencia_binaria_falha_sem_total() {
    let w = mock_world(
        "#set page(numbering: (current, total) => [#current/#total])\n= A <a>\n#pagebreak()\n#ref(<a>, form: \"page\")",
    );
    let source = w.source.clone();
    let mut timings = Timings::default();
    let (result, _) =
        compile_to_paged_document_full_error(&w, &source, false, &mut timings);
    let errors = result.expect_err("referência passa somente current");
    assert!(errors
        .iter()
        .any(|error| error.message.contains("missing argument: total")));
}

#[test]
fn p1160_retorno_int_e_none_preservam_morfologia() {
    let w = mock_world(
        "#set page(numbering: (current, total) => 42)\nA\n#pagebreak()\n#set page(numbering: (current, total) => none)\nB",
    );
    let source = w.source.clone();
    let mut timings = Timings::default();
    let (result, _) =
        compile_to_paged_document_full_error(&w, &source, false, &mut timings);
    let doc = result.expect("retornos heterogéneos devem compilar");
    let margins = doc
        .pages
        .iter()
        .map(|page| frame_items_text(&page.foreground))
        .collect::<Vec<_>>();
    assert_eq!(margins, ["42", ""]);
}

#[test]
fn p1160_page_run_restaura_numbering_externo() {
    let w = mock_world(
        "#set page(numbering: \"I\")\n#page(numbering: (current, total) => [X#current/#total])[A#pagebreak()B]\n#pagebreak()\nC",
    );
    let source = w.source.clone();
    let mut timings = Timings::default();
    let (result, _) =
        compile_to_paged_document_full_error(&w, &source, false, &mut timings);
    let doc = result.expect("page-run funcional deve compilar");
    let margins = doc
        .pages
        .iter()
        .map(|page| frame_items_text(&page.foreground))
        .collect::<Vec<_>>();
    assert_eq!(margins, ["X1/3", "X2/3", "III"]);
}

#[test]
fn p1160_callback_preserva_markup_forte() {
    let w = mock_world("#set page(numbering: (a, b) => [*B*])\nX");
    let source = w.source.clone();
    let mut timings = Timings::default();
    let (result, _) =
        compile_to_paged_document_full_error(&w, &source, false, &mut timings);
    let doc = result.expect("markup forte deve compilar");
    assert!(contains_bold_text(&doc.pages[0].foreground, "B"));
}

// ── Passo 140B: dispatch font-aware ───────────────────────────────

use ecow::EcoString;
use typst_core::entities::font_book::{
    Coverage, FontFlags, FontInfo, FontStretch, FontStyle, FontWeight,
};
use typst_core::entities::font_list::{FontFamily, FontList};
use typst_core::entities::layout_types::{
    FrameItem, PagedDocument, Point, Pt, TextStyle,
};

fn text_item_with_font(font: Option<FontList>) -> FrameItem {
    let mut style = TextStyle::regular(Pt(12.0));
    style.font = font;
    FrameItem::Text {
        pos: Point::ZERO,
        text: "X".into(),
        style,
        bidi: None,
    }
}

fn page_with(items: Vec<FrameItem>) -> Page {
    Page {
        width: 100.0,
        height: 100.0,
        numbering: None,
        logical_number: 1,
        supplement: typst_core::entities::content::Content::Empty,
        bleed: Default::default(),
        fill: Default::default(),
        background: vec![],
        foreground: vec![],
        items,
    }
}

fn font_list(name: &str) -> FontList {
    FontList::single(EcoString::from(name))
}

#[test]
fn first_font_from_doc_documento_vazio_devolve_none() {
    let doc = PagedDocument::new(vec![]);
    assert!(first_font_from_doc(&doc).is_none());
}

#[test]
fn first_font_from_doc_sem_font_devolve_none() {
    let doc = PagedDocument::new(vec![page_with(vec![text_item_with_font(None)])]);
    assert!(first_font_from_doc(&doc).is_none());
}

#[test]
fn first_font_from_doc_com_font_primeira_vence() {
    // Dois items na mesma página com fonts diferentes.
    let doc = PagedDocument::new(vec![page_with(vec![
        text_item_with_font(Some(font_list("Primeira"))),
        text_item_with_font(Some(font_list("Segunda"))),
    ])]);
    let fl = first_font_from_doc(&doc).expect("deve resolver");
    assert_eq!(fl.as_slice()[0].name.as_str(), Some("primeira"));
}

#[test]
fn first_font_from_doc_font_em_pagina_segunda_encontrada() {
    // Página 1 sem, font, página 2 com font.
    let doc = PagedDocument::new(vec![
        page_with(vec![text_item_with_font(None)]),
        page_with(vec![text_item_with_font(Some(font_list("Inria Serif")))]),
    ]);
    let fl = first_font_from_doc(&doc).expect("deve encontrar na segunda");
    assert_eq!(fl.as_slice()[0].name.as_str(), Some("inria serif"));
}

// ── resolve_font ──────────────────────────────────────────────────

/// MockWorld com `FontBook` e bytes de font injectados por índice.
struct FontMockWorld {
    library: Library,
    book: FontBook,
    fonts: Vec<Option<Font>>,
}
impl World for FontMockWorld {
    fn library(&self) -> &Library {
        &self.library
    }
    fn book(&self) -> &FontBook {
        &self.book
    }
    fn main(&self) -> FileId {
        FileId::from_raw(NonZeroU16::new(1).unwrap())
    }
    fn source(&self, _: FileId) -> FileResult<Source> {
        Err(FileError::NotFound)
    }
    fn file(&self, _: FileId) -> FileResult<Bytes> {
        Err(FileError::NotFound)
    }
    fn font(&self, i: usize) -> Option<Font> {
        self.fonts.get(i).cloned().flatten()
    }
    fn today(
        &self,
        _: Option<typst_core::entities::duration::Duration>,
    ) -> Option<Datetime> {
        None
    }
}

fn font_info(family: &str) -> FontInfo {
    FontInfo {
        family: family.into(),
        variant: FontVariant {
            style: FontStyle::Normal,
            weight: FontWeight::REGULAR,
            stretch: FontStretch::NORMAL,
        },
        flags: FontFlags::default(),
        coverage: Coverage::default(),
    }
}

#[test]
fn resolve_font_match_primeiro_devolve_bytes() {
    let mut book = FontBook::new();
    book.push(font_info("Inria Serif"));
    let bytes_esperados = vec![0x11, 0x22, 0x33];
    let world = FontMockWorld {
        library: Library::new(),
        book,
        fonts: vec![Some(Font::from_data(bytes_esperados.clone()))],
    };
    let fl = font_list("Inria Serif");
    let got = resolve_font(&fl, &FontVariant::default(), world.book(), &world)
        .expect("deve resolver");
    assert_eq!(got, bytes_esperados);
}

#[test]
fn resolve_font_nao_match_devolve_none() {
    let mut book = FontBook::new();
    book.push(font_info("Outra"));
    let world = FontMockWorld {
        library: Library::new(),
        book,
        fonts: vec![Some(Font::from_data(vec![0]))],
    };
    let fl = font_list("Não Existe");
    assert!(resolve_font(&fl, &FontVariant::default(), world.book(), &world).is_none());
}

#[test]
fn resolve_font_font_book_vazio_devolve_none() {
    let world = FontMockWorld {
        library: Library::new(),
        book: FontBook::new(),
        fonts: vec![],
    };
    let fl = font_list("Qualquer");
    assert!(resolve_font(&fl, &FontVariant::default(), world.book(), &world).is_none());
}

// ── Passo 141: array fallback chain ───────────────────────────────

fn font_list_multi(names: &[&str]) -> FontList {
    let families = names.iter().map(|n| FontFamily::new(EcoString::from(*n))).collect();
    FontList::new(families).expect("lista não-vazia")
}

#[test]
fn resolve_font_lista_match_indice_0() {
    let mut book = FontBook::new();
    book.push(font_info("A"));
    let bytes_a = vec![0xAA, 0xAA];
    let world = FontMockWorld {
        library: Library::new(),
        book,
        fonts: vec![Some(Font::from_data(bytes_a.clone()))],
    };
    let fl = font_list_multi(&["A", "B", "C"]);
    let got = resolve_font(&fl, &FontVariant::default(), world.book(), &world)
        .expect("deve resolver");
    assert_eq!(got, bytes_a, "primeira família vence quando existe");
}

#[test]
fn resolve_font_lista_match_indice_1() {
    // FontBook só tem "B"; "X" não resolve, "B" sim.
    let mut book = FontBook::new();
    book.push(font_info("B"));
    let bytes_b = vec![0xBB, 0xBB];
    let world = FontMockWorld {
        library: Library::new(),
        book,
        fonts: vec![Some(Font::from_data(bytes_b.clone()))],
    };
    let fl = font_list_multi(&["X", "B", "C"]);
    let got = resolve_font(&fl, &FontVariant::default(), world.book(), &world)
        .expect("deve resolver via B");
    assert_eq!(got, bytes_b, "segunda família vence quando primeira falha");
}

#[test]
fn resolve_font_lista_match_indice_2() {
    // FontBook só tem "C"; "X" e "Y" não resolvem.
    let mut book = FontBook::new();
    book.push(font_info("C"));
    let bytes_c = vec![0xCC, 0xCC];
    let world = FontMockWorld {
        library: Library::new(),
        book,
        fonts: vec![Some(Font::from_data(bytes_c.clone()))],
    };
    let fl = font_list_multi(&["X", "Y", "C"]);
    let got = resolve_font(&fl, &FontVariant::default(), world.book(), &world)
        .expect("deve resolver via C");
    assert_eq!(got, bytes_c, "terceira família vence quando duas primeiras falham");
}

#[test]
fn resolve_font_lista_sem_match_devolve_none() {
    // FontBook só tem "Outra"; nenhuma família da lista resolve.
    let mut book = FontBook::new();
    book.push(font_info("Outra"));
    let world = FontMockWorld {
        library: Library::new(),
        book,
        fonts: vec![Some(Font::from_data(vec![0xFF]))],
    };
    let fl = font_list_multi(&["X", "Y", "Z"]);
    assert!(
        resolve_font(&fl, &FontVariant::default(), world.book(), &world).is_none(),
        "nenhuma família resolve → fallback Helvetica via None"
    );
}

// ── Passo 146: multi-font per document ────────────────────────────

#[test]
fn collect_fonts_from_doc_documento_vazio_devolve_vazio() {
    let doc = PagedDocument::new(vec![]);
    let world = mock_world("");
    assert!(collect_fonts_from_doc(&doc, &world).is_empty());
}

#[test]
fn collect_fonts_from_doc_uma_font_devolve_unitario() {
    let doc = PagedDocument::new(vec![page_with(vec![text_item_with_font(Some(
        font_list("Inria"),
    ))])]);
    let world = mock_world("");
    let collected = collect_fonts_from_doc(&doc, &world);
    assert_eq!(collected.len(), 1);
    assert_eq!(collected[0].0.as_slice()[0].name.as_str(), Some("inria"));
}

#[test]
fn collect_fonts_from_doc_duas_distintas_devolve_par_em_ordem() {
    let doc = PagedDocument::new(vec![page_with(vec![
        text_item_with_font(Some(font_list("Primeira"))),
        text_item_with_font(Some(font_list("Segunda"))),
    ])]);
    let world = mock_world("");
    let collected = collect_fonts_from_doc(&doc, &world);
    assert_eq!(collected.len(), 2);
    assert_eq!(collected[0].0.as_slice()[0].name.as_str(), Some("primeira"));
    assert_eq!(collected[1].0.as_slice()[0].name.as_str(), Some("segunda"));
}

#[test]
fn collect_fonts_from_doc_duas_iguais_dispersas_dedup() {
    // "A" e "B" intercalados em duas páginas: dedup deve produzir
    // [A, B] em ordem de primeira ocorrência.
    let doc = PagedDocument::new(vec![
        page_with(vec![
            text_item_with_font(Some(font_list("A"))),
            text_item_with_font(Some(font_list("B"))),
        ]),
        page_with(vec![
            text_item_with_font(Some(font_list("A"))),
            text_item_with_font(Some(font_list("B"))),
        ]),
    ]);
    let world = mock_world("");
    let collected = collect_fonts_from_doc(&doc, &world);
    assert_eq!(
        collected.len(),
        2,
        "dedup estrutural: A e B aparecem cada um uma vez no resultado"
    );
    assert_eq!(collected[0].0.as_slice()[0].name.as_str(), Some("a"));
    assert_eq!(collected[1].0.as_slice()[0].name.as_str(), Some("b"));
}

// resolve_fonts (plural)

#[test]
fn resolve_fonts_todos_resolvem() {
    let mut book = FontBook::new();
    book.push(font_info("A"));
    book.push(font_info("B"));
    let world = FontMockWorld {
        library: Library::new(),
        book,
        fonts: vec![Some(Font::from_data(vec![0xAA])), Some(Font::from_data(vec![0xBB]))],
    };
    let inputs = vec![
        (font_list("A"), FontVariant::default(), FontVariations::default()),
        (font_list("B"), FontVariant::default(), FontVariations::default()),
    ];
    let out = resolve_fonts(&inputs, &world);
    assert_eq!(out.len(), 2);
    assert_eq!(out[0].1, vec![0xAA]);
    assert_eq!(out[1].1, vec![0xBB]);
}

#[test]
fn resolve_fonts_alguns_nao_resolvem_filtrados() {
    // FontBook só tem "A"; "B" não resolve → filtrado silenciosamente.
    let mut book = FontBook::new();
    book.push(font_info("A"));
    let world = FontMockWorld {
        library: Library::new(),
        book,
        fonts: vec![Some(Font::from_data(vec![0xAA]))],
    };
    let inputs = vec![
        (font_list("A"), FontVariant::default(), FontVariations::default()),
        (font_list("B"), FontVariant::default(), FontVariations::default()),
    ];
    let out = resolve_fonts(&inputs, &world);
    assert_eq!(out.len(), 1, "B silenciosamente filtrado");
    assert_eq!(out[0].1, vec![0xAA]);
}

#[test]
fn resolve_fonts_nenhum_resolve_devolve_vazio() {
    let mut book = FontBook::new();
    book.push(font_info("Outra"));
    let world = FontMockWorld {
        library: Library::new(),
        book,
        fonts: vec![Some(Font::from_data(vec![0]))],
    };
    let inputs = vec![
        (font_list("X"), FontVariant::default(), FontVariations::default()),
        (font_list("Y"), FontVariant::default(), FontVariations::default()),
    ];
    assert!(resolve_fonts(&inputs, &world).is_empty());
}

// ── Passo 407: regex keys em FontList (DEBT-52) ─────────────────────

use typst_core::entities::regex::Regex;

fn font_list_regex(pattern: &str) -> FontList {
    FontList::new(vec![FontFamily::new_regex(Regex::new(pattern).unwrap(), vec![])])
        .expect("lista não-vazia")
}

#[test]
fn resolve_font_regex_key_match() {
    let mut book = FontBook::new();
    book.push(font_info("Name Bold"));
    let bytes = vec![0xDD, 0xDD];
    let world = FontMockWorld {
        library: Library::new(),
        book,
        fonts: vec![Some(Font::from_data(bytes.clone()))],
    };
    let fl = font_list_regex("Name.*");
    let got = resolve_font(&fl, &FontVariant::default(), world.book(), &world)
        .expect("deve resolver via regex");
    assert_eq!(got, bytes);
}

#[test]
fn resolve_font_regex_key_no_match() {
    let mut book = FontBook::new();
    book.push(font_info("Other"));
    let world = FontMockWorld {
        library: Library::new(),
        book,
        fonts: vec![Some(Font::from_data(vec![0xEE]))],
    };
    let fl = font_list_regex("Name.*");
    assert!(resolve_font(&fl, &FontVariant::default(), world.book(), &world).is_none());
}

#[test]
fn resolve_font_literal_still_works_after_p407() {
    let mut book = FontBook::new();
    book.push(font_info("Inria Serif"));
    let bytes = vec![0x11, 0x22];
    let world = FontMockWorld {
        library: Library::new(),
        book,
        fonts: vec![Some(Font::from_data(bytes.clone()))],
    };
    let fl = font_list("Inria Serif");
    let got = resolve_font(&fl, &FontVariant::default(), world.book(), &world)
        .expect("literal continua a resolver");
    assert_eq!(got, bytes);
}

// ── P711: collect_context_blocks acumula StyleChain da posição ────

use typst_core::entities::func::Func;
use typst_core::entities::style::{Style, Styles};
use typst_core::entities::value::Value;

fn dummy_context_block(id: u64) -> Content {
    let closure = Func::native("dummy", |_ctx, _args, _world, _file| Ok(Value::None));
    Content::ContextBlock(Arc::new(ContextBlockElem { id, closure }))
}

#[test]
fn collect_context_blocks_acumula_size_de_set_ancestral() {
    let styles = Styles::from_iter([Style::Size(
        typst_core::entities::layout_types::Length::pt(20.0),
    )]);
    let content = Content::styled(Box::new(dummy_context_block(1)), styles);
    let blocks = collect_context_blocks(&content, &StyleChain::default_chain());
    let (_, chain) = blocks.get(&1).expect("bloco 1 deve ter sido colectado");
    assert_eq!(
        chain.size(),
        20.0,
        "P711: a cadeia colectada tem de reflectir o #set ancestral, não o default"
    );
}

#[test]
fn collect_context_blocks_sem_set_mantem_default() {
    let content = dummy_context_block(2);
    let blocks = collect_context_blocks(&content, &StyleChain::default_chain());
    let (_, chain) = blocks.get(&2).expect("bloco 2 deve ter sido colectado");
    assert_eq!(
        chain.size(),
        11.0,
        "sem #set ancestral, o tamanho por defeito (11pt) fica inalterado"
    );
}

#[test]
fn collect_context_blocks_sets_aninhados_o_mais_interno_vence() {
    // #set text(size: 20pt) [ #set text(size: 30pt) [ context ] ]
    let inner_styles = Styles::from_iter([Style::Size(
        typst_core::entities::layout_types::Length::pt(30.0),
    )]);
    let inner = Content::styled(Box::new(dummy_context_block(3)), inner_styles);
    let outer_styles = Styles::from_iter([Style::Size(
        typst_core::entities::layout_types::Length::pt(20.0),
    )]);
    let content = Content::styled(Box::new(inner), outer_styles);
    let blocks = collect_context_blocks(&content, &StyleChain::default_chain());
    let (_, chain) = blocks.get(&3).expect("bloco 3 deve ter sido colectado");
    assert_eq!(
        chain.size(),
        30.0,
        "o #set mais interno (mais próximo do bloco) tem de vencer"
    );
}

#[test]
fn collect_context_blocks_multiplos_blocos_em_sequence_cadeias_independentes() {
    let a = Content::styled(
        Box::new(dummy_context_block(4)),
        Styles::from_iter([Style::Size(typst_core::entities::layout_types::Length::pt(
            14.0,
        ))]),
    );
    let b = dummy_context_block(5);
    let content = Content::sequence(vec![a, b]);
    let blocks = collect_context_blocks(&content, &StyleChain::default_chain());
    assert_eq!(blocks.get(&4).unwrap().1.size(), 14.0);
    assert_eq!(
        blocks.get(&5).unwrap().1.size(),
        11.0,
        "bloco irmão fora do Styled não deve herdar o #set do outro ramo"
    );
}

#[test]
fn collect_context_blocks_table_cell_header_footer_caption_keep_visual_styles() {
    use typst_core::entities::layout_types::TrackSizing;

    let inner = Content::styled(
        Box::new(dummy_context_block(41)),
        Styles::from_iter([Style::Size(typst_core::entities::layout_types::Length::pt(
            18.0,
        ))]),
    );
    let cell = Content::table_cell(inner, None, None, None, None);
    let mut table = match Content::table(vec![TrackSizing::Auto], vec![], vec![cell]) {
        Content::Table(table) => table.as_ref().clone(),
        _ => unreachable!(),
    };
    table.header = Some(dummy_context_block(43));
    table.footer = Some(dummy_context_block(44));
    table.caption = Some(dummy_context_block(42));
    let wrapped = Content::styled(
        Box::new(Content::Table(Arc::new(table))),
        Styles::from_iter([Style::Size(typst_core::entities::layout_types::Length::pt(
            14.0,
        ))]),
    );
    let blocks = collect_context_blocks(&wrapped, &StyleChain::default_chain());
    assert_eq!(blocks.len(), 4);
    assert_eq!(blocks.get(&41).unwrap().1.size(), 18.0);
    assert_eq!(blocks.get(&42).unwrap().1.size(), 14.0);
    assert_eq!(blocks.get(&43).unwrap().1.size(), 14.0);
    assert_eq!(blocks.get(&44).unwrap().1.size(), 14.0);
}

#[test]
fn collect_context_blocks_reused_id_in_table_keeps_later_visual_occurrence() {
    use typst_core::entities::layout_types::TrackSizing;

    let cell = |size| {
        Content::table_cell(
            Content::styled(
                Box::new(dummy_context_block(51)),
                Styles::from_iter([Style::Size(
                    typst_core::entities::layout_types::Length::pt(size),
                )]),
            ),
            None,
            None,
            None,
            None,
        )
    };
    let table = Content::table(
        vec![TrackSizing::Auto, TrackSizing::Auto],
        vec![],
        vec![cell(14.0), cell(20.0)],
    );
    let blocks = collect_context_blocks(&table, &StyleChain::default_chain());
    assert_eq!(blocks.len(), 1);
    assert_eq!(blocks.get(&51).unwrap().1.size(), 20.0);
}

// ── P1467 (RED — Agente B) — resolução documental/CLI de
// `format.pdf.tagged`/`format.pdf.pretty` ao nível de integração de
// pipeline. A entry point pública sugerida pelo L0
// (`infra/pipeline.md` §P1467 "API e resolução") ainda não existe
// neste baseline — a suite inteira falha por erro de compilação até
// `compile_to_pdf_bytes_full_error_and_document_id_with_features_and_tagged_and_pretty_override`
// ser materializada (Agente A, em paralelo).

fn has_tagged_pdf_morphology(pdf: &[u8]) -> bool {
    let raw = String::from_utf8_lossy(pdf);
    raw.contains("/StructTreeRoot") && raw.contains("/MarkInfo")
}

fn newline_count(pdf: &[u8]) -> usize {
    pdf.iter().filter(|&&byte| byte == b'\n').count()
}

fn compile_pdf_with_overrides(
    source_text: &str,
    pdf_tagged_override: Option<bool>,
    pretty_override: Option<bool>,
) -> Vec<u8> {
    let world = mock_world(source_text);
    let (result, _warnings) =
        compile_to_pdf_bytes_full_error_and_document_id_with_features_and_tagged_and_pretty_override(
            &world,
            &world.source,
            false,
            None,
            StreamMode::Verbose,
            pdf_tagged_override,
            typst_core::entities::compiler_features::Features::default(),
            pretty_override,
        );
    result.expect("compilação PDF P1467 deve ter sucesso")
}

#[test]
fn p1467_pipeline_absence_of_both_resolves_document_auto_to_tagged() {
    let pdf = compile_pdf_with_overrides("Hello", None, None);
    assert!(
        has_tagged_pdf_morphology(&pdf),
        "auto sem seleção de páginas deve resolver para tagueado"
    );
}

#[test]
fn p1467_pipeline_document_tagged_false_untags_without_override() {
    let pdf =
        compile_pdf_with_overrides("#set format.pdf(tagged: false)\nHello", None, None);
    assert!(!has_tagged_pdf_morphology(&pdf));
}

#[test]
fn p1467_pipeline_document_tagged_true_is_explicit_and_tags() {
    let pdf =
        compile_pdf_with_overrides("#set format.pdf(tagged: true)\nHello", None, None);
    assert!(has_tagged_pdf_morphology(&pdf));
}

#[test]
fn p1467_pipeline_override_false_wins_over_document_tagged_true() {
    let pdf = compile_pdf_with_overrides(
        "#set format.pdf(tagged: true)\nHello",
        Some(false),
        None,
    );
    assert!(!has_tagged_pdf_morphology(&pdf));
}

#[test]
fn p1467_pipeline_override_true_wins_over_document_tagged_false() {
    let pdf = compile_pdf_with_overrides(
        "#set format.pdf(tagged: false)\nHello",
        Some(true),
        None,
    );
    assert!(has_tagged_pdf_morphology(&pdf));
}

#[test]
fn p1467_pipeline_pretty_override_wins_over_document_pretty_false() {
    let compact =
        compile_pdf_with_overrides("#set format.pdf(pretty: false)\nHello", None, None);
    let pretty = compile_pdf_with_overrides(
        "#set format.pdf(pretty: false)\nHello",
        None,
        Some(true),
    );
    assert!(
        newline_count(&pretty) > newline_count(&compact),
        "--pretty deve vencer o documento e produzir mais newlines estruturais"
    );
}

#[test]
fn p1467_pipeline_pretty_override_false_wins_over_document_pretty_true() {
    let baseline = compile_pdf_with_overrides("Hello", None, None);
    let overridden = compile_pdf_with_overrides(
        "#set format.pdf(pretty: true)\nHello",
        None,
        Some(false),
    );
    assert_eq!(
        newline_count(&overridden),
        newline_count(&baseline),
        "override explícito false deve produzir a mesma forma compacta do baseline"
    );
}

#[test]
fn p1467_pipeline_document_pretty_true_without_override() {
    let compact = compile_pdf_with_overrides("Hello", None, None);
    let pretty =
        compile_pdf_with_overrides("#set format.pdf(pretty: true)\nHello", None, None);
    assert!(newline_count(&pretty) > newline_count(&compact));
}

#[test]
fn p1467_pipeline_absence_of_both_flags_is_compact_by_default() {
    let baseline = compile_pdf_with_overrides("Hello", None, None);
    let explicit_false =
        compile_pdf_with_overrides("#set format.pdf(pretty: false)\nHello", None, None);
    assert_eq!(newline_count(&baseline), newline_count(&explicit_false));
}

#[test]
fn p1467_pipeline_tagged_and_pretty_combine_independently() {
    let untagged_pretty = compile_pdf_with_overrides(
        "#set format.pdf(tagged: false)\n#set format.pdf(pretty: true)\nHello",
        None,
        None,
    );
    let untagged_compact =
        compile_pdf_with_overrides("#set format.pdf(tagged: false)\nHello", None, None);

    assert!(!has_tagged_pdf_morphology(&untagged_pretty));
    assert!(!has_tagged_pdf_morphology(&untagged_compact));
    assert!(newline_count(&untagged_pretty) > newline_count(&untagged_compact));
}

#[test]
fn p1467_svg_output_is_neutral_to_pdf_format_options() {
    let world_without = mock_world("#rect(width: 10pt, height: 10pt)");
    let (without, _, _) =
        compile_to_svg_string_with_timings_full_error_and_features_and_pretty_override(
            &world_without,
            &world_without.source,
            false,
            typst_core::entities::compiler_features::Features::default(),
            None,
        );

    let world_with_pdf = mock_world(
        "#set format.pdf(tagged: false, pretty: true)\n#rect(width: 10pt, height: 10pt)",
    );
    let (with_pdf_options, _, _) =
        compile_to_svg_string_with_timings_full_error_and_features_and_pretty_override(
            &world_with_pdf,
            &world_with_pdf.source,
            false,
            typst_core::entities::compiler_features::Features::default(),
            None,
        );

    assert_eq!(
        without.expect("SVG sem opções PDF deve compilar"),
        with_pdf_options.expect("SVG com #set format.pdf deve compilar e ser idêntico"),
        "P1467: format.pdf não pode influenciar o export SVG"
    );
}

// ── P1468 (RED — Agente B) — resolução de `format.pdf.pages` e
// interação com `tagged`. `resolve_pdf_pages_and_tags` (contrato de
// `infra/pipeline.md` §P1468) ainda não existe neste baseline (só
// `resolve_pdf_tags`, P1467); a nova entry point pública
// `..._tagged_and_pretty_and_pages_override` também não existe. A
// suite inteira falha por erro de compilação até ambas serem
// materializadas (Agente A, em paralelo).
use typst_core::entities::document_format_options::{PageRange, PageRanges};

fn single_page(n: usize) -> PageRanges {
    let p = std::num::NonZeroUsize::new(n).unwrap();
    PageRanges::new(vec![PageRange { start: Some(p), end: Some(p) }])
}

#[test]
fn p1468_resolve_pages_absent_matches_p1467_tagged_resolution() {
    let auto = DocumentFormatOptions::default();
    let (pages, tags, diagnostics) = resolve_pdf_pages_and_tags(None, None, &auto)
        .expect("sem pages, sem erro (comportamento P1467)");
    assert_eq!(pages, None);
    assert_eq!(tags, PdfTags::Enabled, "auto sem pages continua tagueado (P1467)");
    assert!(diagnostics.is_empty());

    let mut tagged_false = DocumentFormatOptions::default();
    tagged_false.pdf.tagged = PdfTagged::Explicit(false);
    let (pages, tags, diagnostics) =
        resolve_pdf_pages_and_tags(None, None, &tagged_false)
            .expect("sem pages, sem erro (comportamento P1467)");
    assert_eq!(pages, None);
    assert_eq!(tags, PdfTags::Disabled);
    assert!(diagnostics.is_empty());
}

#[test]
fn p1468_resolve_pages_present_with_document_tagged_explicit_true_errors() {
    let mut options = DocumentFormatOptions::default();
    options.pdf.tagged = PdfTagged::Explicit(true);
    let errors = resolve_pdf_pages_and_tags(Some(single_page(1)), None, &options)
        .expect_err("documento pede tagged:true + pages deve falhar");
    assert!(
        errors.iter().any(|e| e
            .message
            .contains("cannot enable tagged PDF and export a page range")),
        "{errors:?}"
    );
}

#[test]
fn p1468_resolve_pages_present_with_auto_tagged_warns_and_disables() {
    let options = DocumentFormatOptions::default(); // tagged == Auto
    let (pages, tags, diagnostics) =
        resolve_pdf_pages_and_tags(Some(single_page(1)), None, &options)
            .expect("auto + pages não deve ser erro, só warning");
    assert_eq!(pages, Some(single_page(1)));
    assert_eq!(tags, PdfTags::Disabled);
    assert!(
        diagnostics
            .iter()
            .any(|d| d.message.contains("exporting a page range disables PDF tagging")),
        "{diagnostics:?}"
    );
    let hints: Vec<_> = diagnostics.iter().flat_map(|d| d.hints.iter()).collect();
    assert!(
        hints
            .iter()
            .any(|h| h.contains("the resulting PDF will be inaccessible")),
        "{hints:?}"
    );
    assert!(
        hints
            .iter()
            .any(|h| h.contains("set `pdf(tagged: false)` to silence this warning")),
        "{hints:?}"
    );
}

#[test]
fn p1468_resolve_pages_present_with_document_tagged_explicit_false_is_silent() {
    let mut options = DocumentFormatOptions::default();
    options.pdf.tagged = PdfTagged::Explicit(false);
    let (pages, tags, diagnostics) =
        resolve_pdf_pages_and_tags(Some(single_page(1)), None, &options)
            .expect("tagged:false + pages não deve falhar");
    assert_eq!(pages, Some(single_page(1)));
    assert_eq!(tags, PdfTags::Disabled);
    assert!(diagnostics.is_empty(), "{diagnostics:?}");
}

#[test]
fn p1468_resolve_pages_cli_override_wins_over_document_pages() {
    let mut options = DocumentFormatOptions::default();
    options.pdf.tagged = PdfTagged::Explicit(false);
    options.pdf.pages = Some(single_page(5));
    let (pages, _tags, _diagnostics) =
        resolve_pdf_pages_and_tags(Some(single_page(1)), None, &options)
            .expect("override deve vencer, sem erro");
    assert_eq!(
        pages,
        Some(single_page(1)),
        "override CLI de pages deve vencer o documento"
    );
}

#[test]
fn p1468_resolve_pages_falls_back_to_document_then_none() {
    let mut with_document_pages = DocumentFormatOptions::default();
    with_document_pages.pdf.tagged = PdfTagged::Explicit(false);
    with_document_pages.pdf.pages = Some(single_page(3));
    let (pages, _, _) = resolve_pdf_pages_and_tags(None, None, &with_document_pages)
        .expect("documento define pages, sem override");
    assert_eq!(pages, Some(single_page(3)));

    let neither = DocumentFormatOptions::default();
    let (pages, _, _) =
        resolve_pdf_pages_and_tags(None, None, &neither).expect("nenhum define pages");
    assert_eq!(pages, None);
}

#[test]
fn p1468_resolve_pages_tagged_override_still_applies_when_pages_absent() {
    // Regressão P1467: `tagged_override` continua a funcionar quando
    // `pages` não está presente (comportamento idêntico a
    // `resolve_pdf_tags`).
    let options = DocumentFormatOptions::default();
    let (pages, tags, diagnostics) =
        resolve_pdf_pages_and_tags(None, Some(false), &options)
            .expect("override tagged sem pages não deve falhar");
    assert_eq!(pages, None);
    assert_eq!(tags, PdfTags::Disabled);
    assert!(diagnostics.is_empty());
}

#[test]
fn p1468_pipeline_new_entry_point_with_none_pages_matches_old_entry_point() {
    let world = mock_world("#set format.pdf(pretty: true)\nHello");
    let (old_result, _old_warnings) =
        compile_to_pdf_bytes_full_error_and_document_id_with_features_and_tagged_and_pretty_override(
            &world,
            &world.source,
            false,
            None,
            StreamMode::Verbose,
            None,
            typst_core::entities::compiler_features::Features::default(),
            None,
        );
    let (new_result, _new_warnings) =
        compile_to_pdf_bytes_full_error_and_document_id_with_features_and_tagged_and_pretty_and_pages_override(
            &world,
            &world.source,
            false,
            None,
            StreamMode::Verbose,
            None,
            typst_core::entities::compiler_features::Features::default(),
            None,
            None,
        );
    assert_eq!(
        old_result.expect("compilação antiga deve suceder"),
        new_result.expect("compilação nova deve suceder"),
        "P1468: entry point antiga (P1467) deve continuar byte-idêntica quando pages=None"
    );
}

#[test]
fn p1468_pipeline_pages_override_without_document_tagged_decision_disables_tags_with_warning(
) {
    let world = mock_world("Hello\n#pagebreak()\nWorld");
    let (result, warnings) =
        compile_to_pdf_bytes_full_error_and_document_id_with_features_and_tagged_and_pretty_and_pages_override(
            &world,
            &world.source,
            false,
            None,
            StreamMode::Verbose,
            None,
            typst_core::entities::compiler_features::Features::default(),
            None,
            Some(single_page(1)),
        );
    let pdf = result.expect("pages sem tagged explícito não deve falhar a compilação");
    assert!(
        !has_tagged_pdf_morphology(&pdf),
        "auto + pages deve resolver destagueado (warning, não erro)"
    );
    assert!(
        warnings
            .iter()
            .any(|w| w.message.contains("exporting a page range disables PDF tagging")),
        "{warnings:?}"
    );
}

#[test]
fn p1468_pipeline_pages_override_with_document_tagged_true_errors_end_to_end() {
    let world = mock_world("#set format.pdf(tagged: true)\nHello");
    let (result, _warnings) =
        compile_to_pdf_bytes_full_error_and_document_id_with_features_and_tagged_and_pretty_and_pages_override(
            &world,
            &world.source,
            false,
            None,
            StreamMode::Verbose,
            None,
            typst_core::entities::compiler_features::Features::default(),
            None,
            Some(single_page(1)),
        );
    let errors = result.expect_err("documento pede tagged:true + pages CLI deve falhar");
    assert!(
        errors.iter().any(|e| e
            .message
            .contains("cannot enable tagged PDF and export a page range")),
        "{errors:?}"
    );
}

// ── P1469 (RED — Agente B) — resolução de `format.pdf.standard` e
// interação com `tagged`/`pages` (contrato de linguagem, escopo
// reduzido — ver `infra/pipeline.md` P1469 e
// `entities/document_format_options.md` P1469). `PdfStandard`/
// `PdfStandards`/`PdfVersion`/`PdfArchival` (entities) e
// `resolve_pdf_pages_tags_and_standards` ainda não existem neste
// baseline (`65e68ad45dcbfd8db00aeda78d0d2fbaaadefb27`); este módulo
// falha por erro de compilação até o carrier P1469 (mesmo gate
// ADR-0127) ser materializado por outro agente em paralelo.
use typst_core::entities::document_format_options::{
    PdfArchival, PdfStandard, PdfStandards, PdfVersion,
};

fn standards(names: &[PdfStandard]) -> PdfStandards {
    PdfStandards::new(names.iter().copied()).expect("combinação de teste deve ser válida")
}

#[test]
fn p1469_resolve_standard_absent_matches_p1468_pages_and_tags_behavior() {
    // Sem `standard` (nem override, nem documento): comportamento
    // (pages, tags) idêntico a `resolve_pdf_pages_and_tags` (P1468), e
    // `standards_final == PdfStandards::default()` (versão efetiva
    // 1.7, sem archival/accessibility).
    let options = DocumentFormatOptions::default();
    let (pages, tags, standards_final, diagnostics) =
        resolve_pdf_pages_tags_and_standards(None, None, None, &options)
            .expect("sem standard, sem erro");
    assert_eq!(pages, None);
    assert_eq!(tags, PdfTags::Enabled);
    assert_eq!(standards_final, PdfStandards::default());
    assert_eq!(standards_final.version, PdfVersion::V1_7);
    assert!(diagnostics.is_empty());
}

#[test]
fn p1469_resolve_standard_non_accessible_profile_with_tagged_false_has_no_error() {
    let mut options = DocumentFormatOptions::default();
    options.pdf.tagged = PdfTagged::Explicit(false);
    let (_, tags, standards_final, diagnostics) = resolve_pdf_pages_tags_and_standards(
        None,
        None,
        Some(standards(&[PdfStandard::A2b])),
        &options,
    )
    .expect("a-2b (não acessível) + tagged:false não deve falhar");
    assert_eq!(tags, PdfTags::Disabled);
    assert_eq!(standards_final.archival, Some(PdfArchival::A2b));
    assert!(diagnostics.is_empty());
}

#[test]
fn p1469_resolve_standard_accessible_profile_with_document_tagged_explicit_false_errors_without_pages_hint(
) {
    let mut options = DocumentFormatOptions::default();
    options.pdf.tagged = PdfTagged::Explicit(false);
    let errors = resolve_pdf_pages_tags_and_standards(
        None,
        None,
        Some(standards(&[PdfStandard::A1a])),
        &options,
    )
    .expect_err("perfil acessível + tagged:false documental deve falhar");
    assert!(
        errors.iter().any(|e| e
            .message
            .contains("cannot set `pdf(tagged: false)` when exporting a")),
        "{errors:?}"
    );
    let hints: Vec<_> = errors.iter().flat_map(|e| e.hints.iter()).collect();
    assert!(
        hints.is_empty(),
        "sem `pages` ativo, não deve haver hint de pages: {hints:?}"
    );
}

#[test]
fn p1469_resolve_standard_accessible_profile_with_pages_active_errors_with_pages_hint() {
    let options = DocumentFormatOptions::default(); // tagged == Auto
    let errors = resolve_pdf_pages_tags_and_standards(
        Some(single_page(1)),
        None,
        Some(standards(&[PdfStandard::Ua1])),
        &options,
    )
    .expect_err("perfil acessível + pages ativo deve falhar");
    assert!(
        errors.iter().any(|e| e
            .message
            .contains("cannot set `pdf(tagged: false)` when exporting a")),
        "{errors:?}"
    );
    let hints: Vec<_> = errors.iter().flat_map(|e| e.hints.iter()).collect();
    assert!(
        hints
            .iter()
            .any(|h| h.contains("using `pdf(pages: ...)` implies `pdf(tagged: false)`")),
        "causa raiz foi `pages`, hint deve aparecer: {hints:?}"
    );
}

#[test]
fn p1469_resolve_standard_accessible_profile_with_tagged_true_or_no_decision_has_no_error(
) {
    // `tagged: true` explícito (documento) + perfil acessível: sem erro.
    let mut explicit_true = DocumentFormatOptions::default();
    explicit_true.pdf.tagged = PdfTagged::Explicit(true);
    let (_, tags, _, diagnostics) = resolve_pdf_pages_tags_and_standards(
        None,
        None,
        Some(standards(&[PdfStandard::A2a])),
        &explicit_true,
    )
    .expect("tagged:true explícito + perfil acessível não deve falhar");
    assert_eq!(tags, PdfTags::Enabled);
    assert!(diagnostics.is_empty());

    // Documento sem decisão (Auto) + perfil acessível + sem `pages`:
    // `tagged` resolve `Enabled` por P1467, sem conflito.
    let no_decision = DocumentFormatOptions::default();
    let (_, tags, _, diagnostics) = resolve_pdf_pages_tags_and_standards(
        None,
        None,
        Some(standards(&[PdfStandard::A2a])),
        &no_decision,
    )
    .expect("auto sem pages + perfil acessível não deve falhar");
    assert_eq!(tags, PdfTags::Enabled);
    assert!(diagnostics.is_empty());
}

#[test]
fn p1469_resolve_standard_precedence_cli_override_wins_over_document() {
    let mut options = DocumentFormatOptions::default();
    options.pdf.standard = Some(standards(&[PdfStandard::A1b]));

    let (_, _, standards_final, _) = resolve_pdf_pages_tags_and_standards(
        None,
        None,
        Some(standards(&[PdfStandard::A4])),
        &options,
    )
    .expect("a-4 sozinho não deve falhar (não acessível)");
    assert_eq!(
        standards_final.archival,
        Some(PdfArchival::A4),
        "override CLI de standard deve vencer o documento"
    );

    // Sem override: cai para o documento.
    let (_, _, standards_final, _) =
        resolve_pdf_pages_tags_and_standards(None, None, None, &options)
            .expect("documento a-1b sozinho não deve falhar");
    assert_eq!(standards_final.archival, Some(PdfArchival::A1b));

    // Nenhum dos dois: cai para o default.
    let neither = DocumentFormatOptions::default();
    let (_, _, standards_final, _) =
        resolve_pdf_pages_tags_and_standards(None, None, None, &neither)
            .expect("nenhum define standard");
    assert_eq!(standards_final, PdfStandards::default());
}

#[test]
fn p1469_resolve_standard_new_function_with_none_standard_matches_old_resolve_pdf_pages_and_tags(
) {
    // Regressão: com `standard_override = None` e documento sem
    // `pdf.standard`, o resultado (pages, tags, diagnostics) deve ser
    // idêntico ao de `resolve_pdf_pages_and_tags` (P1468), e o
    // `PdfStandards` efetivo deve ser o default.
    let options = DocumentFormatOptions::default();
    let (old_pages, old_tags, old_diagnostics) =
        resolve_pdf_pages_and_tags(Some(single_page(2)), None, &options)
            .expect("P1468 não deve falhar");
    let (new_pages, new_tags, standards_final, new_diagnostics) =
        resolve_pdf_pages_tags_and_standards(Some(single_page(2)), None, None, &options)
            .expect("P1469 com standard=None não deve falhar");

    assert_eq!(old_pages, new_pages);
    assert_eq!(old_tags, new_tags);
    assert_eq!(
        old_diagnostics.iter().map(|d| d.message.clone()).collect::<Vec<_>>(),
        new_diagnostics.iter().map(|d| d.message.clone()).collect::<Vec<_>>(),
    );
    assert_eq!(standards_final, PdfStandards::default());
}

#[test]
fn p1469_pipeline_new_entry_point_with_none_standard_matches_old_entry_point() {
    // Mesmo padrão de `p1468_pipeline_new_entry_point_with_none_pages_matches_old_entry_point`:
    // a entry point antiga (`..._and_pages_override`, P1468) deve
    // continuar byte-idêntica quando a nova entry point
    // (`..._and_pages_and_standard_override`, P1469) recebe
    // `standard_override = None`.
    let world = mock_world("#set format.pdf(pretty: true)\nHello");
    let (old_result, _old_warnings) =
        compile_to_pdf_bytes_full_error_and_document_id_with_features_and_tagged_and_pretty_and_pages_override(
            &world,
            &world.source,
            false,
            None,
            StreamMode::Verbose,
            None,
            typst_core::entities::compiler_features::Features::default(),
            None,
            None,
        );
    let (new_result, _new_warnings) =
        compile_to_pdf_bytes_full_error_and_document_id_with_features_and_tagged_and_pretty_and_pages_and_standard_override(
            &world,
            &world.source,
            false,
            None,
            StreamMode::Verbose,
            None,
            typst_core::entities::compiler_features::Features::default(),
            None,
            None,
            None,
        );
    assert_eq!(
        old_result.expect("compilação antiga deve suceder"),
        new_result.expect("compilação nova deve suceder"),
        "P1469: entry point antiga (P1468) deve continuar byte-idêntica quando standard=None"
    );
}

fn p1480_compile_with_standard(
    source: &str,
    standard: Option<PdfStandards>,
) -> Result<Vec<u8>, Vec<SourceDiagnostic>> {
    let world = mock_world(source);
    let (result, _warnings) =
        compile_to_pdf_bytes_full_error_and_document_id_with_features_and_tagged_and_pretty_and_pages_and_standard_override(
            &world,
            &world.source,
            false,
            None,
            StreamMode::Verbose,
            None,
            typst_core::entities::compiler_features::Features::default(),
            None,
            None,
            standard,
        );
    result
}

#[test]
fn bundle_ua1_requires_title_before_publishing_pdf() {
    use typst_core::entities::compiler_features::{Feature, Features};

    let run = |body: &str| {
        let source = format!("#document(\"one.pdf\")[{body}]");
        let world = mock_world(&source);
        let mut features = Features::empty();
        features.enable(Feature::Bundle);
        compile_to_bundle_files(
            &world,
            &world.source,
            BundleCompileOptions {
                full_error: false,
                features,
                document_id: None,
                pdf_stream_mode: StreamMode::Verbose,
                pdf_serialization: PdfSerializationMode::Compact,
                pdf_tagged_override: None,
                pdf_pages_override: None,
                pdf_standard_override: None,
                png_ppi_override: None,
                html_serialization: crate::export::HtmlSerializationMode::Crystalline,
                pretty_override: None,
            },
        )
        .0
    };
    let missing = match run("#set format.pdf(standard: \"ua-1\")\nHello") {
        Err(errors) => errors,
        Ok(_) => panic!("PDF/UA-1 sem título deve falhar no bundle"),
    };
    assert_eq!(missing.len(), 1);
    assert_eq!(missing[0].message, "PDF/UA-1 error: missing document title");
    assert_eq!(missing[0].hints, vec!["set the title with `set document(title: [...])`"]);

    let titled = {
        let source = "#document(\"one.pdf\", title: [Okay])[#set format.pdf(standard: \"ua-1\")\nHello]";
        let world = mock_world(source);
        let mut features = Features::empty();
        features.enable(Feature::Bundle);
        compile_to_bundle_files(
            &world,
            &world.source,
            BundleCompileOptions {
                full_error: false,
                features,
                document_id: None,
                pdf_stream_mode: StreamMode::Verbose,
                pdf_serialization: PdfSerializationMode::Compact,
                pdf_tagged_override: None,
                pdf_pages_override: None,
                pdf_standard_override: None,
                png_ppi_override: None,
                html_serialization: crate::export::HtmlSerializationMode::Crystalline,
                pretty_override: None,
            },
        )
        .0
        .expect("título válido deve permitir o PDF/UA-1 no bundle")
    };
    assert_eq!(titled.len(), 1);
    assert_eq!(titled[0].path, "one.pdf");
    let titled_pdf = String::from_utf8_lossy(&titled[0].data);
    assert!(titled_pdf.contains("<pdfuaid:part>1</pdfuaid:part>"));
    assert!(titled_pdf.contains("<dc:title>"));

    assert!(run("Hello").is_ok(), "sem UA-1 o título continua opcional");

    let two = concat!(
        "#document(\"first.pdf\", title: [Okay])[#set format.pdf(standard: \"ua-1\")\nFirst]\n",
        "#document(\"second.pdf\")[#set format.pdf(standard: \"ua-1\")\nSecond]",
    );
    let world = mock_world(two);
    let mut features = Features::empty();
    features.enable(Feature::Bundle);
    let (result, _, _) = compile_to_bundle_files(
        &world,
        &world.source,
        BundleCompileOptions {
            full_error: false,
            features,
            document_id: None,
            pdf_stream_mode: StreamMode::Verbose,
            pdf_serialization: PdfSerializationMode::Compact,
            pdf_tagged_override: None,
            pdf_pages_override: None,
            pdf_standard_override: None,
            png_ppi_override: None,
            html_serialization: crate::export::HtmlSerializationMode::Crystalline,
            pretty_override: None,
        },
    );
    let errors = match result {
        Err(errors) => errors,
        Ok(_) => panic!("o segundo documento não pode herdar o primeiro título"),
    };
    assert_eq!(errors[0].message, "PDF/UA-1 error: missing document title");
}

#[test]
fn p1480_ua1_sem_titulo_retorna_diagnostico_exato() {
    let errors =
        p1480_compile_with_standard("Hello", Some(standards(&[PdfStandard::Ua1])))
            .expect_err("ua-1 sem document(title:) deve falhar");

    assert_eq!(errors.len(), 1, "esperado um único diagnóstico: {errors:?}");
    assert_eq!(errors[0].message, "PDF/UA-1 error: missing document title");
    assert_eq!(errors[0].hints, vec!["set the title with `set document(title: [...])`"]);
}

#[test]
fn p1481_ua1_imagem_real_exige_alt_artifact_nao() {
    use typst_core::entities::elements::pdf_artifact::ArtifactKind;
    use typst_core::entities::layout_types::{SemanticKind, SemanticPlacement};
    let image = FrameItem::Image {
        pos: Point::ZERO,
        data: Arc::new(vec![0xff, 0xd8, 0xff]),
        alt: None,
        width: Pt(10.0),
        height: Pt(10.0),
        intrinsic_width: 1,
        intrinsic_height: 1,
        clip_rect: None,
        orientation: 1,
    };
    let ua1 = standards(&[PdfStandard::Ua1]);
    let mut doc = PagedDocument::new(vec![page_with(vec![image.clone()])]);
    let diag = require_ua1_image_alt(&doc, &ua1).expect("imagem real sem alt");
    assert_eq!(diag.message, "PDF/UA-1 error: missing alt text");
    assert_eq!(diag.hints, vec!["make sure your images and equations have alt text"]);
    assert!(require_ua1_image_alt(&doc, &PdfStandards::default()).is_none());
    if let FrameItem::Image { alt, .. } = &mut doc.pages[0].items[0] {
        *alt = Some("".into());
    }
    assert!(require_ua1_image_alt(&doc, &ua1).is_none());
    doc.pages[0].items = vec![FrameItem::Semantic {
        kind: SemanticKind::Artifact(ArtifactKind::Other),
        placement: SemanticPlacement::Inline,
        alt: None,
        items: vec![image],
    }];
    assert!(require_ua1_image_alt(&doc, &ua1).is_none());
}

#[test]
fn p1480_ua1_com_titulo_compila_e_emite_identificacao() {
    let pdf = p1480_compile_with_standard(
        "#set document(title: \"PDF/UA parity audit\")\nHello",
        Some(standards(&[PdfStandard::Ua1])),
    )
    .expect("ua-1 com título deve continuar a compilar");
    let rendered = String::from_utf8_lossy(&pdf);
    assert!(
        rendered.contains("<pdfuaid:part>1</pdfuaid:part>"),
        "pipeline ua-1 deve chegar ao builder com o standard resolvido"
    );
    assert!(
        rendered.contains("/ViewerPreferences << /DisplayDocTitle true >>"),
        "pipeline ua-1 deve emitir a preferência de título"
    );
}

#[test]
fn p1480_sem_ua1_nao_exige_titulo() {
    let default_pdf = p1480_compile_with_standard("Hello", None)
        .expect("sem standard, título deve continuar opcional");
    assert!(
        !String::from_utf8_lossy(&default_pdf).contains("pdfuaid"),
        "sem standard não deve emitir identificação PDF/UA"
    );

    let pdfa_pdf =
        p1480_compile_with_standard("Hello", Some(standards(&[PdfStandard::A2u])))
            .expect("perfil archival isolado não deve exigir título");
    assert!(
        !String::from_utf8_lossy(&pdfa_pdf).contains("pdfuaid"),
        "a-2u isolado não deve emitir identificação PDF/UA"
    );
}

#[test]
fn p1493_html_frame_position_converge_no_pipeline_html() {
    let world = mock_world(concat!(
        "#html.frame(box(width: 100pt, height: 50pt, {\n",
        "  place(top + left, dx: 30pt, dy: 20pt, context {\n",
        "    let loc = here()\n",
        "    assert.eq(loc.page(), none)\n",
        "    assert.eq(loc.position(), (x: 30pt, y: 20pt))\n",
        "  })\n",
        "}))\n",
    ));

    let (result, _warnings) = compile_to_html_string_with_features(
        &world,
        &world.source,
        typst_core::entities::compiler_features::Features::html(),
    );
    let html = result.expect("posição local no html.frame deve convergir");
    assert!(html.contains("<svg"), "html.frame deve produzir SVG: {html:?}");
}

#[test]
fn p1493_bundle_misto_estabiliza_posicao_html_sem_contaminar_pagina() {
    use typst_core::entities::compiler_features::{Feature, Features};

    let world = mock_world(concat!(
        "#document(\"paged.pdf\")[\n",
        "  #context {\n",
        "    assert.eq(target(), \"paged\")\n",
        "    assert.eq(here().page(), 1)\n",
        "    [paged-control]\n",
        "  }\n",
        "]\n",
        "#document(\"island.html\")[\n",
        "  #html.frame(box(width: 100pt, height: 50pt, {\n",
        "    place(top + left, dx: 30pt, dy: 20pt, context {\n",
        "      assert.eq(target(), \"html\")\n",
        "      assert.eq(here().page(), none)\n",
        "      assert.eq(here().position(), (x: 30pt, y: 20pt))\n",
        "    })\n",
        "  }))\n",
        "]\n",
    ));
    let mut features = Features::empty();
    features.enable(Feature::Bundle);
    features.enable(Feature::Html);

    let files = compile_to_bundle_files(
        &world,
        &world.source,
        BundleCompileOptions {
            full_error: false,
            features,
            document_id: None,
            pdf_stream_mode: StreamMode::Verbose,
            pdf_serialization: PdfSerializationMode::Compact,
            pdf_tagged_override: None,
            pdf_pages_override: None,
            pdf_standard_override: None,
            png_ppi_override: None,
            html_serialization: crate::export::HtmlSerializationMode::Crystalline,
            pretty_override: None,
        },
    )
    .0
    .expect("bundle misto P1493 deve convergir");

    let paged = files
        .iter()
        .find(|file| file.path == "paged.pdf")
        .expect("bundle deve conter paged.pdf");
    let html = files
        .iter()
        .find(|file| file.path == "island.html")
        .expect("bundle deve conter island.html");
    assert!(!paged.is_html_document);
    assert!(html.is_html_document);
}

#[test]
fn p1493_descendente_unico_leitor_dispara_fixpoint_html() {
    let world = mock_world(concat!(
        "#html.frame(box(width: 100pt, height: 50pt, {\n",
        "  place(top + left, dx: 30pt, dy: 20pt, context {\n",
        "    context {\n",
        "      assert.eq(here().page(), none)\n",
        "      assert.eq(here().position(), (x: 30pt, y: 20pt))\n",
        "    }\n",
        "  })\n",
        "}))\n",
    ));

    let (result, _warnings) = compile_to_html_string_with_features(
        &world,
        &world.source,
        typst_core::entities::compiler_features::Features::html(),
    );
    let html = result.expect("leitor criado pelo contexto externo deve convergir");
    assert!(html.contains("<svg"), "html.frame deve produzir SVG: {html:?}");
}

#[test]
fn p1493_fase_html_permanece_latched_ate_store_esvaziar() {
    use typst_core::entities::compiler_features::{Feature, Features};

    let world = mock_world(concat!(
        "#document(\"island.html\")[\n",
        "  #context {\n",
        "    let found = query(<p1493-stale-location>)\n",
        "    if found.len() == 0 {\n",
        "      html.frame(box(width: 100pt, height: 50pt, {\n",
        "        place(top + left, dx: 30pt, dy: 20pt, context {\n",
        "          let loc = here()\n",
        "          let observed = loc.position()\n",
        "          [#metadata(loc) <p1493-stale-location>]\n",
        "        })\n",
        "      }))\n",
        "    } else {\n",
        "      let stale = found.first().value\n",
        "      let settled = stale.position() == none\n",
        "      [#html.frame(box(width: 1pt, height: 1pt, [#metadata(stale) <p1493-stale-location>])) #if settled [settled-after-empty]]\n",
        "    }\n",
        "  }\n",
        "]\n",
    ));
    let mut features = Features::empty();
    features.enable(Feature::Bundle);
    features.enable(Feature::Html);

    let files = compile_to_bundle_files(
        &world,
        &world.source,
        BundleCompileOptions {
            full_error: false,
            features,
            document_id: None,
            pdf_stream_mode: StreamMode::Verbose,
            pdf_serialization: PdfSerializationMode::Compact,
            pdf_tagged_override: None,
            pdf_pages_override: None,
            pdf_standard_override: None,
            png_ppi_override: None,
            html_serialization: crate::export::HtmlSerializationMode::Crystalline,
            pretty_override: None,
        },
    )
    .0
    .expect("store HTML nonempty→empty deve estabilizar");
    let html_file = files
        .iter()
        .find(|file| file.path == "island.html")
        .expect("bundle deve conter island.html");
    let html = String::from_utf8_lossy(&html_file.data);
    assert!(
        html.contains("settled-after-empty"),
        "a geração vazia deve ser observada antes de aceitar: {html:?}"
    );
}

#[test]
fn p1493_html_frame_position_preserva_place_com_resultado_visivel() {
    let world = mock_world(concat!(
        "#html.frame(box(width: 100pt, height: 50pt, {\n",
        "  place(top + left, dx: 30pt, dy: 20pt, context {\n",
        "    assert.eq(here().page(), none)\n",
        "    assert.eq(here().position(), (x: 30pt, y: 20pt))\n",
        "    [html-control]\n",
        "  })\n",
        "}))\n",
    ));

    let (result, _warnings) = compile_to_html_string_with_features(
        &world,
        &world.source,
        typst_core::entities::compiler_features::Features::html(),
    );
    result.expect("conteúdo visível não pode deslocar a posição do context");
}
#[test]
fn p1494_bundle_ignora_espaco_exterior_e_preserva_origem_no_body() {
    let body = Content::sequence(vec![
        Content::text("水"),
        Content::SpaceFromNewline,
        Content::text("果"),
    ]);
    let document =
        Content::document("one.typ".into(), body, None, None, vec![], None, vec![]);
    let root =
        Content::sequence(vec![Content::SpaceFromNewline, document, Content::Space]);
    let mut entries = Vec::new();
    collect_bundle_entries(&root, &mut entries).expect("bundle válido");
    assert_eq!(entries.len(), 1);
    let Content::Document { body, .. } = &entries[0] else {
        panic!("entrada deve manter Document");
    };
    let Content::Sequence(parts) = body.as_ref() else {
        panic!("body deve manter sequência lexical");
    };
    assert!(matches!(parts[1], Content::SpaceFromNewline));
}

#[test]
fn font_lifecycle_shape_resources_do_not_request_fonts() {
    struct NoFontWorld(MockWorld);
    impl World for NoFontWorld {
        fn library(&self) -> &Library {
            self.0.library()
        }
        fn book(&self) -> &FontBook {
            panic!("shapes must not acquire a font catalog")
        }
        fn main(&self) -> FileId {
            self.0.main()
        }
        fn source(&self, id: FileId) -> FileResult<Source> {
            self.0.source(id)
        }
        fn file(&self, id: FileId) -> FileResult<Bytes> {
            self.0.file(id)
        }
        fn font(&self, _: usize) -> Option<Font> {
            panic!("shapes must not load a font")
        }
        fn today(
            &self,
            _: Option<typst_core::entities::duration::Duration>,
        ) -> Option<Datetime> {
            None
        }
    }
    let world = NoFontWorld(mock_world(
        "#set page(width: 64pt, height: 64pt)\n#rect(width: 10pt, height: 10pt, fill: blue)",
    ));
    let source = world.source(world.main()).unwrap();
    // Layout owns baseline metrics; this test isolates export resource demand.
    let mut timings = Timings::default();
    let (result, _) =
        compile_to_paged_document_full_error(&world.0, &source, false, &mut timings);
    let doc = result.expect("shape layout must succeed");
    assert!(!doc.pages.is_empty());
    let combos = collect_fonts_from_doc(&doc, &world);
    assert!(combos.is_empty());
    assert!(resolve_fonts(&combos, &world).is_empty());
    assert!(resolve_export_fonts(&doc, &world).unwrap().is_empty());
}

#[test]
fn infra_axis_gate_unknown_axis_preserves_bytes() {
    use typst_core::entities::layout_types::{Point, Pt, TextStyle};
    let bytes = include_bytes!("../../fixtures/fonts/UbuntuSans-Variable.ttf");
    let info = crate::fonts::font_info_from_bytes(bytes, 0).unwrap();
    let face = ttf_parser::Face::parse(bytes, 0).unwrap();
    assert!(face
        .variation_axes()
        .into_iter()
        .any(|axis| axis.tag == ttf_parser::Tag::from_bytes(b"wght")));
    assert!(!face
        .variation_axes()
        .into_iter()
        .any(|axis| axis.tag == ttf_parser::Tag::from_bytes(b"ZZZZ")));
    let world =
        crate::world::SystemWorld::for_eval(std::env::temp_dir()).with_fonts(vec![
            crate::fonts::FontSlot::new_embedded(
                "<UbuntuSans fixture>".into(),
                bytes.to_vec(),
            ),
        ]);
    let doc_for = |tag: [u8; 4], value: f32| {
        PagedDocument::new(vec![page_with(vec![FrameItem::Text {
            pos: Point::ZERO,
            text: "Axis control".into(),
            style: TextStyle {
                font: Some(FontList::single(info.family.clone().into())),
                variations: Some(FontVariations(vec![(tag, value)])),
                ..TextStyle::regular(Pt(12.0))
            },
            bidi: None,
        }])])
    };
    let unknown = resolve_export_fonts(&doc_for(*b"ZZZZ", 1.0), &world)
        .expect("unknown axes must not require an instancer");
    assert_eq!(unknown.len(), 1);
    assert_eq!(unknown[0].1.as_slice(), bytes);
    if !variable_font_instancer_available() {
        let error = resolve_export_fonts(&doc_for(*b"wght", 700.0), &world)
            .expect_err("matching axes still require an instancer");
        assert!(error.message.contains("Python/fontTools"));
    }
}

#[test]
fn unchanged_context_session_preserves_carrier() {
    let world = mock_world("Hello");
    let content = Content::Sequence(
        vec![Content::heading(1, Content::text("Original")), Content::text("Tail")]
            .into(),
    );
    let intr = introspect_with_introspector(&content);
    let (result, warnings) = context_stabilization::prepare(
        content.clone(),
        &intr,
        &world,
        &world.source,
        false,
        typst_core::entities::compiler_features::Features::default(),
        None,
    );
    let prepared = result.unwrap();
    assert!(warnings.is_empty());
    assert!(
        prepared.settled.is_none(),
        "an unselected session must not retain a layout cache"
    );
    assert_eq!(prepared.content.plain_text(), content.plain_text());
    let (Content::Sequence(before), Content::Sequence(after)) =
        (&content, &prepared.content)
    else {
        panic!("expected sequences")
    };
    assert!(
        Arc::ptr_eq(before, after),
        "a session without producers must preserve its carrier"
    );
}

#[test]
fn unchanged_outline_phase_preserves_carrier_and_index() {
    let world = mock_world("Hello");
    let content = Content::Sequence(
        vec![Content::heading(1, Content::text("Original")), Content::text("Tail")]
            .into(),
    );
    let intr = introspect_with_introspector(&content);
    let (result, warnings) =
        paged::realize_outlines_for_pipeline(&content, &intr, &world, &world.source);
    let (output, index) = result.unwrap();
    assert!(warnings.is_empty());
    assert_eq!(
        index.query_by_kind(ElementKind::Heading),
        intr.query_by_kind(ElementKind::Heading)
    );
    let (Content::Sequence(before), Content::Sequence(after)) = (&content, &output)
    else {
        panic!("expected sequences")
    };
    assert!(
        Arc::ptr_eq(before, after),
        "a phase without outline transformations must preserve its carrier"
    );
}

thread_local! {
    static INITIAL_LAYOUT_CALLS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

pub(super) fn record_initial_layout_call() {
    INITIAL_LAYOUT_CALLS.with(|count| count.set(count.get() + 1));
}

#[test]
fn independent_table_reuses_first_layout_and_binds_all_pages() {
    let world = mock_world(
        "#set page(width: 200pt, height: 120pt, margin: 10pt)\n#set text(font: \"Libertinus Serif\", size: 10pt)\n#table(columns: 2, ..range(40).map(i => [Cell #i]))",
    );
    INITIAL_LAYOUT_CALLS.with(|count| count.set(0));
    let mut timings = Timings::default();
    let (result, _) =
        compile_to_paged_document_full_error(&world, &world.source, false, &mut timings);
    let doc = result.expect("table must compile");
    assert!(doc.pages.len() > 1, "fixture must exercise multiple pages");
    assert!(doc.plain_text().contains("Cell") && doc.plain_text().contains("39"));
    for (index, page) in doc.pages.iter().enumerate() {
        assert_eq!(page.logical_number, index + 1);
    }
    INITIAL_LAYOUT_CALLS.with(|count| {
        assert_eq!(
            count.get(),
            1,
            "independent table must reuse complete first candidate"
        )
    });
}

#[test]
fn local_cell_fill_reuses_first_layout_without_page_dependencies() {
    let world = mock_world(
        "#set page(width: 200pt, height: 120pt, margin: 10pt)\n\
         #set text(font: \"Libertinus Serif\", size: 10pt)\n\
         #let c = { set table.cell(fill: red); table.cell[Local] }\n\
         #table(columns: 2, ..range(40).map(_ => c))",
    );
    INITIAL_LAYOUT_CALLS.with(|count| count.set(0));
    let mut timings = Timings::default();
    let (result, _) =
        compile_to_paged_document_full_error(&world, &world.source, false, &mut timings);
    let doc = result.expect("local cell table must compile");
    assert!(doc.pages.len() > 1);
    assert_eq!(doc.plain_text().matches("Local").count(), 40);
    INITIAL_LAYOUT_CALLS.with(|count| assert_eq!(count.get(), 1));
}

#[test]
fn local_cell_show_recipe_keeps_numbering_fixpoint() {
    let world = mock_world(
        "#let c = { show table.cell: it => it; set table.cell(fill: red); table.cell[Local] }\n\
         #table(columns: 2, ..range(4).map(_ => c))",
    );
    INITIAL_LAYOUT_CALLS.with(|count| count.set(0));
    let mut timings = Timings::default();
    let (result, _) =
        compile_to_paged_document_full_error(&world, &world.source, false, &mut timings);
    let doc = result.expect("local show recipe must compile");
    assert_eq!(doc.plain_text().matches("Local").count(), 4);
    INITIAL_LAYOUT_CALLS.with(|count| assert_eq!(count.get(), 2));
}

#[test]
fn numbered_table_keeps_second_layout() {
    let world = mock_world(
        "#set page(numbering: \"1 / 1\")\n#table(columns: 2, [A], [B], [C], [D])",
    );
    INITIAL_LAYOUT_CALLS.with(|count| count.set(0));
    let mut timings = Timings::default();
    let (result, _) =
        compile_to_paged_document_full_error(&world, &world.source, false, &mut timings);
    let doc = result.expect("numbered table must compile");
    assert_eq!(doc.pages.len(), 1);
    assert_eq!(frame_items_text(&doc.pages[0].foreground), "1 / 1");
    let body = frame_items_text(&doc.pages[0].items);
    for cell in ["A", "B", "C", "D"] {
        assert_eq!(body.matches(cell).count(), 1, "{body:?}");
    }
    // The historical name refers to the ordinary route, superseded here by
    // the approved physical protocol. Keep language acceptance above intact.
    INITIAL_LAYOUT_CALLS.with(|count| assert_eq!(count.get(), 0));
}

#[test]
fn initial_layout_proof_rejects_hidden_and_unknown_dependencies() {
    use typst_core::entities::content::{RealizedContent, SetPageDelta};
    use typst_core::entities::page_running::PageMarginal;
    assert!(paged::initial_layout_independent(&Content::text("ready")));
    let header = Content::SetPage(Arc::new(SetPageDelta {
        header: Some(PageMarginal::Content(Arc::new(Content::text("hidden")))),
        ..SetPageDelta::default()
    }));
    assert!(!paged::initial_layout_independent(&header));
    let background = Content::SetPage(Arc::new(SetPageDelta {
        background: Some(Some(Arc::new(Content::text("layer")))),
        ..SetPageDelta::default()
    }));
    assert!(!paged::initial_layout_independent(&background));
    let heading = Content::heading(1, Content::text("origin"));
    let realized = Content::Realized(Arc::new(RealizedContent {
        original: heading.clone(),
        visual: Content::text("ready"),
    }));
    assert!(!paged::initial_layout_independent(&heading));
    assert!(!paged::initial_layout_independent(&realized));
    let styles = typst_core::entities::style::Styles::new()
        .push_custom("par.line.numbering", Value::Str("1".into()));
    assert!(!paged::initial_layout_independent(&Content::styled(
        Content::text("ready"),
        styles
    )));
}

#[test]
fn page_display_uses_realized_logical_page_at_each_context() {
    for (prefix, expected) in
        [("", ["1", "2"]), ("#counter(page).update(7)\n", ["7", "8"])]
    {
        for expression in [
            "counter(page).display()",
            "counter(page).display(\"1\")",
            "counter(page).display(n => str(n))",
        ] {
            let program = format!(
                "{prefix}#context {expression}\n#pagebreak()\n#context {expression}"
            );
            let w = mock_world(&program);
            let doc = compile_to_paged_document_full_error(
                &w,
                &w.source,
                false,
                &mut Timings::default(),
            )
            .0
            .expect("page context should compile");
            assert_eq!(
                doc.pages
                    .iter()
                    .map(|p| frame_items_text(&p.items))
                    .collect::<Vec<_>>(),
                expected,
                "{program}"
            );
        }
    }
}

#[test]
fn page_display_does_not_replace_nonpage_counter_state() {
    let w = mock_world(
        "#counter(\"custom\").update(3)\n#context counter(\"custom\").display(\"1\")",
    );
    let doc = compile_to_paged_document_full_error(
        &w,
        &w.source,
        false,
        &mut Timings::default(),
    )
    .0
    .expect("custom counter should compile");
    assert_eq!(frame_items_text(&doc.pages[0].items), "3");
}

#[test]
fn page_snapshot_current_and_final_follow_physical_pages_and_causal_updates() {
    for (program, expected) in [
        (
            "#context [B#counter(page).display(\"1 / 1\", both: true) F#counter(page).final().first()]",
            vec!["B1/1F1"],
        ),
        (
            "#context counter(page).display(\"1 / 1\", both: true)\n#pagebreak()\n#context counter(page).final().first()",
            vec!["1/2", "2"],
        ),
        (
            "#counter(page).update(7)\n#context counter(page).display(\"1 / 1\", both: true)\n#pagebreak()\n#context counter(page).final().first()",
            vec!["7/8", "8"],
        ),
        (
            "#context counter(page).display(\"1 / 1\", both: true)\n#pagebreak()\n#counter(page).update(7)\n#context counter(page).display(\"1 / 1\", both: true)",
            vec!["1/7", "7/7"],
        ),
        (
            "#context [B#counter(page).display(\"1 / 1\", both: true) F#counter(page).final().first()]\n#counter(page).step()",
            vec!["B1/2F2"],
        ),
        (
            "#context counter(page).display(\"1 / 1\", both: true)\n#pagebreak()\n#counter(page).update(n => n + 5)\n#context counter(page).get().first()",
            vec!["1/7", "7"],
        ),
        ("#context counter(page).final().first()\n#pagebreak()\nLast", vec!["2", "Last"]),
        (
            "#counter(page).update(())\n#context [G#repr(counter(page).get()) F#repr(counter(page).final())]\n#pagebreak()\n#context [G#repr(counter(page).get()) F#repr(counter(page).final())]",
            vec!["G(0,)F(1,)", "G(1,)F(1,)"],
        ),
        (
            "#counter(page).update((7, 2))\n#context [G#repr(counter(page).get()) F#repr(counter(page).final())]\n#pagebreak()\n#context [G#repr(counter(page).get()) F#repr(counter(page).final())]",
            vec!["G(7,)F(8,)", "G(8,)F(8,)"],
        ),
    ] {
        let world = mock_world(program);
        let doc = compile_to_paged_document_full_error(
            &world,
            &world.source,
            false,
            &mut Timings::default(),
        )
        .0
        .expect(program);
        assert_eq!(
            doc.pages
                .iter()
                .map(|p| frame_items_text(&p.items))
                .collect::<Vec<_>>(),
            expected,
            "{program}"
        );
    }
}

#[test]
fn page_snapshot_html_final_preserves_manual_updates() {
    let world =
        mock_world("#counter(page).update(7)\n#context counter(page).final().first()");
    let html = compile_to_html_string_with_features(
        &world,
        &world.source,
        typst_core::entities::compiler_features::Features::html(),
    )
    .0
    .expect("HTML page counter");
    assert!(html.contains(">7<"), "{html}");
}

#[test]
fn page_snapshot_retention_keeps_one_complete_generation() {
    use typst_core::entities::location::Location;
    for cells in [4, 320] {
        let program = format!(
            "#context counter(page).final().first()\n#table(columns: 4, ..range({cells}).map(i => [Cell #i]))\n#pagebreak()\nLast"
        );
        let world = mock_world(&program);
        let paged::PagedCompilation { document: doc, introspector: intr } =
            compile_to_paged_document_full_error_and_features_with_introspector(
                &world,
                &world.source,
                false,
                typst_core::entities::compiler_features::Features::default(),
                &mut Timings::default(),
            )
            .0
            .expect("retained snapshot");
        assert!(!intr.page_counter_values.is_empty());
        assert!(intr
            .page_counter_values
            .keys()
            .all(|loc| intr.page_counter_final_values.contains_key(loc)));
        assert!(intr
            .page_counter_final_values
            .values()
            .all(|value| value == &vec![doc.pages.len()]));
        let entries =
            intr.page_counter_values.len() + intr.page_counter_final_values.len();
        let slots = intr.page_counter_values.capacity()
            + intr.page_counter_final_values.capacity();
        let payload = intr
            .page_counter_values
            .values()
            .chain(intr.page_counter_final_values.values())
            .map(|v| v.capacity() * std::mem::size_of::<usize>())
            .sum::<usize>();
        let lower_bound = std::mem::size_of_val(&intr.page_counter_values)
            + std::mem::size_of_val(&intr.page_counter_final_values)
            + slots * std::mem::size_of::<(Location, Vec<usize>)>()
            + payload;
        eprintln!(
            "PAGE_RETENTION cells={cells} pages={} entries={entries} slots={slots} vector_bytes={payload} known_reserved_bytes={lower_bound}",
            doc.pages.len()
        );
    }
}

#[test]
fn page_snapshot_failed_callback_does_not_poison_later_compilation() {
    let mut world = mock_world(
        "#counter(page).update(n => false)\n#context counter(page).final().first()",
    );
    let bad = compile_to_paged_document_full_error(
        &world,
        &world.source,
        false,
        &mut Timings::default(),
    )
    .0;
    assert!(bad.is_err(), "an invalid Page callback must fail");
    world.source = Source::new(
        world.source.id(),
        "#context counter(page).display(\"1 / 1\", both: true)\n#pagebreak()\nLast"
            .into(),
    )
    .expect("valid Source test fixture");
    let doc = compile_to_paged_document_full_error(
        &world,
        &world.source,
        false,
        &mut Timings::default(),
    )
    .0
    .expect("recovery in the same World");
    assert_eq!(frame_items_text(&doc.pages[0].items), "1/2");
    assert_eq!(doc.pages.len(), 2);
}

thread_local! {
    static UNCACHED_LAYOUT_METRICS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

pub(super) fn record_uncached_layout_metrics() {
    UNCACHED_LAYOUT_METRICS.with(|count| count.set(count.get() + 1));
}

#[test]
fn selected_context_shares_metric_cache_across_layout_attempts() {
    let world = mock_world(
        "#let c = counter(heading.where(level: 1))\n#context [#c.final() #table(columns: 2, ..range(64).map(i => [Cell #i]))]",
    );
    UNCACHED_LAYOUT_METRICS.with(|count| count.set(0));
    INITIAL_LAYOUT_CALLS.with(|count| count.set(0));
    let doc = compile_to_paged_document_full_error(
        &world,
        &world.source,
        false,
        &mut Timings::default(),
    )
    .0
    .expect("selected contextual table");
    assert!(doc.plain_text().contains("Cell") && doc.plain_text().contains("63"));
    assert!(
        INITIAL_LAYOUT_CALLS.with(|count| count.get()) >= 2,
        "fixture must exercise multiple layout attempts"
    );
    for (index, page) in doc.pages.iter().enumerate() {
        assert_eq!(page.logical_number, index + 1);
    }
    UNCACHED_LAYOUT_METRICS.with(|count| {
        assert_eq!(count.get(), 0, "selected attempts must share the session cache")
    });
}

thread_local! {
    static UNCACHED_RECONCILIATION_METRICS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

pub(super) fn record_uncached_reconciliation_metrics() {
    UNCACHED_RECONCILIATION_METRICS.with(|count| count.set(count.get() + 1));
}

#[test]
fn selected_context_retains_metrics_until_final_reconciliation() {
    let world = mock_world(
        "#let c = counter(heading.where(level: 1))\n#context [#c.final() #table(columns: 2, ..range(64).map(i => [Cell #i]))]",
    );
    UNCACHED_RECONCILIATION_METRICS.with(|count| count.set(0));
    let doc = compile_to_paged_document_full_error(
        &world,
        &world.source,
        false,
        &mut Timings::default(),
    )
    .0
    .expect("selected contextual table");
    assert!(doc.plain_text().contains("Cell") && doc.plain_text().contains("63"));
    for (index, page) in doc.pages.iter().enumerate() {
        assert_eq!(page.logical_number, index + 1);
    }
    UNCACHED_RECONCILIATION_METRICS.with(|count| {
        assert_eq!(count.get(), 0, "reconciliation must borrow the selected layout cache")
    });
}

thread_local! {
    static UNCACHED_BIDI_METRICS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

pub(super) fn record_uncached_bidi_metrics() {
    UNCACHED_BIDI_METRICS.with(|count| count.set(count.get() + 1));
}

#[test]
fn selected_context_retains_metrics_until_bidi() {
    let world = mock_world(
        "#set text(dir: rtl, lang: \"ar\")\n#let c = counter(heading.where(level: 1))\n#context { let _ = c.final(); [مرحبا بالعالم ABC 123] }",
    );
    UNCACHED_BIDI_METRICS.with(|count| count.set(0));
    INITIAL_LAYOUT_CALLS.with(|count| count.set(0));
    let doc = compile_to_paged_document_full_error(
        &world,
        &world.source,
        false,
        &mut Timings::default(),
    )
    .0
    .expect("selected RTL context");
    assert!(doc.plain_text().contains("مرحبا") && doc.plain_text().contains("123"));
    assert!(INITIAL_LAYOUT_CALLS.with(|count| count.get()) >= 2);
    for (index, page) in doc.pages.iter().enumerate() {
        assert_eq!(page.logical_number, index + 1);
    }
    UNCACHED_BIDI_METRICS.with(|count| {
        assert_eq!(count.get(), 0, "bidi must borrow the selected layout cache")
    });
}

#[test]
fn bidi_complete_context_reaches_pipeline_for_mixed_source() {
    let src = "#set page(width: 220pt, height: 600pt, margin: 20pt)\n#set text(dir: rtl)\nABC 123 مرحبا";
    let doc = p1502_compile_paged(src);

    assert!(
        doc.pages[0]
            .items
            .iter()
            .filter(|i| matches!(
                i,
                FrameItem::Text { .. }
                    | FrameItem::TextShaped { .. }
                    | FrameItem::TextSeparator { .. }
            ))
            .all(|i| crate::bidi_context::span(i).is_some()),
        "entire textual pipeline paragraph must preserve certified origin"
    );
}

#[test]
fn expression_source_identity_context_errors_and_warnings_keep_their_sources() {
    let expression = Source::new_with_parser(
        FileId::from_raw(NonZeroU16::new(2).unwrap()),
        "missing".into(),
        typst_core::compiler::parse::parse_code,
    )
    .expect("valid Source test fixture");
    let features = typst_core::entities::compiler_features::Features::empty();
    let world = mock_world("#document_missing");
    let (result, _) =
        eval_source_with_document_features(&world, &world.source, &expression, features);
    let errors = result.unwrap_err();
    assert!(errors.iter().all(|e| e.span.id() == Some(world.main())));
    let world = mock_world("**\n= Document");
    let (result, warnings) =
        eval_source_with_document_features(&world, &world.source, &expression, features);
    assert!(result
        .unwrap_err()
        .iter()
        .all(|e| e.span.id() == Some(expression.id())));
    assert!(warnings.iter().any(
        |e| e.message == "no text within stars" && e.span.id() == Some(world.main())
    ));
}

#[test]
fn expression_source_identity_document_program_runs_once() {
    struct CountingWorld {
        inner: MockWorld,
        reads: std::sync::atomic::AtomicUsize,
    }
    impl World for CountingWorld {
        fn library(&self) -> &Library {
            self.inner.library()
        }
        fn book(&self) -> &FontBook {
            self.inner.book()
        }
        fn main(&self) -> FileId {
            self.inner.main()
        }
        fn source(&self, id: FileId) -> FileResult<Source> {
            self.inner.source(id)
        }
        fn file(&self, _: FileId) -> FileResult<Bytes> {
            Err(FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<Font> {
            None
        }
        fn today(
            &self,
            _: Option<typst_core::entities::duration::Duration>,
        ) -> Option<Datetime> {
            None
        }
        fn resolve_path(
            &self,
            file: FileId,
            path: &str,
        ) -> Result<typst_core::entities::path::RootedPath, String> {
            assert_eq!(file, self.main());
            Ok(typst_core::entities::path::RootedPath::new(
                typst_core::entities::path::VirtualRoot::Project,
                typst_core::entities::path::VirtualPath::new(path).unwrap(),
            ))
        }
        fn read_path(
            &self,
            _: &typst_core::entities::path::RootedPath,
        ) -> Result<Arc<Vec<u8>>, String> {
            self.reads.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            Ok(Arc::new(b"marker".to_vec()))
        }
    }
    let world = CountingWorld {
        inner: mock_world("#let probe = read(\"marker.txt\")\n= Document"),
        reads: std::sync::atomic::AtomicUsize::new(0),
    };
    let expression = Source::new_with_parser(
        FileId::from_raw(NonZeroU16::new(2).unwrap()),
        "query(heading).len()".into(),
        typst_core::compiler::parse::parse_code,
    )
    .expect("valid Source test fixture");
    let (result, _) = eval_source_with_document_features(
        &world,
        &world.inner.source,
        &expression,
        typst_core::entities::compiler_features::Features::empty(),
    );
    assert_eq!(result.unwrap(), Value::Int(1));
    assert_eq!(world.reads.load(std::sync::atomic::Ordering::Relaxed), 1);
    let reading = Source::new_with_parser(
        expression.id(),
        "read(\"marker.txt\")".into(),
        typst_core::compiler::parse::parse_code,
    )
    .expect("valid Source test fixture");
    let (result, _) = eval_source_with_sink_features(
        &world,
        &reading,
        typst_core::entities::compiler_features::Features::empty(),
    );
    assert_eq!(result.unwrap(), Value::Str("marker".into()));
    assert_eq!(world.reads.load(std::sync::atomic::Ordering::Relaxed), 2);
}

#[test]
fn query_compiled_context_reports_callback_and_retains_final_metadata_and_warning() {
    let world = mock_world("#let bad() = unknown\n#context bad()");
    let (result, warnings) = compile_query_context(&world, &world.source);
    assert!(warnings.is_empty());
    let errors = result.unwrap_err();
    assert!(errors[0].message.contains("unknown variable `unknown`"));
    assert_eq!(errors[0].span.id(), Some(world.source.id()));
    assert!(!errors[0].trace.is_empty(), "callback trace must survive compilation");

    let world = mock_world("**\n#context metadata(\"context-value\")");
    let (result, warnings) = compile_query_context(&world, &world.source);
    let intr = result.unwrap();
    assert_eq!(warnings.len(), 1, "{warnings:?}");
    let values = intr.query_metadata();
    assert_eq!(values.len(), 1);
    assert!(matches!(&values[0], Value::Str(value) if value == "context-value"));
}

#[test]
fn context_frontier_text_context_global() {
    let world = mock_world(
        r###"#show "A":[L]
#let c=context[A]
#show "A":[R]
#c"###,
    );
    let (result, _) = compile_to_paged_document_full_error(
        &world,
        &world.source,
        false,
        &mut Timings::default(),
    );
    let text = result.expect("contextual recipe must compile").plain_text();
    assert_eq!(text.trim(), "R");
    let html = compile_to_html_string_with_features(
        &world,
        &world.source,
        typst_core::entities::compiler_features::Features::html(),
    )
    .0
    .expect("contextual recipe must compile as HTML");
    assert!(html.contains(">R<"), "{html}");
}

#[test]
fn context_frontier_text_context_local() {
    let world = mock_world(
        r###"#let c=[#show "A":[L]
#context[A]]
#show "A":[R]
#c"###,
    );
    let (result, _) = compile_to_paged_document_full_error(
        &world,
        &world.source,
        false,
        &mut Timings::default(),
    );
    let text = result.expect("contextual recipe must compile").plain_text();
    assert_eq!(text.trim(), "L");
    let html = compile_to_html_string_with_features(
        &world,
        &world.source,
        typst_core::entities::compiler_features::Features::html(),
    )
    .0
    .expect("contextual recipe must compile as HTML");
    assert!(html.contains(">L<"), "{html}");
}

#[test]
fn context_frontier_text_created_before_rule() {
    let world = mock_world(
        r###"#let c=context[A]
#show "A":[R]
#c"###,
    );
    let (result, _) = compile_to_paged_document_full_error(
        &world,
        &world.source,
        false,
        &mut Timings::default(),
    );
    let text = result.expect("contextual recipe must compile").plain_text();
    assert_eq!(text.trim(), "R");
    let html = compile_to_html_string_with_features(
        &world,
        &world.source,
        typst_core::entities::compiler_features::Features::html(),
    )
    .0
    .expect("contextual recipe must compile as HTML");
    assert!(html.contains(">R<"), "{html}");
}

#[test]
fn context_frontier_text_local_inside_context() {
    let world = mock_world(
        r###"#show "A":[R]
#context { show "A":[L]; [A] }"###,
    );
    let (result, _) = compile_to_paged_document_full_error(
        &world,
        &world.source,
        false,
        &mut Timings::default(),
    );
    let text = result.expect("contextual recipe must compile").plain_text();
    assert_eq!(text.trim(), "L");
    let html = compile_to_html_string_with_features(
        &world,
        &world.source,
        typst_core::entities::compiler_features::Features::html(),
    )
    .0
    .expect("contextual recipe must compile as HTML");
    assert!(html.contains(">L<"), "{html}");
}

#[test]
fn context_frontier_text_nested_context() {
    let world = mock_world(
        r###"#show "A":[R]
#context [#context[A]]"###,
    );
    let (result, _) = compile_to_paged_document_full_error(
        &world,
        &world.source,
        false,
        &mut Timings::default(),
    );
    let text = result.expect("contextual recipe must compile").plain_text();
    assert_eq!(text.trim(), "R");
    let html = compile_to_html_string_with_features(
        &world,
        &world.source,
        typst_core::entities::compiler_features::Features::html(),
    )
    .0
    .expect("contextual recipe must compile as HTML");
    assert!(html.contains(">R<"), "{html}");
}

#[test]
fn context_frontier_text_local_nested_context() {
    let world = mock_world(
        r###"#let c=[#show "A":[L]
#context [#context[A]]]
#show "A":[R]
#c"###,
    );
    let (result, _) = compile_to_paged_document_full_error(
        &world,
        &world.source,
        false,
        &mut Timings::default(),
    );
    let text = result.expect("contextual recipe must compile").plain_text();
    assert_eq!(text.trim(), "L");
    let html = compile_to_html_string_with_features(
        &world,
        &world.source,
        typst_core::entities::compiler_features::Features::html(),
    )
    .0
    .expect("contextual recipe must compile as HTML");
    assert!(html.contains(">L<"), "{html}");
}

#[test]
fn context_frontier_text_regex_context() {
    let world = mock_world(
        r###"#show regex("A+"):it=>[R]
#context[AAA]"###,
    );
    let (result, _) = compile_to_paged_document_full_error(
        &world,
        &world.source,
        false,
        &mut Timings::default(),
    );
    let text = result.expect("contextual recipe must compile").plain_text();
    assert_eq!(text.trim(), "R");
    let html = compile_to_html_string_with_features(
        &world,
        &world.source,
        typst_core::entities::compiler_features::Features::html(),
    )
    .0
    .expect("contextual recipe must compile as HTML");
    assert!(html.contains(">R<"), "{html}");
}

#[test]
fn context_frontier_text_hidden_context() {
    let world = mock_world(
        r###"#show strong:[H]
#show "A":it=>panic("hidden")
#context[*A*]"###,
    );
    let (result, _) = compile_to_paged_document_full_error(
        &world,
        &world.source,
        false,
        &mut Timings::default(),
    );
    let text = result.expect("contextual recipe must compile").plain_text();
    assert_eq!(text.trim(), "H");
    let html = compile_to_html_string_with_features(
        &world,
        &world.source,
        typst_core::entities::compiler_features::Features::html(),
    )
    .0
    .expect("contextual recipe must compile as HTML");
    assert!(html.contains(">H<"), "{html}");
}

#[test]
fn context_frontier_text_context_product() {
    let world = mock_world(
        r###"#show "A":[AA]
#context[A]"###,
    );
    let (result, _) = compile_to_paged_document_full_error(
        &world,
        &world.source,
        false,
        &mut Timings::default(),
    );
    let text = result.expect("contextual recipe must compile").plain_text();
    assert_eq!(text.trim(), "AA");
    let html = compile_to_html_string_with_features(
        &world,
        &world.source,
        typst_core::entities::compiler_features::Features::html(),
    )
    .0
    .expect("contextual recipe must compile as HTML");
    assert!(html.contains(">AA<"), "{html}");
}

#[test]
fn context_frontier_text_styled_context() {
    let world = mock_world(
        r###"#show "A":[R]
#context text(fill:red)[A]"###,
    );
    let (result, _) = compile_to_paged_document_full_error(
        &world,
        &world.source,
        false,
        &mut Timings::default(),
    );
    let text = result.expect("contextual recipe must compile").plain_text();
    assert_eq!(text.trim(), "R");
    let html = compile_to_html_string_with_features(
        &world,
        &world.source,
        typst_core::entities::compiler_features::Features::html(),
    )
    .0
    .expect("contextual recipe must compile as HTML");
    assert!(html.contains(">R<"), "{html}");
}

#[test]
fn context_occurrence_equation_one() {
    let world = mock_world(
        r###"#show math.equation.where():it=>[EQ]
#context math.frac([1],[2])
#context[Q#query(math.equation).len()]"###,
    );
    let (result, _) = compile_to_paged_document_full_error(
        &world,
        &world.source,
        false,
        &mut Timings::default(),
    );
    let doc = result.expect("distinct contextual occurrences and aliases must compile");
    assert_eq!(doc.plain_text().split_whitespace().collect::<String>(), "EQQ1");
}

#[test]
fn context_occurrence_equation_two() {
    let world = mock_world(
        r###"#show math.equation.where():it=>[EQ]
#context math.frac([1],[2])
#context math.frac([3],[4])
#context[Q#query(math.equation).len()]"###,
    );
    let (result, _) = compile_to_paged_document_full_error(
        &world,
        &world.source,
        false,
        &mut Timings::default(),
    );
    let doc = result.expect("distinct contextual occurrences and aliases must compile");
    assert_eq!(doc.plain_text().split_whitespace().collect::<String>(), "EQEQQ2");
}

#[test]
fn context_occurrence_equation_heading_prefix() {
    let world = mock_world(
        r###"#show heading:it=>it
= H
#show math.equation.where():it=>[EQ]
#context math.frac([1],[2])
#context[Q#query(math.equation).len()]"###,
    );
    let (result, _) = compile_to_paged_document_full_error(
        &world,
        &world.source,
        false,
        &mut Timings::default(),
    );
    let doc = result.expect("distinct contextual occurrences and aliases must compile");
    assert_eq!(doc.plain_text().split_whitespace().collect::<String>(), "HEQQ1");
}

#[test]
fn context_occurrence_equation_locations() {
    let world = mock_world(
        r###"#show math.equation.where():it=>context [#query(math.equation).position(e=>e.location()==it.location())]
#context math.frac([1],[2])
#context math.frac([3],[4])"###,
    );
    let (result, _) = compile_to_paged_document_full_error(
        &world,
        &world.source,
        false,
        &mut Timings::default(),
    );
    let doc = result.expect("distinct contextual occurrences and aliases must compile");
    assert_eq!(doc.plain_text().split_whitespace().collect::<String>(), "01");
}

#[test]
fn context_occurrence_equation_equal_two() {
    let world = mock_world(
        r###"#show math.equation.where():it=>[EQ]
#context math.frac([1],[2])
#context math.frac([1],[2])
#context[Q#query(math.equation).len()]"###,
    );
    let (result, _) = compile_to_paged_document_full_error(
        &world,
        &world.source,
        false,
        &mut Timings::default(),
    );
    let doc = result.expect("distinct contextual occurrences and aliases must compile");
    assert_eq!(doc.plain_text().split_whitespace().collect::<String>(), "EQEQQ2");
}

#[test]
fn context_occurrence_equation_nested_contexts() {
    let world = mock_world(
        r###"#show math.equation.where():it=>[EQ]
#context [#context math.frac([1],[2])]
#context [#context math.frac([3],[4])]
#context[Q#query(math.equation).len()]"###,
    );
    let (result, _) = compile_to_paged_document_full_error(
        &world,
        &world.source,
        false,
        &mut Timings::default(),
    );
    let doc = result.expect("distinct contextual occurrences and aliases must compile");
    assert_eq!(doc.plain_text().split_whitespace().collect::<String>(), "EQEQQ2");
}

#[test]
fn context_occurrence_equation_alias_count() {
    let world = mock_world(
        r###"#show math.equation.where():it=>[#it#it]
#context math.frac([1],[2])
#context math.frac([3],[4])
#context[Q#query(math.equation).len()]"###,
    );
    let (result, _) = compile_to_paged_document_full_error(
        &world,
        &world.source,
        false,
        &mut Timings::default(),
    );
    let doc = result.expect("distinct contextual occurrences and aliases must compile");
    assert!(doc
        .plain_text()
        .split_whitespace()
        .collect::<String>()
        .ends_with("Q2"));
}

#[test]
fn context_occurrence_equation_generated() {
    let world = mock_world(
        r###"#show math.equation.where(block:false):it=>math.equation([x],block:true)
#context math.frac([1],[2])
#context math.frac([3],[4])
#context[Q#query(math.equation).len()]"###,
    );
    let (result, _) = compile_to_paged_document_full_error(
        &world,
        &world.source,
        false,
        &mut Timings::default(),
    );
    let doc = result.expect("distinct contextual occurrences and aliases must compile");
    assert_eq!(doc.plain_text().split_whitespace().collect::<String>(), "xxQ4");
}

#[test]
fn context_occurrence_equation_grow_selected_query() {
    let world = mock_world(
        r###"#set heading(numbering:"1.")
= H
#show math.equation.where():it=>[EQ]
#context { if counter(heading.where(level:1)).final() == (0,) { math.frac([1],[2]) } else { [#math.frac([1],[2]) #math.frac([3],[4])] } }
#context { let _ = counter(heading.where(level:1)).final(); [Q#query(math.equation).len()] }"###,
    );
    let (result, _) = compile_to_paged_document_full_error(
        &world,
        &world.source,
        false,
        &mut Timings::default(),
    );
    let doc = result.expect("distinct contextual occurrences and aliases must compile");
    assert_eq!(doc.plain_text().split_whitespace().collect::<String>(), "1.HEQEQQ2");
}

#[test]
fn context_occurrence_equation_shrink_selected_query() {
    let world = mock_world(
        r###"#set heading(numbering:"1.")
= H
#show math.equation.where():it=>[EQ]
#context { if counter(heading.where(level:1)).final() == (0,) { [#math.frac([1],[2]) #math.frac([3],[4])] } else { math.frac([1],[2]) } }
#context { let _ = counter(heading.where(level:1)).final(); [Q#query(math.equation).len()] }"###,
    );
    let (result, _) = compile_to_paged_document_full_error(
        &world,
        &world.source,
        false,
        &mut Timings::default(),
    );
    let doc = result.expect("distinct contextual occurrences and aliases must compile");
    assert_eq!(doc.plain_text().split_whitespace().collect::<String>(), "1.HEQQ1");
}

fn physical_page_assertions_compile(program: &str) {
    let world = mock_world(program);
    let (result, warnings) = compile_query_context(&world, &world.source);
    assert!(warnings.is_empty(), "{warnings:?}");
    assert!(result.is_ok(), "{program}\n{result:?}");
}

#[test]
fn physical_page_header_one_page_counter_membership() {
    physical_page_assertions_compile(
        r#"#set page(width:200pt,height:200pt,margin:30pt,header:figure([H],caption:[HC]))
#figure([B],caption:[BC])
#context {
  assert.eq(query(figure).len(), 2)
  assert.eq(counter(figure).final(), (2,))
  assert.eq(query(figure).map(it => counter(figure).at(it.location())), ((1,), (2,)))
}"#,
    );
}

#[test]
fn physical_page_header_repeats_as_distinct_occurrences() {
    physical_page_assertions_compile(
        r#"#set page(width:220pt,height:240pt,margin:60pt,header:[#figure([HEADER],caption:[HC]) <header>])
#figure([BODYONE],caption:[B1])
#pagebreak()
#figure([BODYTWO],caption:[B2])
#context {
  assert.eq(query(figure).len(), 4)
  assert.eq(query(figure).map(it => it.location()).dedup().len(), 4)
  assert.eq(query(figure).map(it => it.location().page()), (1, 1, 2, 2))
  assert.eq(query(<header>).map(it => counter(figure).at(it.location())), ((1,), (3,)))
  assert.eq(counter(figure).final(), (4,))
}"#,
    );
}

#[test]
fn physical_page_footer_actions_follow_body() {
    physical_page_assertions_compile(
        r#"#set page(width:220pt,height:240pt,margin:60pt,footer:[#figure([FOOTER],caption:[FC]) <footer>])
#figure([BODYONE],caption:[B1])
#pagebreak()
#figure([BODYTWO],caption:[B2])
#context {
  assert.eq(query(figure).len(), 4)
  assert.eq(query(<footer>).map(it => counter(figure).at(it.location())), ((2,), (4,)))
  assert.eq(query(<footer>).map(it => it.location().page()), (1, 2))
  assert.eq(counter(figure).final(), (4,))
}"#,
    );
}

#[test]
fn physical_page_context_header_uses_real_pages_and_effective_styles() {
    let world = mock_world(
        r#"#set text(10pt)
#set page(width:220pt,height:240pt,margin:60pt,header:context metadata((here().page(), repr(text.size))))
#set text(20pt)
BODYONE
#pagebreak()
#set text(30pt)
BODYTWO"#,
    );
    let intr = compile_query_context(&world, &world.source)
        .0
        .expect("contextual header");
    let values = intr.query_metadata();
    assert_eq!(values.len(), 2, "{values:?}");
    for (value, page, size) in [(&values[0], 1, "20pt"), (&values[1], 2, "30pt")] {
        let Value::Array(fields) = value else { panic!("{value:?}") };
        assert_eq!(fields.len(), 2);
        assert!(matches!(&fields[0], Value::Int(number) if *number == page));
        assert!(matches!(&fields[1], Value::Str(actual) if actual == size));
    }
}

#[test]
fn physical_page_inner_styles_override_page_environment() {
    let world = mock_world(
        r#"#set page(width:220pt,height:240pt,margin:60pt,header:[#set text(18pt)
#context metadata("H" + repr(text.size))],footer:[#set text(12pt)
#context metadata("F" + repr(text.size))])
#set text(20pt)
BODY"#,
    );
    let intr = compile_query_context(&world, &world.source).0.expect("inner styles");
    let values = intr.query_metadata();
    assert_eq!(values.len(), 2, "{values:?}");
    assert!(matches!(&values[0], Value::Str(value) if value == "H18pt"));
    assert!(matches!(&values[1], Value::Str(value) if value == "F12pt"));
}

#[test]
fn physical_page_overridden_dormant_template_is_inert() {
    physical_page_assertions_compile(
        r#"#set page(width:220pt,height:240pt,margin:60pt,header:[#figure([UNUSED],caption:[UNUSED],supplement:it=>panic("UNEMITTED_HEADER"))])
#set page(header:none)
#figure([BODY],caption:[BC])
#context {
  assert.eq(query(figure).len(), 1)
  assert.eq(counter(figure).final(), (1,))
}"#,
    );
}

#[test]
fn physical_page_repeated_header_reference_reports_ambiguity() {
    let world = mock_world(
        r#"#set page(width:220pt,height:240pt,margin:60pt,header:[#figure([HEADER],caption:[HC]) <header>])
#figure([BODYONE],caption:[B1])
See @header.
#pagebreak()
#figure([BODYTWO],caption:[B2])
See @header."#,
    );
    let errors = compile_query_context(&world, &world.source).0.unwrap_err();
    assert_eq!(errors.len(), 2, "{errors:?}");
    for error in errors {
        assert_eq!(
            error.message,
            "label `<header>` occurs multiple times in the document"
        );
        assert_eq!(error.span.id(), Some(world.source.id()));
    }
}

#[test]
fn physical_page_all_four_slots_share_one_causal_order() {
    physical_page_assertions_compile(
        r#"#set page(width:220pt,height:240pt,margin:60pt,background:[#figure([BACKGROUND],caption:[BGC])],header:[#figure([HEADER],caption:[HC])],footer:[#figure([FOOTER],caption:[FC])],foreground:[#figure([FOREGROUND],caption:[FGC])])
#figure([BODY],caption:[BC])
#context {
  assert.eq(query(figure).len(), 5)
  assert.eq(query(figure).map(it => counter(figure).at(it.location())), ((1,), (2,), (3,), (4,), (5,)))
  assert.eq(counter(figure).final(), (5,))
}"#,
    );
}

#[test]
fn physical_page_reused_noncontext_header_retains_realization_warning() {
    let world = mock_world(
        r#"#show "H": it => {
  let _ = numbering("א", 0)
  [HEADER]
}
#set page(width:220pt,height:240pt,margin:60pt,header:[H])
BODYONE
#pagebreak()
BODYTWO"#,
    );
    let (result, warnings) = compile_query_context(&world, &world.source);
    result.expect("repeated show header must compile");
    assert_eq!(warnings.len(), 1, "{warnings:?}");
    assert_eq!(warnings[0].message, "the numeral system `hebrew` cannot represent zero");
    assert_eq!(warnings[0].span.id(), Some(world.source.id()));
}

#[test]
fn physical_page_header_warning_survives_realization_error() {
    let world = mock_world(
        r#"#show "H": it => {
  let _ = numbering("א", 0)
  panic("RUNNING_FAILURE")
}
#set page(width:220pt,height:240pt,margin:60pt,header:[H])
BODY"#,
    );
    let (result, warnings) = compile_query_context(&world, &world.source);
    let errors = result.expect_err("actual requested header must preserve its panic");
    assert!(
        errors.iter().any(|error| error.message.contains("RUNNING_FAILURE")),
        "{errors:?}"
    );
    assert_eq!(warnings.len(), 1, "{warnings:?}");
    assert_eq!(warnings[0].message, "the numeral system `hebrew` cannot represent zero");
    assert_eq!(warnings[0].span.id(), Some(world.source.id()));
}

fn physical_page_assert_deferred_route(insertion: &str) {
    physical_page_assertions_compile(&format!(
        r#"#set page(width:280pt,height:320pt,margin:60pt,header:[#figure([HEADER],caption:[HC]) <header>])
#figure([FIRST],caption:[C1]) <first>
{insertion}
#figure([SECOND],caption:[C2]) <second>
#context {{
  assert.eq(query(figure).len(), 4)
  assert.eq(query(figure).map(it=>it.location()).dedup().len(), 4)
  assert.eq(query(<header>).map(it=>counter(figure).at(it.location())), ((1,),))
  assert.eq(query(<first>).map(it=>counter(figure).at(it.location())), ((2,),))
  assert.eq(query(<deferred>).map(it=>counter(figure).at(it.location())), ((3,),))
  assert.eq(query(<second>).map(it=>counter(figure).at(it.location())), ((4,),))
  assert.eq(counter(figure).final(), (4,))
}}"#
    ));
}

#[test]
fn physical_page_footnote_facts_follow_source_causal_order() {
    physical_page_assert_deferred_route(
        "ANCHOR#footnote[#figure([NOTE],caption:[NC]) <deferred>]",
    );
}

#[test]
fn physical_page_top_float_facts_follow_source_causal_order() {
    physical_page_assert_deferred_route(
        "#place(top,float:true)[#figure([FLOAT],caption:[FC]) <deferred>]",
    );
}

#[test]
fn physical_page_bottom_float_facts_follow_source_causal_order() {
    physical_page_assert_deferred_route(
        "#place(bottom,float:true)[#figure([FLOAT],caption:[FC]) <deferred>]",
    );
}

#[test]
fn physical_page_hidden_facts_preserve_membership() {
    physical_page_assert_deferred_route(
        "#hide[#figure([HIDDEN],caption:[DC]) <deferred>]",
    );
}

#[test]
fn physical_page_cells_and_columns_preserve_source_routes() {
    for body in [
        "#grid(columns:1,[#figure([FIRST],caption:[C1]) <first>],[#figure([SECOND],caption:[C2]) <second>])",
        "#table(columns:1,[#figure([FIRST],caption:[C1]) <first>],[#figure([SECOND],caption:[C2]) <second>])",
        "#set page(columns:2)\n#figure([FIRST],caption:[C1]) <first>\n#colbreak()\n#figure([SECOND],caption:[C2]) <second>",
    ] {
        physical_page_assertions_compile(&format!(
            r#"#set page(width:280pt,height:320pt,margin:60pt,header:[#figure([HEADER],caption:[HC]) <header>])
{body}
#context {{
  assert.eq(query(figure).len(), 3)
  assert.eq(query(figure).map(it=>it.location()).dedup().len(), 3)
  assert.eq(query(<header>).map(it=>counter(figure).at(it.location())), ((1,),))
  assert.eq(query(<first>).map(it=>counter(figure).at(it.location())), ((2,),))
  assert.eq(query(<second>).map(it=>counter(figure).at(it.location())), ((3,),))
  assert.eq(counter(figure).final(), (3,))
}}"#
        ));
    }
}

#[test]
fn physical_page_prepared_header_show_alias_keeps_one_member_per_insertion() {
    physical_page_assertions_compile(
        r#"#show figure.where():it=>[#it#it]
#set page(width:280pt,height:320pt,margin:60pt,header:[#figure([HEADER],caption:[HC]) <header>])
#figure([FIRST],caption:[C1]) <first>
#pagebreak()
#figure([SECOND],caption:[C2]) <second>
#context {
  assert.eq(query(figure).len(), 4)
  assert.eq(query(figure).map(it=>it.location()).dedup().len(), 4)
  assert.eq(query(<header>).map(it=>counter(figure).at(it.location())), ((1,), (3,)))
  assert.eq(query(<first>).map(it=>counter(figure).at(it.location())), ((2,),))
  assert.eq(query(<second>).map(it=>counter(figure).at(it.location())), ((4,),))
  assert.eq(counter(figure).final(), (4,))
}"#,
    );
}

fn physical_page_compile_bundle(source: &str) -> Vec<BundleFile> {
    use typst_core::entities::compiler_features::{Feature, Features};
    let world = mock_world(source);
    let mut features = Features::empty();
    features.enable(Feature::Bundle);
    features.enable(Feature::Html);
    compile_to_bundle_files(
        &world,
        &world.source,
        BundleCompileOptions {
            full_error: false,
            features,
            document_id: None,
            pdf_stream_mode: StreamMode::Verbose,
            pdf_serialization: PdfSerializationMode::Compact,
            pdf_tagged_override: None,
            pdf_pages_override: None,
            pdf_standard_override: None,
            png_ppi_override: None,
            html_serialization: crate::export::HtmlSerializationMode::Crystalline,
            pretty_override: None,
        },
    )
    .0
    .expect("complete global physical Bundle generation")
}

#[test]
fn physical_page_bundle_uses_global_index_and_skips_html_template() {
    let files = physical_page_compile_bundle(
        r#"#document("first.pdf")[
#set page(width:220pt,height:240pt,margin:60pt,header:[#figure([H1],caption:[HC1]) <header>])
#figure([B1],caption:[BC1])
#context {
  assert.eq(query(figure).len(), 4)
  assert.eq(query(<header>).map(it=>counter(figure).at(it.location())), ((1,), (3,)))
  assert.eq(counter(figure).final(), (4,))
}]
#document("ignored.html")[
#set page(header:context panic("HTML_HEADER_MUST_STAY_DORMANT"))
HTML]
#document("last.pdf")[
#set page(width:220pt,height:240pt,margin:60pt,header:[#figure([H2],caption:[HC2]) <header>])
#figure([B2],caption:[BC2])
#context {
  assert.eq(query(figure).len(), 4)
  assert.eq(counter(figure).final(), (4,))
}]"#,
    );
    assert_eq!(
        files.iter().map(|file| file.path.as_str()).collect::<Vec<_>>(),
        ["first.pdf", "ignored.html", "last.pdf"]
    );
    assert!(!files[0].is_html_document);
    assert!(files[1].is_html_document);
    assert!(!files[2].is_html_document);
}

#[test]
fn physical_page_bundle_context_keeps_effective_style_and_page_scope() {
    let files = physical_page_compile_bundle(
        r#"#document("first.pdf")[
#set page(width:220pt,height:240pt,margin:60pt,header:context metadata((target(),repr(text.size),counter(page).get())))
#set text(20pt)
FIRST]
#document("bridge.html")[BRIDGE]
#document("last.pdf")[
#set page(width:220pt,height:240pt,margin:60pt,header:context metadata((target(),repr(text.size),counter(page).get())))
#set text(30pt)
LAST
#context assert.eq(query(metadata).map(it=>it.value), (("paged","20pt",(1,)), ("paged","30pt",(1,))))]"#,
    );
    assert_eq!(files.len(), 3);
    assert!(files[1].is_html_document);
}

#[test]
fn physical_page_partial_body_styles_do_not_replace_page_environment() {
    for (body, expected) in [
        ("BODY #text(size:10pt)[LOCAL]\n", "20pt"),
        ("#block[#set text(10pt)\nLOCAL]\n", "20pt"),
        ("BEFORE #text(size:10pt)[LOCAL] AFTER\n", "20pt"),
        ("#set text(30pt)\nBODY\n", "30pt"),
    ] {
        let world = mock_world(&format!(
            "#set text(20pt)\n#set page(width:280pt,height:320pt,margin:60pt,header:context metadata(repr(text.size)))\n{body}"
        ));
        let (result, warnings) = compile_query_context(&world, &world.source);
        assert!(warnings.is_empty(), "{body}: {warnings:?}");
        let intr = result.expect("page style probe must compile");
        let values = intr.query_metadata();
        assert_eq!(values.len(), 1, "{body}: {values:?}");
        assert!(
            matches!(&values[0], Value::Str(value) if value == expected),
            "{body}: {values:?}"
        );
    }
}

#[test]
fn physical_page_body_query_with_contextual_header_retains_descendants() {
    physical_page_assertions_compile(
        r#"#set page(width:220pt,height:240pt,margin:60pt,header:context [#figure([HEADER],caption:[HC]) <header>])
#figure([BODY],caption:[BC])
#context {
  assert.eq(query(figure).len(), 2)
  assert.eq(query(figure).map(it=>it.location()).dedup().len(), 2)
  assert.eq(query(figure).map(it=>counter(figure).at(it.location())), ((1,), (2,)))
  assert.eq(counter(figure).final(), (2,))
  metadata((query(figure).len(),counter(figure).final()))
}
"#,
    );
}

#[test]
fn physical_page_changed_running_membership_updates_final_paint() {
    let world = mock_world(
        r#"#set page(width:280pt,height:320pt,margin:60pt,header:context {
  if counter(heading).final().first() > 0 {
    figure([HEADER],caption:[HC])
  }
})
#set heading(numbering:"1.")
= ROOT
#figure([BODY],caption:[BC])
"#,
    );
    let (result, warnings) = compile_to_paged_document_full_error(
        &world,
        &world.source,
        false,
        &mut Timings::default(),
    );
    assert!(warnings.is_empty(), "{warnings:?}");
    let document = result.expect("changed running contribution must compile");
    let text: String = document
        .pages
        .iter()
        .flat_map(|page| {
            [
                frame_items_text(&page.background),
                frame_items_text(&page.items),
                frame_items_text(&page.foreground),
            ]
        })
        .collect();
    let compact: String = text.split_whitespace().collect();
    assert!(compact.contains("Figure1:HC"), "{text}");
    assert!(compact.contains("Figure2:BC"), "{text}");
}

#[test]
fn physical_page_existing_outline_phase_preserves_body_facts() {
    for child in [
        r#"#metadata("borrowed") <borrowed>"#,
        r#"#figure([INNER],caption:none) <borrowed>"#,
    ] {
        for prepared_entry in [false, true] {
            let setup = if prepared_entry {
                "#show outline.entry: it=>it.body()\n"
            } else {
                "#outline()\n"
            };
            let entry = if prepared_entry {
                "#context outline.entry(1,query(heading).first())\n"
            } else {
                ""
            };
            let program = format!(
                "#set page(width:320pt,height:420pt,margin:50pt,header:[RUNNING])\n{setup}#heading([HEADING {child}])\n{entry}#context {{\n assert.eq(query(heading).filter(it=>it.outlined).len(),1)\n assert(query(<borrowed>).len()>0)\n}}\n"
            );
            physical_page_assertions_compile(&program);
        }
    }
}

#[test]
fn physical_page_outline_generated_context_retains_actual_environment() {
    physical_page_assertions_compile(
        r#"#set text(size:20pt)
#let deferred = context { metadata(text.size) }
#show outline.entry: it=>[#deferred #it.body()]
#set page(width:320pt,height:420pt,margin:50pt,header:[RUNNING])
#outline(title:none)
= HEADING
#context { assert.eq(query(metadata).map(it=>it.value), (20pt,)) }
"#,
    );
}

fn physical_page_assert_deferred_crosspage(
    program: &str,
    expected_pages: [i64; 5],
    second_header_state: i64,
    second_page: i64,
    second_state: i64,
    expected_first_note_text: Option<&str>,
) {
    let world = mock_world(program);
    let (result, warnings) =
        compile_to_paged_document_full_error_and_features_with_introspector(
            &world,
            &world.source,
            false,
            typst_core::entities::compiler_features::Features::default(),
            &mut Timings::default(),
        );
    assert!(warnings.is_empty(), "{warnings:?}");
    let compiled = result.unwrap_or_else(|errors| {
        panic!("deferred cross-page observations must compile: {program}\n{errors:?}")
    });
    assert_eq!(compiled.document.pages.len(), 2);
    let values = compiled.introspector.query_metadata();
    assert_eq!(values.len(), 1, "{values:?}");
    let Value::Dict(fields) = &values[0] else { panic!("{:?}", values[0]) };
    let integers =
        |values: &[i64]| Value::Array(values.iter().copied().map(Value::Int).collect());
    let snapshot = |page, state| Value::Array(vec![Value::Int(page), integers(&[state])]);
    let expected = [
        ("count", Value::Int(5)),
        ("unique", Value::Int(5)),
        ("pages", integers(&expected_pages)),
        ("states", Value::Array((1..=5).map(|state| integers(&[state])).collect())),
        ("final", integers(&[5])),
        ("header", Value::Array(vec![snapshot(1, 1), snapshot(2, second_header_state)])),
        ("first", Value::Array(vec![snapshot(1, 2)])),
        ("second", Value::Array(vec![snapshot(second_page, second_state)])),
        ("deferred", Value::Array(vec![snapshot(2, 3)])),
    ];
    assert_eq!(fields.len(), expected.len(), "{fields:?}");
    let visible_pages: Vec<_> = compiled
        .document
        .pages
        .iter()
        .map(|page| {
            [
                frame_items_text(&page.background),
                frame_items_text(&page.items),
                frame_items_text(&page.foreground),
            ]
            .concat()
        })
        .collect();
    for (key, value) in expected {
        assert_eq!(
            fields.get(key),
            Some(&value),
            "{key}: {fields:?}; visible_pages: {visible_pages:?}"
        );
    }
    if let Some(expected_note_text) = expected_first_note_text {
        let page_text = |index: usize| {
            let page = &compiled.document.pages[index];
            [
                frame_items_text(&page.background),
                frame_items_text(&page.items),
                frame_items_text(&page.foreground),
            ]
            .concat()
            .split_whitespace()
            .collect::<String>()
        };
        let first_page = page_text(0);
        let second_page = page_text(1);
        assert!(first_page.contains("FIRST"), "{first_page}");
        assert!(first_page.contains(expected_note_text), "{first_page}");
        assert!(!first_page.contains("SECOND"), "{first_page}");
        assert!(!first_page.contains("Figure3:NC"), "{first_page}");
        assert!(second_page.contains("SECOND"), "{second_page}");
        assert!(second_page.contains("Figure5:C2"), "{second_page}");
        assert!(second_page.contains("Figure3:NC"), "{second_page}");
    }
}

#[test]
fn physical_page_deferred_note_causes_next_body_page() {
    physical_page_assert_deferred_crosspage(
        r###"#set page(width:280pt,height:320pt,margin:60pt,header:[#figure([HEADER],caption:[HC]) <header>])
#figure([FIRST],caption:[C1]) <first>
ANCHOR#footnote[#block(height:110pt)[NOTE TEXT] #figure([NOTE],caption:[NC]) <deferred>]
#figure([SECOND],caption:[C2]) <second>

#context [#metadata((
 count: query(figure).len(),
 unique: query(figure).map(it=>it.location()).dedup().len(),
 pages: query(figure).map(it=>it.location().page()),
 states: query(figure).map(it=>counter(figure).at(it.location())),
 final: counter(figure).final(),
 header: query(<header>).map(it=>(it.location().page(),counter(figure).at(it.location()))),
 first: query(<first>).map(it=>(it.location().page(),counter(figure).at(it.location()))),
 second: query(<second>).map(it=>(it.location().page(),counter(figure).at(it.location()))),
 deferred: query(<deferred>).map(it=>(it.location().page(),counter(figure).at(it.location()))),
)) <probe>]
"###,
        [1, 1, 2, 2, 2],
        4,
        2,
        5,
        Some("NOTETEXT"),
    );
}

#[test]
fn physical_page_deferred_note_child_on_later_page() {
    physical_page_assert_deferred_crosspage(
        r###"#set page(width:280pt,height:320pt,margin:60pt,header:[#figure([HEADER],caption:[HC]) <header>])
#figure([FIRST],caption:[C1]) <first>
ANCHOR#footnote[START #v(180pt) #figure([NOTE],caption:[NC]) <deferred>]
#figure([SECOND],caption:[C2]) <second>

#context [#metadata((
 count: query(figure).len(),
 unique: query(figure).map(it=>it.location()).dedup().len(),
 pages: query(figure).map(it=>it.location().page()),
 states: query(figure).map(it=>counter(figure).at(it.location())),
 final: counter(figure).final(),
 header: query(<header>).map(it=>(it.location().page(),counter(figure).at(it.location()))),
 first: query(<first>).map(it=>(it.location().page(),counter(figure).at(it.location()))),
 second: query(<second>).map(it=>(it.location().page(),counter(figure).at(it.location()))),
 deferred: query(<deferred>).map(it=>(it.location().page(),counter(figure).at(it.location()))),
)) <probe>]
"###,
        [1, 1, 2, 2, 2],
        4,
        2,
        5,
        Some("START"),
    );
}

#[test]
fn physical_page_deferred_top_place_float_next_page() {
    physical_page_assert_deferred_crosspage(
        r###"#set page(width:280pt,height:320pt,margin:60pt,header:[#figure([HEADER],caption:[HC]) <header>])
#figure(block(height:120pt)[FIRST],caption:[C1]) <first>
#place(top,float:true)[#figure(block(height:100pt)[FLOAT],caption:[FC]) <deferred>]
#figure([SECOND],caption:[C2]) <second>

#context [#metadata((
 count: query(figure).len(),
 unique: query(figure).map(it=>it.location()).dedup().len(),
 pages: query(figure).map(it=>it.location().page()),
 states: query(figure).map(it=>counter(figure).at(it.location())),
 final: counter(figure).final(),
 header: query(<header>).map(it=>(it.location().page(),counter(figure).at(it.location()))),
 first: query(<first>).map(it=>(it.location().page(),counter(figure).at(it.location()))),
 second: query(<second>).map(it=>(it.location().page(),counter(figure).at(it.location()))),
 deferred: query(<deferred>).map(it=>(it.location().page(),counter(figure).at(it.location()))),
)) <probe>]
"###,
        [1, 1, 2, 1, 2],
        5,
        1,
        4,
        None,
    );
}

#[test]
fn physical_page_deferred_bottom_place_float_next_page() {
    physical_page_assert_deferred_crosspage(
        r###"#set page(width:280pt,height:320pt,margin:60pt,header:[#figure([HEADER],caption:[HC]) <header>])
#figure(block(height:120pt)[FIRST],caption:[C1]) <first>
#place(bottom,float:true)[#figure(block(height:100pt)[FLOAT],caption:[FC]) <deferred>]
#figure([SECOND],caption:[C2]) <second>

#context [#metadata((
 count: query(figure).len(),
 unique: query(figure).map(it=>it.location()).dedup().len(),
 pages: query(figure).map(it=>it.location().page()),
 states: query(figure).map(it=>counter(figure).at(it.location())),
 final: counter(figure).final(),
 header: query(<header>).map(it=>(it.location().page(),counter(figure).at(it.location()))),
 first: query(<first>).map(it=>(it.location().page(),counter(figure).at(it.location()))),
 second: query(<second>).map(it=>(it.location().page(),counter(figure).at(it.location()))),
 deferred: query(<deferred>).map(it=>(it.location().page(),counter(figure).at(it.location()))),
)) <probe>]
"###,
        [1, 1, 2, 1, 2],
        5,
        1,
        4,
        None,
    );
}

// Source-qualified integrated acceptance for the approved Repeat physical channel.
fn ordinary_repeat_zero_copy_pipeline_source(header: bool, position: bool) {
    let original = r###"#set page(width:240pt,height:auto,margin:8pt)
#set text(font:"DejaVu Sans",size:11pt)
#let s=state("acceptance-repeat-source",0)
#let c=counter("acceptance-repeat-source")
BEFORE#box(width:80pt,height:16pt)[#repeat([#s.update(n=>n+1)#c.step()X#metadata("source") <acceptance-repeat-source>],gap:-100pt,justify:false)]TAIL
#context [STATE#s.final() COUNTER#c.final().first() LABELS#query(<acceptance-repeat-source>).len()]
"###;
    let original = if position {
        original.replace(
            "#context [STATE#s.final() COUNTER#c.final().first() LABELS#query(<acceptance-repeat-source>).len()]",
            "#context { let p=here().position(); [STATE#s.final() COUNTER#c.final().first() LABELS#query(<acceptance-repeat-source>).len() PAGE#here().page() POSPAGE#p.page POSX#p.x POSY#p.y] }",
        )
    } else {
        original.to_owned()
    };
    let program = if header {
        original.replacen(
            "height:auto,margin:8pt)",
            "height:auto,margin:8pt,header:[])",
            1,
        )
    } else {
        original.to_owned()
    };
    let bytes = include_bytes!("../../fixtures/fonts/DejaVuSans.ttf");
    let world =
        crate::world::SystemWorld::for_stdin(std::env::temp_dir(), program.clone())
            .with_fonts(vec![crate::fonts::FontSlot::new_embedded(
                "<DejaVu Sans source-qualified fixture>".into(),
                bytes.to_vec(),
            )]);
    let source = world.source(world.main()).expect("actual stdin source");
    let (result, warnings) =
        compile_to_paged_document_full_error_and_features_with_introspector(
            &world,
            &source,
            false,
            typst_core::entities::compiler_features::Features::default(),
            &mut Timings::default(),
        );
    assert!(warnings.is_empty(), "unexpected warnings, header={header}: {warnings:?}");
    let compiled = result.unwrap_or_else(|errors| {
        panic!("actual source must compile, header={header}: {errors:?}")
    });
    assert_eq!(compiled.document.pages.len(), 1);
    let text: String = compiled
        .document
        .pages
        .iter()
        .map(|page| frame_items_text(&page.items))
        .collect::<String>()
        .chars()
        .filter(|ch| !ch.is_whitespace())
        .collect();
    for expected in ["STATE0", "COUNTER0", "LABELS0"] {
        assert!(text.contains(expected), "header={header}, missing {expected}: {text}");
    }
    if position {
        for expected in ["PAGE1", "POSPAGE1", "POSX158.33pt", "POSY16.36pt"] {
            assert!(
                text.contains(expected),
                "header={header}, missing {expected}: {text}"
            );
        }
    }
    assert_eq!(text.matches("BEFORE").count(), 1);
    assert_eq!(text.matches("TAIL").count(), 1);
    let piece_observation = if position {
        assert_eq!(text.matches("POSX").count(), 1);
        text.replace("POSX", "")
    } else {
        text.clone()
    };
    assert!(!piece_observation.contains('X'), "no actual Repeat copy exists: {text}");
    assert!(compiled.introspector.query_metadata().is_empty());
    assert_eq!(
        compiled.introspector.label_count(&typst_core::entities::label::Label(
            "acceptance-repeat-source".into()
        )),
        0
    );
}

#[test]
fn ordinary_repeat_zero_copy_pipeline_without_running_marginal() {
    ordinary_repeat_zero_copy_pipeline_source(false, false);
}

#[test]
fn ordinary_repeat_zero_copy_pipeline_with_empty_header() {
    ordinary_repeat_zero_copy_pipeline_source(true, false);
}

#[test]
fn ordinary_repeat_zero_copy_pipeline_position_without_running_marginal() {
    ordinary_repeat_zero_copy_pipeline_source(false, true);
}

#[test]
fn ordinary_repeat_zero_copy_pipeline_position_with_empty_header() {
    ordinary_repeat_zero_copy_pipeline_source(true, true);
}

#[test]
fn finite_rollback_observes_complete_settled_pages() {
    let program = r###"#set page(width:180pt,height:72pt,margin:8pt)
#set text(font:"DejaVu Sans",size:11pt)
BEFORE

#box(width:1fr,height:40pt)[LEFT#h(1fr)RIGHT]TAIL
"###;
    let world =
        crate::world::SystemWorld::for_stdin(std::env::temp_dir(), program.to_owned())
            .with_fonts(vec![crate::fonts::FontSlot::new_embedded(
                "<DejaVu Sans complete-page parity fixture>".into(),
                include_bytes!("../../fixtures/fonts/DejaVuSans.ttf").to_vec(),
            )]);
    let source = world.source(world.main()).expect("actual stdin source");
    let (result, warnings) = compile_to_paged_document_full_error(
        &world,
        &source,
        false,
        &mut Timings::default(),
    );
    assert!(warnings.is_empty(), "unexpected warnings: {warnings:?}");
    let document = result.expect("the immutable finite-page source must compile");
    assert_eq!(document.pages.len(), 2);
    let text: Vec<String> = document
        .pages
        .iter()
        .map(|page| frame_items_text(&page.items).split_whitespace().collect())
        .collect();
    assert_eq!(text, ["BEFORE", "LEFTRIGHTTAIL"]);
    let fonts = resolve_export_fonts(&document, &world).expect("actual bound fonts");
    assert!(!fonts.is_empty(), "the declared DejaVu font must be bound");
    for (index, page) in document.pages.iter().enumerate() {
        let destinations = svg_destination_context_for_page(&document, index + 1);
        let glyph_fonts = svg_glyph_font_context_for_page(page, &world);
        let svg = export_svg_with_fonts_and_contexts(
            page,
            &crate::export::SvgOptions::default(),
            &fonts,
            &destinations,
            &glyph_fonts,
        );
        assert!(svg.starts_with("<svg"), "all pages must have actual SVG output");
        println!("TEKT_FINITE_SVG_PAGE\t{}\t{}", index + 1, svg);
    }
}

fn remaining_html_compile(
    source: &str,
) -> (Result<String, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>, Source) {
    use typst_core::entities::compiler_features::{Feature, Features};
    let world = crate::world::SystemWorld::for_stdin(std::env::temp_dir(), source.into())
        .with_fonts(vec![crate::fonts::FontSlot::new_embedded(
            "<HTML page-rule regression DejaVu>".into(),
            include_bytes!("../../fixtures/fonts/DejaVuSans.ttf").to_vec(),
        )]);
    let source = world.source(world.main()).unwrap();
    let mut features = Features::empty();
    features.enable(Feature::Html);
    let (result, warnings) =
        compile_to_html_string_with_features(&world, &source, features);
    (result, warnings, source)
}

#[test]
fn remaining_html_figure_show_products_preserve_block_morphology() {
    let text = "#set figure(kind:raw)\n#show figure:it=>repr(it.kind)\n#figure(table([A]))\n#figure(table([A]),kind:auto)\n";
    let (result, warnings, _) = remaining_html_compile(text);
    let html = result.expect("both measured Figure kind replacements compile");
    assert!(html.ends_with("<body><p>raw</p><p>table</p></body></html>"), "{html}");
    assert!(warnings.is_empty(), "unexpected pipeline diagnostic: {warnings:?}");
}

#[test]
fn remaining_html_suppressed_nonfigure_products_keep_empty_body() {
    let (result, warnings, _) =
        remaining_html_compile("#show heading: none\n#heading[A]#heading[B]");
    let html = result.expect("suppressed Heading products compile");
    assert!(html.ends_with("<body></body></html>"), "{html}");
    assert!(warnings.is_empty(), "unexpected pipeline diagnostic: {warnings:?}");
}

#[test]
fn remaining_html_suppressed_figure_product_keeps_surrounding_paragraphs() {
    let (result, warnings, _) =
        remaining_html_compile("#show figure: none\nA#figure[X]B");
    let html = result.expect("suppressed Figure product compiles");
    assert!(html.ends_with("<body><p>A</p><p>B</p></body></html>"), "{html}");
    assert!(warnings.is_empty(), "unexpected pipeline diagnostic: {warnings:?}");
}

#[test]
fn remaining_html_page_warning_keeps_true_source_and_dormant_template() {
    for text in [
        "#set page(width:240pt,header:context panic(\"DORMANT\"))\nHTML\n",
        "#let p=page\n#set p(width:240pt,header:context panic(\"DORMANT\"))\nHTML\n",
        "#set page(width:240pt)\n",
    ] {
        let (result, warnings, source) = remaining_html_compile(text);
        let html = result.unwrap_or_else(|e| panic!("{text}: {e:?}"));
        if text.contains("HTML") {
            assert!(html.contains("<p>HTML</p>"), "{html}");
        }
        let page: Vec<_> = warnings
            .iter()
            .filter(|w| w.message == "page set rule was ignored during HTML export")
            .collect();
        assert_eq!(page.len(), 1, "{text}: {warnings:?}");
        let start = text.find("set ").unwrap();
        let end = text[start..].find('\n').map(|n| start + n).unwrap_or(text.len());
        assert_eq!(source.span_byte_range(page[0].span), Some(start..end));
        assert!(page[0].hints.is_empty());
        assert!(
            !warnings.iter().any(|w| w.message.contains("ainda não suportado")),
            "{warnings:?}"
        );
    }
}

#[test]
fn remaining_html_page_false_and_uninserted_remain_inert() {
    for text in [
        "#set page(width:240pt) if false\nHTML\n",
        "#let c=[#set page(width:240pt)\nDORMANT]\nHTML\n",
    ] {
        let (result, warnings, _) = remaining_html_compile(text);
        assert!(result.unwrap().contains("<p>HTML</p>"));
        assert!(
            !warnings.iter().any(|w| w.message.contains("page set rule")),
            "{warnings:?}"
        );
    }
}

fn remaining_html_page_container_error(text: &str) {
    let (result, warnings, source) = remaining_html_compile(text);
    let errors =
        result.expect_err("a reached page setting inside a container must be refused");
    assert_eq!(errors.len(), 1, "{errors:?}");
    assert_eq!(
        errors[0].message,
        "page configuration is not allowed inside of containers"
    );
    let start = text.find("set page(").unwrap();
    let end = text[start..].find('\n').map(|n| start + n).unwrap();
    assert_eq!(source.span_byte_range(errors[0].span), Some(start..end));
    assert!(
        !warnings.iter().any(|w| w.message.contains("page set rule")),
        "{warnings:?}"
    );
}

#[test]
fn remaining_html_page_nested_content_is_refused() {
    remaining_html_page_container_error("#[#set page(width:240pt)\nINNER]OUTER\n");
}

#[test]
fn remaining_html_page_frame_content_is_refused() {
    remaining_html_page_container_error(
        "#html.frame[#set page(width:80pt,height:auto,margin:0pt)\nFRAME]\nHTML\n",
    );
}

#[test]
fn remaining_html_reused_heading_warning_keeps_first_source_once() {
    let text = "#let deep=heading(level:6)[DEEP]\n#deep\n#deep\n";
    let (result, warnings, source) = remaining_html_compile(text);
    let html = result.expect("reusing a heading remains valid HTML");
    assert_eq!(
        html.matches("<div role=\"heading\" aria-level=\"7\">DEEP</div>")
            .count(),
        2
    );
    let headings: Vec<_> = warnings
        .iter()
        .filter(|w| w.message.starts_with("heading of level 6 was transformed"))
        .collect();
    assert_eq!(headings.len(), 1, "same source occurrence warns once: {warnings:?}");
    let start = text.find("heading(").unwrap();
    assert_eq!(
        source.span_byte_range(headings[0].span),
        Some(start..start + "heading(level:6)[DEEP]".len())
    );
    assert_eq!(
        headings[0].message,
        "heading of level 6 was transformed to <div role=\"heading\" aria-level=\"7\">, which is not supported by all assistive technology"
    );
    assert_eq!(headings[0].hints.len(), 2);
    assert_eq!(headings[0].hints[0], "HTML only supports <h1> to <h6>, not <h7>");
    assert_eq!(
        headings[0].hints[1],
        "you may want to restructure your document so that it doesn't contain deep headings"
    );
}

#[test]
fn remaining_html_distinct_heading_warnings_preserve_causal_page_order() {
    let text =
        "#heading(level:6)[FIRST]\n#set page(width:240pt)\n#heading(level:6)[SECOND]\n";
    let (result, warnings, source) = remaining_html_compile(text);
    let html = result.expect("independent headings remain valid HTML");
    assert!(html.contains("<div role=\"heading\" aria-level=\"7\">FIRST</div><div role=\"heading\" aria-level=\"7\">SECOND</div>"), "{html}");
    let relevant: Vec<_> = warnings
        .iter()
        .filter(|w| {
            w.message.starts_with("heading of level 6 was transformed")
                || w.message == "page set rule was ignored during HTML export"
        })
        .collect();
    assert_eq!(
        relevant.len(),
        3,
        "independent source occurrences both warn: {warnings:?}"
    );
    for (warning, spelling) in relevant.iter().zip([
        "heading(level:6)[FIRST]",
        "set page(width:240pt)",
        "heading(level:6)[SECOND]",
    ]) {
        let start = text.find(spelling).unwrap();
        assert_eq!(
            source.span_byte_range(warning.span),
            Some(start..start + spelling.len())
        );
    }
    assert!(relevant[0].message.starts_with("heading of level 6 was transformed"));
    assert_eq!(relevant[1].message, "page set rule was ignored during HTML export");
    assert!(relevant[2].message.starts_with("heading of level 6 was transformed"));
    assert_ne!(relevant[0].span, relevant[2].span);
    assert_eq!(relevant[0].hints, relevant[2].hints);
}

#[test]
fn remaining_html_page_only_body_is_empty_with_original_warning() {
    for (text, count) in [("#set page(width:240pt)\n", 1), ("#set page()\n", 0)] {
        let (result, warnings, _) = remaining_html_compile(text);
        let html = result.expect("page-only source is valid semantic HTML");
        assert!(
            html.ends_with("<body></body></html>"),
            "page setting has no visual paragraph: {html}"
        );
        assert_eq!(
            warnings
                .iter()
                .filter(|w| w.message == "page set rule was ignored during HTML export")
                .count(),
            count
        );
    }
}

#[test]
fn stack_context_origins_follow_physical_child_pages() {
    let world = mock_world(
        r#"#set page(width:400pt,height:400pt,margin:5pt)
#stack(spacing:0pt,..range(20).map(i=>context [#metadata((i,here().page(),here()))<stack-item>#rect(width:30pt,height:20pt)]))"#,
    );
    let (result, warnings) =
        compile_to_paged_document_full_error_and_features_with_introspector(
            &world,
            &world.source,
            false,
            typst_core::entities::compiler_features::Features::default(),
            &mut Timings::default(),
        );
    assert!(warnings.is_empty(), "{warnings:?}");
    let compiled = result.unwrap();
    assert_eq!(compiled.document.pages.len(), 2);
    let intr = compiled.introspector;
    let values = intr.query_metadata();
    eprintln!("contexts={:?} metadata={:?}", intr.context_block_locations, values);
    for location in intr.context_block_locations.values() {
        eprintln!("context {location:?}: {:?}", intr.positions.position_of(*location));
    }
    assert_eq!(values.len(), 20);
    for (index, value) in values.iter().enumerate() {
        let Value::Array(fields) = value else { panic!("{value:?}") };
        assert_eq!(fields[0], Value::Int(index as i64));
        let expected = if index < 19 { 1 } else { 2 };
        let Value::Location(location) = fields[2] else { panic!("{fields:?}") };
        assert_eq!(
            intr.positions.position_of(location).unwrap().page.get(),
            expected,
            "index{index} location{location:?}"
        );
        assert_eq!(fields[1], Value::Int(expected as i64), "index{index}");
    }
}

#[test]
fn issue8894_stroke_thickness_resolves_at_each_font_occurrence() {
    use typst_core::entities::{geometry::LineCap, layout_types::FrameItem};
    fn collect(items: &[FrameItem], strokes: &mut Vec<(f64, LineCap)>) {
        for item in items {
            match item {
                FrameItem::Shape { stroke: Some(stroke), .. } => {
                    strokes.push((stroke.thickness, stroke.cap))
                }
                FrameItem::Group { items, .. }
                | FrameItem::Link { items, .. }
                | FrameItem::Semantic { items, .. }
                | FrameItem::TextReplacement { items, .. } => collect(items, strokes),
                FrameItem::Shape { stroke: None, .. }
                | FrameItem::TextSeparator { .. }
                | FrameItem::Text { .. }
                | FrameItem::TextShaped { .. }
                | FrameItem::Line { .. }
                | FrameItem::Glyph { .. }
                | FrameItem::Image { .. } => {}
            }
        }
    }
    for (thickness, expected) in
        [("10em", [110.0, 200.0]), ("3pt + 2em", [25.0, 43.0]), ("110pt", [110.0, 110.0])]
    {
        for dictionary in [true, false] {
            let stroke = if dictionary {
                format!("(paint: red, thickness: {thickness}, cap: \"round\")")
            } else {
                thickness.into()
            };
            let source = format!("#let s = {stroke}\n#text(size: 11pt)[#curve(stroke: s, curve.move((20pt,20pt)), curve.line((20pt,20pt)))]\n#text(size: 20pt)[#curve(stroke: s, curve.move((20pt,20pt)), curve.line((40pt,20pt)))]");
            let world = mock_world(&source);
            let mut timings = Timings::default();
            let (result, _) = compile_to_paged_document_full_error(
                &world,
                &world.source,
                false,
                &mut timings,
            );
            let doc = result.expect("valid stroke fixture");
            let mut observed = Vec::new();
            for page in &doc.pages {
                collect(&page.items, &mut observed);
            }
            assert_eq!(observed.len(), 2, "{source}");
            for ((width, cap), expected) in observed.into_iter().zip(expected) {
                assert!(
                    (width - expected).abs() < 1e-9,
                    "thickness {width} != {expected}: {source}"
                );
                assert_eq!(cap, if dictionary { LineCap::Round } else { LineCap::Butt });
            }
        }
    }
}

#[test]
fn issue8583_rectangle_keeps_relative_corners_and_side_strokes() {
    use typst_core::entities::{
        frame_paint::FramePaint,
        geometry::ShapeKind,
        layout_types::{Color, FrameItem},
    };
    fn color_is(c: &Color, wanted: [u8; 3]) -> bool {
        let (r, g, b, a) = c.to_rgba_f32();
        a > 0.0
            && [
                (r * 255.0).round() as u8,
                (g * 255.0).round() as u8,
                (b * 255.0).round() as u8,
            ] == wanted
    }
    fn shapes(items: &[FrameItem], out: &mut Vec<FrameItem>) {
        for item in items {
            match item {
                FrameItem::Shape { .. } => out.push(item.clone()),
                FrameItem::Group { items, .. }
                | FrameItem::Link { items, .. }
                | FrameItem::Semantic { items, .. }
                | FrameItem::TextReplacement { items, .. } => shapes(items, out),
                FrameItem::Text { .. }
                | FrameItem::TextShaped { .. }
                | FrameItem::Glyph { .. }
                | FrameItem::TextSeparator { .. }
                | FrameItem::Image { .. }
                | FrameItem::Line { .. } => {}
            }
        }
    }
    for (radius, stroke, rounded, red) in [
        ("(bottom: 100%)", "none", true, false),
        ("0pt", "(bottom: 2em + red, x: .25pt + red)", false, true),
        ("(bottom: 100%)", "(bottom: 2em + red, x: .25pt + red)", true, true),
        ("10pt", "2pt + red", true, true),
    ] {
        let source = format!(
            "#rect(width:60pt,height:60pt,fill:blue,radius:{radius},stroke:{stroke})"
        );
        let world = mock_world(&source);
        let mut timings = Timings::default();
        let (result, _) = compile_to_paged_document_full_error(
            &world,
            &world.source,
            false,
            &mut timings,
        );
        let doc = result.expect("valid rectangle source");
        let mut all = Vec::new();
        for page in &doc.pages {
            shapes(&page.items, &mut all);
        }
        let has_round = all.iter().any(|item| matches!(item, FrameItem::Shape { kind: ShapeKind::RoundedRect { .. }, fill: Some(FramePaint::Solid(c)), .. } if color_is(c,[0,116,217])));
        let has_red = all.iter().any(|item| match item {
            FrameItem::Shape { fill: Some(FramePaint::Solid(c)), .. }
                if color_is(c, [255, 65, 54]) =>
            {
                true
            }
            FrameItem::Shape { stroke: Some(s), .. } => {
                matches!(&s.paint, FramePaint::Solid(c) if color_is(c,[255,65,54]))
            }
            FrameItem::Shape { .. }
            | FrameItem::Group { .. }
            | FrameItem::Link { .. }
            | FrameItem::Semantic { .. }
            | FrameItem::TextReplacement { .. }
            | FrameItem::Text { .. }
            | FrameItem::TextShaped { .. }
            | FrameItem::Glyph { .. }
            | FrameItem::TextSeparator { .. }
            | FrameItem::Image { .. }
            | FrameItem::Line { .. } => false,
        });
        assert_eq!(has_round, rounded, "relative corner lost: {source}");
        assert_eq!(has_red, red, "side stroke lost: {source}");
    }
}
