//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/bibtex.md
//! @prompt-hash 7dca55a1
//! @layer L1
//! @updated 2026-06-24
//!
//! **P450** — Parser BibTeX minimal para carregamento de ficheiros `.bib`.
//!
//! Subset suportado (95% dos casos reais):
//! - Tipos: `article`, `book`, `inproceedings`, `misc`, `phdthesis`, `techreport`.
//! - Campos obrigatórios: `title`, `author`, `year`.
//! - Campos opcionais: `doi`, `url`, `journal`, `booktitle`, `volume`, `pages`.
//! - Literais de string entre `{}` (com contagem de aninhamento) ou `""`.
//! - Autores no formato `Last, First and Last2, First2`.
//! - UTF-8 only; encoding inválido é rejeitado a montante.
//!
//! Scope-out: LaTeX macros arbitrários, `@string`, `@comment` com conteúdo
//! processável, ficheiros ISO-8859-1.

use std::fmt;

use crate::entities::bib_entry::BibEntry;

/// Erro de parsing BibTeX com mensagem legível.
#[derive(Debug, Clone, PartialEq)]
pub struct BibTeXError {
    pub message: String,
}

impl BibTeXError {
    fn new(msg: impl Into<String>) -> Self {
        Self { message: msg.into() }
    }
}

impl fmt::Display for BibTeXError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for BibTeXError {}

/// Representação intermédia de um autor.
struct Author {
    name: String,
    given: Option<String>,
}

impl Author {
    fn to_canonical(&self) -> String {
        match &self.given {
            Some(g) if !g.is_empty() => format!("{}, {}", self.name.trim(), g.trim()),
            _ => self.name.trim().to_string(),
        }
    }
}

/// Parseia conteúdo BibTeX e devolve entradas cristalinas.
///
/// Rejeita tipos de entrada desconhecidos e falha em sintaxe inválida.
pub fn parse_bibtex(src: &str) -> Result<Vec<BibEntry>, BibTeXError> {
    let mut entries = Vec::new();
    let mut pos = skip_noise(src, 0);

    while pos < src.len() {
        if src[pos..].starts_with('@') {
            let (entry, next) = parse_entry(src, pos)?;
            entries.push(entry);
            pos = next;
        } else {
            // Texto arbitrário fora de comentários/whitespace é sintaxe inválida.
            return Err(BibTeXError::new(format!(
                "conteúdo inesperado fora de entrada BibTeX em posição {}",
                pos
            )));
        }
        pos = skip_noise(src, pos);
    }

    Ok(entries)
}

/// Parseia uma entrada `@type{key, fields}`.
fn parse_entry(src: &str, start: usize) -> Result<(BibEntry, usize), BibTeXError> {
    let mut pos = start;
    expect_char(src, &mut pos, '@')?;

    let kind_start = pos;
    while pos < src.len() && src.as_bytes()[pos].is_ascii_alphabetic() {
        pos += 1;
    }
    if pos == kind_start {
        return Err(BibTeXError::new("esperado tipo de entrada após '@'"));
    }
    let kind = std::str::from_utf8(&src.as_bytes()[kind_start..pos])
        .unwrap()
        .to_lowercase();

    if !is_supported_type(&kind) {
        return Err(BibTeXError::new(format!(
            "tipo de entrada BibTeX não suportado: '@{}'",
            kind
        )));
    }

    skip_noise_inline(src, &mut pos);
    expect_char(src, &mut pos, '{')?;

    let key = parse_key(src, &mut pos)?;
    let mut fields: std::collections::HashMap<String, String> =
        std::collections::HashMap::new();

    skip_noise_inline(src, &mut pos);
    if peek(src, pos) == Some(',') {
        pos += 1;
    }

    loop {
        pos = skip_noise(src, pos);
        if peek(src, pos) == Some('}') {
            pos += 1;
            break;
        }

        let (name, value, next) = parse_field(src, pos)?;
        fields.insert(name.to_lowercase(), value);
        pos = next;

        skip_noise_inline(src, &mut pos);
        if peek(src, pos) == Some(',') {
            pos += 1;
        } else if peek(src, pos) != Some('}') {
            return Err(BibTeXError::new(format!(
                "esperado ',' ou '}}' após campo em '{}'",
                key
            )));
        }
    }

    let entry = build_entry(&key, fields)?;
    Ok((entry, pos))
}

fn parse_key(src: &str, pos: &mut usize) -> Result<String, BibTeXError> {
    skip_noise_inline(src, pos);
    let start = *pos;
    while *pos < src.len() {
        let c = src.as_bytes()[*pos];
        if c == b',' || c == b'}' || is_whitespace(c) {
            break;
        }
        *pos += 1;
    }
    if *pos == start {
        return Err(BibTeXError::new("esperado key da entrada"));
    }
    let key = String::from_utf8_lossy(&src.as_bytes()[start..*pos]).into_owned();
    skip_noise_inline(src, pos);
    Ok(key)
}

fn parse_field(src: &str, start: usize) -> Result<(String, String, usize), BibTeXError> {
    let mut pos = start;
    skip_noise_inline(src, &mut pos);

    let name_start = pos;
    while pos < src.len() {
        let c = src.as_bytes()[pos];
        if c == b'=' || c == b',' || c == b'}' || is_whitespace(c) {
            break;
        }
        pos += 1;
    }
    if pos == name_start {
        return Err(BibTeXError::new("esperado nome de campo"));
    }
    let name = String::from_utf8_lossy(&src.as_bytes()[name_start..pos]).into_owned();

    skip_noise_inline(src, &mut pos);
    expect_char(src, &mut pos, '=')?;
    skip_noise_inline(src, &mut pos);

    let (value, next) = parse_value(src, pos)?;
    Ok((name, value, next))
}

fn parse_value(src: &str, start: usize) -> Result<(String, usize), BibTeXError> {
    let mut pos = start;
    skip_noise_inline(src, &mut pos);

    if pos >= src.len() {
        return Err(BibTeXError::new("valor de campo inesperadamente terminado"));
    }

    match src.as_bytes()[pos] {
        b'{' => parse_braced(src, pos),
        b'"' => parse_quoted(src, pos),
        byte if byte.is_ascii_digit() => parse_number(src, pos),
        b'-' => parse_number(src, pos),
        _ => Err(BibTeXError::new(format!("valor de campo inválido em posição {}", pos))),
    }
}

fn parse_braced(src: &str, start: usize) -> Result<(String, usize), BibTeXError> {
    let mut iter = src[start..].char_indices();
    iter.next(); // pula '{'
    let mut depth = 1;
    let mut content = String::new();

    for (idx, c) in iter {
        match c {
            '{' => {
                depth += 1;
                content.push('{');
            }
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return Ok((content, start + idx + c.len_utf8()));
                }
                content.push('}');
            }
            _ => content.push(c),
        }
    }

    Err(BibTeXError::new("chaveta não fechada no valor do campo"))
}

fn parse_quoted(src: &str, start: usize) -> Result<(String, usize), BibTeXError> {
    let mut iter = src[start..].char_indices();
    iter.next(); // pula '"'
    let mut content = String::new();

    for (idx, c) in iter {
        if c == '"' {
            return Ok((content, start + idx + 1));
        }
        content.push(c);
    }

    Err(BibTeXError::new("aspas não fechadas no valor do campo"))
}

fn parse_number(src: &str, start: usize) -> Result<(String, usize), BibTeXError> {
    let mut pos = start;
    while pos < src.len() && src.as_bytes()[pos].is_ascii_digit() {
        pos += 1;
    }
    let num = String::from_utf8_lossy(&src.as_bytes()[start..pos]).into_owned();
    Ok((num, pos))
}

fn build_entry(
    key: &str,
    fields: std::collections::HashMap<String, String>,
) -> Result<BibEntry, BibTeXError> {
    let get = |name: &str| fields.get(name).cloned();

    let title = get("title")
        .ok_or_else(|| BibTeXError::new(format!("entrada '{}' em falta: title", key)))?;
    let author_raw = get("author")
        .ok_or_else(|| BibTeXError::new(format!("entrada '{}' em falta: author", key)))?;
    let year_str = get("year")
        .ok_or_else(|| BibTeXError::new(format!("entrada '{}' em falta: year", key)))?;
    let year = year_str
        .parse::<u32>()
        .map_err(|_| BibTeXError::new(format!("entrada '{}': year inválido", key)))?;

    let author = parse_authors(&author_raw);

    let mut entry = BibEntry::new(key.to_string(), author, title, year);

    if let Some(v) = get("volume") {
        entry.volume = Some(v);
    }
    if let Some(p) = get("pages") {
        entry.pages = Some(p);
    }
    if let Some(j) = get("journal") {
        entry.journal = Some(j);
    } else if let Some(bt) = get("booktitle") {
        entry.journal = Some(bt);
    }
    if let Some(d) = get("doi") {
        entry.doi = Some(d);
    }
    if let Some(u) = get("url") {
        entry.url = Some(u);
    }
    if let Some(p) = get("publisher") {
        entry.publisher = Some(p);
    }

    Ok(entry)
}

fn parse_authors(raw: &str) -> String {
    let parts: Vec<&str> = raw.split(" and ").collect();
    let authors: Vec<String> = parts
        .iter()
        .map(|s| {
            let s = s.trim();
            let author = if let Some(idx) = s.find(',') {
                let name = s[..idx].trim();
                let given = s[idx + 1..].trim();
                Author {
                    name: name.to_string(),
                    given: Some(given.to_string()),
                }
            } else {
                Author { name: s.to_string(), given: None }
            };
            author.to_canonical()
        })
        .collect();
    authors.join(" and ")
}

fn is_supported_type(kind: &str) -> bool {
    matches!(
        kind,
        "article" | "book" | "inproceedings" | "misc" | "phdthesis" | "techreport"
    )
}

fn expect_char(src: &str, pos: &mut usize, expected: char) -> Result<(), BibTeXError> {
    skip_noise_inline(src, pos);
    if *pos >= src.len() {
        return Err(BibTeXError::new(format!(
            "esperado '{}' mas o ficheiro terminou",
            expected
        )));
    }
    let c = src.as_bytes()[*pos] as char;
    if c != expected {
        return Err(BibTeXError::new(format!(
            "esperado '{}' mas encontrado '{}' em posição {}",
            expected, c, *pos
        )));
    }
    *pos += 1;
    Ok(())
}

fn peek(src: &str, pos: usize) -> Option<char> {
    src[pos..].chars().next()
}

fn is_whitespace(c: u8) -> bool {
    c == b' ' || c == b'\t' || c == b'\n' || c == b'\r'
}

fn skip_noise(src: &str, mut pos: usize) -> usize {
    while pos < src.len() {
        let c = src.as_bytes()[pos];
        if is_whitespace(c) {
            pos += 1;
        } else if c == b'%' {
            // Comentário até ao fim da linha.
            pos += 1;
            while pos < src.len() && src.as_bytes()[pos] != b'\n' {
                pos += 1;
            }
        } else {
            break;
        }
    }
    pos
}

fn skip_noise_inline(src: &str, pos: &mut usize) {
    *pos = skip_noise(src, *pos);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p450_bibtex_article_minimo() {
        let src = r#"
@article{smith2024,
  author = {Smith, John},
  title = {On Crystal Math},
  year = {2024},
  journal = {Journal of Examples}
}
"#;
        let entries = parse_bibtex(src).unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].key, "smith2024");
        assert_eq!(entries[0].author, "Smith, John");
        assert_eq!(entries[0].title, "On Crystal Math");
        assert_eq!(entries[0].year, 2024);
        assert_eq!(entries[0].journal.as_deref(), Some("Journal of Examples"));
    }

    #[test]
    fn p450_bibtex_book_multiplos_autores() {
        let src = r#"
@book{doe2023,
  author = {Doe, Jane and Smith, John},
  title = {A Book of Examples},
  year = {2023},
  publisher = {Example Press}
}
"#;
        let entries = parse_bibtex(src).unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].key, "doe2023");
        assert_eq!(entries[0].author, "Doe, Jane and Smith, John");
        assert_eq!(entries[0].title, "A Book of Examples");
        assert_eq!(entries[0].year, 2023);
    }

    #[test]
    fn p450_bibtex_escaping_chavetas() {
        // Chavetas interiores são preservadas (protecção de capitalização BibTeX).
        let src = r#"
@article{test,
  author = {Doe, Jane},
  title = {The {Crystal} Math},
  year = {2022}
}
"#;
        let entries = parse_bibtex(src).unwrap();
        assert_eq!(entries[0].title, "The {Crystal} Math");
    }

    #[test]
    fn p450_bibtex_campo_opcional_omitido() {
        let src = r#"
@misc{simple,
  author = {Anon},
  title = {Simple},
  year = {2021}
}
"#;
        let entries = parse_bibtex(src).unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].doi, None);
        assert_eq!(entries[0].url, None);
        assert_eq!(entries[0].journal, None);
        assert_eq!(entries[0].volume, None);
        assert_eq!(entries[0].pages, None);
    }

    #[test]
    fn p450_bibtex_tipo_desconhecido_erro() {
        let src = r#"@online{x, author={A}, title={T}, year={2020}}"#;
        let err = parse_bibtex(src).unwrap_err();
        assert!(err.message.contains("não suportado"), "{}", err.message);
    }

    #[test]
    fn p450_bibtex_multiplas_entries() {
        let src = r#"
@article{a, author={A}, title={One}, year={2020}}
@book{b, author={B}, title={Two}, year={2021}}
"#;
        let entries = parse_bibtex(src).unwrap();
        assert_eq!(entries.len(), 2);
        assert!(entries.iter().any(|e| e.key == "a"));
        assert!(entries.iter().any(|e| e.key == "b"));
    }

    #[test]
    fn p450_bibtex_doi_e_url() {
        let src = r#"
@article{x,
  author = {A},
  title = {T},
  year = {2020},
  doi = {10.1000/x},
  url = {https://example.org}
}
"#;
        let entries = parse_bibtex(src).unwrap();
        assert_eq!(entries[0].doi.as_deref(), Some("10.1000/x"));
        assert_eq!(entries[0].url.as_deref(), Some("https://example.org"));
    }

    #[test]
    fn p450_bibtex_sintaxe_invalida_erro() {
        let src = r#"isto não é bibtex"#;
        assert!(parse_bibtex(src).is_err());
    }

    // ── P1043: Pares de independência testcase() para bibtex.rs:199 ──────────

    #[test]
    fn p1043_bibtex_numeric_digit_isolada() {
        // C1=T, C2=_: valor numérico não-delimitado iniciando com dígito ASCII
        let src = r#"@article{a, author={A}, title={T}, year=2024}"#;
        let entries = parse_bibtex(src).unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].year, 2024);
    }

    #[test]
    fn p1043_bibtex_numeric_minus_isolada() {
        // C1=F, C2=T: valor iniciando com '-' (entra em parse_number, consome 0 dígitos e deixa '-' no stream)
        let src = r#"@article{a, author={A}, title={T}, year=-5}"#;
        let err = parse_bibtex(src).unwrap_err();
        assert!(err.message.contains("esperado ',' ou '}'"), "{}", err.message);
    }

    #[test]
    fn p1043_bibtex_numeric_non_digit_isolada() {
        // C1=F, C2=F: valor não-delimitado iniciando com letra (rejeitado em parse_value)
        let src = r#"@article{a, author={A}, title={T}, year=abc}"#;
        let err = parse_bibtex(src).unwrap_err();
        assert!(err.message.contains("valor de campo inválido"), "{}", err.message);
    }
}
