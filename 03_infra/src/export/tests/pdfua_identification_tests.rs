//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/tests/pdfua_identification_tests.md
//! @prompt-hash 4ecec022
//! @layer L3
//! @updated 2026-09-23
//!
//! **P1480** — PDF/UA-1: identificação XMP `pdfuaid` e preferência
//! `/ViewerPreferences /DisplayDocTitle true`. A conformidade estrutural
//! restante pertence a P1481; o erro de título ausente é testado em
//! `pipeline.rs`.

use super::*;
use regex::Regex;
use typst_core::entities::document_format_options::{PdfStandard, PdfStandards};
use typst_core::entities::layout_types::PagedDocument;

fn simple_doc() -> PagedDocument {
    layout(&Content::text("P1480 PDF/UA identification"))
}

fn standards_for(names: &[PdfStandard]) -> PdfStandards {
    PdfStandards::new(names.iter().copied()).expect("standards válidos para P1480")
}

fn export_with_standard(standard: PdfStandards) -> Vec<u8> {
    export_pdf_with_document_id_and_tags_and_serialization_and_pages_and_standard(
        &simple_doc(),
        None,
        StreamMode::Verbose,
        PdfTags::Enabled,
        PdfSerializationMode::Compact,
        None,
        standard,
    )
}

fn xmp_packet(pdf: &[u8]) -> String {
    let text = String::from_utf8_lossy(pdf).into_owned();
    let re = Regex::new(r#"(?s)<\?xpacket begin.*?<\?xpacket end="r"\?>"#).unwrap();
    re.find(&text).map(|m| m.as_str().to_string()).unwrap_or_default()
}

fn object_body(pdf: &[u8], id: usize) -> String {
    let text = String::from_utf8_lossy(pdf).into_owned();
    let pattern = format!(r"(?ms)^{id} 0 obj\n(.*?)\nendobj");
    Regex::new(&pattern)
        .ok()
        .and_then(|re| re.captures(&text))
        .map(|captures| captures[1].to_string())
        .unwrap_or_default()
}

fn catalog_body(pdf: &[u8]) -> String {
    object_body(pdf, 1)
}

#[test]
fn p1480_ua1_emite_identificacao_xmp_na_posicao_contratada() {
    let pdf = export_with_standard(standards_for(&[PdfStandard::Ua1]));
    let xmp = xmp_packet(&pdf);
    assert!(
        xmp.contains(r#"xmlns:pdfuaid="http://www.aiim.org/pdfua/ns/id/""#),
        "ua-1 deve declarar o namespace pdfuaid; xmp={xmp}"
    );
    assert_eq!(
        xmp.matches("<pdfuaid:part>1</pdfuaid:part>").count(),
        1,
        "ua-1 deve identificar exatamente a parte 1; xmp={xmp}"
    );
    assert!(
        !xmp.contains("pdfaid:"),
        "ua-1 isolado não deve inventar identificação PDF/A; xmp={xmp}"
    );
    let part = xmp.find("<pdfuaid:part>1</pdfuaid:part>").unwrap();
    let pages = xmp.find("<xmpTPg:NPages>").unwrap();
    assert!(part < pages, "pdfuaid:part deve preceder xmpTPg:NPages; xmp={xmp}");
}

#[test]
fn p1480_ua1_emite_preferencia_de_exibir_titulo() {
    let pdf = export_with_standard(standards_for(&[PdfStandard::Ua1]));
    let catalog = catalog_body(&pdf);
    assert!(
        catalog.contains("/ViewerPreferences << /DisplayDocTitle true >>"),
        "catálogo ua-1 deve pedir exibição do título; catalog={catalog}"
    );
}

#[test]
fn p1480_perfis_sem_ua1_nao_ganham_observaveis_pdfua() {
    let cases = [
        ("default", PdfStandards::default()),
        ("a-2u", standards_for(&[PdfStandard::A2u])),
        ("a-4f", standards_for(&[PdfStandard::A4f])),
    ];
    for (name, standard) in cases {
        let pdf = export_with_standard(standard);
        let xmp = xmp_packet(&pdf);
        let catalog = catalog_body(&pdf);
        assert!(!xmp.contains("pdfuaid"), "{name} não deve emitir pdfuaid; xmp={xmp}");
        assert!(
            !catalog.contains("/ViewerPreferences"),
            "{name} não deve ganhar ViewerPreferences; catalog={catalog}"
        );
    }
}

#[test]
fn p1480_pdfa_e_pdfua_coexistem_sem_substituicao() {
    let a2u = xmp_packet(&export_with_standard(standards_for(&[PdfStandard::A2u])));
    let combined_pdf =
        export_with_standard(standards_for(&[PdfStandard::A2u, PdfStandard::Ua1]));
    let combined = xmp_packet(&combined_pdf);
    let pdfaid = "<pdfaid:part>2</pdfaid:part><pdfaid:conformance>U</pdfaid:conformance>";
    assert!(a2u.contains(pdfaid), "controle a-2u sem pdfaid esperado; xmp={a2u}");
    assert!(
        combined.contains(pdfaid),
        "a-2u+ua-1 deve conservar o corpo pdfaid de a-2u; xmp={combined}"
    );
    assert!(
        combined.contains("<pdfuaid:part>1</pdfuaid:part>"),
        "a-2u+ua-1 deve acrescentar identificação PDF/UA; xmp={combined}"
    );
    assert!(
        catalog_body(&combined_pdf)
            .contains("/ViewerPreferences << /DisplayDocTitle true >>"),
        "a-2u+ua-1 deve conservar a preferência PDF/UA"
    );
}

#[test]
fn p1480_pdfa_pdfua_declara_schema_de_extensao_pdfua() {
    let combined = xmp_packet(&export_with_standard(standards_for(&[
        PdfStandard::A2u,
        PdfStandard::Ua1,
    ])));
    for namespace in [
        r#"xmlns:pdfaExtension="http://www.aiim.org/pdfa/ns/extension/""#,
        r#"xmlns:pdfaSchema="http://www.aiim.org/pdfa/ns/schema#""#,
        r#"xmlns:pdfaProperty="http://www.aiim.org/pdfa/ns/property#""#,
    ] {
        assert!(
            combined.contains(namespace),
            "a-2u+ua-1 deve declarar {namespace}; xmp={combined}"
        );
    }
    for field in [
        "<pdfaSchema:schema>PDF/UA Identification schema</pdfaSchema:schema>",
        "<pdfaSchema:namespaceURI>http://www.aiim.org/pdfua/ns/id/</pdfaSchema:namespaceURI>",
        "<pdfaSchema:prefix>pdfuaid</pdfaSchema:prefix>",
        "<pdfaProperty:category>internal</pdfaProperty:category>",
        "<pdfaProperty:description>Part Number of ISO 14289 (PDF/UA)</pdfaProperty:description>",
        "<pdfaProperty:name>part</pdfaProperty:name>",
        "<pdfaProperty:valueType>Integer</pdfaProperty:valueType>",
    ] {
        assert!(
            combined.contains(field),
            "a-2u+ua-1 deve declarar o campo {field}; xmp={combined}"
        );
    }

    let ua_only = xmp_packet(&export_with_standard(standards_for(&[PdfStandard::Ua1])));
    assert!(
        !ua_only.contains("pdfaExtension"),
        "ua-1 isolado não deve ganhar schema de extensão PDF/A; xmp={ua_only}"
    );

    let a2u_only = xmp_packet(&export_with_standard(standards_for(&[PdfStandard::A2u])));
    assert!(
        !a2u_only.contains("pdfaExtension"),
        "a-2u isolado não deve ganhar extensão específica de PDF/UA; xmp={a2u_only}"
    );
}
