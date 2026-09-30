//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/tests/p1477_pdfa_icc_version_and_cidset_tests.md
//! @prompt-hash 0031cfe9
//! @layer L3
//! @updated 2026-09-22
//!
//! ── P1477 — Núcleo PDF/A-1a/1b: segundo perfil ICC (v2) por versão do PDF,
//! `/CIDSet` universal ────────────────────────────────────────────────────
//!
//! Testes RED (Agente B, contrato "RED sem ver a implementação") contra o
//! baseline pré-P1477 (`8b6f41db0`, confirmação humana registada, código
//! ainda não escrito). Ver `00_nucleo/materialization/typst-passo-1477.md`
//! (medição completa) e `infra/export/images.md`/`infra/export/builder.md`
//! §P1477 (contrato). Réplica deliberada do padrão de infraestrutura de
//! `p1475_pdfa_conformance_core_tests`/`p1476_pdfa_attachments_af_tests`
//! acima (extração por regex de catálogo/corpo de objeto/cabeçalho-antes-
//! de-stream), estendida com extração *binária* do conteúdo do stream (via
//! `regex::bytes::Regex` + `/Length` exato, nunca por procura textual de
//! "endstream" — o conteúdo comprimido pode conter qualquer byte) para
//! poder descomprimir e medir o tamanho real do perfil ICC embutido ou o
//! bitset `/CIDSet`.

use super::*;
use regex::Regex;
use std::io::Read;
use typst_core::entities::document_format_options::{PdfStandard, PdfStandards};
use typst_core::entities::layout_types::PagedDocument;

fn simple_doc() -> PagedDocument {
    layout(&Content::text("P1477 núcleo ICC v2 + CIDSet"))
}

fn standards_for(names: &[PdfStandard]) -> PdfStandards {
    PdfStandards::new(names.iter().copied()).expect("standards válidos para P1477")
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

fn catalog_body(pdf: &[u8]) -> String {
    object_body(pdf, 1)
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

fn object_header_before_stream(pdf: &[u8], id: usize) -> String {
    let text = String::from_utf8_lossy(pdf).into_owned();
    let pattern = format!(r"(?ms)^{id} 0 obj\n(.*?)stream\n");
    Regex::new(&pattern)
        .ok()
        .and_then(|re| re.captures(&text))
        .map(|c| c[1].to_string())
        .unwrap_or_default()
}

/// Extrai os bytes BRUTOS (binários, pré-descompressão) do stream de um
/// objeto indireto, usando o `/Length` declarado no seu próprio
/// dicionário para cortar exatamente — nunca por procura textual de
/// "endstream" (o conteúdo comprimido pode conter esse padrão de bytes
/// por acaso). `regex::bytes::Regex` opera sobre `&[u8]` sem exigir
/// UTF-8 válido, ao contrário de `String::from_utf8_lossy` (que insere
/// U+FFFD e desalinha offsets face ao buffer original quando há bytes
/// binários antes do ponto de interesse).
fn object_stream_raw_bytes(pdf: &[u8], id: usize) -> Vec<u8> {
    let pattern = format!(r"(?ms)^{id} 0 obj\n.*?/Length (\d+).*?stream\n");
    let re = regex::bytes::Regex::new(&pattern)
        .unwrap_or_else(|e| panic!("regex inválida para objeto {id}: {e}"));
    let caps = re.captures(pdf).unwrap_or_else(|| {
        panic!(
            "objeto {id} (stream) não encontrado; pdf_len={} primeiros_bytes={}",
            pdf.len(),
            String::from_utf8_lossy(&pdf[..pdf.len().min(400)])
        )
    });
    let len: usize = std::str::from_utf8(&caps[1])
        .expect("/Length deve ser ASCII")
        .parse()
        .expect("/Length deve ser inteiro");
    let start = caps.get(0).unwrap().end();
    pdf[start..start + len].to_vec()
}

/// Descomprime um stream `/FlateDecode` (zlib) — mesmo formato usado por
/// `build_icc_profile_stream` (P777/P883) e pelo `/CIDSet` (P1477,
/// mesmo padrão declarado no contrato).
fn inflate(compressed: &[u8]) -> Vec<u8> {
    let mut decoder = flate2::read::ZlibDecoder::new(compressed);
    let mut out = Vec::new();
    decoder
        .read_to_end(&mut out)
        .expect("stream deve ser zlib/FlateDecode válido");
    out
}

/// Localiza o perfil ICC PARTILHADO (`/c0 [/ICCBased {id} 0 R]`, os
/// pontos P777/P956/P1477 item 1 de `builder.md` — build_helvetica/
/// build_cidfont/build_multifont, sempre alocado quando `verbose` é
/// verdadeiro independentemente de `standard`) e devolve o seu tamanho
/// descomprimido em bytes.
fn shared_icc_profile_len(pdf: &[u8]) -> usize {
    let text = String::from_utf8_lossy(pdf).into_owned();
    let re = Regex::new(r"/c0 \[/ICCBased (\d+) 0 R\]").unwrap();
    let caps = re.captures(&text).unwrap_or_else(|| {
        panic!(
            "PDF sem /c0 [/ICCBased {{id}} 0 R] nos recursos da página (modo verbose deveria sempre embutir, P956); pdf={text}"
        )
    });
    let id: usize = caps[1].parse().unwrap();
    inflate(&object_stream_raw_bytes(pdf, id)).len()
}

// ── Item 1 (builder.md P1477) — perfil ICC partilhado (/c0) selecionado
// por versão do PDF (images.md P1477 item 3, regra krilla §2.1 do passo)

#[test]
fn p1477_icc_v2_selected_for_versions_1_4_1_5_1_6_shared_point() {
    // RED esperado: `srgb_icc_profile_for_version` ainda não existe —
    // os três pontos partilhados (build_helvetica/build_cidfont/
    // build_multifont, builder.rs:1053/1434/1936) continuam a chamar
    // `srgb_icc_profile_bytes()` incondicional (v4, 480 bytes), nunca o
    // v2 (736 bytes) exigido para versões 1.4-1.6.
    for raw in [PdfStandard::V1_4, PdfStandard::V1_5, PdfStandard::V1_6] {
        let pdf = export_with_standard(standards_for(&[raw]));
        let len = shared_icc_profile_len(&pdf);
        assert_eq!(
            len, 736,
            "{raw:?}: perfil ICC partilhado (/c0) deve ser o v2 (736 bytes) para versões 1.4-1.6; obtido {len} bytes"
        );
    }
}

#[test]
fn p1477_icc_v4_regression_for_versions_1_7_2_0_shared_point() {
    // Guarda de não-regressão: já passa hoje (1.7/2.0 continuam com o
    // v4, 480 bytes — comportamento atual correto, `images.md` P1477
    // "regressão zero").
    for raw in [PdfStandard::V1_7, PdfStandard::V2_0] {
        let pdf = export_with_standard(standards_for(&[raw]));
        let len = shared_icc_profile_len(&pdf);
        assert_eq!(
            len, 480,
            "{raw:?}: perfil ICC partilhado (/c0) deve continuar o v4 (480 bytes, regressão zero); obtido {len} bytes"
        );
    }
}

#[test]
fn p1477_icc_v4_regression_without_any_standard_shared_point() {
    // Guarda de não-regressão: já passa hoje. Sem `standard` nenhum,
    // `self.standard.version` é `V1_7` (default) — perfil v4
    // preservado (builder.md P1477 item 1, "regressão zero para o caso
    // comum").
    let pdf = export_with_standard(PdfStandards::default());
    let len = shared_icc_profile_len(&pdf);
    assert_eq!(
        len, 480,
        "sem standard, perfil ICC partilhado (/c0) deve continuar o v4 (480 bytes); obtido {len} bytes"
    );
}

#[test]
fn p1477_icc_v2_selected_for_a1a_and_a1b_shared_point() {
    // RED esperado: a-1a/a-1b forçam versão implícita V1_4
    // (`PdfArchival::{A1a,A1b}::{min,max}_version() == V1_4`, medido em
    // `document_format_options.rs:176/186`) — devem produzir o perfil
    // v2 no ponto partilhado, tal como qualquer documento cuja versão
    // efetiva seja 1.4-1.6 (a regra é por versão, não por archival —
    // typst-passo-1477.md §2.1).
    for raw in [PdfStandard::A1a, PdfStandard::A1b] {
        let pdf = export_with_standard(standards_for(&[raw]));
        let len = shared_icc_profile_len(&pdf);
        assert_eq!(
            len, 736,
            "{raw:?}: perfil ICC partilhado (/c0) deve ser o v2 (736 bytes, versão implícita 1.4); obtido {len} bytes"
        );
    }
}

// ── Item 1 (ponto 4) — perfil ICC do /OutputIntent selecionado por
// versão. Gate de PRESENÇA do /OutputIntent é pré-existente
// (P1475/P1476: só A2/A3/A4(f/e) hoje, nunca A1a/A1b —
// `builder.rs:2768-2781`, medido nesta baseline); P1477 só troca o
// perfil usado NO objeto já emitido por esse gate. ─────────────────────

#[test]
fn p1477_icc_v4_regression_for_output_intent_default_version_gated_archival() {
    // Guarda de não-regressão: já passa hoje. A2b sozinho (sem versão
    // explícita) resolve `self.standard.version == V1_7` (default do
    // range [V1_4,V1_7] de A2/A3 quando nenhuma versão é citada —
    // `document_format_options.rs:436-450`) — /OutputIntent deve
    // continuar v4.
    let pdf = export_with_standard(standards_for(&[PdfStandard::A2b]));
    let catalog = catalog_body(&pdf);
    let intents_re = Regex::new(r"/OutputIntents\s*\[\s*(\d+)\s+0\s+R\s*\]").unwrap();
    let caps = intents_re
        .captures(&catalog)
        .unwrap_or_else(|| panic!("A2b: catálogo sem /OutputIntents; catalog={catalog}"));
    let intent_id: usize = caps[1].parse().unwrap();
    let intent_obj = object_body(&pdf, intent_id);
    let profile_re = Regex::new(r"/DestOutputProfile\s+(\d+)\s+0\s+R").unwrap();
    let profile_id: usize = profile_re
        .captures(&intent_obj)
        .map(|c| c[1].parse().unwrap())
        .unwrap_or_else(|| {
            panic!("A2b: /OutputIntent sem /DestOutputProfile; {intent_obj}")
        });
    let len = inflate(&object_stream_raw_bytes(&pdf, profile_id)).len();
    assert_eq!(
        len, 480,
        "A2b sem versão explícita: perfil do /OutputIntent deve continuar v4 (480 bytes); obtido {len}"
    );
}

#[test]
fn p1477_icc_v2_selected_for_output_intent_with_explicit_version_1_4_to_1_6() {
    // RED esperado: combinar um archival já gated para /OutputIntent
    // (A2/A3, P1475) com uma versão explícita 1.4-1.6 é uma combinação
    // válida (está dentro do range [min_version,max_version] desse
    // archival, `document_format_options.rs:367-453`) e força
    // `self.standard.version` a 1.4-1.6 SEM mudar o gate de presença do
    // /OutputIntent (que continua a existir — A2/A3 sempre o
    // produzem). O único efeito observável é o perfil embutido trocar
    // de v4 para v2. Testa o ponto 4 do contrato (builder.md P1477
    // item 1, `emit_pdfa_output_intent`) sem depender da ambiguidade
    // sobre se A1a/A1b também ganham /OutputIntent (ver teste separado
    // abaixo).
    let cases: &[(PdfStandard, PdfStandard)] =
        &[(PdfStandard::A2b, PdfStandard::V1_4), (PdfStandard::A3u, PdfStandard::V1_6)];
    let intents_re = Regex::new(r"/OutputIntents\s*\[\s*(\d+)\s+0\s+R\s*\]").unwrap();
    let profile_re = Regex::new(r"/DestOutputProfile\s+(\d+)\s+0\s+R").unwrap();
    for (archival, version) in cases.iter().copied() {
        let pdf = export_with_standard(standards_for(&[archival, version]));
        let catalog = catalog_body(&pdf);
        let caps = intents_re.captures(&catalog).unwrap_or_else(|| {
            panic!(
                "{archival:?}+{version:?}: catálogo sem /OutputIntents; catalog={catalog}"
            )
        });
        let intent_id: usize = caps[1].parse().unwrap();
        let intent_obj = object_body(&pdf, intent_id);
        let profile_id: usize = profile_re
            .captures(&intent_obj)
            .map(|c| c[1].parse().unwrap())
            .unwrap_or_else(|| {
                panic!(
                    "{archival:?}+{version:?}: /OutputIntent sem /DestOutputProfile; {intent_obj}"
                )
            });
        let len = inflate(&object_stream_raw_bytes(&pdf, profile_id)).len();
        assert_eq!(
            len, 736,
            "{archival:?}+{version:?}: perfil do /OutputIntent deve ser v2 (736 bytes); obtido {len}"
        );
    }
}

#[test]
fn p1477_output_intent_v2_for_a1a_a1b_per_l0_acceptance_section_5() {
    // RED esperado — literal do critério de fechamento §5 do passo
    // ("a-1a/a-1b ... embutem v2 nos três pontos partilhados E no
    // /OutputIntent"). ATENÇÃO — ambiguidade do L0 resolvida por conta
    // própria (Agente B, sem ver a implementação): o gate ATUAL de
    // `emit_pdfa_output_intent` (`builder.rs:2768-2781`, medido nesta
    // baseline) só inclui A2a/A2b/A2u/A3a/A3b/A3u/A4/A4f/A4e — NUNCA
    // A1a/A1b. Para este teste passar, a implementação de P1477 tem de
    // ADICIONAR A1a/A1b a esse `matches!` (não só trocar o perfil no
    // ponto já gated) — item não enumerado explicitamente na secção 3
    // "Contrato" de `builder.md` P1477 (que só fala em "trocar as
    // quatro chamadas", presumindo implicitamente que as quatro já
    // disparam para a-1a/a-1b, o que não é o caso nesta baseline). Isto
    // também CONTRADIZ a guarda de não-regressão de P1475
    // `p1475_output_intent_absent_for_none_a1_ua1_alone_regression_guard`
    // para os casos A1a/A1b especificamente — mesmo padrão de mudança
    // de escopo intencional já visto em P1476 para A4f/A4e (ver
    // comentário de `p1476_output_intent_present_for_a4f_a4e` acima);
    // não corrigido aqui — fora do escopo de Agente B tocar testes de
    // P1475. Ver relatório final do Agente B.
    let intents_re = Regex::new(r"/OutputIntents\s*\[\s*(\d+)\s+0\s+R\s*\]").unwrap();
    let profile_re = Regex::new(r"/DestOutputProfile\s+(\d+)\s+0\s+R").unwrap();
    for raw in [PdfStandard::A1a, PdfStandard::A1b] {
        let pdf = export_with_standard(standards_for(&[raw]));
        let catalog = catalog_body(&pdf);
        let caps = intents_re.captures(&catalog).unwrap_or_else(|| {
            panic!(
                "{raw:?}: catálogo sem /OutputIntents (P1477 §5, RED esperado — requer estender o gate de emit_pdfa_output_intent a A1a/A1b); catalog={catalog}"
            )
        });
        let intent_id: usize = caps[1].parse().unwrap();
        let intent_obj = object_body(&pdf, intent_id);
        let profile_id: usize = profile_re
            .captures(&intent_obj)
            .map(|c| c[1].parse().unwrap())
            .unwrap_or_else(|| {
                panic!("{raw:?}: /OutputIntent sem /DestOutputProfile; {intent_obj}")
            });
        let len = inflate(&object_stream_raw_bytes(&pdf, profile_id)).len();
        assert_eq!(
            len, 736,
            "{raw:?}: perfil do /OutputIntent deve ser v2 (736 bytes); obtido {len}"
        );
    }
}

// ── Item 2 (builder.md P1477) — `/CIDSet` no `/FontDescriptor` de toda
// CIDFont embutida ───────────────────────────────────────────────────

/// Bitset "denso desde o bit 0": todos os bits `1` contíguos a partir
/// da posição 0, seguidos só de bits `0` até ao fim (typst-passo-
/// 1477.md §2.4 — nenhum gap no meio, qualquer que seja N). Aceita o
/// caso-limite de um bitset totalmente preenchido a 1.
fn assert_dense_bitset_from_zero(bytes: &[u8], context: &str) {
    let bits: Vec<bool> = bytes
        .iter()
        .flat_map(|&b| (0..8).map(move |i| (b & (0x80 >> i)) != 0))
        .collect();
    assert!(!bits.is_empty(), "{context}: bitset vazio");
    match bits.iter().position(|&b| !b) {
        None => {
            // Todo o bitset a 1 — denso por definição (N == capacidade).
        }
        Some(idx) => {
            assert!(
                idx > 0,
                "{context}: bitset sem nenhum bit a 1 (nenhum CID presente); bits={bits:?}"
            );
            assert!(
                bits[idx..].iter().all(|&b| !b),
                "{context}: bitset não é denso desde o bit 0 — bit=1 encontrado depois de um bit=0 em idx={idx}; bits={bits:?}"
            );
        }
    }
}

/// Últimos `max_chars` caracteres de `text`, cortando numa fronteira de
/// carácter válida — `&text[text.len() - N..]` pode cair a meio de uma
/// sequência UTF-8 multi-byte (ex.: o marcador binário `%PDF-1.7\n%⋯`
/// no início do ficheiro, reencodado como U+FFFD por
/// `String::from_utf8_lossy`) e entrar em pânico na indexação — bug
/// observado na primeira corrida destes testes (mensagem de pânico
/// mascarada por "byte index ... is not a char boundary").
fn safe_tail(text: &str, max_chars: usize) -> &str {
    match text.char_indices().rev().nth(max_chars.saturating_sub(1)) {
        Some((idx, _)) => &text[idx..],
        None => text,
    }
}

fn cidset_id_from_font_descriptor(pdf: &[u8]) -> usize {
    let text = String::from_utf8_lossy(pdf).into_owned();
    let re = Regex::new(r"/CIDSet\s+(\d+)\s+0\s+R").unwrap();
    re.captures(&text).map(|c| c[1].parse().unwrap()).unwrap_or_else(|| {
        panic!(
            "/FontDescriptor sem /CIDSet {{id}} 0 R (P1477 item 3.3/item 2, RED esperado); pdf_tail={}",
            safe_tail(&text, 2000)
        )
    })
}

/// Achado de integração (papel C): a versão original deste helper usava
/// `Cantarell-VF.otf` (CFF2/HVAR, mesma fonte citada em §P772u de
/// `builder.md` como caso conhecido em que `subsetter::subset` falha
/// com `Unimplemented` — CFF2 variável não suporta subsetting nesta
/// versão da crate `subsetter`). Isso fazia os dois testes de `/CIDSet`
/// cairem no caminho de fallback (`glyph_mapping.is_empty()`, fonte
/// embutida por inteiro, CIDs = GIDs originais esparsos) em vez do
/// caminho de subset bem sucedido que o teste pretendia exercitar —
/// `assert_dense_bitset_from_zero` falhava com um bitset genuinamente
/// esparso, não por bug de implementação. `NotoSans-Regular.ttf`
/// (TrueType simples, mesma fonte já usada com sucesso pelo fixture
/// `p307b/sources/09-cidfont.typ`, confirmado por Agente A a produzir
/// um bitset denso `0..29`) substitui Cantarell para os dois testes que
/// exigem subset bem sucedido.
fn subsettable_font_data() -> Vec<u8> {
    let fixture_path =
        concat!(env!("CARGO_MANIFEST_DIR"), "/fixtures/fonts/NotoSans-Regular.ttf");
    std::fs::read(fixture_path).unwrap_or_else(|e| {
        panic!(
            "fixture NotoSans-Regular.ttf não encontrada (necessária para forçar build_cidfont/build_multifont com subset bem sucedido): {e}"
        )
    })
}

/// Achado de integração (papel C, P1479): as duas asserções de
/// `/CIDSet` (`build_cidfont`/`build_multifont` abaixo) eram
/// praticamente idênticas linha-a-linha (`cidset_id_from_font_
/// descriptor` → `object_stream_raw_bytes` → `inflate` → `assert_
/// dense_bitset_from_zero`), duplicação que se revelou ser o gatilho
/// real do achado `[PARSE]` do `crystalline-lint` (confirmado por
/// bissecção: trivializar só uma das duas funções não resolvia,
/// trivializar as duas resolvia — o problema estava na repetição do
/// padrão, não no conteúdo de nenhuma delas isoladamente). Extraído
/// para um único helper, sem mudança de asserção nem de mensagem.
fn assert_cidset_present_and_dense(pdf: &[u8], context: &str) {
    let cidset_id = cidset_id_from_font_descriptor(pdf);
    let header = object_header_before_stream(pdf, cidset_id);
    assert!(
        header.contains("/Filter /FlateDecode"),
        "objeto {cidset_id} (/CIDSet) deve ser comprimido com FlateDecode (mesmo padrão de build_icc_profile_stream, P883); header={header}; contexto={context}"
    );
    let raw_bytes = object_stream_raw_bytes(pdf, cidset_id);
    let decompressed = inflate(&raw_bytes);
    assert_dense_bitset_from_zero(&decompressed, context);
}

#[test]
fn p1477_cidset_present_in_font_descriptor_and_stream_is_dense_bitset() {
    // RED esperado: `build_cidfont` ainda não constrói/anexa `/CIDSet`
    // a nenhum `/FontDescriptor` (builder.md P1477 item 2). Mesmo texto
    // usado na medição vanilla do passo (§2.4). Requer subset bem
    // sucedido (bitset denso) — ver nota em `subsettable_font_data`.
    let font_data = subsettable_font_data();
    let doc = layout(&Content::text("Um parágrafo curto"));
    let pdf = export_pdf_with_font(&doc, &font_data, StreamMode::Verbose);
    assert_cidset_present_and_dense(&pdf, "objeto /CIDSet (build_cidfont)");
}

#[test]
fn p1477_cidset_absent_for_helvetica_path_regression_guard() {
    // Guarda de não-regressão / não-objetivo explícito (typst-passo-
    // 1477.md §6): `/CIDSet` não se aplica ao caminho build_helvetica
    // (Type1/Helvetica não é CIDFont, regra veraPDF 6.3.5 não se
    // aplica) — já é o comportamento atual (nada emite este campo
    // hoje) e deve continuar assim.
    let pdf = export_with_standard(PdfStandards::default());
    let raw_text = String::from_utf8_lossy(&pdf);
    assert!(
        !raw_text.contains("/CIDSet"),
        "caminho Helvetica (sem fonte embutida) não deve emitir /CIDSet"
    );
}

#[test]
fn p1477_cidset_universal_present_regardless_of_standard() {
    // RED esperado — decisão CONFIRMADA (typst-passo-1477.md, cabeçalho
    // "Estado", resolução via AskUserQuestion): `/CIDSet` é universal,
    // presente em toda CIDFont embutida com `/FontFile*`, independente
    // de `standard` (incluindo nenhum) — não gated a A1a/A1b.
    let font_data = subsettable_font_data();
    let doc = layout(&Content::text("Um parágrafo curto"));
    for standard in [
        PdfStandards::default(),
        standards_for(&[PdfStandard::A1b]),
        standards_for(&[PdfStandard::A2b]),
    ] {
        let pdf = export_pdf_with_font_and_document_id_and_tags_and_serialization_and_pages_and_standard(
            &doc,
            &font_data,
            None,
            StreamMode::Verbose,
            PdfTags::Enabled,
            PdfSerializationMode::Compact,
            None,
            standard,
        );
        let text = String::from_utf8_lossy(&pdf);
        assert!(
            text.contains("/CIDSet"),
            "/CIDSet deve estar presente independentemente de standard; pdf_tail={}",
            safe_tail(&text, 1500)
        );
    }
}

#[test]
fn p1477_cidset_present_for_multifont_path() {
    // RED esperado: `build_multifont` (builder.rs:~1888-1906, loop por
    // fonte) ainda não emite `/CIDSet` — cobertura do segundo ponto de
    // emissão citado no contrato (builder.md P1477 item 2), distinto
    // de `build_cidfont`. Single-font (`fonts.len() == 1`) é caso
    // particular válido do caminho multifont (doc comment de
    // `export_pdf_multifont`, mod.rs:582-584).
    // Requer subset bem sucedido (bitset denso) — ver nota em
    // `subsettable_font_data`.
    let font_data = subsettable_font_data();
    let doc = layout(&Content::text("Um parágrafo curto"));
    let variant = FontVariant::default();
    let font_list = FontList::single(ecow::EcoString::from("Noto Sans"));
    let pdf = export_pdf_multifont(
        &doc,
        &[((font_list, variant, FontVariations::default()), font_data)],
        StreamMode::Verbose,
    );
    assert_cidset_present_and_dense(&pdf, "objeto /CIDSet (build_multifont)");
}
