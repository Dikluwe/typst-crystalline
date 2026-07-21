//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/fontdb.md
//! @layer L3
//! @updated 2026-06-30
//!
//! **P515** — Descoberta automática de fontes do sistema via `fontdb`.
//! Ativação da ADR-0020. L3 puro.

use std::path::{Path, PathBuf};

use typst_core::entities::font_book::FontBook;

use crate::fonts::{font_info_from_bytes, FontSlot};

/// Carrega as fontes instaladas no sistema operativo.
///
/// Retorna os `FontSlot` descobertos e o `FontBook` populado com os
/// metadados das faces válidas. Faces que falhem a parsear são ignoradas
/// silenciosamente — não fazem panic.
///
/// Em ambientes sem fontes de sistema (containers mínimos, CI sem X11),
/// retorna vectores vazios.
pub fn load_system_fonts() -> (Vec<FontSlot>, FontBook) {
    let mut db = fontdb::Database::new();
    db.load_system_fonts();

    let mut slots = Vec::new();
    let mut book = FontBook::new();
    for face in db.faces() {
        let (path, index) = match &face.source {
            fontdb::Source::File(path) | fontdb::Source::SharedFile(path, _) => {
                (path.clone(), face.index)
            }
            fontdb::Source::Binary(_) => {
                // fontdb pode manter fontes em memória; nesta fase ignoramos
                // fontes binárias não persistidas em disco.
                continue;
            }
        };

        // P674 — reutiliza os bytes já carregados pelo fontdb em vez de reler
        // o ficheiro do disco. Isto corta pela metade o I/O de arranque.
        let info = db
            .with_face_data(face.id, |data, idx| font_info_from_bytes(data, idx))
            .flatten();

        slots.push(FontSlot::new(path, index));
        if let Some(info) = info {
            book.push(info);
        }
    }

    // Preservar a ordem estável devolvida pelo fontdb; o FontBook reflecte
    // essa ordem. Duplicados de path+index são mantidos — a deduplicação
    // é responsabilidade do consumidor (SystemWorld) se desejada.
    (slots, book)
}

/// Mesmo que `load_system_fonts`, mas limitado a um directório específico.
///
/// Útil para testes determinísticos e para ambientes onde `load_system_fonts`
/// não é reprodutível.
pub fn load_fonts_from_dir<P: AsRef<Path>>(dir: P) -> (Vec<FontSlot>, FontBook) {
    let mut db = fontdb::Database::new();
    db.load_fonts_dir(dir.as_ref());

    let mut slots = Vec::new();
    let mut book = FontBook::new();
    for face in db.faces() {
        let (path, index) = match &face.source {
            fontdb::Source::File(path) | fontdb::Source::SharedFile(path, _) => {
                (path.clone(), face.index)
            }
            fontdb::Source::Binary(_) => continue,
        };

        let info = db
            .with_face_data(face.id, |data, idx| font_info_from_bytes(data, idx))
            .flatten();

        slots.push(FontSlot::new(path, index));
        if let Some(info) = info {
            book.push(info);
        }
    }

    (slots, book)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_system_fonts_nao_panic() {
        // Deve executar sem panic mesmo em sistemas sem fontes.
        let (slots, book) = load_system_fonts();
        // Invariante: o número de entradas no book não excede o número de slots.
        assert!(book.len() <= slots.len());
    }

    #[test]
    fn load_fonts_from_dir_vazio() {
        let dir = std::env::temp_dir().join(format!(
            "typst-fontdb-empty-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.subsec_nanos())
                .unwrap_or(0)
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let (slots, book) = load_fonts_from_dir(&dir);
        assert!(slots.is_empty());
        assert!(book.is_empty());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn load_fonts_from_dir_ignora_nao_fontes() {
        let dir = std::env::temp_dir().join(format!(
            "typst-fontdb-nonfont-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.subsec_nanos())
                .unwrap_or(0)
        ));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("readme.txt"), b"not a font").unwrap();
        let (slots, book) = load_fonts_from_dir(&dir);
        assert!(slots.is_empty());
        assert!(book.is_empty());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
