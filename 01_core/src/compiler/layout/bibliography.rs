//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/bibliography.md
//! @prompt-hash 040a91a7
//! @layer L1
//! @updated 2026-06-25
//!
//! Atomização (ADR-0109, P381): o layout de `Bibliography` movido do monólito
//! `layout_content` para o arquivo da feature (forma B). Content-preserving.
//!
//! **P418** — renderização CSL via cache pré-computado em
//! `layout_with_introspector`. Fallback para `format_bib_entry` quando o cache
//! não está disponível (ex: style inválido ou entries vazias).

use crate::entities::content::Content;
use crate::entities::elements::bibliography::BibliographyElem;
use crate::entities::introspector::Introspector;

use super::{FontMetrics, ImageSizer, Layouter};

/// Helper privado P159D + P159E + P159G — formata `BibEntry`
/// para render Bibliography. Concatenação condicional dos
/// fields opcionais quando presentes; backwards compat preserva
/// formato P159E quando todos os 6 fields P159G são `None`.
///
/// Ordem APA-like extendida (decisões diagnósticos P159D §10 +
/// P159E §8.2 + P159G §8.2):
/// `author. title (Ed. editor) (series) journal vol. volume,`
/// `pp. pages. location: publisher (year). isbn:XXX url, doi:YYY [note].`
///
/// **P159G**: editor/series após title; location antes de
/// publisher; organization substitutivo a publisher quando
/// publisher ausente; isbn antes de url/doi; note ao final.
/// **P468** — corpo da entry sem prefixo `[key]`. Usado abaixo
/// para adicionar `[N]` numeração por ordem de citação.
fn format_bib_entry_body(e: &crate::entities::bib_entry::BibEntry) -> String {
    let mut out = format!("{}. {}", e.author, e.title);
    format_bib_entry_body_fields(e, &mut out);
    out
}

fn format_bib_entry_body_fields(
    e: &crate::entities::bib_entry::BibEntry,
    out: &mut String,
) {
    // P159G — editor/series após title.
    if let Some(ed) = &e.editor {
        out.push_str(&format!(" (Ed. {})", ed));
    }
    if let Some(se) = &e.series {
        out.push_str(&format!(" ({})", se));
    }
    // P159D — journal/volume/pages.
    if let Some(j) = &e.journal {
        out.push_str(&format!(" {}", j));
    }
    if let Some(v) = &e.volume {
        out.push_str(&format!(" vol. {}", v));
    }
    if let Some(p) = &e.pages {
        out.push_str(&format!(", pp. {}", p));
    }
    // P159G — location antes de publisher; organization substitutivo
    // a publisher quando publisher ausente.
    let pub_slot: Option<String> = match (&e.publisher, &e.organization) {
        (Some(pb), _) => Some(pb.clone()),
        (None, Some(o)) => Some(o.clone()),
        (None, None) => None,
    };
    match (&e.location, &pub_slot) {
        (Some(l), Some(pb)) => out.push_str(&format!(". {}: {}", l, pb)),
        (Some(l), None) => out.push_str(&format!(". {}", l)),
        (None, Some(pb)) => out.push_str(&format!(". {}", pb)),
        (None, None) => {}
    }
    out.push_str(&format!(" ({}).", e.year));
    // P159G — isbn antes de url/doi.
    if let Some(i) = &e.isbn {
        out.push_str(&format!(" isbn:{}", i));
    }
    // P159E — par natural url/doi após (year). per Opção C.
    match (&e.url, &e.doi) {
        (Some(u), Some(d)) => out.push_str(&format!(" {}, doi:{}.", u, d)),
        (Some(u), None) => out.push_str(&format!(" {}.", u)),
        (None, Some(d)) => out.push_str(&format!(" doi:{}.", d)),
        (None, None) => {
            // Fechar com `.` se isbn presente sem url/doi.
            if e.isbn.is_some() {
                out.push('.');
            }
        }
    }
    // P159G — note ao final.
    if let Some(n) = &e.note {
        out.push_str(&format!(" [{}]", n));
    }
}

/// Layout de `bibliography(...)`: renderiza o title (se presente) e a lista de
/// referências. Com cache CSL disponível, usa a bibliografia pré-renderizada;
/// senão usa o fallback local `format_bib_entry_body`.
///
/// **P468** — fallback reordena entries por `citation_order` e prefixa com `[N]`.
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    b: &BibliographyElem,
) {
    let body = if let Some(content) = layouter
        .bib_render_cache
        .as_ref()
        .and_then(|cache| cache.bibliography.clone())
    {
        content
    } else {
        fallback_body(layouter, b)
    };
    if b.title.is_none() && body.is_empty() {
        return;
    }

    let previous_segment = layouter
        .text_segment_chain
        .clone()
        .unwrap_or_else(|| layouter.chain.clone());
    super::paragraph::close_in_chain(layouter, previous_segment, true);
    if let Some(title) = &b.title {
        layouter.layout_content(title);
        layouter.flush_line();
    }
    if !body.is_empty() {
        let block = Content::block(
            body,
            None,
            None,
            crate::entities::sides::Sides::uniform(
                crate::entities::layout_types::Length::ZERO,
            ),
            true,
        );
        layouter.layout_content(&block);
    }
}

fn fallback_body<M: FontMetrics, S: ImageSizer>(
    layouter: &Layouter<M, S>,
    b: &BibliographyElem,
) -> Content {
    // Fallback P418/P468: reordena entries por ordem de primeira citação.
    // Entries não citadas ficam no final (na ordem original).
    let citation_order: Vec<String> = layouter.introspector.citation_order().to_vec();
    let mut ordered: Vec<_> = b.entries.iter().collect();
    ordered.sort_by_key(|e| {
        citation_order.iter().position(|k| k == &e.key).unwrap_or(usize::MAX)
    });
    let mut parts = Vec::with_capacity(ordered.len() * 2);
    for (idx, e) in ordered.iter().enumerate() {
        let n = idx + 1;
        let body = format_bib_entry_body(e);
        // **P472** — back-refs: lista de posições onde a entry foi citada.
        let refs = layouter.introspector.back_refs_for_key(&e.key);
        let back_ref_str = if refs.is_empty() {
            String::new()
        } else {
            let cited: String =
                refs.iter().map(|p| format!("[{}]", p)).collect::<Vec<_>>().join("");
            format!(" ↑{}", cited)
        };
        let line = format!("[{}] {}{}", n, body, back_ref_str);
        parts.push(Content::text(line));
        parts.push(Content::linebreak());
    }
    Content::sequence(parts)
}

#[cfg(test)]
mod a2_bib_flow_tests {
    use super::*;
    use crate::compiler::layout::{bib_csl::BibRenderCache, FixedMetrics};
    use crate::entities::{
        bib_entry::BibEntry, image_sizer::NullImageSizer, introspector::TagIntrospector,
        layout_types::FrameItem,
    };
    use comemo::Track;
    use std::collections::HashMap;

    fn bibliography(title: bool, fallback: bool) -> BibliographyElem {
        BibliographyElem {
            entries: if fallback {
                vec![BibEntry::new("k", "AUTHOR", "ENTRY", 2020)]
            } else {
                vec![]
            },
            path: None,
            title: title.then(|| Content::heading(1, Content::text("TITLE"))),
            style: None,
            locale: None,
        }
    }

    fn render(prefix: bool, title: bool, fallback: bool) -> Vec<(String, f64, f64)> {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            FixedMetrics,
            NullImageSizer,
            12.0,
            (&intr as &dyn Introspector).track(),
        );
        if !fallback {
            l.bib_render_cache = Some(BibRenderCache {
                citations: HashMap::new(),
                bibliography: Some(Content::text("ENTRY")),
            });
        }
        if prefix {
            l.layout_content(&Content::text("CITE"));
        }
        layout(&mut l, &bibliography(title, fallback));
        l.layout_content(&Content::Parbreak);
        l.layout_content(&Content::text("OUTSIDE"));
        l.finish()
            .pages
            .iter()
            .flat_map(|page| {
                crate::compiler::layout::tests::frame_items_recursive(&page.items)
            })
            .filter_map(|item| match item {
                FrameItem::Text { text, pos, .. } => {
                    Some((text.to_string(), pos.x.0, pos.y.0))
                }
                _ => None,
            })
            .collect()
    }

    fn y(rows: &[(String, f64, f64)], needle: &str) -> f64 {
        rows.iter().find(|(text, _, _)| text.contains(needle)).unwrap().2
    }

    #[test]
    fn a2_bib_flow_cached_body_starts_after_citation() {
        let rows = render(true, false, false);
        assert!(y(&rows, "ENTRY") > y(&rows, "CITE"), "{rows:?}");
        assert!(y(&rows, "OUTSIDE") > y(&rows, "ENTRY"), "{rows:?}");
    }

    #[test]
    fn a2_bib_flow_title_body_and_citation_are_separate() {
        let rows = render(true, true, false);
        assert!(y(&rows, "TITLE") > y(&rows, "CITE"), "{rows:?}");
        let cite_x = rows.iter().find(|(text, _, _)| text == "CITE").unwrap().1;
        let title_x = rows.iter().find(|(text, _, _)| text == "TITLE").unwrap().1;
        assert_eq!(title_x, cite_x, "title must start its own line: {rows:?}");
        assert!(y(&rows, "ENTRY") > y(&rows, "TITLE"), "{rows:?}");
        assert!(y(&rows, "OUTSIDE") > y(&rows, "ENTRY"), "{rows:?}");
    }

    #[test]
    fn a2_bib_flow_fallback_is_also_a_block() {
        let rows = render(true, false, true);
        assert!(y(&rows, "ENTRY") > y(&rows, "CITE"), "{rows:?}");
        assert!(y(&rows, "OUTSIDE") > y(&rows, "ENTRY"), "{rows:?}");
    }

    #[test]
    fn a2_bib_flow_first_body_keeps_page_origin() {
        let rows = render(false, false, false);
        let doc = crate::compiler::layout::layout(&Content::text("ENTRY"));
        let baseline =
            crate::compiler::layout::tests::frame_items_recursive(&doc.pages[0].items)
                .into_iter()
                .find_map(|item| match item {
                    FrameItem::Text { text, pos, .. } if text.as_str() == "ENTRY" => {
                        Some(pos.y.0)
                    }
                    _ => None,
                })
                .unwrap();
        assert_eq!(y(&rows, "ENTRY"), baseline);
    }

    #[test]
    fn a2_bib_flow_empty_body_does_not_flush_or_add_margin() {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            FixedMetrics,
            NullImageSizer,
            12.0,
            (&intr as &dyn Introspector).track(),
        );
        l.layout_content(&Content::text("CITE"));
        let before = (
            l.regions.current.cursor_x,
            l.regions.current.cursor_y,
            l.regions.current.current_line.len(),
        );
        layout(&mut l, &bibliography(false, false));
        assert_eq!(
            before,
            (
                l.regions.current.cursor_x,
                l.regions.current.cursor_y,
                l.regions.current.current_line.len()
            )
        );
    }
}
