//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/builder/serialization.md
//! @prompt-hash 0da3711a
//! @layer L3
//! @updated 2026-11-09

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

fn utf16_hex_capacity(units: usize) -> Option<usize> {
    let capacity = units.checked_mul(4)?.checked_add(6)?;
    (capacity <= isize::MAX as usize).then_some(capacity)
}

/// Converte uma string para representação PDF UTF-16BE com BOM, em hex.
/// Resultado: `<FEFF...>` adequado para valores `/Title` em bookmarks.
pub(super) fn utf16be_hex_string(s: &str) -> String {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    let mut out = match utf16_hex_capacity(s.encode_utf16().count()) {
        Some(capacity) => String::with_capacity(capacity),
        None => String::new(),
    };
    out.push_str("<FEFF");
    for unit in s.encode_utf16() {
        for shift in [12, 8, 4, 0] {
            out.push(char::from(HEX[((unit >> shift) & 0xF) as usize]));
        }
    }
    out.push('>');
    out
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

#[cfg(test)]
mod c19_utf16_tests {
    use super::utf16be_hex_string as encode;
    // Reference derives UTF-16 units from scalar values, not encode_utf16/table shifts.
    fn reference(s: &str) -> String {
        use std::fmt::Write;
        let mut expected = String::from("<FEFF");
        for ch in s.chars() {
            let scalar = u32::from(ch);
            if scalar <= 0xFFFF {
                write!(&mut expected, "{scalar:04X}").unwrap();
            } else {
                let offset = scalar - 0x10000;
                let high = 0xD800 + offset / 1024;
                let low = 0xDC00 + offset % 1024;
                write!(&mut expected, "{high:04X}{low:04X}").unwrap();
            }
        }
        expected.push('>');
        expected
    }
    fn decode(s: &str) -> String {
        assert!(s.starts_with("<FEFF") && s.ends_with('>'));
        let payload = &s[5..s.len() - 1];
        assert_eq!(payload.len() % 4, 0);
        assert!(
            payload
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'A'..=b'F').contains(&b)),
            "uppercase ASCII hex only"
        );
        let units = payload
            .as_bytes()
            .chunks_exact(4)
            .map(|chunk| u16::from_str_radix(std::str::from_utf8(chunk).unwrap(), 16).unwrap())
            .collect::<Vec<_>>();
        String::from_utf16(&units).unwrap()
    }
    fn assert_codec(input: &str) {
        let snapshot = input.to_owned();
        let output = encode(input);
        assert_eq!(
            output,
            reference(input),
            "scalar-unit spelling for {input:?}"
        );
        assert_eq!(decode(&output), input, "UTF-16 round trip");
        assert_eq!(input, snapshot, "shared input changed");
    }

    #[test]
    fn fixed_bom_empty_ascii_and_byte_order_vectors() {
        for (input, expected) in [
            ("", "<FEFF>"),
            ("A", "<FEFF0041>"),
            ("Title", "<FEFF005400690074006C0065>"),
            ("\u{1234}\u{ABCD}", "<FEFF1234ABCD>"),
            ("é漢", "<FEFF00E96F22>"),
            ("🙂", "<FEFFD83DDE42>"),
            ("𝄞", "<FEFFD834DD1E>"),
            ("\u{10000}", "<FEFFD800DC00>"),
            ("\u{10FFFF}", "<FEFFDBFFDFFF>"),
        ] {
            assert_eq!(encode(input), expected);
            assert_codec(input);
        }
    }

    #[test]
    fn controls_nul_and_pdf_delimiters_are_encoded_without_literal_escaping() {
        assert_eq!(
            encode("\0\n\r\t\u{1F}\u{7F}"),
            "<FEFF0000000A000D0009001F007F>"
        );
        assert_eq!(encode("()<>[]\\"), "<FEFF00280029003C003E005B005D005C>");
        let all_bytes = (0..=255)
            .map(|v| char::from_u32(v).unwrap())
            .collect::<String>();
        assert_codec(&all_bytes);
    }

    #[test]
    fn embedded_bom_and_noncharacters_are_not_removed_or_replaced() {
        assert_eq!(encode("\u{FEFF}"), "<FEFFFEFF>");
        assert_eq!(encode("\u{FEFF}\u{FFFE}\u{FFFF}"), "<FEFFFEFFFFFEFFFF>");
        assert_codec("a\u{FEFF}b\u{FFFE}\u{FFFF}\u{10FFFF}");
    }

    #[test]
    fn unicode_plane_and_surrogate_boundaries_preserve_each_scalar() {
        let points = [
            0, 1, 15, 16, 255, 256, 0xD7FE, 0xD7FF, 0xE000, 0xE001, 0xFFFD, 0xFFFE, 0xFFFF,
            0x10000, 0x10001, 0x103FF, 0x10400, 0x1F642, 0x1FFFF, 0x20000, 0x10FFFE, 0x10FFFF,
        ];
        let input = points
            .into_iter()
            .map(|p| char::from_u32(p).unwrap())
            .collect::<String>();
        assert_codec(&input);
        let sampled = (0..=0x10FFFF)
            .step_by(997)
            .filter_map(char::from_u32)
            .collect::<String>();
        assert_codec(&sampled);
    }

    #[test]
    fn every_valid_bmp_scalar_round_trips_with_its_unit_order() {
        let bmp = (0..=0xFFFF).filter_map(char::from_u32).collect::<String>();
        assert_codec(&bmp);
    }

    #[test]
    fn canonical_and_combining_forms_remain_distinct() {
        assert_eq!(encode("é"), "<FEFF00E9>");
        assert_eq!(encode("e\u{301}"), "<FEFF00650301>");
        assert_ne!(encode("é"), encode("e\u{301}"));
        assert_codec("é e\u{301} Å A\u{30A} 한 한");
    }

    #[test]
    fn empty_long_small_long_have_no_output_or_input_state_retention() {
        let mut long = String::with_capacity(32768);
        long.push_str(&"Título é🙂漢字 \u{FEFF} <>&\\".repeat(512));
        let original = long.clone();
        let capacity = long.capacity();
        let first = encode(&long);
        assert_eq!(encode(""), "<FEFF>");
        assert_eq!(encode("B"), "<FEFF0042>");
        assert_eq!(encode(&long), first);
        assert_eq!(long, original);
        assert_eq!(long.capacity(), capacity);
        assert_eq!(first, reference(&long));
        assert_eq!(decode(&first), long);
    }

    #[test]
    fn known_utf16_units_reserve_the_complete_hex_result() {
        // The output-size contract includes the fixed BOM and delimiters.
        // Long varied inputs detect spare capacity from incremental assembly.
        for input in ["A".repeat(4096), "🙂é漢".repeat(1023), String::new()] {
            let result = encode(&input);
            let units = input
                .chars()
                .map(|c| if u32::from(c) > 0xFFFF { 2 } else { 1 })
                .sum::<usize>();
            assert_eq!(result.len(), 6 + 4 * units);
            assert_eq!(
                result.capacity(),
                result.len(),
                "complete known-size reservation"
            );
        }
    }

    use super::super::PdfBuilder;
    use typst_core::entities::document_format_options::PdfVersion;

    fn serialize_with(
        objects: Vec<(usize, Vec<u8>)>,
        info_id: Option<usize>,
        xmp_ids: Option<(String, String)>,
        version: PdfVersion,
    ) -> Vec<u8> {
        let mut builder = PdfBuilder::new();
        builder.objects = objects;
        builder.info_id = info_id;
        builder.xmp_ids = xmp_ids;
        builder.standard.version = version;
        builder.serialize()
    }

    fn serialize(objects: Vec<(usize, Vec<u8>)>) -> Vec<u8> {
        serialize_with(objects, None, None, PdfVersion::V1_7)
    }
    fn find_last(data: &[u8], needle: &[u8]) -> usize {
        data.windows(needle.len())
            .rposition(|w| w == needle)
            .unwrap()
    }
    #[derive(Debug)]
    struct Index {
        start: usize,
        rows: Vec<(usize, bool)>,
        trailer: String,
    }
    fn index(pdf: &[u8]) -> Index {
        let at = find_last(pdf, b"\nstartxref\n") + 11;
        let digits = pdf[at..].split(|b| *b == b'\n').next().unwrap();
        let start = std::str::from_utf8(digits)
            .unwrap()
            .parse::<usize>()
            .unwrap();
        assert!(pdf[start..].starts_with(b"xref\n"));
        let mut lines = pdf[start..].split(|b| *b == b'\n');
        assert_eq!(lines.next().unwrap(), b"xref");
        let counts = std::str::from_utf8(lines.next().unwrap())
            .unwrap()
            .split_whitespace()
            .collect::<Vec<_>>();
        assert_eq!(counts[0], "0");
        let count = counts[1].parse::<usize>().unwrap();
        let mut rows = Vec::new();
        for id in 0..count {
            let line = lines.next().unwrap();
            assert_eq!(line.len(), 19);
            let s = std::str::from_utf8(line).unwrap();
            let fields = s.split_whitespace().collect::<Vec<_>>();
            assert_eq!(fields.len(), 3);
            assert_eq!(fields[0].len(), 10);
            assert_eq!(fields[1], if id == 0 { "65535" } else { "00000" });
            let off = fields[0].parse::<usize>().unwrap();
            assert!(fields[2] == "n" || fields[2] == "f");
            rows.push((off, fields[2] == "n"));
        }
        assert_eq!(lines.next().unwrap(), b"trailer");
        let trailer = String::from_utf8(lines.next().unwrap().to_vec()).unwrap();
        assert!(trailer.starts_with("<<"));
        assert!(trailer.ends_with(">>"));
        assert!(pdf.ends_with(b"\n%%EOF\n"));
        Index {
            start,
            rows,
            trailer,
        }
    }
    fn stream(data: &[u8]) -> Vec<u8> {
        let mut out = format!("<< /Length {} >>\nstream\n", data.len()).into_bytes();
        out.extend_from_slice(data);
        out.extend_from_slice(b"\nendstream");
        out
    }
    fn valid_objects() -> Vec<(usize, Vec<u8>)> {
        vec![
 (1,b"<< /Type /Catalog /Pages 2 0 R >>".to_vec()),
 (2,b"<< /Type /Pages /Kids [3 0 R] /Count 1 >>".to_vec()),
 (3,b"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 100 100] /Resources << >> /Contents 4 0 R >>".to_vec()),
 (4,stream(b"q\n% xref\n% trailer\n% startxref\n% \0\xff\x80\nQ"))]
    }
    fn assert_objects(pdf: &[u8], objects: &[(usize, Vec<u8>)]) {
        let x = index(pdf);
        let max = objects.iter().map(|(id, _)| *id).max().unwrap_or(0);
        assert_eq!(x.rows.len(), max + 1);
        assert_eq!(x.rows[0], (0, false));
        assert!(x.trailer.contains(&format!("/Size {}", max + 1)));
        assert!(x.trailer.contains("/Root 1 0 R"));
        let mut cursor = 15;
        for (id, body) in objects {
            let header = format!("{id} 0 obj\n");
            assert_eq!(&pdf[cursor..cursor + header.len()], header.as_bytes());
            cursor += header.len();
            assert_eq!(&pdf[cursor..cursor + body.len()], body);
            cursor += body.len();
            assert_eq!(&pdf[cursor..cursor + 8], b"\nendobj\n");
            cursor += 8;
        }
        assert_eq!(cursor, x.start);
        for id in 1..=max {
            if objects.iter().any(|(n, _)| *n == id) {
                let header = format!("{id} 0 obj\n");
                assert!(x.rows[id].1);
                assert_eq!(
                    &pdf[x.rows[id].0..x.rows[id].0 + header.len()],
                    header.as_bytes()
                );
            } else {
                assert_eq!(
                    x.rows[id],
                    (0, false),
                    "gap {id} must be free, not a fabricated object"
                );
            }
        }
    }
    #[test]
    fn versions_binary_comment_order_and_opaque_payload_are_preserved() {
        for (version, name) in [
            (PdfVersion::V1_4, "1.4"),
            (PdfVersion::V1_5, "1.5"),
            (PdfVersion::V1_6, "1.6"),
            (PdfVersion::V1_7, "1.7"),
            (PdfVersion::V2_0, "2.0"),
        ] {
            let mut xs = valid_objects();
            xs.rotate_left(3);
            let pdf = serialize_with(xs.clone(), None, None, version);
            assert_eq!(&pdf[..9], format!("%PDF-{name}\n").as_bytes());
            assert_eq!(&pdf[9..15], b"%\xe2\xe3\xcf\xd3\n");
            assert_objects(&pdf, &xs);
        }
    }
    #[test]
    fn sparse_ids_use_highest_emitted_not_number_of_entries_and_have_real_offsets() {
        let mut xs = valid_objects();
        xs.push((1000, b"null".to_vec()));
        xs.swap(0, 4);
        let pdf = serialize(xs.clone());
        assert_objects(&pdf, &xs);
        let x = index(&pdf);
        assert_eq!(x.rows.len(), 1001);
        assert!(x.rows[1000].1);
        assert_eq!(x.rows[999], (0, false));
        assert!(!x.trailer.contains("/Info"));
        assert!(!x.trailer.contains("/ID"));
    }
    #[test]
    fn optional_info_and_stored_instance_document_ids_are_exact_and_not_regenerated() {
        let mut xs = valid_objects();
        xs.push((7, b"<< /Title (title) >>".to_vec()));
        let pdf = serialize_with(
            xs.clone(),
            Some(7),
            Some(("INSTANCE-fixed".into(), "DOCUMENT-fixed".into())),
            PdfVersion::V1_7,
        );
        assert_objects(&pdf, &xs);
        let x = index(&pdf);
        assert!(x.trailer.contains(" /Info 7 0 R"));
        assert!(x
            .trailer
            .contains(" /ID [ (INSTANCE-fixed) (DOCUMENT-fixed) ]"));
        let noids = serialize_with(xs, None, None, PdfVersion::V1_7);
        let x = index(&noids);
        assert!(!x.trailer.contains("/ID"));
        assert!(!x.trailer.contains("/Info"));
    }
    #[test]
    fn empty_defensive_store_is_characterized_without_claiming_document_validity() {
        let pdf = serialize(vec![]);
        let x = index(&pdf);
        assert_eq!(x.rows, vec![(0, false)]);
        assert!(x.trailer.contains("/Size 1 /Root 1 0 R"));
    }
    #[test]
    fn duplicate_id_keeps_emission_order_and_last_offset_characterization() {
        let xs = vec![
            (1, b"null".to_vec()),
            (5, b"(old)".to_vec()),
            (5, b"(new)".to_vec()),
        ];
        let pdf = serialize(xs.clone());
        assert_objects(&pdf, &xs);
        let x = index(&pdf);
        assert!(pdf[x.rows[5].0..].starts_with(b"5 0 obj\n(new)\nendobj"));
    }
    #[test]
    fn large_binary_and_many_small_objects_keep_complete_lengths_and_xref() {
        let mut xs = valid_objects();
        let data = (0..1_048_576).map(|i| (i % 251) as u8).collect::<Vec<_>>();
        xs.push((17, stream(&data)));
        let pdf = serialize(xs.clone());
        assert_objects(&pdf, &xs);
        let mut xs = valid_objects();
        for id in 5..1005 {
            xs.push((id, format!("({id})").into_bytes()));
        }
        let pdf = serialize(xs.clone());
        assert_objects(&pdf, &xs);
    }

    #[test]
    fn utf16_hex_capacity_checks_arithmetic_and_isize_representability() {
        let natural = isize::MAX as usize;
        let last = (natural - 6) / 4;
        for units in [
            0,
            1,
            2,
            3,
            127,
            1024,
            last - 1,
            last,
            last + 1,
            natural,
            usize::MAX / 4,
            (usize::MAX - 6) / 4,
            usize::MAX,
        ] {
            let mathematical = 6u128 + 4u128 * units as u128;
            let expected = if mathematical <= natural as u128 {
                Some(mathematical as usize)
            } else {
                None
            };
            assert_eq!(super::utf16_hex_capacity(units), expected, "units={units}");
        }
    }
}
