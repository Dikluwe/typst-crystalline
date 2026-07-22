//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/fonts.md
//! @prompt-hash 5baa6a4c
//! @layer L3
//! @updated 2026-03-26

use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use typst_core::entities::font_book::{
    FontBook, FontFlags, FontInfo, FontStretch, FontStyle, FontVariant, FontWeight,
};
use typst_core::entities::world_types::Font;

/// Slot de fonte com carregamento lazy.
///
/// A fonte pode vir do disco (`path`) ou de bytes embutidos (`embedded`).
/// `ttf-parser` valida que os bytes são uma fonte OpenType/TrueType
/// válida antes de retornar `Some(Font)` — bytes inválidos retornam `None`.
/// `ttf-parser` não escapa a esta fronteira: L1 recebe apenas `Font(Vec<u8>)`.
pub struct FontSlot {
    pub path: PathBuf,
    /// Índice da face num TrueType Collection (.ttc). Sempre 0 para fontes simples.
    /// Ignorado quando `embedded` está presente.
    pub index: u32,
    /// Bytes embutidos (ex: vinda de `typst-assets`). Quando presentes,
    /// `get()` usa estes bytes em vez de ler do disco.
    embedded: Option<Vec<u8>>,
    font: OnceLock<Option<Font>>,
}

impl FontSlot {
    pub fn new(path: PathBuf, index: u32) -> Self {
        Self { path, index, embedded: None, font: OnceLock::new() }
    }

    /// Cria um slot a partir de bytes embutidos (P753).
    /// O path é mantido apenas para referência/depuração; não é lido.
    pub fn new_embedded(path: PathBuf, data: Vec<u8>) -> Self {
        Self {
            path,
            index: 0,
            embedded: Some(data),
            font: OnceLock::new(),
        }
    }

    /// Carrega e valida a fonte (apenas na primeira chamada).
    /// Retorna `None` se os bytes não forem uma fonte OpenType/TrueType válida.
    ///
    /// **P609** — se o ficheiro for uma TrueType/OpenType Collection (.ttc/.otc),
    /// extrai a face correspondente a `self.index` para bytes independentes antes
    /// de expor. Isto permite que o export/subsetter receba uma fonte simples
    /// válida para `Face::parse(data, 0)` e `oxifont_subset`, em vez de falhar
    /// silenciosamente e embutir a coleção inteira.
    pub fn get(&self) -> Option<Font> {
        self.font
            .get_or_init(|| {
                let data = if let Some(bytes) = &self.embedded {
                    bytes.clone()
                } else {
                    std::fs::read(&self.path).ok()?
                };
                // P609: extrair face de uma coleção, se aplicável (só aplica a fontes de ficheiro).
                let data = if self.embedded.is_none() {
                    extract_collection_face(&data, self.index).unwrap_or(data)
                } else {
                    data
                };
                // Validar que é uma fonte válida — ttf_parser não escapa a fronteira
                ttf_parser::Face::parse(&data, 0).ok()?;
                Some(Font::from_data(data))
            })
            .clone()
    }
}

/// Extrai a face de índice `index` de uma TrueType/OpenType Collection.
///
/// Devolve `Some(fonte_simples)` se `data` for uma coleção e o índice for
/// válido. Devolve `None` se não for uma coleção (incluindo quando só há
/// uma única face), mantendo o comportamento anterior para fontes normais.
///
/// Formato do cabeçalho `ttcf`:
/// - tag 'ttcf' (4 bytes)
/// - version (4 bytes)
/// - numFonts (4 bytes)
/// - offsetTableOffsets[ numFonts ] (4 bytes cada)
///
/// **P838** — a extracção anterior (P609) fazia slice dos bytes da face
/// (`data[start..end]`), mas os offsets do directório de tabelas de uma face
/// em TTC são **absolutos ao início da colecção** — o slice ficava com todos
/// os offsets fora de alcance e `Face::parse` falhava (medido: todas as
/// faces de `NotoSansCJK-*.ttc` ficavam incarregáveis; o fallback CJK caía
/// sempre em `Droid Sans Fallback`, um `.ttf` simples). A extracção correcta
/// **reconstrói o ficheiro**: cabeçalho sfnt da face + directório com
/// offsets recalculados + bytes das tabelas (copiados dos offsets absolutos
/// da colecção), com `checkSumAdjustment` do `head` recalculado no fim.
fn extract_collection_face(data: &[u8], index: u32) -> Option<Vec<u8>> {
    let count = ttf_parser::fonts_in_collection(data)?;
    if count <= 1 || index >= count {
        return None;
    }
    if data.len() < 12 || &data[..4] != b"ttcf" {
        return None;
    }
    let face_off = read_u32(data, 12 + 4 * index as usize)? as usize;

    // Cabeçalho sfnt da face (12 bytes) + directório de tabelas.
    let header = data.get(face_off..face_off.checked_add(12)?)?;
    let num_tables = u16::from_be_bytes([header[4], header[5]]) as usize;
    let dir_len = 16usize.checked_mul(num_tables)?;
    let dir = data.get(face_off + 12..face_off + 12 + dir_len)?;

    // Novo ficheiro: cabeçalho (12) + directório (16*n) + tabelas alinhadas a 4.
    let mut out = Vec::with_capacity(12 + dir_len);
    out.extend_from_slice(header);
    out.extend_from_slice(dir);

    let mut head_pos_in_out = None;
    for i in 0..num_tables {
        let rec = i * 16;
        let tag = &dir[rec..rec + 4];
        let abs_off = read_u32(dir, rec + 8)? as usize;
        let len = read_u32(dir, rec + 12)? as usize;
        let table = data.get(abs_off..abs_off.checked_add(len)?)?;

        let new_off = out.len();
        out.extend_from_slice(table);
        // Padding a 4 bytes (conta no offset seguinte, não no length).
        while out.len() % 4 != 0 {
            out.push(0);
        }
        let rec_out = 12 + rec;
        out[rec_out + 8..rec_out + 12].copy_from_slice(&(new_off as u32).to_be_bytes());
        if tag == b"head" {
            head_pos_in_out = Some(new_off);
        }
    }

    // Recalcular o checkSumAdjustment do `head`: zero no campo, soma do
    // ficheiro em u32 BE, ajuste = 0xB1B0AFBA - soma.
    let head = head_pos_in_out?;
    out[head + 8..head + 12].copy_from_slice(&0u32.to_be_bytes());
    let mut sum: u32 = 0;
    for chunk in out.chunks(4) {
        let mut word = [0u8; 4];
        word[..chunk.len()].copy_from_slice(chunk);
        sum = sum.wrapping_add(u32::from_be_bytes(word));
    }
    let adj = 0xB1B0AFBAu32.wrapping_sub(sum);
    out[head + 8..head + 12].copy_from_slice(&adj.to_be_bytes());

    Some(out)
}

/// Lê um u32 big-endian de `data` em `off`, com verificação de limites.
fn read_u32(data: &[u8], off: usize) -> Option<u32> {
    let b = data.get(off..off.checked_add(4)?)?;
    Some(u32::from_be_bytes([b[0], b[1], b[2], b[3]]))
}

/// Descobre fontes nos paths fornecidos.
///
/// Cada path pode ser um ficheiro de fonte directamente ou um
/// directório (varrido recursivamente). Fontes TrueType Collection
/// (`.ttc`) produzem múltiplos slots — um por face.
pub fn discover_fonts(font_paths: &[PathBuf]) -> Vec<FontSlot> {
    let mut slots = Vec::new();
    for path in font_paths {
        if path.is_dir() {
            discover_in_dir(path, &mut slots);
        } else if is_font_file(path) {
            push_slots(path, &mut slots);
        }
    }
    slots
}

fn is_font_file(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|e| e.to_str()),
        Some("ttf" | "otf" | "ttc" | "otc")
    )
}

fn face_count(path: &Path) -> u32 {
    std::fs::read(path)
        .ok()
        .and_then(|data| ttf_parser::fonts_in_collection(&data))
        .unwrap_or(1)
}

fn push_slots(path: &Path, slots: &mut Vec<FontSlot>) {
    let count = face_count(path);
    for index in 0..count {
        slots.push(FontSlot::new(path.to_path_buf(), index));
    }
}

fn discover_in_dir(dir: &Path, slots: &mut Vec<FontSlot>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            discover_in_dir(&path, slots);
        } else if is_font_file(&path) {
            push_slots(&path, slots);
        }
    }
}

/// Extrai `FontInfo` de bytes de fonte OpenType/TrueType (ADR-0022).
///
/// `ttf_parser` fica em L3 — L1 recebe apenas `FontInfo` com campos primitivos.
/// Retorna `None` se os bytes não forem uma fonte válida ou o índice não existir.
///
/// **P838** — `flags.serif` é detectado via panose (OS/2 bytes 32..45) com o
/// critério idêntico ao vanilla (`typst-library/src/text/font/info.rs:131-138`):
/// `matches!(panose, [2, 2..=10, ..])`. Necessário ao scoring de similaridade
/// de `FontBook::select_fallback`.
pub fn font_info_from_bytes(data: &[u8], index: u32) -> Option<FontInfo> {
    let face = ttf_parser::Face::parse(data, index).ok()?;

    // Preferir nome em inglês (en-US); fallback para qualquer idioma
    let family = face
        .names()
        .into_iter()
        .filter(|n| {
            n.name_id == ttf_parser::name_id::TYPOGRAPHIC_FAMILY
                || n.name_id == ttf_parser::name_id::FAMILY
        })
        .filter_map(|n| n.to_string())
        .next()
        .or_else(|| face.names().into_iter().filter_map(|n| n.to_string()).next())?;

    let style = if face.is_italic() {
        FontStyle::Italic
    } else if face.is_oblique() {
        FontStyle::Oblique
    } else {
        FontStyle::Normal
    };

    let weight = FontWeight(face.weight().to_number());
    let stretch = FontStretch::from_number(face.width().to_number());

    // P838 — serif via panose (OS/2 bytes 32..45), critério do vanilla:
    // família de texto latino (2) com estilo serifado (2..=10).
    let serif = face
        .raw_face()
        .table(ttf_parser::Tag::from_bytes(b"OS/2"))
        .and_then(|os2| os2.get(32..45))
        .is_some_and(|panose| matches!(panose, [2, 2..=10, ..]));

    Some(FontInfo {
        family,
        variant: FontVariant { style, weight, stretch },
        flags: FontFlags { monospace: face.is_monospaced(), serif },
    })
}

/// Popula um `FontBook` a partir de uma lista de `FontSlot`.
///
/// Lê os bytes de cada slot e extrai `FontInfo`. Para slots embutidos,
/// usa os bytes em memória em vez de reler do disco.
/// A leitura duplica o I/O com `FontSlot::get()` — optimização futura (Passo 11).
pub fn build_font_book(slots: &[FontSlot]) -> FontBook {
    let mut book = FontBook::new();
    for slot in slots {
        let data = slot.embedded.clone().or_else(|| std::fs::read(&slot.path).ok());
        if let Some(data) = data {
            if let Some(info) = font_info_from_bytes(&data, slot.index) {
                book.push(info);
            }
        }
    }
    book
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TempDir(PathBuf);
    impl TempDir {
        fn path(&self) -> &Path {
            &self.0
        }
    }
    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    fn tempdir() -> TempDir {
        let path = std::env::temp_dir().join(format!(
            "typst-fonts-test-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.subsec_nanos())
                .unwrap_or(0)
        ));
        std::fs::create_dir_all(&path).unwrap();
        TempDir(path)
    }

    #[test]
    fn font_slot_path_inexistente_retorna_none() {
        let slot = FontSlot::new(PathBuf::from("/nao/existe.ttf"), 0);
        assert!(slot.get().is_none());
    }

    #[test]
    fn font_slot_bytes_invalidos_retorna_none() {
        let dir = tempdir();
        let path = dir.path().join("invalid.ttf");
        std::fs::write(&path, b"not a font").unwrap();
        let slot = FontSlot::new(path, 0);
        assert!(slot.get().is_none());
    }

    #[test]
    fn discover_fonts_directorio_vazio() {
        let dir = tempdir();
        let slots = discover_fonts(&[dir.path().to_path_buf()]);
        assert!(slots.is_empty());
    }

    #[test]
    fn discover_fonts_ignora_ficheiros_nao_fonte() {
        let dir = tempdir();
        std::fs::write(dir.path().join("readme.txt"), b"text").unwrap();
        std::fs::write(dir.path().join("data.bin"), b"binary").unwrap();
        let slots = discover_fonts(&[dir.path().to_path_buf()]);
        assert!(slots.is_empty());
    }

    #[test]
    fn discover_fonts_cria_slot_para_ttf() {
        let dir = tempdir();
        // Ficheiro com extensão .ttf (conteúdo inválido — slot criado, get() retorna None)
        std::fs::write(dir.path().join("fake.ttf"), b"not a font").unwrap();
        let slots = discover_fonts(&[dir.path().to_path_buf()]);
        // Slot é criado mesmo com bytes inválidos — validação acontece em get()
        assert_eq!(slots.len(), 1);
        assert!(slots[0].get().is_none());
    }

    #[test]
    fn font_slot_get_chamada_repetida_consistente() {
        let slot = FontSlot::new(PathBuf::from("/nao/existe.ttf"), 0);
        // OnceLock garante que o resultado é sempre o mesmo
        assert_eq!(slot.get(), slot.get());
    }

    #[test]
    fn font_info_bytes_invalidos() {
        assert!(font_info_from_bytes(b"not a font", 0).is_none());
    }

    #[test]
    fn build_font_book_com_slots_invalidos() {
        let dir = tempdir();
        std::fs::write(dir.path().join("fake.ttf"), b"not a font").unwrap();
        let slots = discover_fonts(&[dir.path().to_path_buf()]);
        let book = build_font_book(&slots);
        // Bytes inválidos → sem entradas no FontBook
        assert!(book.is_empty());
    }

    // ── P838 — flags.serif via panose OS/2 (critério do vanilla) ────────────
    /// Critério idêntico ao vanilla (`info.rs:131-138`):
    /// `matches!(panose, [2, 2..=10, ..])` sobre OS/2 bytes 32..45.
    #[test]
    fn p838_serif_detectado_via_panose() {
        // Fixture sans-serif: NimbusSans panose [2, 11, …] → 11 ∉ 2..=10.
        let data = std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fixtures/fonts/NimbusSans-Regular.otf"
        ))
        .expect("fixture NimbusSans-Regular.otf necessária");
        let info = font_info_from_bytes(&data, 0).expect("fixture válida");
        assert!(!info.flags.serif, "NimbusSans-Regular é sans → serif=false");

        // Serif real do sistema (skip se ausente): DejaVu Serif panose
        // [2, 6, …] → serif=true.
        if let Ok(data) = std::fs::read("/usr/share/fonts/truetype/dejavu/DejaVuSerif.ttf") {
            let info = font_info_from_bytes(&data, 0).expect("DejaVu Serif válida");
            assert!(info.flags.serif, "DejaVu Serif panose [2,6,…] → serif=true");
        }

        // Sans real do sistema (skip se ausente).
        if let Ok(data) = std::fs::read("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf") {
            let info = font_info_from_bytes(&data, 0).expect("DejaVu Sans válida");
            assert!(!info.flags.serif, "DejaVu Sans panose [2,11,…] → serif=false");
        }
    }

    // ── P838 — extracção de faces de colecções (.ttc) com offsets reescritos ──

    /// Constrói uma colecção TTC sintética com `n` faces, todas apontando
    /// para a mesma fonte (`font`), com offsets de tabela absolutos ao
    /// início da colecção (como num TTC real).
    fn build_synthetic_ttc(font: &[u8], n: usize) -> Vec<u8> {
        let face_off = 12 + 4 * n; // cabeçalho ttcf + tabela de offsets
        let num_tables = u16::from_be_bytes([font[4], font[5]]) as usize;
        let dir_len = 16 * num_tables;

        let mut ttc = Vec::new();
        ttc.extend_from_slice(b"ttcf");
        ttc.extend_from_slice(&0x00010000u32.to_be_bytes()); // version 1.0
        ttc.extend_from_slice(&(n as u32).to_be_bytes());
        for _ in 0..n {
            ttc.extend_from_slice(&(face_off as u32).to_be_bytes());
        }
        // Cabeçalho sfnt da fonte (12 bytes) tal qual.
        ttc.extend_from_slice(&font[..12]);
        // Directório com offsets convertidos para absolutos (+face_off).
        for i in 0..num_tables {
            let rec = 12 + i * 16;
            let mut record = font[rec..rec + 16].to_vec();
            let off = u32::from_be_bytes([record[8], record[9], record[10], record[11]]);
            record[8..12].copy_from_slice(&(off + face_off as u32).to_be_bytes());
            ttc.extend_from_slice(&record);
        }
        // Dados das tabelas (tudo o que segue o directório na fonte original).
        ttc.extend_from_slice(&font[12 + dir_len..]);
        ttc
    }

    /// **P838** — a extracção P609 fazia slice dos bytes da face, mas os
    /// offsets do directório de tabelas em TTC são absolutos ao início da
    /// colecção: o slice ficava corrompido e `Face::parse` falhava. A nova
    /// extracção reconstrói o ficheiro com offsets reescritos.
    #[test]
    fn p838_extract_collection_face_produz_fonte_valida() {
        let font = std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fixtures/fonts/NimbusSans-Regular.otf"
        ))
        .expect("fixture NimbusSans-Regular.otf necessária");
        let ttc = build_synthetic_ttc(&font, 2);

        let extracted =
            extract_collection_face(&ttc, 0).expect("face 0 de colecção válida extrai-se");
        let face = ttf_parser::Face::parse(&extracted, 0).expect("face extraída parseia");
        let orig = ttf_parser::Face::parse(&font, 0).unwrap();

        // Mesma família e mesma cobertura que a fonte original.
        let name = |f: &ttf_parser::Face| {
            f.names()
                .into_iter()
                .filter(|n| n.name_id == ttf_parser::name_id::FAMILY)
                .filter_map(|n| n.to_string())
                .next()
        };
        assert_eq!(name(&face), name(&orig));
        assert_eq!(face.glyph_index('A'), orig.glyph_index('A'));
        assert!(face.glyph_index('A').is_some());

        // checkSumAdjustment consistente: soma do ficheiro ≡ 0xB1B0AFBA.
        let mut sum: u32 = 0;
        for chunk in extracted.chunks(4) {
            let mut word = [0u8; 4];
            word[..chunk.len()].copy_from_slice(chunk);
            sum = sum.wrapping_add(u32::from_be_bytes(word));
        }
        assert_eq!(sum, 0xB1B0AFBA, "checkSumAdjustment do head recalculado");
    }

    /// **P838** — regressão real: as faces CJK das colecções do sistema
    /// ficavam incarregáveis (`Face::parse` falhava nos bytes sliced), o que
    /// escondia todas as Noto CJK do fallback global (achado #24 de P831).
    #[test]
    fn p838_extract_collection_face_noto_cjk_real() {
        let Ok(data) = std::fs::read("/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc")
        else {
            eprintln!("SKIP: NotoSansCJK-Regular.ttc não disponível");
            return;
        };
        let extracted =
            extract_collection_face(&data, 0).expect("face 0 (JP) extrai-se da colecção");
        let face = ttf_parser::Face::parse(&extracted, 0).expect("face JP parseia");
        assert!(
            face.glyph_index('日').is_some(),
            "Noto Sans CJK JP extraída cobre CJK"
        );
        let info = font_info_from_bytes(&extracted, 0).expect("FontInfo da face extraída");
        assert_eq!(info.family, "Noto Sans CJK JP");
        assert!(!info.flags.serif, "Noto Sans CJK panose [2,11] → serif=false");
    }
}
