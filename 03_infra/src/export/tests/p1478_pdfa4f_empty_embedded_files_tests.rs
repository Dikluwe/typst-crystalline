//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/tests/p1478_pdfa4f_empty_embedded_files_tests.md
//! @prompt-hash 242796c4
//! @layer L3
//! @updated 2026-09-22
//!
//! **P1478** — A4f sem anexo requer `/EmbeddedFiles` vazio (retificação de
//! P1477). Ver `typst-passo-1478.md` e `infra/export/builder.md` §P1478.
//!
//! Localizado aqui (dentro de `builder.rs`, não em `export/tests.rs`) por
//! causa do achado `[PARSE]` do `crystalline-lint` em `tests.rs:11146`
//! (P1477, tamanho acumulado do ficheiro) — este módulo evita agravar
//! esse problema, per instrução explícita do passo §5.
//!
//! **P1479** — movido de `builder.rs` para cá (localização temporária
//! decidida em P1478 só para não agravar o achado `[PARSE]` que P1479
//! resolve). Ajuste de imports: `PdfStandards`/`AttachedFileRelationship`/
//! `PagedDocument` explicitados (antes vinham de bare-imports do topo de
//! `builder.rs`, não disponíveis a partir de `tests/mod.rs`).

use super::*;
use regex::Regex;
use std::sync::Arc;
use typst_core::compiler::layout::layout;
use typst_core::entities::content::Content;
use typst_core::entities::document_format_options::{PdfStandard, PdfStandards};
use typst_core::entities::elements::pdf_attach::{
    AttachedFileRelationship, PdfAttachElem,
};
use typst_core::entities::layout_types::PagedDocument;

fn plain_doc() -> PagedDocument {
    layout(&Content::text("P1478 núcleo A4f sem anexo"))
}

fn attachment_elem() -> PdfAttachElem {
    PdfAttachElem {
        path: "data.txt".into(),
        data: Arc::new(b"dados de teste".to_vec()),
        relationship: Some(AttachedFileRelationship::Supplement),
        mime_type: Some("text/plain".into()),
        description: Some("dados de teste".into()),
    }
}

fn doc_with_attachment() -> PagedDocument {
    layout(&Content::sequence(vec![
        Content::text("P1478 núcleo A4f sem anexo"),
        Content::pdf_attach(attachment_elem()),
    ]))
}

fn doc_with_label_no_attachment() -> PagedDocument {
    layout(&Content::label(
        "sec1",
        Content::text("P1478 núcleo A4f sem anexo, com destino nomeado"),
    ))
}

fn standards_for(names: &[PdfStandard]) -> PdfStandards {
    PdfStandards::new(names.iter().copied()).expect("standards válidos para P1478")
}

fn export_doc_with_standard(doc: &PagedDocument, standard: PdfStandards) -> Vec<u8> {
    super::super::export_pdf_with_document_id_and_tags_and_serialization_and_pages_and_standard(
        doc,
        None,
        StreamMode::Verbose,
        super::super::PdfTags::Enabled,
        PdfSerializationMode::Compact,
        None,
        standard,
    )
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

/// Segue `/Names {id} 0 R` no catálogo até ao dicionário `/Names`, e
/// dentro dele `/EmbeddedFiles {id2} 0 R` até à name tree. `None` se
/// qualquer um dos dois níveis estiver ausente.
fn embedded_files_body(pdf: &[u8]) -> Option<String> {
    let catalog = catalog_body(pdf);
    let names_id: usize = Regex::new(r"/Names\s+(\d+)\s+0\s+R")
        .unwrap()
        .captures(&catalog)?
        .get(1)?
        .as_str()
        .parse()
        .ok()?;
    let names_dict = object_body(pdf, names_id);
    let ef_id: usize = Regex::new(r"/EmbeddedFiles\s+(\d+)\s+0\s+R")
        .unwrap()
        .captures(&names_dict)?
        .get(1)?
        .as_str()
        .parse()
        .ok()?;
    Some(object_body(pdf, ef_id))
}

// ── Caso 1/2 — A4f sem anexo: /EmbeddedFiles vazio, sem /AF/Filespec ──

#[test]
fn p1478_a4f_sem_anexo_produz_embedded_files_vazio() {
    let pdf = export_doc_with_standard(&plain_doc(), standards_for(&[PdfStandard::A4f]));
    let ef = embedded_files_body(&pdf).unwrap_or_else(|| {
        panic!(
            "A4f sem anexo: catálogo sem /Names -> /EmbeddedFiles resolúvel (P1478 RED esperado); pdf={}",
            String::from_utf8_lossy(&pdf)
        )
    });
    assert_eq!(
        ef.trim(),
        "<< /Names [] >>",
        "A4f sem anexo: /EmbeddedFiles deve resolver para uma name tree com /Names vazio; corpo={ef}"
    );
}

#[test]
fn p1478_a4f_sem_anexo_nao_produz_af_nem_filespec() {
    let pdf = export_doc_with_standard(&plain_doc(), standards_for(&[PdfStandard::A4f]));
    let catalog = catalog_body(&pdf);
    assert!(
        !catalog.contains("/AF ["),
        "A4f sem anexo: catálogo não deve ter /AF (nenhuma Filespec para referenciar); catalog={catalog}"
    );
    let raw = String::from_utf8_lossy(&pdf);
    assert!(
        !raw.contains("/Type /Filespec"),
        "A4f sem anexo: não deve existir objeto Filespec"
    );
    assert!(
        !raw.contains("/Type /EmbeddedFile"),
        "A4f sem anexo: não deve existir objeto EmbeddedFile"
    );
}

// ── Caso 3 — A4e sem anexo: /EmbeddedFiles ausente ──────────────────

#[test]
fn p1478_a4e_sem_anexo_nao_produz_embedded_files() {
    let pdf = export_doc_with_standard(&plain_doc(), standards_for(&[PdfStandard::A4e]));
    assert!(
        embedded_files_body(&pdf).is_none(),
        "A4e sem anexo não deve ganhar /EmbeddedFiles (regressão) — só A4f exige árvore vazia"
    );
}

// ── Caso 4 — tabela negativa: nenhum outro perfil ganha árvore vazia ─

#[test]
fn p1478_outros_perfis_sem_anexo_nao_produzem_embedded_files_vazio() {
    let cases: &[Option<PdfStandard>] = &[
        None,
        Some(PdfStandard::A1a),
        Some(PdfStandard::A1b),
        Some(PdfStandard::A2a),
        Some(PdfStandard::A2b),
        Some(PdfStandard::A2u),
        Some(PdfStandard::A3a),
        Some(PdfStandard::A3b),
        Some(PdfStandard::A3u),
        Some(PdfStandard::A4),
    ];
    for case in cases.iter().copied() {
        let standard = match case {
            None => PdfStandards::default(),
            Some(raw) => standards_for(&[raw]),
        };
        let pdf = export_doc_with_standard(&plain_doc(), standard);
        assert!(
            embedded_files_body(&pdf).is_none(),
            "{case:?} sem anexo não deve ganhar /EmbeddedFiles (regressão de escopo — só A4f)"
        );
    }
}

// ── Caso 5 — A4f com anexo: comportamento P1476 preservado ─────────

#[test]
fn p1478_a4f_com_anexo_preserva_comportamento_p1476() {
    let pdf = export_doc_with_standard(
        &doc_with_attachment(),
        standards_for(&[PdfStandard::A4f]),
    );
    let ef = embedded_files_body(&pdf).expect(
        "A4f com anexo: /EmbeddedFiles deve continuar resolúvel (regressão P1476)",
    );
    assert!(
        ef.contains("data.txt"),
        "A4f com anexo: name tree deve conter a entrada do anexo; corpo={ef}"
    );
    let catalog = catalog_body(&pdf);
    assert!(
        catalog.contains("/AF ["),
        "A4f com anexo: catálogo deve continuar a ter /AF (regressão P1476); catalog={catalog}"
    );
    let raw = String::from_utf8_lossy(&pdf);
    assert!(
        raw.contains("/AFRelationship /Supplement"),
        "A4f com anexo: Filespec deve continuar a ter /AFRelationship (regressão P1476)"
    );
}

// ── Caso 6 — A4f sem anexo, com destino nomeado: /Dests e
// /EmbeddedFiles coexistem sob a mesma /Names do catálogo ───────────

#[test]
fn p1478_a4f_sem_anexo_com_destino_nomeado_preserva_dests() {
    let pdf = export_doc_with_standard(
        &doc_with_label_no_attachment(),
        standards_for(&[PdfStandard::A4f]),
    );
    let catalog = catalog_body(&pdf);
    let names_id: usize = Regex::new(r"/Names\s+(\d+)\s+0\s+R")
        .unwrap()
        .captures(&catalog)
        .unwrap_or_else(|| panic!("catálogo sem /Names; catalog={catalog}"))
        .get(1)
        .unwrap()
        .as_str()
        .parse()
        .unwrap();
    let names_dict = object_body(&pdf, names_id);
    assert!(
        names_dict.contains("/Dests"),
        "/Names deve preservar /Dests (destino nomeado pré-existente); corpo={names_dict}"
    );
    assert!(
        names_dict.contains("/EmbeddedFiles"),
        "/Names deve ganhar /EmbeddedFiles no mesmo dicionário, sem substituir /Dests; corpo={names_dict}"
    );
    let ef = embedded_files_body(&pdf)
        .expect("/EmbeddedFiles deve continuar resolúvel com /Dests presente");
    assert_eq!(ef.trim(), "<< /Names [] >>");
}
