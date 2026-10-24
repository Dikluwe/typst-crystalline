//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/tests/pdfa_associated_files_tests.md
//! @prompt-hash 80fe1eac
//! @layer L3
//! @updated 2026-09-22
//!
//! ── P1476 — Anexos PDF/A: `/AFRelationship`, `/AF`, extensão de `/Info`/
//! `/OutputIntent`/`pdfaid` a A4f/A4e ────────────────────────────────────
//!
//! Testes RED (Agente B, contrato "RED sem ver a implementação") contra o
//! baseline pós-P1475 (`088f38c29`, antes de P1476). Réplica deliberada do
//! padrão de infraestrutura de `p1475_pdfa_conformance_core_tests` acima
//! (extração por regex de trailer/xmp/catálogo/corpo de objeto), estendida
//! para documentos com anexo: `Content::pdf_attach(PdfAttachElem { .. })`
//! é a mesma via que `01_core/src/compiler/layout/pdf_attach.rs:13-18`
//! usa para popular `layouter.attachments` a partir de `#pdf.attach(...)`
//! — construir o `Content` diretamente em Rust é equivalente a compilar a
//! fonte Typst para efeitos deste teste (a API pública de `PdfAttachElem`
//! já existe desde P1286, não é implementação nova de P1476).

use super::*;
use regex::Regex;
use std::sync::Arc;
use typst_core::entities::document_format_options::{PdfStandard, PdfStandards};
use typst_core::entities::elements::pdf_attach::{
    AttachedFileRelationship, PdfAttachElem,
};
use typst_core::entities::layout_types::PagedDocument;

fn plain_doc() -> PagedDocument {
    layout(&Content::text("P1476 núcleo anexos PDF/A"))
}

fn attachment_elem(relationship: Option<AttachedFileRelationship>) -> PdfAttachElem {
    PdfAttachElem {
        path: "data.txt".into(),
        data: Arc::new(b"dados de teste".to_vec()),
        relationship,
        mime_type: Some("text/plain".into()),
        description: Some("dados de teste".into()),
    }
}

fn doc_with_attachment(relationship: Option<AttachedFileRelationship>) -> PagedDocument {
    layout(&Content::sequence(vec![
        Content::text("P1476 núcleo anexos PDF/A"),
        Content::pdf_attach(attachment_elem(relationship)),
    ]))
}

fn standards_for(names: &[PdfStandard]) -> PdfStandards {
    PdfStandards::new(names.iter().copied()).expect("standards válidos para P1476")
}

fn export_doc_with_standard(doc: &PagedDocument, standard: PdfStandards) -> Vec<u8> {
    export_pdf_with_document_id_and_tags_and_serialization_and_pages_and_standard(
        doc,
        None,
        StreamMode::Verbose,
        PdfTags::Enabled,
        PdfSerializationMode::Compact,
        None,
        standard,
    )
    .output
}

/// Réplica de `p1475_pdfa_conformance_core_tests::xmp_packet` — módulo
/// irmão, sem visibilidade partilhada (funções `fn` locais a um `mod`
/// não são reexportadas por `use super::*` num módulo diferente).
fn xmp_packet(pdf: &[u8]) -> String {
    let text = String::from_utf8_lossy(pdf).into_owned();
    let re = Regex::new(r#"(?s)<\?xpacket begin.*?<\?xpacket end="r"\?>"#).unwrap();
    re.find(&text).map(|m| m.as_str().to_string()).unwrap_or_default()
}

fn trailer_dict(pdf: &[u8]) -> String {
    let text = String::from_utf8_lossy(pdf).into_owned();
    let re = Regex::new(r"(?s)trailer\s*<<(.*?)>>").unwrap();
    re.captures(&text).map(|c| c[1].to_string()).unwrap_or_default()
}

fn object_body(pdf: &[u8], id: usize) -> String {
    let text = String::from_utf8_lossy(pdf).into_owned();
    let pattern = format!(r"(?ms)^{id} 0 obj\n(.*?)\nendobj");
    Regex::new(&pattern)
        .ok()
        .and_then(|re| re.captures(&text))
        .map(|c| c[1].to_string())
        .unwrap_or_default()
}

fn catalog_body(pdf: &[u8]) -> String {
    object_body(pdf, 1)
}

/// Localiza a linha da Filespec (`/Type /Filespec`) em qualquer parte
/// do PDF e devolve `(id do objeto, linha completa)`. A Filespec é
/// emitida como dicionário de uma única linha
/// (`emit_attachments`, `builder.rs:2455-2462`), portanto uma busca
/// por linha basta e não depende de onde `/AFRelationship` viria a
/// ser inserido dentro do dicionário (posição não especificada pelo
/// L0 — só o conteúdo é).
fn filespec_id_and_line(pdf: &[u8]) -> Option<(usize, String)> {
    let text = String::from_utf8_lossy(pdf).into_owned();
    let re = Regex::new(r"(?m)^(\d+) 0 obj\n(<< /Type /Filespec.*>>)$").unwrap();
    re.captures(&text).map(|c| (c[1].parse().unwrap(), c[2].to_string()))
}

/// Oráculo do teste — mapeamento 1:1 esperado (typst-passo-1476.md
/// §2.3, `infra/export/builder.md` P1476 item 1), calculado de forma
/// independente da implementação (nunca chama `af_relationship_name`
/// privada de `builder.rs`).
fn expected_af_relationship_name(
    relationship: Option<AttachedFileRelationship>,
) -> &'static str {
    match relationship {
        Option::None => "Unspecified",
        Some(AttachedFileRelationship::Source) => "Source",
        Some(AttachedFileRelationship::Data) => "Data",
        Some(AttachedFileRelationship::Alternative) => "Alternative",
        Some(AttachedFileRelationship::Supplement) => "Supplement",
    }
}

const GATED_ARCHIVALS: &[PdfStandard] = &[
    PdfStandard::A3a,
    PdfStandard::A3b,
    PdfStandard::A3u,
    PdfStandard::A4f,
    PdfStandard::A4e,
];

const UNGATED_CASES: &[Option<PdfStandard>] = &[
    None,
    Some(PdfStandard::A1a),
    Some(PdfStandard::A1b),
    Some(PdfStandard::A2a),
    Some(PdfStandard::A2b),
    Some(PdfStandard::A2u),
    Some(PdfStandard::A4),
];

fn standard_for_case(case: Option<PdfStandard>) -> PdfStandards {
    match case {
        None => PdfStandards::default(),
        Some(raw) => standards_for(&[raw]),
    }
}

// ── Item 1 — `/AFRelationship` na Filespec (L0 §3.2 item 1) ─────────

#[test]
fn p1476_af_relationship_present_with_correct_value_for_gated_archivals() {
    // RED esperado: `emit_attachments` (builder.rs:2420) ainda não
    // inclui `/AFRelationship` na Filespec para nenhum standard.
    let relationships: &[Option<AttachedFileRelationship>] = &[
        None,
        Some(AttachedFileRelationship::Source),
        Some(AttachedFileRelationship::Data),
        Some(AttachedFileRelationship::Alternative),
        Some(AttachedFileRelationship::Supplement),
    ];
    for &archival in GATED_ARCHIVALS {
        for relationship in relationships {
            let doc = doc_with_attachment(relationship.clone());
            let pdf = export_doc_with_standard(&doc, standards_for(&[archival]));
            let (_, filespec_line) = filespec_id_and_line(&pdf).unwrap_or_else(|| {
                panic!(
                    "{archival:?}/{relationship:?}: Filespec (/Type /Filespec) não encontrada no PDF"
                )
            });
            let expected = format!(
                "/AFRelationship /{}",
                expected_af_relationship_name(relationship.clone())
            );
            assert!(
                filespec_line.contains(&expected),
                "{archival:?}/{relationship:?}: esperado {expected} na Filespec (P1476 item 1, RED esperado); filespec={filespec_line}"
            );
        }
    }
}

#[test]
fn p1476_af_relationship_absent_outside_gate_regression_guard() {
    // Guarda de não-regressão (scope-out negativo confirmado,
    // typst-passo-1476.md §2.5): fora do gate, mesmo com anexo e
    // relationship explícito, a Filespec nunca ganha
    // `/AFRelationship` — já é o comportamento atual (nada emite este
    // campo hoje), deve continuar assim fora do conjunto
    // A3a/A3b/A3u/A4f/A4e.
    for case in UNGATED_CASES.iter().copied() {
        let standard = standard_for_case(case);
        let doc = doc_with_attachment(Some(AttachedFileRelationship::Supplement));
        let pdf = export_doc_with_standard(&doc, standard);
        let (_, filespec_line) = filespec_id_and_line(&pdf).unwrap_or_else(|| {
            panic!(
                "{case:?}: Filespec não encontrada (o anexo deve continuar a ser emitido mesmo fora do gate de /AFRelationship, P1286)"
            )
        });
        assert!(
            !filespec_line.contains("/AFRelationship"),
            "{case:?}: Filespec não deve ter /AFRelationship fora do gate; filespec={filespec_line}"
        );
    }
}

// ── Item 2 — `/AF` no catálogo (L0 §3.2 item 2/3) ───────────────────

#[test]
fn p1476_af_array_present_in_catalog_for_gated_archivals() {
    // RED esperado: nenhum `/AF` é emitido no catálogo hoje.
    let af_re = Regex::new(r"/AF\s*\[\s*(\d+)\s+0\s+R\s*\]").unwrap();
    for &archival in GATED_ARCHIVALS {
        let doc = doc_with_attachment(Some(AttachedFileRelationship::Supplement));
        let pdf = export_doc_with_standard(&doc, standards_for(&[archival]));
        let catalog = catalog_body(&pdf);
        let caps = af_re.captures(&catalog).unwrap_or_else(|| {
            panic!(
                "{archival:?}: catálogo sem /AF [{{id}} 0 R] (P1476 item 1, RED esperado); catalog={catalog}"
            )
        });
        let af_id: usize = caps[1].parse().unwrap();
        let (filespec_id, _) = filespec_id_and_line(&pdf)
            .unwrap_or_else(|| panic!("{archival:?}: Filespec não encontrada"));
        assert_eq!(
            af_id, filespec_id,
            "{archival:?}: /AF deve referenciar o id da Filespec do único anexo"
        );
    }
}

#[test]
fn p1476_af_array_absent_without_attachment_regression_guard() {
    // Guarda de não-regressão: sem nenhum anexo, o catálogo nunca deve
    // ganhar `/AF`, mesmo dentro do gate archival — já é o
    // comportamento atual (emit_attachments retorna cedo quando
    // `doc.attachments.is_empty()`, builder.rs:2421-2423).
    let af_array_re = Regex::new(r"/AF\s*\[").unwrap();
    for &archival in GATED_ARCHIVALS {
        let pdf = export_doc_with_standard(&plain_doc(), standards_for(&[archival]));
        let catalog = catalog_body(&pdf);
        assert!(
            !af_array_re.is_match(&catalog),
            "{archival:?}: catálogo não deve ter /AF sem nenhum anexo; catalog={catalog}"
        );
    }
}

#[test]
fn p1476_af_array_absent_outside_gate_regression_guard() {
    // Guarda de não-regressão: fora do gate, mesmo com anexo presente,
    // o catálogo não deve ganhar `/AF`.
    let af_array_re = Regex::new(r"/AF\s*\[").unwrap();
    for case in UNGATED_CASES.iter().copied() {
        let standard = standard_for_case(case);
        let doc = doc_with_attachment(Some(AttachedFileRelationship::Supplement));
        let pdf = export_doc_with_standard(&doc, standard);
        let catalog = catalog_body(&pdf);
        assert!(
            !af_array_re.is_match(&catalog),
            "{case:?}: catálogo não deve ter /AF fora do gate mesmo com anexo; catalog={catalog}"
        );
    }
}

// ── Item 3 — XMP `pdfaid` ganha `conformance` em A4f/A4e (L0 §3.5) ──

#[test]
fn p1476_pdfaid_a4f_a4e_include_conformance_letter() {
    // RED esperado: `pdfaid_elem` (builder.rs:2830) ainda trata
    // A4f/A4e como scope-out (string vazia); achado de P1476 (§2.2 do
    // passo) é que A4f/A4e usam as três chaves part+rev+conformance,
    // ao contrário de A4 "puro" (só part+rev, P1475).
    let cases: &[(PdfStandard, &str)] =
        &[(PdfStandard::A4f, "F"), (PdfStandard::A4e, "E")];
    for (raw, letter) in cases.iter().copied() {
        let pdf = export_doc_with_standard(&plain_doc(), standards_for(&[raw]));
        let xmp = xmp_packet(&pdf);
        let expected = format!(
            "<pdfaid:part>4</pdfaid:part><pdfaid:rev>2020</pdfaid:rev><pdfaid:conformance>{letter}</pdfaid:conformance>"
        );
        assert!(
            xmp.contains(&expected),
            "{raw:?}: esperado {expected} (P1476 item 4, RED esperado); xmp={xmp}"
        );
        assert!(
            xmp.contains(r#"xmlns:pdfaid="http://www.aiim.org/pdfa/ns/id/""#),
            "{raw:?}: xmlns:pdfaid deve ser declarado; xmp={xmp}"
        );
    }
}

// ── Item 4 — `/Info` suprimido estendido a A4f/A4e (L0 §3.3) ────────

#[test]
fn p1476_info_suppressed_for_a4f_a4e() {
    // RED esperado: `emit_info` (builder.rs:2684) ainda só suprime
    // para `Some(PdfArchival::A4)`; A4f/A4e continuam a produzir
    // `/Info` neste baseline. NOTA: isto contradiz, para os casos
    // A4f/A4e, a asserção positiva ("/Info presente") do teste P1475
    // `p1475_info_present_for_all_other_cases_regression_guard`
    // (tests.rs, módulo `p1475_pdfa_conformance_core_tests`) — ver
    // relatório final do Agente B sobre esta contradição inerente ao
    // contrato de P1476 (não corrigido aqui; fora do escopo de
    // Agente B tocar testes de P1475).
    for raw in [PdfStandard::A4f, PdfStandard::A4e] {
        for doc in
            [plain_doc(), doc_with_attachment(Some(AttachedFileRelationship::Supplement))]
        {
            let pdf = export_doc_with_standard(&doc, standards_for(&[raw]));
            let trailer = trailer_dict(&pdf);
            assert!(
                !trailer.contains("/Info"),
                "{raw:?}: trailer não deve referenciar /Info (P1476 item 2, RED esperado); trailer={trailer}"
            );
            let raw_text = String::from_utf8_lossy(&pdf);
            assert!(
                !raw_text.contains("/CreationDate"),
                "{raw:?}: não deve existir objeto /Info (marcador /CreationDate ausente)"
            );
        }
    }
}

// ── Item 5 — `/OutputIntent` estendido a A4f/A4e (L0 §3.4) ──────────

#[test]
fn p1476_output_intent_present_for_a4f_a4e() {
    // RED esperado: `emit_pdfa_output_intent` (builder.rs:2729-2740)
    // ainda não inclui A4f/A4e no `matches!`. NOTA: isto contradiz,
    // para os casos A4f/A4e, a asserção negativa do teste P1475
    // `p1475_output_intent_absent_for_none_a1_a4f_a4e_ua1_alone_regression_guard`
    // — mesma contradição inerente ao contrato de P1476, ver
    // relatório final.
    let intents_re = Regex::new(r"/OutputIntents\s*\[\s*(\d+)\s+0\s+R\s*\]").unwrap();
    for raw in [PdfStandard::A4f, PdfStandard::A4e] {
        let pdf = export_doc_with_standard(&plain_doc(), standards_for(&[raw]));
        let catalog = catalog_body(&pdf);
        let caps = intents_re.captures(&catalog).unwrap_or_else(|| {
            panic!(
                "{raw:?}: catálogo sem /OutputIntents [{{id}} 0 R] (P1476 item 3, RED esperado); catalog={catalog}"
            )
        });
        let intent_id: usize = caps[1].parse().unwrap();
        let intent_obj = object_body(&pdf, intent_id);
        assert!(
            intent_obj.contains("/Type /OutputIntent"),
            "{raw:?}: objeto {intent_id} sem /Type /OutputIntent; {intent_obj}"
        );
        assert!(
            intent_obj.contains("/S /GTS_PDFA1"),
            "{raw:?}: /S deve ser /GTS_PDFA1; {intent_obj}"
        );
    }
}

// ── Fronteira do gate — A4 (sem f/e) não é embedded-files-capable ───

#[test]
fn p1476_a4_alone_with_attachment_stays_outside_embedded_files_gate_regression_guard() {
    // Guarda de não-regressão sobre a fronteira exata do gate (L0
    // §3.1, `archival_supports_embedded_files`): A4 sozinho (sem
    // sufixo f/e) tem anexo emitido (P1286, sempre incondicional),
    // mas não entra no conjunto que suporta embedded files conformes
    // — sem /AFRelationship, sem /AF, e pdfaid continua sem
    // `conformance` mesmo com o anexo presente (o eixo `conformance`
    // é decidido só pelo archival, nunca pela presença de anexos).
    let doc = doc_with_attachment(Some(AttachedFileRelationship::Supplement));
    let pdf = export_doc_with_standard(&doc, standards_for(&[PdfStandard::A4]));

    let (_, filespec_line) = filespec_id_and_line(&pdf)
        .expect("A4 + anexo: Filespec deve continuar a ser emitida (P1286)");
    assert!(
        !filespec_line.contains("/AFRelationship"),
        "A4 (sem f/e) não suporta embedded files conformes — Filespec não deve ter /AFRelationship; filespec={filespec_line}"
    );

    let catalog = catalog_body(&pdf);
    assert!(
        !Regex::new(r"/AF\s*\[").unwrap().is_match(&catalog),
        "A4 (sem f/e) + anexo: catálogo não deve ter /AF; catalog={catalog}"
    );

    let xmp = xmp_packet(&pdf);
    assert!(
        xmp.contains("<pdfaid:part>4</pdfaid:part><pdfaid:rev>2020</pdfaid:rev>"),
        "A4 + anexo: pdfaid deve continuar part+rev; xmp={xmp}"
    );
    assert!(
        !xmp.contains("pdfaid:conformance"),
        "A4 (sem f/e) + anexo: não deve ganhar pdfaid:conformance; xmp={xmp}"
    );
}
