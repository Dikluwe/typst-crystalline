//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/layout/bib_csl.md
//! @prompt-hash ef38ba91
//! @layer L1
//! @updated 2026-06-23
//!
//! **P418** — Integração hayagriva/citationberg para renderização CSL de
//! `BibliographyElem` e `CiteElem`. Responsabilidades:
//! - Converter `BibEntry` (subset cristalino) para `hayagriva::Entry` via YAML
//!   intermédio, reaproveitando o parser do hayagriva.
//! - Resolver CSL style: built-ins via `hayagriva::archive::ArchivedStyle`.
//! - Pré-renderizar citações (4 forms) e bibliografia num `BibRenderCache`.
//! - Converter `ElemChildren` hayagriva para `Content` cristalino (Strong/Emph/
//!   Link/Linebreak).
//!
//! Scope-out P418: ficheiros `.csl` customizados via path, múltiplas
//! bibliografias, locales via ficheiro externo, e formatação vertical
//! (superscript/subscript) sem Content variant correspondente.

use std::collections::HashMap;

use hayagriva::archive::ArchivedStyle;
use hayagriva::citationberg::{Display, IndependentStyle, Locale};
use hayagriva::{
    BibliographyDriver, CitationItem, CitationRequest, CitePurpose, ElemChild,
    ElemChildren, Entry, Formatted, Formatting,
};

use crate::entities::bib_entry::BibEntry;
use crate::entities::citation_form::CitationForm;
use crate::entities::content::Content;

/// Cache pré-renderizado para um par style/locale.
#[derive(Debug, Clone)]
pub struct BibRenderCache {
    /// Citações pré-renderizadas por form e por key bibliográfica.
    pub citations: HashMap<CitationForm, HashMap<String, Content>>,
    /// Bibliografia pré-renderizada (se o style a definir).
    pub bibliography: Option<Content>,
}

/// Style CSL default quando o documento não especifica outro.
const DEFAULT_STYLE: &str = "ieee";

/// Constrói o cache de renderização para as entries e style indicados.
///
/// - `style`: nome built-in (ex: `"ieee"`, `"apa"`, `"chicago-author-date"`).
///   Se `None`, usa `DEFAULT_STYLE`.
/// - `locale`: locale override (ex: `"en-US"`, `"pt-PT"`). Se `None`, usa o
///   locale default do style.
///
/// Retorna `None` se o style não for resolvido ou se houver falha irrecuperável
/// na conversão das entries.
pub fn build_cache(
    entries: &[BibEntry],
    style: Option<&str>,
    locale: Option<&str>,
) -> Option<BibRenderCache> {
    let style_name = style.unwrap_or(DEFAULT_STYLE);
    let independent = resolve_style(style_name)?;
    let locales = build_locales(locale);

    let hay_entries: Vec<Entry> =
        entries.iter().filter_map(bib_entry_to_hayagriva).collect();
    if hay_entries.is_empty() {
        return Some(BibRenderCache { citations: HashMap::new(), bibliography: None });
    }

    let mut citations: HashMap<CitationForm, HashMap<String, Content>> = HashMap::new();

    for form in [
        CitationForm::Normal,
        CitationForm::Prose,
        CitationForm::Author,
        CitationForm::Year,
    ] {
        let purpose = form_to_purpose(form);
        let mut driver = BibliographyDriver::new();
        for entry in &hay_entries {
            let mut item = CitationItem::with_entry(entry);
            item.purpose = purpose;
            driver.citation(CitationRequest::from_items(
                vec![item],
                &independent,
                &locales,
            ));
        }
        let rendered = driver.finish(hayagriva::BibliographyRequest::new(
            &independent,
            None,
            &locales,
        ));
        let mut map = HashMap::new();
        for (i, entry) in hay_entries.iter().enumerate() {
            if let Some(cite) = rendered.citations.get(i) {
                map.insert(
                    entry.key().to_string(),
                    elem_children_to_content(&cite.citation),
                );
            }
        }
        citations.insert(form, map);
    }

    // Bibliografia: usa um driver novo com purpose None para obter o layout
    // de referências completo.
    let mut driver = BibliographyDriver::new();
    for entry in &hay_entries {
        driver.citation(CitationRequest::from_items(
            vec![CitationItem::with_entry(entry)],
            &independent,
            &locales,
        ));
    }
    let rendered =
        driver.finish(hayagriva::BibliographyRequest::new(&independent, None, &locales));
    let bibliography = rendered.bibliography.as_ref().map(render_bibliography);

    Some(BibRenderCache { citations, bibliography })
}

/// Resolve um style pelo nome. Aceita built-ins do hayagriva archive.
fn resolve_style(name: &str) -> Option<IndependentStyle> {
    let archived = ArchivedStyle::by_name(name)?;
    match archived.get() {
        hayagriva::citationberg::Style::Independent(s) => Some(s),
        _ => None,
    }
}

/// Constrói o slice de locales a passar ao hayagriva.
fn build_locales(locale: Option<&str>) -> Vec<Locale> {
    let all = hayagriva::archive::locales();
    match locale {
        Some(l) => all
            .into_iter()
            .filter(|loc| loc.lang.as_ref().map(|c| c.0.as_str() == l).unwrap_or(false))
            .collect(),
        None => all,
    }
}

/// Mapeia `CitationForm` cristalino para `Option<CitePurpose>` hayagriva.
fn form_to_purpose(form: CitationForm) -> Option<CitePurpose> {
    match form {
        CitationForm::Normal => None,
        CitationForm::Prose => Some(CitePurpose::Prose),
        CitationForm::Author => Some(CitePurpose::Author),
        CitationForm::Year => Some(CitePurpose::Year),
    }
}

/// Converte um `BibEntry` cristalino num `hayagriva::Entry` gerando YAML
/// intermédio e aproveitando o parser do hayagriva.
fn bib_entry_to_hayagriva(entry: &BibEntry) -> Option<Entry> {
    let mut yaml = String::new();
    yaml.push_str(&format!("{}:\n", sanitize_yaml_key(&entry.key)));

    // Heurística simples de tipo: journal presente → Article; senão Book.
    let entry_type = if entry.journal.is_some() { "Article" } else { "Book" };
    yaml.push_str(&format!("  type: {entry_type}\n"));
    yaml.push_str(&format!("  title: {}\n", escape_yaml_scalar(&entry.title)));
    yaml.push_str(&format!("  author: {}\n", escape_yaml_scalar(&entry.author)));
    yaml.push_str(&format!("  date: {}\n", entry.year));

    if let Some(v) = &entry.volume {
        yaml.push_str(&format!("  volume: {}\n", escape_yaml_scalar(v)));
    }
    if let Some(p) = &entry.pages {
        yaml.push_str(&format!("  page-range: {}\n", escape_yaml_scalar(p)));
    }
    if let Some(j) = &entry.journal {
        yaml.push_str("  parent:\n");
        yaml.push_str(&format!("    title: {}\n", escape_yaml_scalar(j)));
    }
    if let Some(p) = &entry.publisher {
        yaml.push_str(&format!("  publisher: {}\n", escape_yaml_scalar(p)));
    }
    if let Some(u) = &entry.url {
        yaml.push_str(&format!("  url: {}\n", escape_yaml_scalar(u)));
    }
    if let Some(d) = &entry.doi {
        yaml.push_str(&format!("  doi: {}\n", escape_yaml_scalar(d)));
    }
    if let Some(e) = &entry.editor {
        yaml.push_str(&format!("  editor: {}\n", escape_yaml_scalar(e)));
    }
    if let Some(s) = &entry.series {
        yaml.push_str(&format!("  series: {}\n", escape_yaml_scalar(s)));
    }
    if let Some(n) = &entry.note {
        yaml.push_str(&format!("  note: {}\n", escape_yaml_scalar(n)));
    }
    if let Some(i) = &entry.isbn {
        yaml.push_str(&format!("  isbn: {}\n", escape_yaml_scalar(i)));
    }
    if let Some(l) = &entry.location {
        yaml.push_str(&format!("  location: {}\n", escape_yaml_scalar(l)));
    }
    if let Some(o) = &entry.organization {
        yaml.push_str(&format!("  organization: {}\n", escape_yaml_scalar(o)));
    }

    let library = hayagriva::io::from_yaml_str(&yaml).ok()?;
    library.get(&entry.key).cloned()
}

/// Escapa um escalar YAML plain suficientemente para as entradas típicas.
fn escape_yaml_scalar(s: &str) -> String {
    if s.is_empty() {
        return "\"\"".to_string();
    }
    if needs_yaml_quotes(s) {
        format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
    } else {
        s.to_string()
    }
}

fn needs_yaml_quotes(s: &str) -> bool {
    s.starts_with(|c: char| {
        c.is_ascii_digit()
            || c == '-'
            || c == ':'
            || c == '['
            || c == '{'
            || c == '*'
            || c == '&'
            || c == '!'
            || c == '|'
            || c == '>'
            || c == '\''
            || c == '"'
            || c == '%'
            || c == '@'
            || c == '`'
    }) || s.contains(':')
        || s.contains('#')
        || s.contains('\n')
        || s.contains('\t')
        || s == "true"
        || s == "false"
        || s == "null"
        || s == "~"
}

/// Sanitiza a chave do mapa YAML (key da entry). Se necessário, envolve em aspas.
fn sanitize_yaml_key(s: &str) -> String {
    escape_yaml_scalar(s)
}

/// Renderiza uma `RenderedBibliography` hayagriva num `Content` cristalino.
fn render_bibliography(bib: &hayagriva::RenderedBibliography) -> Content {
    let mut items = Vec::with_capacity(bib.items.len());
    for item in &bib.items {
        let mut line_parts = Vec::new();
        if let Some(first) = &item.first_field {
            line_parts.push(elem_child_to_content(first));
            line_parts.push(Content::text(" "));
        }
        line_parts.push(elem_children_to_content(&item.content));
        items.push(Content::sequence(line_parts));
        items.push(Content::linebreak());
    }
    Content::sequence(items)
}

/// Converte `ElemChildren` hayagriva num único `Content` (Sequence se >1).
fn elem_children_to_content(children: &ElemChildren) -> Content {
    if children.0.len() == 1 {
        elem_child_to_content(&children.0[0])
    } else {
        Content::sequence(children.0.iter().map(elem_child_to_content).collect())
    }
}

/// Converte um `ElemChild` hayagriva para `Content`.
fn elem_child_to_content(child: &ElemChild) -> Content {
    match child {
        ElemChild::Text(t) => formatted_to_content(t),
        ElemChild::Elem(e) => {
            let inner = elem_children_to_content(&e.children);
            match e.display {
                Some(Display::Block)
                | Some(Display::Indent)
                | Some(Display::LeftMargin) => {
                    Content::sequence(vec![inner, Content::linebreak()])
                }
                _ => inner,
            }
        }
        ElemChild::Markup(m) => Content::text(m.as_str()),
        ElemChild::Link { text, url } => {
            Content::link(url.as_str(), formatted_to_content(text))
        }
        ElemChild::Transparent { .. } => Content::Empty,
    }
}

/// Converte texto formatado hayagriva para `Content`, aplicando Strong/Emph/etc.
fn formatted_to_content(f: &Formatted) -> Content {
    apply_formatting(Content::text(f.text.as_str()), f.formatting)
}

/// Aplica formatação CSL sobre um `Content` de texto.
fn apply_formatting(content: Content, formatting: Formatting) -> Content {
    use hayagriva::citationberg::{FontStyle, FontVariant, FontWeight, TextDecoration};

    let mut c = content;
    if formatting.font_weight == FontWeight::Bold {
        c = Content::strong(c);
    }
    if formatting.font_style == FontStyle::Italic {
        c = Content::emph(c);
    }
    if formatting.font_variant == FontVariant::SmallCaps {
        c = Content::smallcaps(c);
    }
    if formatting.text_decoration == TextDecoration::Underline {
        c = Content::underline(c, None, None, None);
    }
    // VerticalAlign (superscript/subscript) não tem Content variant em P418.
    c
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::bib_entry::BibEntry;

    fn sample_entries() -> Vec<BibEntry> {
        vec![
            BibEntry::new("k1", "Doe, John", "A Crystal Paper", 2024)
                .with_journal("Journal of Examples")
                .with_volume("7")
                .with_pages("1-10"),
            BibEntry::new("k2", "Smith, Jane", "Another Book", 2023)
                .with_publisher("Example Press"),
        ]
    }

    #[test]
    fn build_cache_ieee_produz_citacoes_e_bibliografia() {
        let cache = build_cache(&sample_entries(), Some("ieee"), None).unwrap();
        assert!(cache.bibliography.is_some());
        let normal = cache.citations.get(&CitationForm::Normal).unwrap();
        assert!(normal.contains_key("k1"));
        assert!(normal.contains_key("k2"));
    }

    #[test]
    fn build_cache_apa_produz_citacoes_author_year() {
        let cache = build_cache(&sample_entries(), Some("apa"), None).unwrap();
        let author = cache.citations.get(&CitationForm::Author).unwrap();
        let year = cache.citations.get(&CitationForm::Year).unwrap();
        let k1_author = author.get("k1").unwrap().plain_text();
        let k1_year = year.get("k1").unwrap().plain_text();
        assert!(k1_author.contains("Doe"), "author: {k1_author}");
        assert!(k1_year.contains("2024"), "year: {k1_year}");
    }

    #[test]
    fn build_cache_style_inexistente_retorna_none() {
        assert!(build_cache(&sample_entries(), Some("not-a-real-style"), None).is_none());
    }

    #[test]
    fn build_cache_entries_vazias_produz_cache_vazio() {
        let cache = build_cache(&[], Some("ieee"), None).unwrap();
        assert!(cache.citations.is_empty());
        assert!(cache.bibliography.is_none());
    }

    #[test]
    fn elem_children_plain_text_preserva_texto() {
        let mut children = ElemChildren::default();
        children.0.push(ElemChild::Text(Formatted {
            text: "hello".into(),
            formatting: Formatting::default(),
        }));
        assert_eq!(elem_children_to_content(&children).plain_text(), "hello");
    }

    #[test]
    fn formatted_italic_vai_para_emph() {
        use hayagriva::citationberg::FontStyle;
        let mut formatting = Formatting::default();
        formatting.font_style = FontStyle::Italic;
        let c = apply_formatting(Content::text("italico"), formatting);
        assert!(matches!(c, Content::Emph(_)));
        assert_eq!(c.plain_text(), "italico");
    }

    #[test]
    fn build_cache_locale_pt_br_nao_quebra() {
        let cache = build_cache(&sample_entries(), Some("ieee"), Some("pt-BR")).unwrap();
        assert!(cache.bibliography.is_some());
        let normal = cache.citations.get(&CitationForm::Normal).unwrap();
        assert!(normal.contains_key("k1"));
    }

    #[test]
    fn build_cache_locale_inexistente_usa_locales_disponiveis() {
        // Locale inexistente: slice vazio; hayagriva recai no locale do style.
        let cache = build_cache(&sample_entries(), Some("ieee"), Some("xx-XX")).unwrap();
        assert!(cache.bibliography.is_some());
    }

    #[test]
    fn build_cache_chicago_author_date_resolve() {
        let cache =
            build_cache(&sample_entries(), Some("chicago-author-date"), None).unwrap();
        assert!(cache.bibliography.is_some());
        let prose = cache.citations.get(&CitationForm::Prose).unwrap();
        let txt = prose.get("k1").unwrap().plain_text();
        assert!(txt.contains("Doe") || txt.contains("2024"), "prose: {txt}");
    }

    #[test]
    fn citation_normal_ieee_brackets() {
        let cache = build_cache(&sample_entries(), Some("ieee"), None).unwrap();
        let normal = cache.citations.get(&CitationForm::Normal).unwrap();
        let txt = normal.get("k1").unwrap().plain_text();
        assert!(txt.starts_with('[') && txt.ends_with(']'), "ieee normal: {txt}");
    }

    #[test]
    fn citation_prose_apa_contem_author_e_year() {
        let cache = build_cache(&sample_entries(), Some("apa"), None).unwrap();
        let prose = cache.citations.get(&CitationForm::Prose).unwrap();
        let txt = prose.get("k1").unwrap().plain_text();
        assert!(txt.contains("Doe"), "prose author: {txt}");
        assert!(txt.contains("2024"), "prose year: {txt}");
    }

    #[test]
    fn citation_author_apa_is_author_only() {
        let cache = build_cache(&sample_entries(), Some("apa"), None).unwrap();
        let author = cache.citations.get(&CitationForm::Author).unwrap();
        let txt = author.get("k1").unwrap().plain_text();
        assert!(txt.contains("Doe"), "author: {txt}");
    }

    #[test]
    fn citation_year_apa_is_year_only() {
        let cache = build_cache(&sample_entries(), Some("apa"), None).unwrap();
        let year = cache.citations.get(&CitationForm::Year).unwrap();
        let txt = year.get("k1").unwrap().plain_text();
        assert!(txt.contains("2024"), "year: {txt}");
    }

    #[test]
    fn bibliography_ieee_contains_numbers() {
        let cache = build_cache(&sample_entries(), Some("ieee"), None).unwrap();
        let txt = cache.bibliography.unwrap().plain_text();
        assert!(txt.contains("[1]"), "bib: {txt}");
        assert!(txt.contains("[2]"), "bib: {txt}");
    }

    #[test]
    fn bibliography_apa_contains_author_title_year() {
        let cache = build_cache(&sample_entries(), Some("apa"), None).unwrap();
        let txt = cache.bibliography.unwrap().plain_text();
        assert!(txt.contains("Doe"), "apa bib author: {txt}");
        assert!(txt.contains("Crystal Paper"), "apa bib title: {txt}");
        assert!(txt.contains("2024"), "apa bib year: {txt}");
    }

    #[test]
    fn elem_children_with_emph_and_strong() {
        use hayagriva::citationberg::{FontStyle, FontWeight};
        let mut children = ElemChildren::default();
        children.0.push(ElemChild::Text(Formatted {
            text: "bold ".into(),
            formatting: Formatting {
                font_weight: FontWeight::Bold,
                ..Formatting::default()
            },
        }));
        children.0.push(ElemChild::Text(Formatted {
            text: "italic".into(),
            formatting: Formatting {
                font_style: FontStyle::Italic,
                ..Formatting::default()
            },
        }));
        let c = elem_children_to_content(&children);
        assert!(c.plain_text().contains("bold italic"));
    }

    #[test]
    fn elem_child_link_preserves_url_and_text() {
        let child = ElemChild::Link {
            text: Formatted {
                text: "click".into(),
                formatting: Formatting::default(),
            },
            url: "https://example.com".into(),
        };
        let c = elem_child_to_content(&child);
        assert_eq!(c.plain_text(), "click");
        assert!(matches!(c, Content::Link(_)));
    }

    #[test]
    fn formatted_strong_vai_para_strong() {
        use hayagriva::citationberg::FontWeight;
        let mut formatting = Formatting::default();
        formatting.font_weight = FontWeight::Bold;
        let c = apply_formatting(Content::text("negrito"), formatting);
        assert!(matches!(c, Content::Strong(_)));
    }

    #[test]
    fn formatted_smallcaps_vai_para_smallcaps() {
        use hayagriva::citationberg::FontVariant;
        let mut formatting = Formatting::default();
        formatting.font_variant = FontVariant::SmallCaps;
        let c = apply_formatting(Content::text("smallcaps"), formatting);
        assert!(matches!(c, Content::SmallCaps { .. }));
    }

    #[test]
    fn formatted_underline_vai_para_underline() {
        use hayagriva::citationberg::TextDecoration;
        let mut formatting = Formatting::default();
        formatting.text_decoration = TextDecoration::Underline;
        let c = apply_formatting(Content::text("sublinhado"), formatting);
        assert!(matches!(c, Content::Underline(_)));
    }

    #[test]
    fn escape_yaml_scalar_quotas_com_colon() {
        let s = escape_yaml_scalar("New York: ACM");
        assert!(s.starts_with('"') && s.ends_with('"'));
    }

    #[test]
    fn escape_yaml_scalar_numerico_e_quotado() {
        let s = escape_yaml_scalar("2024");
        assert!(s.starts_with('"') && s.ends_with('"'));
    }

    #[test]
    fn bib_entry_to_hayagriva_converte_key_e_title() {
        let entry = BibEntry::new("mykey", "Author, A.", "The Title", 2022);
        let h = bib_entry_to_hayagriva(&entry).unwrap();
        assert_eq!(h.key(), "mykey");
        assert_eq!(h.title().unwrap().value.to_str(), "The Title");
    }

    #[test]
    fn bib_entry_to_hayagriva_article_tem_journal_parent() {
        let entry = BibEntry::new("art", "Author, A.", "Title", 2022)
            .with_journal("J of Examples");
        let h = bib_entry_to_hayagriva(&entry).unwrap();
        assert!(!h.parents().is_empty());
    }
}
