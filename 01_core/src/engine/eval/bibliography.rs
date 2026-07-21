//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/eval.md
//! @prompt-hash 3f09960d
//! @layer L1
//! @updated 2026-06-23
//!
//! **P419** — Loading de ficheiros `.bib`/`.yaml`/`.yml` para `BibliographyElem`.
//! Reusa `World::read_bytes` (L3) para leitura e `hayagriva::io` para parsing.
//! Conversão para `BibEntry` cobre o subset canónico (key, author, title, year
//! + fields opcionais quando disponíveis via hayagriva).
//!
//! Scope-out P419: `.json` nativo — `hayagriva::io` não expõe `from_json_str`
//! nesta versão; pode ser adicionado quando a API o permitir.
//!
//! **P420** — Resolução de CSL style: built-in via hayagriva archive ou custom
//! via path local `.csl` XML. Reexporta helpers de `engine/layout/bib_csl.rs`.

use std::path::Path;

use hayagriva::citationberg::IndependentStyle;

use crate::contracts::world::World;
use crate::entities::bib_entry::BibEntry;
use crate::entities::file_id::FileId;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;

/// Lê e parseia um ficheiro bibliográfico do disco, convertendo para
/// `Vec<BibEntry>`.
///
/// - `world` / `current_file` são usados para `World::read_bytes`, que resolve
///   paths relativos ao ficheiro fonte.
/// - Formato detectado pela extensão: `.bib`, `.yaml`/`.yml`.
pub fn load_bib_entries_from_path(
    world: &dyn World,
    current_file: FileId,
    path: &str,
) -> SourceResult<Vec<BibEntry>> {
    let bytes = world
        .read_bytes(current_file, path)
        .map_err(|e| vec![SourceDiagnostic::error(Span::detached(), e)])?;

    let content = std::str::from_utf8(&bytes).map_err(|e| {
        vec![SourceDiagnostic::error(
            Span::detached(),
            format!("bibliography file is not valid UTF-8: {}", e),
        )]
    })?;

    parse_bibliography(content, path)
}

/// Parseia conteúdo bibliográfico segundo a extensão do path.
pub fn parse_bibliography(content: &str, path: &str) -> SourceResult<Vec<BibEntry>> {
    let ext = Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    let library: hayagriva::Library = match ext.as_str() {
        "bib" => {
            // **P450** — parser BibTeX custom minimal.
            return crate::engine::eval::bibtex::parse_bibtex(content).map_err(|e| {
                vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("failed to parse BibTeX '{}': {}", path, e),
                )]
            });
        }
        "yaml" | "yml" => hayagriva::io::from_yaml_str(content).map_err(|e| {
            vec![SourceDiagnostic::error(
                Span::detached(),
                format!("failed to parse YAML '{}': {}", path, e),
            )]
        })?,
        other => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "unsupported bibliography format '{}': expected .bib, .yaml or .yml",
                    other
                ),
            )])
        }
    };

    Ok(library
        .iter()
        .cloned()
        .map(hay_entry_to_bib_entry)
        .collect::<Result<Vec<_>, _>>()?)
}

/// Resolve uma string `style` para um `IndependentStyle` (P420).
///
/// 1. Tenta built-in via `hayagriva::archive`.
/// 2. Se falhar, trata como path local: lê via `World::read_bytes`, decode UTF-8
///    e parseia como CSL XML.
///
/// Erros são devolvidos como `SourceDiagnostic` com mensagens legíveis.
pub fn resolve_style(
    world: &dyn World,
    current_file: FileId,
    style_str: &str,
) -> SourceResult<IndependentStyle> {
    if let Some(style) = crate::engine::layout::bib_csl::resolve_style_name(style_str) {
        return Ok(style);
    }

    load_csl_style_from_path(world, current_file, style_str)
}

/// Carrega um ficheiro `.csl` via `World::read_bytes` e parseia-o como CSL style.
fn load_csl_style_from_path(
    world: &dyn World,
    current_file: FileId,
    path: &str,
) -> SourceResult<IndependentStyle> {
    let bytes = world.read_bytes(current_file, path).map_err(|e| {
        vec![SourceDiagnostic::error(
            Span::detached(),
            format!("failed to read CSL style file '{}': {}", path, e),
        )]
    })?;

    let content = std::str::from_utf8(&bytes).map_err(|e| {
        vec![SourceDiagnostic::error(
            Span::detached(),
            format!("CSL style file is not valid UTF-8 '{}': {}", path, e),
        )]
    })?;

    crate::engine::layout::bib_csl::parse_csl_style(content).map_err(|e| {
        vec![SourceDiagnostic::error(
            Span::detached(),
            format!("failed to parse CSL style '{}': {}", path, e),
        )]
    })
}

/// Converte um `hayagriva::Entry` para o subset `BibEntry` cristalino.
fn hay_entry_to_bib_entry(entry: hayagriva::Entry) -> SourceResult<BibEntry> {
    let key = entry.key().to_string();

    // P644: chave vazia é erro, não omissão silenciosa.
    if key.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "bibliography contains entry with empty key".to_string(),
        )]);
    }

    // Autor: primeiro autor disponível (authors > editors > organization).
    let author = entry
        .authors()
        .and_then(|a| a.first())
        .map(person_to_string)
        .or_else(|| entry.editors().and_then(|e| e.first()).map(person_to_string))
        .or_else(|| entry.organization().map(|o| o.to_string()))
        .unwrap_or_default();

    // Título.
    let title = entry.title().map(|t| t.to_string()).unwrap_or_default();

    // Ano: data de publicação.
    let year = entry.date().map(|d| d.year).unwrap_or(0) as u32;

    let mut bib = BibEntry::new(key, author, title, year);

    // Fields opcionais quando diretamente acessíveis.
    if let Some(v) = entry.volume() {
        bib.volume = Some(v.to_string());
    }
    if let Some(pr) = entry.page_range() {
        bib.pages = Some(pr.to_string());
    }
    if let Some(p) = entry.publisher().and_then(|p| p.name()) {
        bib.publisher = Some(p.to_string());
    }

    Ok(bib)
}

/// Representação canónica de uma pessoa hayagriva.
fn person_to_string(person: &hayagriva::types::Person) -> String {
    if let Some(given) = person.given_name.as_ref() {
        if !person.name.is_empty() {
            return format!("{}, {}", person.name, given);
        }
        return given.clone();
    }
    person.name.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_bibliography_bib_basico() {
        let bib = r#"
@article{smith2024,
  author = {Smith, John},
  title = {On Crystal Math},
  year = {2024},
  journal = {Journal of Examples},
  volume = {12},
  pages = {1--10}
}
"#;
        let entries = parse_bibliography(bib, "refs.bib").unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].key, "smith2024");
        assert_eq!(entries[0].author, "Smith, John");
        assert_eq!(entries[0].title, "On Crystal Math");
        assert_eq!(entries[0].year, 2024);
    }

    #[test]
    fn parse_bibliography_yaml_basico() {
        let yaml = r#"
smith2024:
  type: article
  author: Smith, John
  title: On Crystal Math
  year: 2024
"#;
        let entries = parse_bibliography(yaml, "refs.yaml").unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].key, "smith2024");
        assert_eq!(entries[0].author, "Smith, John");
    }

    #[test]
    fn parse_bibliography_yml_basico() {
        let yaml = r#"
smith2024:
  type: article
  author: Smith, John
  title: On Crystal Math
  year: 2024
"#;
        let entries = parse_bibliography(yaml, "refs.yml").unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].key, "smith2024");
    }

    #[test]
    fn parse_bibliography_json_nao_suportado() {
        // hayagriva 0.10 não expõe from_json_str; scope-out.
        let r = parse_bibliography("{}", "refs.json");
        assert!(r.is_err());
        let msg = r.unwrap_err()[0].message.clone();
        assert!(msg.contains("unsupported bibliography format"), "{msg}");
    }

    #[test]
    fn parse_bibliography_formato_desconhecido() {
        let r = parse_bibliography("x", "refs.txt");
        assert!(r.is_err());
        let msg = r.unwrap_err()[0].message.clone();
        assert!(msg.contains("unsupported bibliography format"), "{msg}");
    }

    #[test]
    fn parse_bibliography_bib_invalido() {
        let r = parse_bibliography("not valid bibtex @", "refs.bib");
        assert!(r.is_err());
    }

    #[test]
    fn parse_bibliography_bib_multiplas_entries() {
        let bib = r#"
@article{smith2024,
  author = {Smith, John},
  title = {One},
  year = {2024}
}
@book{doe2023,
  author = {Doe, Jane},
  title = {Two},
  year = {2023}
}
"#;
        let entries = parse_bibliography(bib, "refs.bib").unwrap();
        assert_eq!(entries.len(), 2);
        let keys: Vec<_> = entries.iter().map(|e| e.key.as_str()).collect();
        assert!(keys.contains(&"smith2024"));
        assert!(keys.contains(&"doe2023"));
    }

    #[test]
    fn parse_bibliography_yaml_vazio() {
        let entries = parse_bibliography("", "refs.yaml").unwrap();
        assert!(entries.is_empty());
    }

    // ── P420 — resolução de CSL style ─────────────────────────────────────────

    use crate::contracts::world::World;
    use crate::entities::file_id::FileId;
    use crate::entities::font_book::FontBook;
    use crate::entities::source::Source;
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library,
    };
    use std::collections::HashMap;
    use std::num::NonZeroU16;
    use std::sync::Arc;

    struct MockWorldFs {
        library: Library,
        book: FontBook,
        main_id: FileId,
        files: HashMap<String, Arc<Vec<u8>>>,
    }

    impl MockWorldFs {
        fn new() -> Self {
            Self {
                library: Library::new(),
                book: FontBook::new(),
                main_id: FileId::from_raw(NonZeroU16::new(1).unwrap()),
                files: HashMap::new(),
            }
        }

        fn add_file(&mut self, path: &str, data: Vec<u8>) {
            self.files.insert(path.to_string(), Arc::new(data));
        }
    }

    impl World for MockWorldFs {
        fn library(&self) -> &Library {
            &self.library
        }
        fn book(&self) -> &FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            self.main_id
        }
        fn source(&self, _: FileId) -> FileResult<Source> {
            Err(FileError::NotFound)
        }
        fn file(&self, _: FileId) -> FileResult<Bytes> {
            Err(FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<Font> {
            None
        }
        fn today(&self, _: Option<i64>) -> Option<Datetime> {
            None
        }

        fn read_bytes(
            &self,
            _current_file: FileId,
            path: &str,
        ) -> Result<Arc<Vec<u8>>, String> {
            self.files.get(path).cloned().ok_or_else(|| {
                format!("failed to read CSL style file '{}': not found", path)
            })
        }
    }

    fn p420_valid_csl() -> &'static [u8] {
        br#"<?xml version="1.0" encoding="UTF-8"?>
<style xmlns="http://purl.org/net/xbiblio/csl" version="1.0" class="in-text" default-locale="en-US">
  <info>
    <title>Custom Eval</title>
    <id>http://example.org/custom-eval</id>
  </info>
  <citation><layout><text variable="title"/></layout></citation>
  <bibliography><layout><text variable="title"/></layout></bibliography>
</style>"#
    }

    #[test]
    fn p420_resolve_style_built_in_ieee() {
        let world = MockWorldFs::new();
        let style = resolve_style(&world, world.main(), "ieee").unwrap();
        assert!(style.info.title.value.contains("IEEE"), "{}", style.info.title.value);
    }

    #[test]
    fn p420_resolve_style_custom_path() {
        let mut world = MockWorldFs::new();
        world.add_file("custom.csl", p420_valid_csl().to_vec());
        let style = resolve_style(&world, world.main(), "custom.csl").unwrap();
        assert_eq!(style.info.title.value, "Custom Eval");
    }

    #[test]
    fn p420_resolve_style_custom_path_nao_encontrado() {
        let world = MockWorldFs::new();
        let err = resolve_style(&world, world.main(), "nao-existe.csl").unwrap_err();
        assert!(
            err[0].message.contains("failed to read CSL style file"),
            "{}",
            err[0].message
        );
    }

    #[test]
    fn p420_resolve_style_xml_malformado() {
        let mut world = MockWorldFs::new();
        world.add_file("bad.csl", b"<style>".to_vec());
        let err = resolve_style(&world, world.main(), "bad.csl").unwrap_err();
        assert!(
            err[0].message.contains("failed to parse CSL style"),
            "{}",
            err[0].message
        );
    }

    #[test]
    fn p420_resolve_style_encoding_invalido() {
        let mut world = MockWorldFs::new();
        world.add_file("bad-encoding.csl", vec![0xFF, 0xFE]);
        let err = resolve_style(&world, world.main(), "bad-encoding.csl").unwrap_err();
        assert!(err[0].message.contains("not valid UTF-8"), "{}", err[0].message);
    }

    // ── P644 — entradas bibliográficas omitidas silenciosamente ───────────────

    #[test]
    fn p644_yaml_chave_vazia_quoted_produz_erro() {
        // P644: chave vazia (mesmo como string YAML quoted) deve propagar
        // erro em hay_entry_to_bib_entry.
        let yaml = r#"
"":
  type: Article
  title: Sem chave
  author: Alguém
  date: 2024
"#;
        let err = parse_bibliography(yaml, "refs.yaml").unwrap_err();
        assert!(
            err[0].message.contains("bibliography contains entry with empty key"),
            "{}",
            err[0].message
        );
    }

    #[test]
    fn p644_yaml_sem_titulo_aceite() {
        // P644: entrada sem título (mas com autor) deve ser aceite, não omitida.
        let yaml = r#"
semtitulo:
  type: Article
  author: Autor Sem Título
  date: 2024
"#;
        let entries = parse_bibliography(yaml, "refs.yaml").unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].key, "semtitulo");
        assert_eq!(entries[0].author, "Autor Sem Título");
        assert!(entries[0].title.is_empty());
    }

    // ── P646 — chave vazia em YAML via path completo ──────────────────────────

    #[test]
    fn p646_yaml_chave_vazia_via_load_bib_entries_produz_erro() {
        // P646: o caminho completo (leitura do ficheiro + parse YAML) deve
        // rejeitar chave vazia, tal como o caminho .bib e o vanilla.
        let yaml = r#"
"":
  type: Article
  title: Sem chave
  author: Alguém
  date: 2024
"#;
        let mut world = MockWorldFs::new();
        world.add_file("refs.yaml", yaml.as_bytes().to_vec());
        let err =
            load_bib_entries_from_path(&world, world.main(), "refs.yaml").unwrap_err();
        assert!(
            err[0].message.contains("bibliography contains entry with empty key"),
            "{}",
            err[0].message
        );
    }
}
