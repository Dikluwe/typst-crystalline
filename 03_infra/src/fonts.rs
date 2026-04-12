//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/fonts.md
//! @prompt-hash 61d58fc9
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
/// A fonte só é lida do disco na primeira chamada a `get()`.
/// `ttf-parser` valida que os bytes são uma fonte OpenType/TrueType
/// válida antes de retornar `Some(Font)` — bytes inválidos retornam `None`.
/// `ttf-parser` não escapa a esta fronteira: L1 recebe apenas `Font(Vec<u8>)`.
pub struct FontSlot {
    pub path:  PathBuf,
    /// Índice da face num TrueType Collection (.ttc). Sempre 0 para fontes simples.
    pub index: u32,
    font:      OnceLock<Option<Font>>,
}

impl FontSlot {
    pub fn new(path: PathBuf, index: u32) -> Self {
        Self { path, index, font: OnceLock::new() }
    }

    /// Carrega e valida a fonte do disco (apenas na primeira chamada).
    /// Retorna `None` se o ficheiro não existir, não for legível, ou não
    /// for uma fonte OpenType/TrueType válida com o índice especificado.
    pub fn get(&self) -> Option<Font> {
        self.font.get_or_init(|| {
            let data = std::fs::read(&self.path).ok()?;
            // Validar que é uma fonte válida — ttf_parser não escapa a fronteira
            ttf_parser::Face::parse(&data, self.index).ok()?;
            Some(Font::from_data(data))
        }).clone()
    }
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
/// Nota: `ttf_parser` não expõe directamente se uma fonte é serif.
/// `flags.serif` fica `false` — heurísticas por nome de família são trabalho futuro.
pub fn font_info_from_bytes(data: &[u8], index: u32) -> Option<FontInfo> {
    let face = ttf_parser::Face::parse(data, index).ok()?;

    // Preferir nome em inglês (en-US); fallback para qualquer idioma
    let family = face.names()
        .into_iter()
        .filter(|n| n.name_id == ttf_parser::name_id::TYPOGRAPHIC_FAMILY
                 || n.name_id == ttf_parser::name_id::FAMILY)
        .filter_map(|n| n.to_string())
        .next()
        .or_else(|| {
            face.names()
                .into_iter()
                .filter_map(|n| n.to_string())
                .next()
        })?;

    let style = if face.is_italic() {
        FontStyle::Italic
    } else if face.is_oblique() {
        FontStyle::Oblique
    } else {
        FontStyle::Normal
    };

    let weight  = FontWeight(face.weight().to_number());
    let stretch = FontStretch::from_number(face.width().to_number());

    Some(FontInfo {
        family,
        variant: FontVariant { style, weight, stretch },
        flags:   FontFlags {
            monospace: face.is_monospaced(),
            serif:     false,
        },
    })
}

/// Popula um `FontBook` a partir de uma lista de `FontSlot`.
///
/// Lê os bytes de cada slot e extrai `FontInfo`.
/// A leitura duplica o I/O com `FontSlot::get()` — optimização futura (Passo 11).
pub fn build_font_book(slots: &[FontSlot]) -> FontBook {
    let mut book = FontBook::new();
    for slot in slots {
        if let Ok(data) = std::fs::read(&slot.path) {
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
        fn path(&self) -> &Path { &self.0 }
    }
    impl Drop for TempDir {
        fn drop(&mut self) { let _ = std::fs::remove_dir_all(&self.0); }
    }

    fn tempdir() -> TempDir {
        let path = std::env::temp_dir().join(format!(
            "typst-fonts-test-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.subsec_nanos()).unwrap_or(0)
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
}
