//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/fontdb.md
//! @layer L3
//! @updated 2026-07-31
//!
//! **P515** — Descoberta automática de fontes do sistema via `fontdb`.
//! Ativação da ADR-0020. L3 puro.
//!
//! **P935** — extrai `FontInfo` + coverage eager reutilizando o mmap interno
//! do `fontdb` (padrão vanilla). O `fontdb::Database` é descartado depois do
//! carregamento; os `FontSlot` guardam apenas path + index.

use std::path::{Path, PathBuf};

use typst_core::entities::font_book::FontBook;

use crate::fonts::{font_info_from_bytes, FontSlot};

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
/// **P935** — `font_info_from_bytes` extrai coverage eager a partir dos bytes
/// já mapeados pelo `fontdb` (mmap). O `Database` é descartado no final.
///
/// Em ambientes sem fontes de sistema (containers mínimos, CI sem X11),
/// retorna vectores vazios.
pub fn load_system_fonts() -> (Vec<FontSlot>, FontBook) {
    let mut db = fontdb::Database::new();
    db.load_system_fonts();
    load_from_db(&db)
}

/// Mesmo que `load_system_fonts`, mas limitado a um directório específico.
///
/// Útil para testes determinísticos e para ambientes onde `load_system_fonts`
/// não é reprodutível.
pub fn load_fonts_from_dir<P: AsRef<Path>>(dir: P) -> (Vec<FontSlot>, FontBook) {
    let mut db = fontdb::Database::new();
    db.load_fonts_dir(dir.as_ref());
    load_from_db(&db)
}

/// **P935** — constrói slots e FontBook a partir de uma `fontdb::Database` já
/// carregada. Preserva a ordem das faces devolvida pelo `fontdb`.
fn load_from_db(db: &fontdb::Database) -> (Vec<FontSlot>, FontBook) {
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

        // P674/P935 — reutiliza os bytes já mapeados pelo fontdb (mmap) para
        // extrair FontInfo + coverage eager, sem reler o ficheiro do disco.
        let info = db
            .with_face_data(face.id, |data, idx| font_info_from_bytes(data, idx))
            .flatten();

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
}
