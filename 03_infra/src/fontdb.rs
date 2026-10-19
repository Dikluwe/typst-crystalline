//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/fontdb.md
//! @prompt-hash 884aacf0
//! @layer L3
//! @updated 2026-10-07
//!
//! **P515** — Descoberta automática de fontes do sistema via `fontdb`.
//! Ativação da ADR-0020. L3 puro.

pub(crate) mod generation;
mod selective;

use std::path::Path;

use typst_core::entities::font_book::FontBook;

use crate::fonts::{extract_coverage, font_info_from_bytes, FontSlot};

/// P1410 — extrai metadados e cobertura enquanto o `fontdb` ainda fornece os
/// bytes da fonte. O `SystemWorld` pode então consultar candidatos sem mapear
/// todos os ficheiros do catálogo na primeira ocorrência de fallback.
fn font_info_with_coverage(
    data: &[u8],
    index: u32,
) -> Option<typst_core::entities::font_book::FontInfo> {
    let mut info = font_info_from_bytes(data, index)?;
    let face = ttf_parser::Face::parse(data, index).ok()?;
    info.coverage = extract_coverage(&face);
    Some(info)
}

fn acquire_info(
    db: &fontdb::Database,
    id: fontdb::ID,
) -> Option<typst_core::entities::font_book::FontInfo> {
    let face = db.face(id)?;
    if let fontdb::Source::File(path) = &face.source {
        if let Some(info) = selective::acquire(path, face.index) {
            return Some(info);
        }
    }
    db.with_face_data(id, font_info_with_coverage).flatten()
}

/// Carrega as fontes instaladas no sistema operativo.
///
/// Retorna os `FontSlot` descobertos e o `FontBook` populado com os
/// metadados das faces válidas. Faces que falhem a parsear são ignoradas
/// silenciosamente — não fazem panic.
///
/// **P839** — faces cuja extracção de `FontInfo` falha (ex.: sem nenhum
/// registo name decodificável) não entram nos slots nem no book: os dois
/// ficam sempre emparelhados por índice, como no vanilla
/// (`typst-kit/src/fonts.rs:176-189`, `filter_map` sobre as faces).
///
/// Em ambientes sem fontes de sistema (containers mínimos, CI sem X11),
/// retorna vectores vazios.
pub fn load_system_fonts() -> (Vec<FontSlot>, FontBook) {
    let mut db = fontdb::Database::new();
    db.load_system_fonts();

    catalog_from_database(&db)
}

pub(crate) fn catalog_from_database(db: &fontdb::Database) -> (Vec<FontSlot>, FontBook) {
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

        // Leitura seletiva de File; SharedFile e tentativas rejeitadas
        // conservam os bytes/callback do fontdb.
        let info = acquire_info(db, face.id);

        // P839 — slot e entrada no book são inseridos juntos (índices
        // alinhados); faces sem info extraível são descartadas de ambos.
        if let Some(info) = info {
            slots.push(FontSlot::new(path, index));
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

        let info = acquire_info(&db, face.id);

        // P839 — slot e entrada no book são inseridos juntos (índices
        // alinhados); faces sem info extraível são descartadas de ambos.
        if let Some(info) = info {
            slots.push(FontSlot::new(path, index));
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
    fn p1410_info_de_sistema_conserva_coverage_exacta() {
        let data = include_bytes!("../fixtures/fonts/NotoSans-Regular.ttf");
        let info =
            font_info_with_coverage(data, 0).expect("fixture deve ser fonte válida");
        assert!(info.coverage.contains('A' as u32));
        assert!(!info.coverage.is_empty());
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

    /// **P839d (#28/I4)** — mesmo emparelhamento slots↔book no caminho
    /// `fontdb`: face que o fontdb aceita mas cuja info não é extraível
    /// (`p839-noname.ttf` — TTF válida sem registos name) não pode criar
    /// slot sem entrada no book (vanilla: `filter_map` em
    /// `typst-kit/src/fonts.rs:176-189`).
    #[test]
    fn p839d_fontdb_slots_book_emparelhados() {
        let dir = std::env::temp_dir().join(format!(
            "typst-fontdb-p839-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.subsec_nanos())
                .unwrap_or(0)
        ));
        std::fs::create_dir_all(&dir).unwrap();
        for f in ["NimbusSans-Regular.otf", "p839-noname.ttf"] {
            std::fs::copy(
                concat!(env!("CARGO_MANIFEST_DIR"), "/fixtures/fonts/").to_string() + f,
                dir.join(f),
            )
            .unwrap();
        }

        let (slots, book) = load_fonts_from_dir(&dir);
        // Nota de medição (P839): o fontdb 0.21 já exclui ele próprio a
        // `p839-noname.ttf` (`Database::faces()` reporta 1 face para este
        // dir) — o teste guarda o invariante estrutural do emparelhamento;
        // o vector medido do achado #28 é o caminho `discover_fonts`
        // (--font-path), coberto em `fonts.rs::tests::p839d_*`.
        assert_eq!(
            slots.len(),
            book.len(),
            "cada slot corresponde a uma entrada do FontBook (índices alinhados)"
        );
        assert_eq!(book.len(), 1, "só a fonte com info extraível entra");
        assert_eq!(slots[0].path.file_name().unwrap(), "NimbusSans-Regular.otf");
        let _ = std::fs::remove_dir_all(&dir);
    }
    #[test]
    fn selective_decline_keeps_legacy_metadata_and_coverage() {
        let bytes = include_bytes!("../fixtures/fonts/NotoSans-Regular.ttf");
        let dir = std::env::temp_dir()
            .join(format!("selective-fallback-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let mut data = bytes.to_vec();
        let count = u16::from_be_bytes(data[4..6].try_into().unwrap()) as usize;
        let at = (0..count)
            .map(|i| 12 + i * 16)
            .find(|&at| &data[at..at + 4] == b"post")
            .unwrap();
        data[at + 8..at + 12].copy_from_slice(&u32::MAX.to_be_bytes());
        let path = dir.join("font.ttf");
        std::fs::write(&path, &data).unwrap();
        assert!(selective::acquire(&path, 0).is_none());
        let mut db = fontdb::Database::new();
        db.load_font_file(&path).unwrap();
        let face = db.faces().next().unwrap();
        assert_eq!(
            acquire_info(&db, face.id),
            db.with_face_data(face.id, font_info_with_coverage).flatten()
        );
        let (slots, book) = load_fonts_from_dir(&dir);
        assert_eq!(slots.len(), 1);
        assert_eq!(book.len(), 1);
        assert!(book.infos().first().unwrap().coverage.contains('A' as u32));
        std::fs::remove_dir_all(dir).unwrap();
    }
    #[test]
    fn shared_source_keeps_retained_bytes_even_when_path_is_different() {
        let bytes = include_bytes!("../fixtures/fonts/NotoSans-Regular.ttf").to_vec();
        let dir =
            std::env::temp_dir().join(format!("selective-shared-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("font.ttf");
        std::fs::write(&path, include_bytes!("../fixtures/fonts/NimbusSans-Regular.otf"))
            .unwrap();
        assert_ne!(selective::acquire(&path, 0), font_info_with_coverage(&bytes, 0));
        let mut db = fontdb::Database::new();
        let ids = db.load_font_source(fontdb::Source::SharedFile(
            path.clone(),
            std::sync::Arc::new(bytes.clone()),
        ));
        assert_eq!(ids.len(), 1);
        assert_eq!(acquire_info(&db, ids[0]), font_info_with_coverage(&bytes, 0));
        std::fs::remove_file(&path).unwrap();
        assert_eq!(acquire_info(&db, ids[0]), font_info_with_coverage(&bytes, 0));
        std::fs::remove_dir_all(dir).unwrap();
    }
}
