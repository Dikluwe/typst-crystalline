//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/tests/pdf_page_labels_tests.md
//! @prompt-hash 73fb9bf5
//! @layer L3
//! @updated 2026-09-24

use super::*;
use regex::Regex;
use std::num::NonZeroUsize;
use typst_core::entities::document_format_options::PageRange;
use typst_core::entities::layout_types::{Page, PagedDocument};

fn page(numbering: Option<&str>) -> Page {
    Page {
        width: 100.0,
        height: 100.0,
        numbering: numbering.map(Into::into),
        logical_number: 1,
        supplement: Content::Empty,
        bleed: Default::default(),
        fill: Default::default(),
        background: vec![],
        foreground: vec![],
        items: vec![],
    }
}

fn four_pages(numbering: Option<&str>, logical_start: usize) -> Vec<Page> {
    (0..4)
        .map(|index| {
            let mut page = page(numbering);
            page.logical_number = logical_start + index;
            page
        })
        .collect()
}

fn selected(pages: Vec<Page>, physical: Option<&[usize]>) -> Vec<u8> {
    selected_with_options(pages, physical, PdfTags::Disabled, PdfStandards::default())
}

fn selected_with_options(
    pages: Vec<Page>,
    physical: Option<&[usize]>,
    tags: PdfTags,
    standards: PdfStandards,
) -> Vec<u8> {
    let ranges = physical.map(|indices| {
        PageRanges::new(
            indices
                .iter()
                .map(|&i| PageRange::at(NonZeroUsize::new(i).unwrap()))
                .collect(),
        )
    });
    export_pdf_with_document_id_and_tags_and_serialization_and_pages_and_standard(
        &PagedDocument::new(pages),
        None,
        StreamMode::Verbose,
        tags,
        PdfSerializationMode::Compact,
        ranges,
        standards,
    )
    .output
}

fn dict(pdf: &[u8], id: usize) -> String {
    extract_object_dict_text(pdf, id).expect("objeto PDF existente")
}

fn reference_after(text: &str, key: &str) -> Option<usize> {
    let tail = text.split_once(key)?.1;
    let re = Regex::new(r"^\s*(\d+)\s+0\s+R").unwrap();
    re.captures(tail).map(|m| m[1].parse().unwrap())
}

fn pdf_utf16_prefix(dict: &str) -> String {
    let re = Regex::new(r"/P\s*<([[:xdigit:]]+)>").unwrap();
    let Some(hex) = re.captures(dict) else { return String::new() };
    let units: Vec<u16> = hex[1]
        .as_bytes()
        .chunks_exact(4)
        .map(|chunk| {
            u16::from_str_radix(std::str::from_utf8(chunk).unwrap(), 16).unwrap()
        })
        .collect();
    assert_eq!(units.first(), Some(&0xFEFF));
    String::from_utf16(&units[1..]).unwrap()
}

/// Lê os rótulos como o leitor: aplica os pontos de arranque da number tree,
/// nunca confunde o índice zero-based do PDF com o número físico original.
fn page_labels(pdf: &[u8]) -> Vec<String> {
    let pages = dict(pdf, 2);
    let count: usize = Regex::new(r"/Count\s+(\d+)")
        .unwrap()
        .captures(&pages)
        .expect("/Count")[1]
        .parse()
        .unwrap();
    let catalog = dict(pdf, 1);
    let Some(tree_id) = reference_after(&catalog, "/PageLabels") else {
        return (1..=count).map(|n| n.to_string()).collect();
    };
    let tree = dict(pdf, tree_id);
    let refs = Regex::new(r"(\d+)\s+(\d+)\s+0\s+R").unwrap();
    let start_re = Regex::new(r"/St\s+(\d+)").unwrap();
    let style_re = Regex::new(r"/S\s*/([A-Za-z]+)").unwrap();
    let mut starts = Vec::new();
    for entry in refs.captures_iter(tree.split_once("/Nums").expect("/Nums").1) {
        let index: usize = entry[1].parse().unwrap();
        let label = dict(pdf, entry[2].parse().unwrap());
        let start = start_re
            .captures(&label)
            .map(|m| m[1].parse::<usize>().unwrap())
            .unwrap_or(1);
        let style = style_re.captures(&label).map(|m| m[1].to_string());
        starts.push((index, start, style, pdf_utf16_prefix(&label)));
    }
    (0..count)
        .map(|index| {
            let &(at, start, ref style, ref prefix) = starts
                .iter()
                .rev()
                .find(|(at, _, _, _)| *at <= index)
                .expect("primeira label em índice 0");
            let number = start + index - at;
            let ordinal = match style.as_deref() {
                Some("D") => number.to_string(),
                Some("r") => match number {
                    1 => "i".into(),
                    2 => "ii".into(),
                    3 => "iii".into(),
                    4 => "iv".into(),
                    _ => panic!("romano fora da fixture"),
                },
                None => String::new(),
                other => panic!("estilo PDF de teste inesperado: {other:?}"),
            };
            format!("{prefix}{ordinal}")
        })
        .collect()
}

#[test]
fn p1486_selecao_descontinua_preserva_rotulos_arabicos_originais() {
    let pdf = selected(four_pages(Some("1"), 1), Some(&[1, 3]));
    assert_eq!(page_labels(&pdf), ["1", "3"]);
}

#[test]
fn p1486_matriz_semantica_de_rotulos() {
    let selections: [(Option<&[usize]>, &[usize]); 5] = [
        (None, &[1, 2, 3, 4]),
        (Some(&[1, 2, 3, 4]), &[1, 2, 3, 4]),
        (Some(&[2, 3, 4]), &[2, 3, 4]),
        (Some(&[1, 3]), &[1, 3]),
        (Some(&[4]), &[4]),
    ];
    for (numbering, start) in [(Some("1"), 1), (Some("i"), 1), (None, 1), (Some("1"), 10)]
    {
        for (selection, physical) in selections {
            let pdf = selected(four_pages(numbering, start), selection);
            let actual = page_labels(&pdf);
            let expected: Vec<String> = physical
                .iter()
                .map(|&page| {
                    let number = start + page - 1;
                    if numbering == Some("i") {
                        ["i", "ii", "iii", "iv"][number - 1].to_string()
                    } else if numbering.is_none() {
                        page.to_string()
                    } else {
                        number.to_string()
                    }
                })
                .collect();
            assert_eq!(
                actual, expected,
                "numbering={numbering:?}, start={start}, selection={selection:?}"
            );
            let catalog = dict(&pdf, 1);
            assert_eq!(
                catalog.contains("/PageLabels"),
                numbering.is_some() || physical.len() < 4
            );
        }
    }
}

#[test]
fn p1486_prefixo_sufixo_literal_e_selecao_vazia() {
    let prefixed = selected(four_pages(Some("p. 1"), 1), Some(&[1, 3]));
    assert_eq!(page_labels(&prefixed), ["p. 1", "p. 3"]);
    let suffixed = selected(four_pages(Some("(1)"), 1), Some(&[2, 4]));
    assert_eq!(page_labels(&suffixed), ["(2)", "(4)"]);
    let empty = selected(four_pages(Some("1"), 1), Some(&[]));
    assert!(page_labels(&empty).is_empty());
    assert!(!dict(&empty, 1).contains("/PageLabels"));
}

#[test]
fn p1486_rotulos_coexistem_com_tagging_pdfa_e_pdfua() {
    use typst_core::entities::document_format_options::PdfStandard;

    for (tags, standard) in [
        (PdfTags::Enabled, PdfStandards::default()),
        (PdfTags::Disabled, PdfStandards::new([PdfStandard::A2b]).unwrap()),
        (PdfTags::Enabled, PdfStandards::new([PdfStandard::Ua1]).unwrap()),
    ] {
        let pdf = selected_with_options(four_pages(Some("1"), 10), None, tags, standard);
        assert_eq!(page_labels(&pdf), ["10", "11", "12", "13"]);
        let catalog = dict(&pdf, 1);
        assert!(catalog.contains("/PageLabels"));
        if tags == PdfTags::Enabled {
            assert!(catalog.contains("/StructTreeRoot"));
        }
        if standard.accessibility.is_some() {
            assert!(catalog.contains("/ViewerPreferences"));
        }
        if standard.archival.is_some() {
            assert!(catalog.contains("/OutputIntents"));
        }
    }
}

#[test]
fn p1486_cid_e_multifont_preservam_rotulos() {
    let fixture =
        concat!(env!("CARGO_MANIFEST_DIR"), "/fixtures/fonts/NimbusSans-Regular.otf");
    let font_data = std::fs::read(fixture).expect("fixture CFF do exporter");
    let ranges = || {
        PageRanges::new(vec![
            PageRange::at(NonZeroUsize::new(1).unwrap()),
            PageRange::at(NonZeroUsize::new(3).unwrap()),
        ])
    };
    let doc = PagedDocument::new(four_pages(Some("1"), 10));
    let cid = export_pdf_with_font_and_document_id_and_tags_and_serialization_and_pages_and_standard(
        &doc,
        &font_data,
        None,
        StreamMode::Verbose,
        PdfTags::Disabled,
        PdfSerializationMode::Compact,
        Some(ranges()),
        PdfStandards::default(),
    );
    assert_eq!(page_labels(&cid), ["10", "12"]);
    let fonts = [(
        (
            FontList::single(ecow::EcoString::from("Nimbus Sans")),
            FontVariant::default(),
            FontVariations::default(),
        ),
        font_data,
    )];
    let multifont = export_pdf_multifont_with_document_id_and_tags_and_serialization_and_pages_and_standard(
        &doc,
        &fonts,
        None,
        StreamMode::Verbose,
        PdfTags::Disabled,
        PdfSerializationMode::Compact,
        Some(ranges()),
        PdfStandards::default(),
    );
    assert_eq!(page_labels(&multifont), ["10", "12"]);
}
