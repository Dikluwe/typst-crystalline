//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/builder/metadata.md
//! @prompt-hash e4ecf10b
//! @layer L3
//! @updated 2026-09-28

use super::navigation::PageIndexConverter;
use super::serialization::{
    escape_pdf_literal, insert_pdf_dict_entry, utf16be_hex_string,
};
use super::PdfBuilder;
use crate::export::build_icc_profile_stream;
use crate::export::srgb_icc_profile_for_version;
use typst_core::entities::document_format_options::PdfAccessibility;
use typst_core::entities::document_format_options::PdfArchival;
use typst_core::entities::layout_types::FrameItem;
use typst_core::entities::layout_types::PagedDocument;

/// **P611** — devolve o timestamp a usar em `/Info` e no XMP.
/// P601 — em testes de snapshot, permitir congelar a data para manter
/// os PDFs de referência determinísticos.
fn current_pdf_timestamp() -> time::OffsetDateTime {
    std::env::var("CRYSTALLINE_PDF_FIXED_EPOCH")
        .ok()
        .or_else(|| std::env::var("SOURCE_DATE_EPOCH").ok())
        .and_then(|s| s.parse::<i64>().ok())
        .and_then(|ts| time::OffsetDateTime::from_unix_timestamp(ts).ok())
        .unwrap_or_else(time::OffsetDateTime::now_utc)
}

/// **P612** — codifica 16 bytes em base64 (URL-safe não é necessário; usa-se
/// o alfabeto standard).
fn base64_encode_16(bytes: [u8; 16]) -> String {
    const ALPHABET: &[u8; 64] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(24);
    for chunk in bytes.chunks(3) {
        let b = match chunk.len() {
            3 => [chunk[0], chunk[1], chunk[2]],
            2 => [chunk[0], chunk[1], 0],
            1 => [chunk[0], 0, 0],
            _ => unreachable!(),
        };
        out.push(ALPHABET[(b[0] >> 2) as usize] as char);
        out.push(ALPHABET[(((b[0] & 0x03) << 4) | (b[1] >> 4)) as usize] as char);
        out.push(ALPHABET[(((b[1] & 0x0F) << 2) | (b[2] >> 6)) as usize] as char);
        out.push(ALPHABET[(b[2] & 0x3F) as usize] as char);
    }
    match bytes.len() % 3 {
        1 => {
            out.pop();
            out.pop();
            out.push_str("==");
        }
        2 => {
            out.pop();
            out.push('=');
        }
        _ => {}
    }
    out
}

/// **P615** — gera 16 bytes pseudoaleatórios para uso em
/// `DocumentID`/`InstanceID`.
///
/// O seed é gerado uma vez por processo via `getrandom`; dentro da mesma
/// execução, os IDs são determinísticos a partir desse seed. Isto satisfaz
/// dois requisitos contraditórios:
/// 1. Execuções separadas do compilador produzem IDs diferentes (paridade
///    com o vanilla 0.15.0).
/// 2. Testes unitários que compilam o mesmo documento duas no mesmo
///    processo esperam bytes PDF idênticos.
fn random_xmp_id_bytes() -> [u8; 16] {
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::OnceLock;

    static SEED: OnceLock<u64> = OnceLock::new();
    static COUNTER: AtomicU64 = AtomicU64::new(0);

    let seed = *SEED.get_or_init(|| {
        let mut bytes = [0u8; 8];
        if getrandom::getrandom(&mut bytes).is_ok() {
            u64::from_be_bytes(bytes)
        } else {
            // Fallback: timestamp de compilação. Nunca deixamos de emitir XMP.
            current_pdf_timestamp().unix_timestamp() as u64
        }
    });

    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let mut state = seed.wrapping_add(n);
    let mut bytes = [0u8; 16];
    for chunk in bytes.chunks_mut(8) {
        // xorshift64* — gerador simples e determinístico.
        state ^= state >> 12;
        state ^= state << 25;
        state ^= state >> 27;
        state = state.wrapping_mul(0x2545_f491_4f6c_dd1d);
        chunk.copy_from_slice(&state.to_be_bytes());
    }
    bytes
}

/// **P615/P617** — devolve `(instance_id, document_id)` para o pacote XMP.
/// Em testes (`CRYSTALLINE_PDF_FIXED_EPOCH` definida), o `InstanceID` é
/// fixo. Em produção, o `InstanceID` é sempre 16 bytes aleatórios.
/// O `DocumentID` usa `external_id` quando fornecido; caso contrário,
/// segue a mesma regra do `InstanceID` (fixo em testes, aleatório em
/// produção), conforme P615.
fn xmp_instance_and_document_id(external_id: Option<[u8; 16]>) -> (String, String) {
    // P615 — em testes (cfg!(test) ou variável de ambiente), o InstanceID
    // é fixo para manter os snapshots de bytes PDF deterministas.
    let is_test = cfg!(test) || std::env::var("CRYSTALLINE_PDF_FIXED_EPOCH").is_ok();
    let instance_id = if is_test {
        "dHlwc3QtY3J5c3QtaW5zdA==".to_string()
    } else {
        base64_encode_16(random_xmp_id_bytes())
    };

    let document_id = if let Some(id) = external_id {
        base64_encode_16(id)
    } else if is_test {
        "dHlwc3QtY3J5c3QtZG9jdQ==".to_string()
    } else {
        base64_encode_16(random_xmp_id_bytes())
    };

    (instance_id, document_id)
}

/// **P1475/P1476/P1477** — corpo do elemento `pdfaid` para um `PdfArchival`
/// concreto. A1/A2/A3 usam `part`+`conformance` (letra via `pdfaid_letter`);
/// A4f/A4e usam as três chaves `part`+`rev`+`conformance` (achado de
/// P1476 §2.2 — `conformance` só desaparece quando não há sufixo `f`/`e`);
/// A4 "puro" usa só `part`+`rev`. `match` exaustivo sobre as 11 variantes
/// de `PdfArchival` (sem `Option` — a resolução de `None` fica no chamador,
/// `emit_xmp_metadata`, via `Option::map`, V16).
fn pdfaid_body(a: PdfArchival) -> String {
    match a {
        PdfArchival::A1a | PdfArchival::A1b => format!(
            "<pdfaid:part>1</pdfaid:part><pdfaid:conformance>{}</pdfaid:conformance>",
            pdfaid_letter(a)
        ),
        PdfArchival::A2a | PdfArchival::A2b | PdfArchival::A2u => format!(
            "<pdfaid:part>2</pdfaid:part><pdfaid:conformance>{}</pdfaid:conformance>",
            pdfaid_letter(a)
        ),
        PdfArchival::A3a | PdfArchival::A3b | PdfArchival::A3u => format!(
            "<pdfaid:part>3</pdfaid:part><pdfaid:conformance>{}</pdfaid:conformance>",
            pdfaid_letter(a)
        ),
        PdfArchival::A4f => "<pdfaid:part>4</pdfaid:part><pdfaid:rev>2020</pdfaid:rev>\
             <pdfaid:conformance>F</pdfaid:conformance>"
            .to_string(),
        PdfArchival::A4e => "<pdfaid:part>4</pdfaid:part><pdfaid:rev>2020</pdfaid:rev>\
             <pdfaid:conformance>E</pdfaid:conformance>"
            .to_string(),
        PdfArchival::A4 => {
            "<pdfaid:part>4</pdfaid:part><pdfaid:rev>2020</pdfaid:rev>".to_string()
        }
    }
}

/// **P1480** — corpo XMP `pdfuaid` (ISO 14289-1, identificação PDF/UA).
/// Só `part` — ao contrário de `pdfaid_body`, não há eixo
/// `conformance`/`rev` em PDF/UA (medido em typst-passo-1480.md §1,
/// vanilla e krilla vendorizado concordam). Match exaustivo sobre
/// `PdfAccessibility` (não `Option`), mesmo padrão de `pdfaid_body`
/// (evita V16).
fn pdfuaid_body(a: PdfAccessibility) -> String {
    match a {
        PdfAccessibility::Ua1 => "<pdfuaid:part>1</pdfuaid:part>".to_string(),
    }
}

/// **P1480** — declaração mínima de extensão XMP exigida quando o schema
/// PDF/UA aparece dentro de um documento que também declara conformidade PDF/A.
fn pdfua_extension_schema() -> &'static str {
    concat!(
        "<pdfaExtension:schemas><rdf:Bag><rdf:li rdf:parseType=\"Resource\">",
        "<pdfaSchema:schema>PDF/UA Identification schema</pdfaSchema:schema>",
        "<pdfaSchema:namespaceURI>http://www.aiim.org/pdfua/ns/id/</pdfaSchema:namespaceURI>",
        "<pdfaSchema:prefix>pdfuaid</pdfaSchema:prefix>",
        "<pdfaSchema:property><rdf:Seq><rdf:li rdf:parseType=\"Resource\">",
        "<pdfaProperty:category>internal</pdfaProperty:category>",
        "<pdfaProperty:description>Part Number of ISO 14289 (PDF/UA)</pdfaProperty:description>",
        "<pdfaProperty:name>part</pdfaProperty:name>",
        "<pdfaProperty:valueType>Integer</pdfaProperty:valueType>",
        "</rdf:li></rdf:Seq></pdfaSchema:property>",
        "</rdf:li></rdf:Bag></pdfaExtension:schemas>"
    )
}

/// **P1475** — letra de conformidade `pdfaid:conformance` (ISO 19005-1/-2/-3),
/// derivada do sufixo do nome do archival. Único ponto de derivação —
/// evita repetir o `match` nos braços A1/A2/A3 de `pdfaid_body`.
/// Não chamado para `A4`/`A4f`/`A4e` sozinhas fora do `conformance` (ver
/// `pdfaid_body`).
fn pdfaid_letter(a: PdfArchival) -> &'static str {
    match a {
        // **P1477** — achado de integração: A1a/A1b confirmados no vanilla
        // (`--pdf-standard a-1b`) com a mesma forma part+conformance de
        // A2/A3 (`<pdfaid:part>1</pdfaid:part><pdfaid:conformance>B
        // </pdfaid:conformance>`) — movidos do braço de scope-out para os
        // braços nominais.
        PdfArchival::A1a | PdfArchival::A2a | PdfArchival::A3a => "A",
        PdfArchival::A1b | PdfArchival::A2b | PdfArchival::A3b => "B",
        PdfArchival::A2u | PdfArchival::A3u => "U",
        PdfArchival::A4 | PdfArchival::A4f | PdfArchival::A4e => {
            debug_assert!(false, "pdfaid_letter called for non-A1/A2/A3 archival");
            ""
        }
    }
}

/// **P611** — escapa caracteres XML especiais para texto dentro de elementos.
fn escape_xml_text(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            ch => out.push(ch),
        }
    }
    out
}

impl PdfBuilder {
    /// **P536/P601** — emite o dicionário `/Info` com Title, Author, Keywords,
    /// Creator, CreationDate e ModDate. Chamado no final de cada caminho de
    /// build antes de `serialize`. O dicionário é sempre emitido, mesmo quando
    /// `document_info` não tem campos de utilizador (P601).
    pub(super) fn emit_info(&mut self, doc: &PagedDocument) {
        // **P1475/P1476** — ISO 19005-4 6.1.3: `/Info` não pode estar
        // presente em PDF/A-4 — inclui A4f/A4e, mesma regra (medido
        // typst-passo-1476.md §2.6). Retorna cedo sem alocar nada;
        // `self.info_id` continua `None` e `serialize()` já omite `/Info`
        // do trailer nesse caso.
        if matches!(
            self.standard.archival,
            Some(PdfArchival::A4 | PdfArchival::A4f | PdfArchival::A4e)
        ) {
            return;
        }
        let mut parts = Vec::new();
        if let Some(title) = &doc.document_info.title {
            parts.push(format!("/Title {}", utf16be_hex_string(title.as_str())));
        }
        if let Some(author) = &doc.document_info.author {
            parts.push(format!("/Author {}", utf16be_hex_string(author.as_str())));
        }
        if let Some(keywords) = &doc.document_info.keywords {
            parts.push(format!("/Keywords {}", utf16be_hex_string(keywords.as_str())));
        }

        // Formato PDF: D:YYYYMMDDHHMMSS (UTC).
        let now = current_pdf_timestamp();
        let date = format!(
            "D:{:04}{:02}{:02}{:02}{:02}{:02}",
            now.year(),
            now.month() as u8,
            now.day(),
            now.hour(),
            now.minute(),
            now.second()
        );
        parts.push(format!("/CreationDate ({})", date));
        parts.push(format!("/ModDate ({})", date));
        parts.push("/Creator (typst-crystalline)".to_string());

        let next_id = self.objects.iter().map(|(id, _)| *id).max().unwrap_or(0) + 1;
        self.add(next_id, format!("<< {} >>", parts.join(" ")));
        self.info_id = Some(next_id);
    }

    /// **P1475/P1476** — emite `/OutputIntent` (perfil ICC sRGB, `/S
    /// /GTS_PDFA1` fixo independentemente do nível) e regista
    /// `/OutputIntents` no catálogo, gated ao conjunto de archivals
    /// (A2a/A2b/A2u/A3a/A3b/A3u/A4/A4f/A4e — A4f/A4e acrescentados em
    /// P1476, mesmo perfil ICC/forma, medição §2.6). Chamado nas três
    /// variantes `build_*`
    /// logo após `emit_info(doc)` e antes de `emit_xmp_metadata(doc)`,
    /// mesma posição relativa nas três, para preservar alocação de IDs
    /// idêntica entre caminhos. Aloca o seu próprio objeto ICC (não reusa o
    /// já alocado por P777 para JPEGs RGB, mesmo quando ambos existem no
    /// documento — decisão mecânica, ADR-0107, sem efeito observável para
    /// veraPDF).
    pub(super) fn emit_pdfa_output_intent(&mut self) {
        // **P1477** — achado de integração: A1a/A1b faltavam neste gate.
        // P1475 excluiu-os deliberadamente (o único perfil ICC disponível
        // então, v4, falha a regra 6.2.3.2 em PDF 1.4); P1477 traz o
        // perfil v2 (`srgb_icc_profile_for_version`, usado abaixo) que
        // remove esse bloqueio — typst-passo-1477.md §5 já previa "a-1a/
        // a-1b... embutem v2... E no /OutputIntent", só não tinha sido
        // adicionado a este `matches!` na primeira redação do contrato.
        let needs_output_intent = matches!(
            self.standard.archival,
            Some(
                PdfArchival::A1a
                    | PdfArchival::A1b
                    | PdfArchival::A2a
                    | PdfArchival::A2b
                    | PdfArchival::A2u
                    | PdfArchival::A3a
                    | PdfArchival::A3b
                    | PdfArchival::A3u
                    | PdfArchival::A4
                    | PdfArchival::A4f
                    | PdfArchival::A4e
            )
        );
        if !needs_output_intent {
            return;
        }

        // `build_icc_profile_stream`/`srgb_icc_profile_for_version` já estão
        // em escopo via `use crate::export::{...}` (builder.rs:130-141), mesmo
        // padrão de uso já existente em builder.rs:1043/1423/1924 (P777).
        // **P1477** — perfil selecionado pela versão efetiva do PDF (v2
        // para V1_4/V1_5/V1_6, v4 para V1_7/V2_0), consistente com os
        // outros três pontos de consumo.
        let profile_id = self.objects.iter().map(|(id, _)| *id).max().unwrap_or(0) + 1;
        self.add_bytes(
            profile_id,
            build_icc_profile_stream(srgb_icc_profile_for_version(self.standard.version)),
        );

        let intent_id = profile_id + 1;
        self.add(
            intent_id,
            format!(
                "<< /Type /OutputIntent /S /GTS_PDFA1 /OutputConditionIdentifier (Custom) \
                 /OutputCondition (sRGB) /RegistryName () /Info (sRGB v4.2) \
                 /DestOutputProfile {profile_id} 0 R >>"
            ),
        );

        if let Some((_, content)) = self.objects.iter_mut().find(|(id, _)| *id == 1) {
            insert_pdf_dict_entry(content, &format!(" /OutputIntents [{intent_id} 0 R]"));
        }
    }

    /// **P611** — emite o stream `/Type /Metadata /Subtype /XML` com um pacote
    /// XMP mínimo, referenciado a partir de `/Metadata` no catálogo. Sempre
    /// emitido, mesmo quando `document_info` está vazio.
    pub(super) fn emit_xmp_metadata(&mut self, doc: &PagedDocument) {
        let language = doc
            .document_language
            .unwrap_or(typst_core::entities::lang::Lang::ENGLISH)
            .as_str()
            .to_owned();
        let now = current_pdf_timestamp();
        let xmp_date = format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
            now.year(),
            now.month() as u8,
            now.day(),
            now.hour(),
            now.minute(),
            now.second()
        );

        // **P1468** — `xmpTPg:NPages` é uma contagem pública observável;
        // reflete só as páginas efetivamente exportadas (`pic`), igual a
        // `/Count`. Sem seleção, `pic` é a identidade (byte-idêntico).
        let pic = PageIndexConverter::new(doc.pages.len(), self.pages.as_ref());
        let n_pages = pic.exported_count();

        // Campos condicionais a metadados do utilizador.
        let title_elem = doc
            .document_info
            .title
            .as_ref()
            .map(|t| format!(
                "<dc:title><rdf:Alt><rdf:li xml:lang=\"x-default\">{}</rdf:li></rdf:Alt></dc:title>",
                escape_xml_text(t.as_str())
            ))
            .unwrap_or_default();
        let keywords_elem = doc
            .document_info
            .keywords
            .as_ref()
            .map(|k| {
                format!("<pdf:Keywords>{}</pdf:Keywords>", escape_xml_text(k.as_str()))
            })
            .unwrap_or_default();
        let creator_elem = doc
            .document_info
            .author
            .as_ref()
            .map(|a| {
                format!(
                    "<dc:creator><rdf:Seq><rdf:li>{}</rdf:li></rdf:Seq></dc:creator>",
                    escape_xml_text(a.as_str())
                )
            })
            .unwrap_or_default();

        // Identificadores (16 bytes em base64).
        // P615 — aleatórios por defeito; P617 — DocumentID pode ser fixado
        // externamente. InstanceID continua aleatório (ou fixo em testes).
        let (instance_id, document_id) = xmp_instance_and_document_id(self.document_id);

        // **P1475** — persiste o par para `serialize()` reusar no trailer
        // `/ID [ (instance) (document) ]`, universal (todo export).
        self.xmp_ids = Some((instance_id.clone(), document_id.clone()));

        // **P1475/P1476** — `pdfaid` identifica a conformidade PDF/A; A2/A3
        // usam `part`+`conformance` (letra via `pdfaid_letter`), A4 "puro"
        // usa `part`+`rev` (sem `conformance`, ISO 19005-4 eliminou o eixo
        // a/b/u para o nível sem qualificador). A4f/A4e usam as **três**
        // chaves `part`+`rev`+`conformance` (achado que corrige a
        // expectativa de P1475 — `conformance` só desaparece quando não há
        // sufixo `f`/`e`; medição typst-passo-1476.md §2.2). Vazio (e sem
        // `xmlns:pdfaid`) fora desse conjunto.
        // Isolado num helper exaustivo só sobre `PdfArchival` (sem
        // `Option`) — evita combinar `Option::None` com a cobertura
        // nominal do domínio no mesmo `match` (V16, mesmo padrão de
        // `af_relationship_name`/`af_relationship_variant_name`, P1476).
        let pdfaid_elem = self.standard.archival.map(pdfaid_body).unwrap_or_default();
        let pdfaid_xmlns = if pdfaid_elem.is_empty() {
            String::new()
        } else {
            " xmlns:pdfaid=\"http://www.aiim.org/pdfa/ns/id/\"".to_string()
        };

        // **P1480** — `pdfuaid` identifica a conformidade PDF/UA-1 (ISO
        // 14289-1), eixo independente de `pdfaid`; ambos coexistem quando
        // archival e accessibility são selecionados juntos (medido em
        // typst-passo-1480.md §1, `--pdf-standard a-2u --pdf-standard
        // ua-1`). Só `part` (sem `rev`/`conformance` — ISO 14289-1 não tem
        // esse eixo). Mesmo padrão isolado-do-`Option` de `pdfaid_body`
        // (evita V16).
        let pdfuaid_elem =
            self.standard.accessibility.map(pdfuaid_body).unwrap_or_default();
        let pdfuaid_xmlns = if pdfuaid_elem.is_empty() {
            String::new()
        } else {
            " xmlns:pdfuaid=\"http://www.aiim.org/pdfua/ns/id/\"".to_string()
        };

        // **P1480** — numa interseção PDF/A+PDF/UA, `pdfuaid:part` é uma
        // propriedade XMP não predefinida e precisa do schema de extensão.
        let pdfua_extension = !pdfaid_elem.is_empty() && !pdfuaid_elem.is_empty();
        let pdfua_extension_xmlns = if pdfua_extension {
            concat!(
                " xmlns:pdfaExtension=\"http://www.aiim.org/pdfa/ns/extension/\"",
                " xmlns:pdfaSchema=\"http://www.aiim.org/pdfa/ns/schema#\"",
                " xmlns:pdfaProperty=\"http://www.aiim.org/pdfa/ns/property#\""
            )
        } else {
            ""
        };
        let pdfua_extension_elem =
            if pdfua_extension { pdfua_extension_schema() } else { "" };

        // Estrutura exacta do vanilla (krilla + xmp-writer), sem quebras de
        // linha entre elementos, para manter a mesma forma do pacote XMP.
        let xml = format!(
            "<?xpacket begin=\"﻿\" id=\"W5M0MpCehiHzreSzNTczkc9d\"?>\
             <x:xmpmeta xmlns:x=\"adobe:ns:meta/\" x:xmptk=\"xmp-writer\">\
             <rdf:RDF xmlns:rdf=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\">\
             <rdf:Description rdf:about=\"\" xmlns:dc=\"http://purl.org/dc/elements/1.1/\" \
             xmlns:xmp=\"http://ns.adobe.com/xap/1.0/\" \
             xmlns:xmpMM=\"http://ns.adobe.com/xap/1.0/mm/\" \
             xmlns:xmpTPg=\"http://ns.adobe.com/xap/1.0/t/pg/\" \
             xmlns:pdf=\"http://ns.adobe.com/pdf/1.3/\"{pdfaid_xmlns}{pdfuaid_xmlns}{pdfua_extension_xmlns}>\
             {title}\
             {keywords}\
             {creator}\
             <xmp:CreatorTool>typst-crystalline</xmp:CreatorTool>\
             <dc:language><rdf:Bag><rdf:li>{language}</rdf:li></rdf:Bag></dc:language>\
             <xmp:ModifyDate>{date}</xmp:ModifyDate>\
             <xmp:CreateDate>{date}</xmp:CreateDate>\
             {pdfua_extension_elem}\
             {pdfuaid_elem}\
             <xmpTPg:NPages>{n_pages}</xmpTPg:NPages>\
             <dc:format>application/pdf</dc:format>\
             <xmpMM:InstanceID>{instance_id}</xmpMM:InstanceID>\
             <xmpMM:DocumentID>{document_id}</xmpMM:DocumentID>\
             <xmpMM:RenditionClass>proof</xmpMM:RenditionClass>\
             {pdfaid_elem}\
             <pdf:PDFVersion>{pdf_version}</pdf:PDFVersion></rdf:Description></rdf:RDF></x:xmpmeta>\
             <?xpacket end=\"r\"?>",
            title = title_elem,
            keywords = keywords_elem,
            creator = creator_elem,
            date = xmp_date,
            language = language,
            n_pages = n_pages,
            instance_id = instance_id,
            document_id = document_id,
            pdfaid_xmlns = pdfaid_xmlns,
            pdfuaid_xmlns = pdfuaid_xmlns,
            pdfua_extension_xmlns = pdfua_extension_xmlns,
            pdfaid_elem = pdfaid_elem,
            pdfuaid_elem = pdfuaid_elem,
            pdfua_extension_elem = pdfua_extension_elem,
            pdf_version = self.standard.version.as_str(),
        );

        let next_id = self.objects.iter().map(|(id, _)| *id).max().unwrap_or(0) + 1;
        let len = xml.len();
        let mut obj =
            format!("<< /Length {len} /Type /Metadata /Subtype /XML >>\nstream\n")
                .into_bytes();
        obj.extend_from_slice(xml.as_bytes());
        obj.extend_from_slice(b"\nendstream");
        self.add_bytes(next_id, obj);
        self.xmp_id = Some(next_id);

        // Editar objecto 1 (catálogo) para incluir /Metadata.
        if let Some((_, content)) = self.objects.iter_mut().find(|(id, _)| *id == 1) {
            let s = String::from_utf8_lossy(content);
            if let Some(idx) = s.rfind(">>") {
                let mut new = s[..idx].to_string();
                new.push_str(&format!(" /Metadata {next_id} 0 R"));
                new.push_str(&s[idx..]);
                *content = new.into_bytes();
            }
        }
        if matches!(self.standard.accessibility, Some(PdfAccessibility::Ua1)) {
            if let Some((_, catalog)) = self.objects.iter_mut().find(|(id, _)| *id == 1) {
                insert_pdf_dict_entry(catalog, &format!(" /Lang ({language})"));
            }
        }
    }

    /// Emite a árvore estrutural Document → Formula/Table e liga MCIDs,
    /// páginas e ParentTree. Não declara conformidade PDF/UA.
    pub(super) fn emit_tag_structure(&mut self, doc: &PagedDocument) {
        if self.pdf_tags != crate::export::PdfTags::Enabled {
            return;
        }

        #[derive(Clone)]
        enum TagAttrs {
            None,
            Header {
                table_id: u64,
                row: u32,
                column: u32,
                level: u32,
                scope: typst_core::entities::elements::table_cell::TableHeaderScope,
                rowspan: u32,
                colspan: u32,
            },
            Data {
                table_id: u64,
                row: u32,
                column: u32,
                rowspan: u32,
                colspan: u32,
            },
        }

        struct TagNode {
            role: &'static str,
            object_id: usize,
            table_id: Option<u64>,
            page_index: Option<usize>,
            mcid: Option<usize>,
            alt: Option<String>,
            lang: Option<String>,
            summary: Option<String>,
            attrs: TagAttrs,
            children: Vec<TagNode>,
        }

        fn semantic_node(
            item: &FrameItem,
            page_index: usize,
            next: &mut usize,
        ) -> Option<TagNode> {
            use typst_core::entities::layout_types::SemanticKind;
            let FrameItem::Semantic { kind, alt, items, .. } = item else {
                return None;
            };
            let mut container = |role: &'static str, table_id: Option<u64>| TagNode {
                role,
                object_id: 0,
                table_id,
                page_index: None,
                mcid: None,
                alt: None,
                lang: None,
                summary: None,
                attrs: TagAttrs::None,
                children: items
                    .iter()
                    .filter_map(|child| semantic_node(child, page_index, next))
                    .collect(),
            };
            match *kind {
                SemanticKind::Formula => {
                    let mcid = *next;
                    *next += 1;
                    Some(TagNode {
                        role: "Formula",
                        object_id: 0,
                        table_id: None,
                        page_index: Some(page_index),
                        mcid: Some(mcid),
                        alt: alt.as_ref().map(ToString::to_string),
                        lang: None,
                        summary: None,
                        attrs: TagAttrs::None,
                        children: vec![],
                    })
                }
                SemanticKind::Label => {
                    let mcid = *next;
                    *next += 1;
                    Some(TagNode {
                        role: "Lbl",
                        object_id: 0,
                        table_id: None,
                        page_index: Some(page_index),
                        mcid: Some(mcid),
                        alt: None,
                        lang: None,
                        summary: None,
                        attrs: TagAttrs::None,
                        children: vec![],
                    })
                }
                SemanticKind::Table { id } => {
                    let mut node = container("Table", Some(id));
                    node.summary = alt.as_ref().map(ToString::to_string);
                    Some(node)
                }
                SemanticKind::TableHead { table_id } => {
                    Some(container("THead", Some(table_id)))
                }
                SemanticKind::TableBody { table_id } => {
                    Some(container("TBody", Some(table_id)))
                }
                SemanticKind::TableRow { table_id, .. } => {
                    Some(container("TR", Some(table_id)))
                }
                SemanticKind::TableHeaderCell {
                    table_id,
                    row,
                    column,
                    level,
                    scope,
                    rowspan,
                    colspan,
                } => {
                    let mcid = *next;
                    *next += 1;
                    Some(TagNode {
                        role: "TH",
                        object_id: 0,
                        table_id: Some(table_id),
                        page_index: Some(page_index),
                        mcid: Some(mcid),
                        alt: None,
                        lang: None,
                        summary: None,
                        attrs: TagAttrs::Header {
                            table_id,
                            row,
                            column,
                            level,
                            scope,
                            rowspan,
                            colspan,
                        },
                        children: vec![],
                    })
                }
                SemanticKind::TableDataCell {
                    table_id,
                    row,
                    column,
                    rowspan,
                    colspan,
                } => {
                    let mcid = *next;
                    *next += 1;
                    Some(TagNode {
                        role: "TD",
                        object_id: 0,
                        table_id: Some(table_id),
                        page_index: Some(page_index),
                        mcid: Some(mcid),
                        alt: None,
                        lang: None,
                        summary: None,
                        attrs: TagAttrs::Data { table_id, row, column, rowspan, colspan },
                        children: vec![],
                    })
                }
                SemanticKind::Artifact(_)
                | SemanticKind::ExplicitLinebreakBoundary
                | SemanticKind::ParbreakBoundary => None,
            }
        }

        fn ua1_leaf_node(
            item: &FrameItem,
            page_index: usize,
            next: &mut usize,
        ) -> Option<TagNode> {
            let (role, alt, lang) = match item {
                FrameItem::TextSeparator { style, .. }
                | FrameItem::Text { style, .. }
                | FrameItem::TextShaped { style, .. }
                | FrameItem::Glyph { style, .. } => {
                    let role = match style.heading_level {
                        Some(1) => "H1",
                        Some(2) => "H2",
                        Some(3) => "H3",
                        Some(4) => "H4",
                        Some(5) => "H5",
                        Some(_) => "H6",
                        None if style.semantic_strong => "Strong",
                        None if style.semantic_emph => "Em",
                        None => "P",
                    };
                    // O texto padrão continua en se o catálogo herdou fr
                    // do primeiro trecho local.
                    (
                        role,
                        None,
                        Some(
                            style
                                .lang
                                .unwrap_or(typst_core::entities::lang::Lang::ENGLISH)
                                .as_str()
                                .to_owned(),
                        ),
                    )
                }
                FrameItem::Image { alt, .. } => {
                    ("Figure", alt.as_ref().map(ToString::to_string), None)
                }
                FrameItem::TextReplacement { .. }
                | FrameItem::Semantic { .. }
                | FrameItem::Line { .. }
                | FrameItem::Shape { .. }
                | FrameItem::Group { .. }
                | FrameItem::Link { .. } => return None,
            };
            let mcid = *next;
            *next += 1;
            Some(TagNode {
                role,
                object_id: 0,
                table_id: None,
                page_index: Some(page_index),
                mcid: Some(mcid),
                alt,
                lang,
                summary: None,
                attrs: TagAttrs::None,
                children: vec![],
            })
        }

        fn collect_top(
            item: &FrameItem,
            page_index: usize,
            next: &mut usize,
            out: &mut Vec<TagNode>,
            inside_artifact: bool,
            ua1: bool,
        ) {
            match item {
                FrameItem::Semantic { kind, items, .. } => {
                    if ua1 && !inside_artifact {
                        match kind {
                            typst_core::entities::layout_types::SemanticKind::ParbreakBoundary => {
                                out.push(TagNode {
                                    role: "Boundary", object_id: 0, table_id: None,
                                    page_index: Some(page_index), mcid: None, alt: None,
                                    lang: None, summary: None, attrs: TagAttrs::None,
                                    children: vec![],
                                });
                                return;
                            }
                            typst_core::entities::layout_types::SemanticKind::ExplicitLinebreakBoundary => return,
                            _ => {}
                        }
                    }
                    let artifact = matches!(
                        kind,
                        typst_core::entities::layout_types::SemanticKind::Artifact(_)
                    );
                    if !inside_artifact {
                        if let Some(node) = semantic_node(item, page_index, next) {
                            out.push(node);
                            return;
                        }
                    }
                    for child in items {
                        collect_top(
                            child,
                            page_index,
                            next,
                            out,
                            inside_artifact || artifact,
                            ua1,
                        );
                    }
                }
                FrameItem::Link { items, .. } => {
                    if ua1 && !inside_artifact {
                        let mut children = Vec::new();
                        for child in items {
                            collect_top(
                                child,
                                page_index,
                                next,
                                &mut children,
                                false,
                                ua1,
                            );
                        }
                        if !children.is_empty() {
                            out.push(TagNode {
                                role: "Link",
                                object_id: 0,
                                table_id: None,
                                page_index: Some(page_index),
                                mcid: None,
                                alt: None,
                                lang: None,
                                summary: None,
                                attrs: TagAttrs::None,
                                children,
                            });
                        }
                    } else {
                        for child in items {
                            collect_top(
                                child,
                                page_index,
                                next,
                                out,
                                inside_artifact,
                                ua1,
                            );
                        }
                    }
                }
                FrameItem::TextReplacement { items, .. }
                | FrameItem::Group { items, .. } => {
                    for child in items {
                        collect_top(child, page_index, next, out, inside_artifact, ua1);
                    }
                }
                FrameItem::TextSeparator { .. }
                | FrameItem::Text { .. }
                | FrameItem::TextShaped { .. }
                | FrameItem::Line { .. }
                | FrameItem::Glyph { .. }
                | FrameItem::Image { .. }
                | FrameItem::Shape { .. } => {
                    if ua1 && !inside_artifact {
                        if let Some(node) = ua1_leaf_node(item, page_index, next) {
                            out.push(node);
                        }
                    }
                }
            }
        }

        fn merge_table(existing: &mut TagNode, mut continuation: TagNode) {
            if existing.summary.is_none() {
                existing.summary = continuation.summary.take();
            }
            for child in continuation.children {
                if child.role == "THead" {
                    if let Some(group) =
                        existing.children.iter_mut().find(|node| node.role == "THead")
                    {
                        group.children.extend(child.children);
                    } else {
                        existing.children.push(child);
                    }
                } else if child.role == "TBody" {
                    if let Some(group) =
                        existing.children.iter_mut().find(|node| node.role == "TBody")
                    {
                        group.children.extend(child.children);
                    } else {
                        existing.children.push(child);
                    }
                } else {
                    existing.children.push(child);
                }
            }
        }

        let mut tags: Vec<TagNode> = Vec::new();
        for (page_index, page) in doc.pages.iter().enumerate() {
            let mut next = 0;
            for item in &page.items {
                let mut found = Vec::new();
                collect_top(
                    item,
                    page_index,
                    &mut next,
                    &mut found,
                    false,
                    matches!(self.standard.accessibility, Some(PdfAccessibility::Ua1)),
                );
                for node in found {
                    if node.role == "Table" {
                        if let Some(existing) = tags.iter_mut().find(|candidate| {
                            candidate.role == "Table"
                                && candidate.table_id == node.table_id
                        }) {
                            merge_table(existing, node);
                        } else {
                            tags.push(node);
                        }
                    } else {
                        tags.push(node);
                    }
                }
            }
        }

        // P1481: runs de layout não são headings/parágrafos distintos.
        // ParbreakBoundary delimita a unidade morfológica da linguagem.
        fn group_ua1_blocks(nodes: Vec<TagNode>) -> Vec<TagNode> {
            fn as_span(node: &mut TagNode) {
                if matches!(node.role, "P" | "H1" | "H2" | "H3" | "H4" | "H5" | "H6") {
                    node.role = "Span";
                }
                for child in &mut node.children {
                    as_span(child);
                }
            }
            fn flush(
                out: &mut Vec<TagNode>,
                role: &mut Option<&'static str>,
                page: &mut Option<usize>,
                children: &mut Vec<TagNode>,
            ) {
                if let Some(role) = role.take() {
                    out.push(TagNode {
                        role,
                        object_id: 0,
                        table_id: None,
                        page_index: None,
                        mcid: None,
                        alt: None,
                        lang: None,
                        summary: None,
                        attrs: TagAttrs::None,
                        children: std::mem::take(children),
                    });
                }
                *page = None;
            }
            let mut out = Vec::new();
            let mut role = None;
            let mut page = None;
            let mut children = Vec::new();
            for mut node in nodes {
                if node.role == "Boundary" {
                    flush(&mut out, &mut role, &mut page, &mut children);
                    continue;
                }
                let desired =
                    if matches!(node.role, "H1" | "H2" | "H3" | "H4" | "H5" | "H6") {
                        Some(node.role)
                    } else if matches!(node.role, "P" | "Strong" | "Em" | "Link") {
                        Some("P")
                    } else {
                        None
                    };
                if desired.is_none() || desired != role || node.page_index != page {
                    flush(&mut out, &mut role, &mut page, &mut children);
                }
                if let Some(block_role) = desired {
                    role = Some(block_role);
                    page = node.page_index;
                    as_span(&mut node);
                    children.push(node);
                } else {
                    out.push(node);
                }
            }
            flush(&mut out, &mut role, &mut page, &mut children);
            out
        }
        if matches!(self.standard.accessibility, Some(PdfAccessibility::Ua1)) {
            tags = group_ua1_blocks(tags);
        }

        let mut next_id = self.objects.iter().map(|(id, _)| *id).max().unwrap_or(0) + 1;
        let root_id = next_id;
        next_id += 1;
        let document_id = next_id;
        next_id += 1;
        let parent_tree_id = next_id;
        next_id += 1;

        fn assign_ids(nodes: &mut [TagNode], next: &mut usize) {
            for node in nodes {
                node.object_id = *next;
                *next += 1;
                assign_ids(&mut node.children, next);
            }
        }
        assign_ids(&mut tags, &mut next_id);

        // A ordem dos nós Link acompanha a ordem de collect_links por página.
        // O objeto da anotação entra como OBJR e a sua chave no ParentTree.
        fn collect_link_refs(
            nodes: &[TagNode],
            annots: &[Vec<(usize, usize)>],
            counts: &mut [usize],
            out: &mut Vec<(usize, usize, usize, usize)>,
        ) {
            for node in nodes {
                if node.role == "Link" {
                    if let Some(page) = node.page_index {
                        if let Some(&(annot, key)) =
                            annots.get(page).and_then(|items| items.get(counts[page]))
                        {
                            out.push((node.object_id, annot, key, page));
                            counts[page] += 1;
                        }
                    }
                }
                collect_link_refs(&node.children, annots, counts, out);
            }
        }
        let mut link_refs = Vec::new();
        let mut link_counts = vec![0; doc.pages.len()];
        collect_link_refs(&tags, &self.ua1_link_annots, &mut link_counts, &mut link_refs);

        #[derive(Clone)]
        struct HeaderInfo {
            object_id: usize,
            table_id: u64,
            row: u32,
            column: u32,
            level: u32,
            scope: typst_core::entities::elements::table_cell::TableHeaderScope,
            id: String,
        }

        fn gather_headers(nodes: &[TagNode], out: &mut Vec<HeaderInfo>) {
            for node in nodes {
                if let TagAttrs::Header { table_id, row, column, level, scope, .. } =
                    node.attrs
                {
                    out.push(HeaderInfo {
                        object_id: node.object_id,
                        table_id,
                        row,
                        column,
                        level,
                        scope,
                        id: format!("U1x{column}y{row}"),
                    });
                }
                gather_headers(&node.children, out);
            }
        }
        let mut headers = Vec::new();
        gather_headers(&tags, &mut headers);

        fn header_refs(
            headers: &[HeaderInfo],
            table_id: u64,
            row: u32,
            column: u32,
            level: Option<u32>,
        ) -> Vec<String> {
            let candidates = headers
                .iter()
                .filter(|header| {
                    if header.table_id != table_id {
                        return false;
                    }
                    if let Some(current) = level {
                        return header.level < current
                            && header.column == column
                            && matches!(
                                header.scope,
                                typst_core::entities::elements::table_cell::TableHeaderScope::Column
                                    | typst_core::entities::elements::table_cell::TableHeaderScope::Both
                            );
                    }
                    (header.column == column
                        && matches!(
                            header.scope,
                            typst_core::entities::elements::table_cell::TableHeaderScope::Column
                                | typst_core::entities::elements::table_cell::TableHeaderScope::Both
                        ))
                        || (header.row == row
                            && matches!(
                                header.scope,
                                typst_core::entities::elements::table_cell::TableHeaderScope::Row
                                    | typst_core::entities::elements::table_cell::TableHeaderScope::Both
                            ))
                })
                .collect::<Vec<_>>();
            let max_level = candidates.iter().map(|header| header.level).max();
            candidates
                .into_iter()
                .filter(|header| Some(header.level) == max_level)
                .map(|header| header.id.clone())
                .collect()
        }

        fn emit_node(
            builder: &mut PdfBuilder,
            node: &TagNode,
            parent_id: usize,
            headers: &[HeaderInfo],
            link_refs: &[(usize, usize, usize, usize)],
        ) {
            let mut children = node
                .children
                .iter()
                .map(|child| format!("{} 0 R", child.object_id))
                .collect::<Vec<_>>()
                .join(" ");
            if let Some(&(_, annot, _, page)) =
                link_refs.iter().find(|(node_id, _, _, _)| *node_id == node.object_id)
            {
                if !children.is_empty() {
                    children.push(' ');
                }
                children.push_str(&format!(
                    "<< /Type /OBJR /Obj {annot} 0 R /Pg {} 0 R >>",
                    3 + page
                ));
            }
            let mut value =
                format!("<< /Type /StructElem /S /{} /P {parent_id} 0 R", node.role);
            if !children.is_empty() {
                value.push_str(&format!(" /K [{children}]"));
            } else if let (Some(page), Some(mcid)) = (node.page_index, node.mcid) {
                value.push_str(&format!(" /Pg {} 0 R /K {mcid}", 3 + page));
            }
            if let Some(alt) = &node.alt {
                value.push_str(&format!(" /Alt {}", utf16be_hex_string(alt)));
            }
            if let Some(lang) = &node.lang {
                value.push_str(&format!(" /Lang ({lang})"));
            }
            match node.attrs {
                TagAttrs::None => {
                    if node.role == "Table" {
                        if let Some(summary) = &node.summary {
                            value.push_str(&format!(
                                " /A [<< /O /Table /Summary {} >> << /O /Layout /Placement /Block >>]",
                                escape_pdf_literal(summary)
                            ));
                        } else {
                            value.push_str(" /A << /O /Layout /Placement /Block >>");
                        }
                    }
                }
                TagAttrs::Header {
                    table_id,
                    row,
                    column,
                    level,
                    scope,
                    rowspan,
                    colspan,
                } => {
                    let id = format!("U1x{column}y{row}");
                    let scope = match scope {
                        typst_core::entities::elements::table_cell::TableHeaderScope::Both => "Both",
                        typst_core::entities::elements::table_cell::TableHeaderScope::Column => "Column",
                        typst_core::entities::elements::table_cell::TableHeaderScope::Row => "Row",
                    };
                    let refs = header_refs(headers, table_id, row, column, Some(level))
                        .into_iter()
                        .map(|id| format!("({id})"))
                        .collect::<Vec<_>>()
                        .join(" ");
                    value.push_str(&format!(
                        " /ID ({id}) /A << /O /Table /Scope /{scope} /Headers [{refs}]"
                    ));
                    if colspan > 1 {
                        value.push_str(&format!(" /ColSpan {colspan}"));
                    }
                    if rowspan > 1 {
                        value.push_str(&format!(" /RowSpan {rowspan}"));
                    }
                    value.push_str(" >>");
                }
                TagAttrs::Data { table_id, row, column, rowspan, colspan } => {
                    let refs = header_refs(headers, table_id, row, column, None)
                        .into_iter()
                        .map(|id| format!("({id})"))
                        .collect::<Vec<_>>()
                        .join(" ");
                    value.push_str(&format!(" /A << /O /Table /Headers [{refs}]"));
                    if colspan > 1 {
                        value.push_str(&format!(" /ColSpan {colspan}"));
                    }
                    if rowspan > 1 {
                        value.push_str(&format!(" /RowSpan {rowspan}"));
                    }
                    value.push_str(" >>");
                }
            }
            value.push_str(" >>");
            builder.add(node.object_id, value);
            for child in &node.children {
                emit_node(builder, child, node.object_id, headers, link_refs);
            }
        }

        let document_k = tags
            .iter()
            .map(|node| format!("{} 0 R", node.object_id))
            .collect::<Vec<_>>()
            .join(" ");
        let id_names = headers
            .iter()
            .map(|header| format!("({}) {} 0 R", header.id, header.object_id))
            .collect::<Vec<_>>()
            .join(" ");
        let ua1 = matches!(self.standard.accessibility, Some(PdfAccessibility::Ua1));
        let role_map = if ua1 { " /RoleMap << /Strong /Span /Em /Span >>" } else { "" };
        let parent_tree_next_key =
            doc.pages.len() + self.ua1_link_annots.iter().map(Vec::len).sum::<usize>();
        self.add(
            root_id,
            format!(
                "<< /Type /StructTreeRoot /K {document_id} 0 R /ParentTree {parent_tree_id} 0 R /ParentTreeNextKey {parent_tree_next_key} /IDTree << /Names [{id_names}] >>{role_map} >>",
            ),
        );
        self.add(
            document_id,
            format!(
                "<< /Type /StructElem /S /Document /P {root_id} 0 R /K [{document_k}] >>"
            ),
        );

        for node in &tags {
            emit_node(self, node, document_id, &headers, &link_refs);
        }

        fn collect_parent_refs(nodes: &[TagNode], out: &mut Vec<(usize, usize, usize)>) {
            for node in nodes {
                if let (Some(page), Some(mcid)) = (node.page_index, node.mcid) {
                    out.push((page, mcid, node.object_id));
                }
                collect_parent_refs(&node.children, out);
            }
        }
        let mut parent_refs = Vec::new();
        collect_parent_refs(&tags, &mut parent_refs);
        let mut nums = Vec::new();
        for page_index in 0..doc.pages.len() {
            let mut page_entries = parent_refs
                .iter()
                .filter(|(page, _, _)| *page == page_index)
                .copied()
                .collect::<Vec<_>>();
            page_entries.sort_by_key(|(_, mcid, _)| *mcid);
            let page_refs = page_entries
                .iter()
                .map(|(_, _, id)| format!("{id} 0 R"))
                .collect::<Vec<_>>()
                .join(" ");
            nums.push(format!("{page_index} [{page_refs}]"));
            if let Some((_, content)) =
                self.objects.iter_mut().find(|(id, _)| *id == 3 + page_index)
            {
                insert_pdf_dict_entry(content, &format!(" /StructParents {page_index}"));
            }
        }
        for &(node_id, _, key, _) in &link_refs {
            nums.push(format!("{key} {node_id} 0 R"));
        }
        self.add(parent_tree_id, format!("<< /Nums [{}] >>", nums.join(" ")));

        if let Some((_, content)) = self.objects.iter_mut().find(|(id, _)| *id == 1) {
            insert_pdf_dict_entry(
                content,
                &format!(" /StructTreeRoot {root_id} 0 R /MarkInfo << /Marked true /Suspects false >>"),
            );
        }
    }

    /// **P1480** — `/ViewerPreferences /DisplayDocTitle true` no catálogo,
    /// exigido por ISO 14289-1 §7.1 teste 10 (medido em
    /// typst-passo-1480.md §1). Independente de `emit_tag_structure`
    /// (viewer preference, não estrutura de tags) — mesmo padrão isolado
    /// de `emit_pdfa_output_intent` (early-return por `standard`, sem
    /// `doc`). `/Direction` do vanilla não é gated por PDF/UA (medido,
    /// fora do escopo) — não emitido aqui.
    pub(super) fn emit_pdfua_viewer_preferences(&mut self) {
        if !matches!(self.standard.accessibility, Some(PdfAccessibility::Ua1)) {
            return;
        }
        if let Some((_, content)) = self.objects.iter_mut().find(|(id, _)| *id == 1) {
            insert_pdf_dict_entry(
                content,
                " /ViewerPreferences << /DisplayDocTitle true >>",
            );
        }
    }
}
