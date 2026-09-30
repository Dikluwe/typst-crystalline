//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/tests/p1475_pdfa_conformance_core_tests.md
//! @prompt-hash c6b15ebc
//! @layer L3
//! @updated 2026-09-22
//!
//! ── P1475 (RED — Agente B) ───────────────────────────────────────────────
//!
//! Núcleo PDF/A-2/3/4: trailer `/ID` universal, `pdf:PDFVersion` dinâmico,
//! XMP `pdfaid`, `/OutputIntent` e supressão de `/Info` em A4. Ver
//! `00_nucleo/materialization/typst-passo-1475.md` §5 e
//! `00_nucleo/prompts/infra/export/builder.md` §P1475 (contrato completo,
//! `file:line` medidos contra o vanilla ratificado).
//!
//! Testa exclusivamente através da fronteira pública já existente do
//! builder (`export_pdf_with_document_id_and_tags_and_serialization_and_pages_and_standard`)
//! e inspeção de bytes do PDF resultante — nenhum símbolo interno novo
//! (`xmp_ids`, `emit_pdfa_output_intent`, ...) é referenciado diretamente,
//! para que um erro de compilação em `builder.rs` (ainda não implementado
//! neste baseline) nunca trave a confirmação RED destes testes.
//!
//! Neste baseline (`7a20ec72d`, `03_infra/src/export/builder.rs` sem os
//! quatro itens de P1475), os testes marcados RED falham por asserção
//! (não por erro de compilação). Os marcados "regression guard" já
//! passam hoje porque o comportamento atual coincide com o esperado
//! (nada emite `pdfaid`/`/OutputIntent` hoje, `/Info` é sempre emitido, e
//! `pdf:PDFVersion` já é `"1.7"` por defeito).

use super::*;
use regex::Regex;
use typst_core::entities::document_format_options::{PdfStandard, PdfStandards};
use typst_core::entities::layout_types::PagedDocument;

fn simple_doc() -> PagedDocument {
    layout(&Content::text("P1475 núcleo PDF/A-2/3/4"))
}

/// Resolve um conjunto de nomes brutos num `PdfStandards` válido — os
/// casos usados aqui nunca conflitam (um único perfil archival, quando
/// muito acompanhado de uma versão explícita).
fn standards_for(names: &[PdfStandard]) -> PdfStandards {
    PdfStandards::new(names.iter().copied()).expect("standards válidos para P1475")
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

/// Extrai o pacote XMP (`<?xpacket ...?> ... <?xpacket end="r"?>`) como
/// string. O payload é XML puro (sem sequências `<<`/`>>` literais),
/// portanto uma extração por regex simples é suficiente e não depende
/// de nenhum parser de dicionário PDF.
fn xmp_packet(pdf: &[u8]) -> String {
    let text = String::from_utf8_lossy(pdf).into_owned();
    let re = Regex::new(r#"(?s)<\?xpacket begin.*?<\?xpacket end="r"\?>"#).unwrap();
    re.find(&text).map(|m| m.as_str().to_string()).unwrap_or_default()
}

/// Extrai o conteúdo do dicionário do trailer (`trailer\n<< ... >>`),
/// tolerante a variação de espaçamento interno (a ordem/espaçamento
/// exatos das chaves não é observável — L0 §3.1).
fn trailer_dict(pdf: &[u8]) -> String {
    let text = String::from_utf8_lossy(pdf).into_owned();
    let re = Regex::new(r"(?s)trailer\s*<<(.*?)>>").unwrap();
    re.captures(&text).map(|c| c[1].to_string()).unwrap_or_default()
}

/// Extrai o corpo de um objeto indireto **sem stream** pelo seu id
/// (`{id} 0 obj\n{corpo}\nendobj`) — suficiente para os dicionários de
/// uma linha emitidos por P1475 (`/OutputIntent`, catálogo). Ancorado a
/// início de linha (`(?m)^`) para não confundir `id=1` com `21`, `31`, etc.
fn object_body(pdf: &[u8], id: usize) -> String {
    let text = String::from_utf8_lossy(pdf).into_owned();
    let pattern = format!(r"(?ms)^{id} 0 obj\n(.*?)\nendobj");
    Regex::new(&pattern)
        .ok()
        .and_then(|re| re.captures(&text))
        .map(|c| c[1].to_string())
        .unwrap_or_default()
}

/// Extrai só o cabeçalho (dicionário) de um objeto **com stream**, antes
/// da palavra-chave `stream` — evita puxar bytes binários (o perfil ICC
/// comprimido) para dentro da string capturada.
fn object_header_before_stream(pdf: &[u8], id: usize) -> String {
    let text = String::from_utf8_lossy(pdf).into_owned();
    let pattern = format!(r"(?ms)^{id} 0 obj\n(.*?)stream\n");
    Regex::new(&pattern)
        .ok()
        .and_then(|re| re.captures(&text))
        .map(|c| c[1].to_string())
        .unwrap_or_default()
}

/// Objeto 1 é sempre o catálogo (`trailer` fixa `/Root 1 0 R`,
/// `emit_xmp_metadata`/`emit_outlines` já editam `id == 1` diretamente
/// — mesma convenção usada em todo `builder.rs`).
fn catalog_body(pdf: &[u8]) -> String {
    object_body(pdf, 1)
}

// ── Item 1 — trailer `/ID` universal (L0 §3.1) ──────────────────────

#[test]
fn p1475_trailer_id_is_present_universal_and_matches_xmp_ids() {
    // RED esperado neste baseline: `serialize()` (builder.rs:3347-3358)
    // ainda não insere `/ID` no trailer.
    let pdf = export_with_standard(PdfStandards::default());
    let raw = String::from_utf8_lossy(&pdf);

    let trailer = trailer_dict(&pdf);
    assert!(!trailer.is_empty(), "trailer não encontrado: {raw}");

    let id_re = Regex::new(r"/ID\s*\[\s*\(([^)]*)\)\s*\(([^)]*)\)\s*\]").unwrap();
    let caps = id_re.captures(&trailer).unwrap_or_else(|| {
        panic!(
            "trailer sem /ID [ (X) (Y) ] (P1475 item 1, RED esperado neste baseline); trailer={trailer}"
        )
    });
    let x = caps[1].to_string();
    let y = caps[2].to_string();
    // Achado de integração (P1475): o vanilla mede X==Y no caso comum
    // (Achado 4.1), mas `xmp_instance_and_document_id` (builder.rs:409-
    // 428, pré-existente P615/P617, fora do escopo deste passo) usa duas
    // constantes fixas DISTINTAS em modo de teste (e duas chamadas
    // independentes a `random_xmp_id_bytes()` em produção) — X != Y é o
    // comportamento real e correto do cristalino hoje. O contrato do
    // item 1 é só "reusar o mesmo par já calculado", nunca "X deve ser
    // igual a Y" — as duas asserções abaixo é que provam o contrato.

    let xmp = xmp_packet(&pdf);
    let inst_re = Regex::new(r"<xmpMM:InstanceID>([^<]*)</xmpMM:InstanceID>").unwrap();
    let doc_re = Regex::new(r"<xmpMM:DocumentID>([^<]*)</xmpMM:DocumentID>").unwrap();
    let inst = inst_re
        .captures(&xmp)
        .map(|c| c[1].to_string())
        .expect("xmpMM:InstanceID ausente do pacote XMP");
    let docid = doc_re
        .captures(&xmp)
        .map(|c| c[1].to_string())
        .expect("xmpMM:DocumentID ausente do pacote XMP");
    assert_eq!(x, inst, "/ID[0] deve bater com xmpMM:InstanceID do mesmo PDF");
    assert_eq!(y, docid, "/ID[1] deve bater com xmpMM:DocumentID do mesmo PDF");
}

// ── Item 2 — `pdf:PDFVersion` dinâmico (L0 §3.2, fluxo contínuo) ────

#[test]
fn p1475_pdf_version_dynamic_with_standard_2_0_no_archival() {
    // RED esperado: `emit_xmp_metadata` (builder.rs:2779) ainda emite o
    // literal fixo "1.7", independente de `self.standard.version`.
    let standard = standards_for(&[PdfStandard::V2_0]);
    let pdf = export_with_standard(standard);
    let xmp = xmp_packet(&pdf);
    assert!(
        xmp.contains("<pdf:PDFVersion>2.0</pdf:PDFVersion>"),
        "esperado <pdf:PDFVersion>2.0</pdf:PDFVersion> com standard \"2.0\"; xmp={xmp}"
    );
}

#[test]
fn p1475_pdf_version_without_standard_stays_1_7_regression_guard() {
    // Guarda de não-regressão: já passa hoje (literal fixo == default).
    let pdf = export_with_standard(PdfStandards::default());
    let xmp = xmp_packet(&pdf);
    assert!(
        xmp.contains("<pdf:PDFVersion>1.7</pdf:PDFVersion>"),
        "regressão: sem standard, pdf:PDFVersion deve continuar \"1.7\"; xmp={xmp}"
    );
}

// ── Item 3 — XMP `pdfaid` gated (L0 §3.3) ───────────────────────────

#[test]
fn p1475_pdfaid_present_for_a2_a3_family_with_correct_letter() {
    // RED esperado: `emit_xmp_metadata` ainda não emite `pdfaid:*`.
    let cases: &[(PdfStandard, &str, &str)] = &[
        (PdfStandard::A2a, "2", "A"),
        (PdfStandard::A2b, "2", "B"),
        (PdfStandard::A2u, "2", "U"),
        (PdfStandard::A3a, "3", "A"),
        (PdfStandard::A3b, "3", "B"),
        (PdfStandard::A3u, "3", "U"),
    ];
    for (raw, part, letter) in cases.iter().copied() {
        let pdf = export_with_standard(standards_for(&[raw]));
        let xmp = xmp_packet(&pdf);
        let expected = format!(
            "<pdfaid:part>{part}</pdfaid:part><pdfaid:conformance>{letter}</pdfaid:conformance>"
        );
        assert!(
            xmp.contains(&expected),
            "{raw:?}: esperado {expected} (P1475 item 3, RED esperado); xmp={xmp}"
        );
        assert!(
            xmp.contains(r#"xmlns:pdfaid="http://www.aiim.org/pdfa/ns/id/""#),
            "{raw:?}: xmlns:pdfaid deve ser declarado quando pdfaid:* está presente; xmp={xmp}"
        );
    }
}

#[test]
fn p1475_pdfaid_present_for_a4_with_rev_2020_no_conformance() {
    // RED esperado — achado novo (§2.2 do passo): A4 usa `pdfaid:rev`,
    // não `pdfaid:conformance`.
    let pdf = export_with_standard(standards_for(&[PdfStandard::A4]));
    let xmp = xmp_packet(&pdf);
    assert!(
        xmp.contains("<pdfaid:part>4</pdfaid:part><pdfaid:rev>2020</pdfaid:rev>"),
        "esperado pdfaid:part=4 + pdfaid:rev=2020 sem conformance; xmp={xmp}"
    );
    assert!(
        !xmp.contains("pdfaid:conformance"),
        "PDF/A-4 não deve ter pdfaid:conformance (ISO 19005-4 eliminou o eixo a/b/u); xmp={xmp}"
    );
    assert!(xmp.contains(r#"xmlns:pdfaid="http://www.aiim.org/pdfa/ns/id/""#));
}

#[test]
fn p1475_pdfaid_absent_without_any_standard_regression_guard() {
    // Guarda de não-regressão: já passa hoje (nada emite pdfaid ainda).
    let pdf = export_with_standard(PdfStandards::default());
    let xmp = xmp_packet(&pdf);
    assert!(!xmp.contains("pdfaid:"), "sem standard, não deve haver pdfaid:*; xmp={xmp}");
    assert!(
        !xmp.contains("xmlns:pdfaid"),
        "sem standard, não deve declarar xmlns:pdfaid; xmp={xmp}"
    );
}

#[test]
fn p1475_pdfaid_absent_for_scope_out_profiles_confirmed_negatively() {
    // Guarda de não-regressão (scope-out §2.5/§5 do passo, confirmado
    // negativamente): ua-1 sozinho não produz pdfaid, e não deve passar
    // a produzir. A4f/A4e SAÍRAM em P1476 (`p1476_pdfaid_a4f_a4e_
    // include_conformance_letter`); A1a/A1b SAÍRAM em P1477
    // (`p1477_icc_v2_selected_for_a1a_and_a1b_shared_point` cobre o
    // perfil ICC; pdfaid de A1a/A1b agora part=1+conformance, mesma
    // forma de A2/A3, medida diretamente contra o vanilla) — mudanças
    // de escopo intencionais, não regressão.
    let cases: &[PdfStandard] = &[PdfStandard::Ua1];
    for raw in cases.iter().copied() {
        let pdf = export_with_standard(standards_for(&[raw]));
        let xmp = xmp_packet(&pdf);
        assert!(
            !xmp.contains("pdfaid:"),
            "{raw:?} sozinho não deve produzir pdfaid:* (scope-out §2.5); xmp={xmp}"
        );
        assert!(
            !xmp.contains("xmlns:pdfaid"),
            "{raw:?} sozinho não deve declarar xmlns:pdfaid; xmp={xmp}"
        );
    }
}

// ── Item 4 — `/OutputIntent` gated (L0 §3.4) ────────────────────────

#[test]
fn p1475_output_intent_present_for_seven_archivals_in_scope() {
    // RED esperado: nenhum `/OutputIntent`/`/OutputIntents` é emitido
    // hoje para nenhum standard.
    let cases: &[PdfStandard] = &[
        PdfStandard::A2a,
        PdfStandard::A2b,
        PdfStandard::A2u,
        PdfStandard::A3a,
        PdfStandard::A3b,
        PdfStandard::A3u,
        PdfStandard::A4,
    ];
    let intents_re = Regex::new(r"/OutputIntents\s*\[\s*(\d+)\s+0\s+R\s*\]").unwrap();
    let profile_re = Regex::new(r"/DestOutputProfile\s+(\d+)\s+0\s+R").unwrap();
    for raw in cases.iter().copied() {
        let pdf = export_with_standard(standards_for(&[raw]));
        let catalog = catalog_body(&pdf);

        let caps = intents_re.captures(&catalog).unwrap_or_else(|| {
            panic!(
                "{raw:?}: catálogo sem /OutputIntents [{{id}} 0 R] (P1475 item 4, RED esperado); catalog={catalog}"
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
            "{raw:?}: /S deve ser /GTS_PDFA1 (fixo, independente do nível); {intent_obj}"
        );

        let profile_caps = profile_re.captures(&intent_obj).unwrap_or_else(|| {
            panic!("{raw:?}: /OutputIntent sem /DestOutputProfile; {intent_obj}")
        });
        let profile_id: usize = profile_caps[1].parse().unwrap();

        let header = object_header_before_stream(&pdf, profile_id);
        assert!(
            !header.is_empty(),
            "{raw:?}: objeto {profile_id} (perfil ICC) sem cabeçalho de stream localizável"
        );
        assert!(header.contains("/N 3"), "{raw:?}: perfil ICC sem /N 3; {header}");
        assert!(
            header.contains("/Filter /FlateDecode"),
            "{raw:?}: perfil ICC sem /Filter /FlateDecode; {header}"
        );
    }
}

#[test]
fn p1475_output_intent_absent_for_none_a1_ua1_alone_regression_guard() {
    // Guarda de não-regressão: nada emite /OutputIntent para nenhum
    // destes casos. A4f/A4e SAÍRAM em P1476
    // (`p1476_output_intent_present_for_a4f_a4e`); A1a/A1b SAÍRAM em
    // P1477 (`p1477_output_intent_v2_for_a1a_a1b_per_l0_acceptance_
    // section_5` — o perfil v2 remove o bloqueio que motivou o
    // scope-out original de P1475 §2.5) — mudanças de escopo
    // intencionais, não regressão.
    let cases: &[Option<PdfStandard>] = &[None, Some(PdfStandard::Ua1)];
    for case in cases.iter().copied() {
        let standard = match case {
            None => PdfStandards::default(),
            Some(raw) => standards_for(&[raw]),
        };
        let pdf = export_with_standard(standard);
        let catalog = catalog_body(&pdf);
        assert!(
            !catalog.contains("/OutputIntents"),
            "{case:?}: catálogo não deve ter /OutputIntents; catalog={catalog}"
        );
        let raw_text = String::from_utf8_lossy(&pdf);
        assert!(
            !raw_text.contains("/Type /OutputIntent"),
            "{case:?}: não deve existir objeto /OutputIntent"
        );
    }
}

// ── Item 5 — supressão de `/Info` só em A4 (L0 §3.5) ────────────────

#[test]
fn p1475_info_suppressed_only_for_a4() {
    // RED esperado: `emit_info` (builder.rs:2668) ainda aloca /Info
    // incondicionalmente, mesmo para archival == Some(A4).
    let pdf = export_with_standard(standards_for(&[PdfStandard::A4]));
    let raw = String::from_utf8_lossy(&pdf);
    let trailer = trailer_dict(&pdf);
    assert!(
        !trailer.contains("/Info"),
        "PDF/A-4: trailer não deve referenciar /Info (ISO 19005-4 6.1.3); trailer={trailer}"
    );
    assert!(
        !raw.contains("/CreationDate"),
        "PDF/A-4: não deve existir objeto /Info (marcador /CreationDate ausente)"
    );
}

#[test]
fn p1475_info_present_for_all_other_cases_regression_guard() {
    // Guarda de não-regressão: `/Info` continua presente em todos os
    // outros casos. A4f/A4e SAÍRAM desta lista em P1476 (agora
    // suprimido, testado em `p1476_info_suppressed_for_a4f_a4e` — mesma
    // regra ISO 19005-4 6.1.3 de A4, mudança de escopo intencional, não
    // regressão; ver `typst-passo-1476.md` item 2).
    let cases: &[Option<PdfStandard>] = &[
        None,
        Some(PdfStandard::A2b),
        Some(PdfStandard::A3b),
        Some(PdfStandard::A1b),
        Some(PdfStandard::Ua1),
    ];
    for case in cases.iter().copied() {
        let standard = match case {
            None => PdfStandards::default(),
            Some(raw) => standards_for(&[raw]),
        };
        let pdf = export_with_standard(standard);
        let trailer = trailer_dict(&pdf);
        assert!(
            trailer.contains("/Info"),
            "{case:?}: trailer deve referenciar /Info (comportamento atual preservado); trailer={trailer}"
        );
        let raw = String::from_utf8_lossy(&pdf);
        assert!(raw.contains("/CreationDate"), "{case:?}: objeto /Info deve existir");
    }
}
