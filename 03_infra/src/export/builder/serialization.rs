//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/builder/serialization.md
//! @prompt-hash 09a7f6f9
//! @layer L3
//! @updated 2026-09-26

use super::PdfBuilder;
use std::collections::HashMap;

/// ISO 32000-1 §7.3.4.2: bytes de controlo requerem escape octal.
fn is_pdf_literal_control(byte: u8) -> bool {
    byte.is_ascii_control()
}

/// **P536** — escapa uma string literal PDF (operando `(...)`).
pub(super) fn escape_pdf_literal(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('(');
    for b in s.bytes() {
        match b {
            b'(' => out.push_str("\\("),
            b')' => out.push_str("\\)"),
            b'\\' => out.push_str("\\\\"),
            byte if is_pdf_literal_control(byte) => {
                out.push_str(&format!("\\{:03o}", byte));
            }
            _ => out.push(b as char),
        }
    }
    out.push(')');
    out
}

/// Codifica uma string como PDF name, escapando delimitadores por `#XX`.
pub(super) fn pdf_name(value: &str) -> String {
    let mut out = String::from("/");
    for byte in value.bytes() {
        if matches!(byte, b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'+')
        {
            out.push(byte as char);
        } else {
            out.push_str(&format!("#{byte:02X}"));
        }
    }
    out
}

pub(super) fn pdf_indirect_ref_after(content: &[u8], key: &str) -> Option<usize> {
    let text = String::from_utf8_lossy(content);
    let tail = text.split_once(key)?.1.trim_start();
    let mut parts = tail.split_whitespace();
    let id = parts.next()?.parse::<usize>().ok()?;
    if parts.next() != Some("0") {
        return None;
    }
    let r_token = parts.next()?;
    // **P1478** — achado de integração: `split_whitespace` funde "R" com um
    // delimitador PDF colado sem espaço (ex.: "R>>") quando a referência é
    // a última chave antes do fecho do dicionário — caso real medido no
    // catálogo quando `/Names` é inserido como último campo. Aceitar "R"
    // seguido só de fim-de-token ou de um caractere não alfanumérico.
    let mut chars = r_token.chars();
    if chars.next() != Some('R') {
        return None;
    }
    match chars.next() {
        None => Some(id),
        Some(c) if !c.is_alphanumeric() => Some(id),
        _ => None,
    }
}

/// Converte uma string para representação PDF UTF-16BE com BOM, em hex.
/// Resultado: `<FEFF...>` adequado para valores `/Title` em bookmarks.
pub(super) fn utf16be_hex_string(s: &str) -> String {
    let mut bytes: Vec<u8> = Vec::with_capacity(2 + s.encode_utf16().count() * 2);
    bytes.extend_from_slice(&[0xFE, 0xFF]); // BOM
    for unit in s.encode_utf16() {
        bytes.extend_from_slice(&unit.to_be_bytes());
    }
    let hex: String = bytes.iter().map(|b| format!("{:02X}", b)).collect();
    format!("<{hex}>")
}

pub(super) fn insert_pdf_dict_entry(content: &mut Vec<u8>, entry: &str) {
    let text = String::from_utf8_lossy(content);
    if let Some(index) = text.rfind(">>") {
        let mut updated = text[..index].to_string();
        updated.push_str(entry);
        updated.push_str(&text[index..]);
        *content = updated.into_bytes();
    }
}

impl PdfBuilder {
    pub(super) fn add(&mut self, id: usize, content: String) {
        self.objects.push((id, content.into_bytes()));
    }

    pub(super) fn add_bytes(&mut self, id: usize, content: Vec<u8>) {
        self.objects.push((id, content));
    }

    pub(super) fn serialize(self) -> Vec<u8> {
        // Header — **P1469** (escopo reduzido — contrato de linguagem):
        // `%PDF-{version}` dinâmico a partir de `self.standard.version`
        // (`PdfVersion::as_str()` produz exatamente "1.4".."2.0", mesmos
        // nomes do cast documental/CLI). Default sem `standard` nenhum
        // (`PdfStandards::default()`, versão `1.7`) preserva o literal
        // anterior, byte-idêntico. + comentário binário (4 bytes > 127) —
        // resto da estrutura do PDF não muda com a versão declarada.
        let mut out: Vec<u8> =
            format!("%PDF-{}\n", self.standard.version.as_str()).into_bytes();
        out.extend_from_slice(b"%\xe2\xe3\xcf\xd3\n");
        let mut offsets: HashMap<usize, usize> = Default::default();

        for (id, content) in &self.objects {
            offsets.insert(*id, out.len());
            out.extend_from_slice(format!("{id} 0 obj\n").as_bytes());
            out.extend_from_slice(content);
            out.extend_from_slice(b"\nendobj\n");
        }

        // xref table
        let xref_start = out.len();
        let max_id = offsets.keys().copied().max().unwrap_or(0);
        out.extend_from_slice(b"xref\n");
        out.extend_from_slice(format!("0 {}\n", max_id + 1).as_bytes());
        out.extend_from_slice(b"0000000000 65535 f \n");
        for id in 1..=max_id {
            if let Some(&off) = offsets.get(&id) {
                out.extend_from_slice(format!("{off:010} 00000 n \n").as_bytes());
            } else {
                out.extend_from_slice(b"0000000000 00000 f \n");
            }
        }

        // Trailer
        let info_ref =
            self.info_id.map(|id| format!(" /Info {id} 0 R")).unwrap_or_default();
        // **P1475** — `/ID` universal (todo export, com ou sem `standard`),
        // a partir do par instance/document já calculado por
        // `emit_xmp_metadata`. `None` só no caminho teórico em que essa
        // função não foi chamada antes de `serialize()` (defensivo, mesmo
        // padrão de `info_ref`).
        let id_entry = self
            .xmp_ids
            .as_ref()
            .map(|(inst, doc)| format!(" /ID [ ({inst}) ({doc}) ]"))
            .unwrap_or_default();
        out.extend_from_slice(
            format!(
                "trailer\n<< /Size {} /Root 1 0 R{}{} >>\nstartxref\n{}\n%%EOF\n",
                max_id + 1,
                info_ref,
                id_entry,
                xref_start
            )
            .as_bytes(),
        );

        out
    }
}

#[cfg(test)]
mod boundary_tests {
    use super::{escape_pdf_literal, is_pdf_literal_control};

    #[test]
    fn p1250a_pdf_literal_control_preserva_fronteiras_normativas() {
        assert!(is_pdf_literal_control(0x00));
        assert!(is_pdf_literal_control(0x1f));
        assert!(!is_pdf_literal_control(0x20));
        assert!(!is_pdf_literal_control(0x7e));
        assert!(is_pdf_literal_control(0x7f));
        assert!(!is_pdf_literal_control(0x80));
        assert_eq!(escape_pdf_literal("\u{1f} \u{7f}"), "(\\037 \\177)");
    }
}
