//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/eval.md
//! @prompt-hash 7a92cc2d
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

use std::path::Path;

use ecow::EcoString;

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

    let content = std::str::from_utf8(&bytes)
        .map_err(|e| {
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
        "bib" => hayagriva::io::from_biblatex_str(content)
            .map_err(|e| vec![SourceDiagnostic::error(
                Span::detached(),
                format!("failed to parse BibLaTeX '{}': {:?}", path, e),
            )])?,
        "yaml" | "yml" => hayagriva::io::from_yaml_str(content)
            .map_err(|e| vec![SourceDiagnostic::error(
                Span::detached(),
                format!("failed to parse YAML '{}': {}", path, e),
            )])?,
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
        .filter_map(hay_entry_to_bib_entry)
        .collect::<Vec<_>>())
}

/// Converte um `hayagriva::Entry` para o subset `BibEntry` cristalino.
fn hay_entry_to_bib_entry(entry: hayagriva::Entry) -> Option<BibEntry> {
    let key = entry.key().to_string();

    // Autor: primeiro autor disponível (authors > editors > organization).
    let author = entry
        .authors()
        .and_then(|a| a.first())
        .map(person_to_string)
        .or_else(|| {
            entry
                .editors()
                .and_then(|e| e.first())
                .map(person_to_string)
        })
        .or_else(|| entry.organization().map(|o| o.to_string()))
        .unwrap_or_default();

    // Título.
    let title = entry.title().map(|t| t.to_string()).unwrap_or_default();

    // Ano: data de publicação.
    let year = entry
        .date()
        .map(|d| d.year)
        .unwrap_or(0) as u32;

    if key.is_empty() || (author.is_empty() && title.is_empty()) {
        // Entry sem dados mínimos: ignora silenciosamente.
        return None;
    }

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

    Some(bib)
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
}
