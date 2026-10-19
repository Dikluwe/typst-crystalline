//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/bibliography.md
//! @prompt-hash 4f429996
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

#[derive(Clone)]
pub(super) struct PendingPrefix {
    body: Content,
    chain: crate::entities::style_chain::StyleChain,
    style: crate::entities::layout_types::TextStyle,
    indent: f64,
    rtl: bool,
}

/// Emit only after the paragraph backend has admitted its actual first line.
pub(super) fn emit_pending_prefix<M: FontMetrics, S: ImageSizer>(l: &mut Layouter<M, S>) {
    use crate::entities::layout_types::{Align2D, HAlign, PlaceScope, Pt, Rect, VAlign};
    let Some(prefix) = l.pending_bib_prefix.take() else { return };
    let baseline = l.baseline_y();
    let style = std::mem::replace(&mut l.style, prefix.style);
    let chain = std::mem::replace(&mut l.chain, prefix.chain);
    let (top, _) = l.metrics.text_edges(l.style.size, &l.style);
    let offset = l.style.baseline_offset.resolve_pt(l.style.size.0);
    let region = l.block_place_region.replace(Rect {
        x: l.regions.current.line_start_x,
        y: baseline - top - Pt(offset),
        w: Pt(l.regions.current.width
            - l.page_config.margin.right
            - l.regions.current.line_start_x.0),
        h: Pt(f64::INFINITY),
    });
    let placed = Content::place(
        Align2D {
            h: Some(if prefix.rtl { HAlign::Right } else { HAlign::Left }),
            v: Some(VAlign::Top),
        },
        if prefix.rtl { prefix.indent } else { -prefix.indent },
        0.0,
        PlaceScope::Column,
        false,
        None,
        prefix.body,
    );
    l.layout_content(&placed);
    l.block_place_region = region;
    l.chain = chain;
    l.style = style;
}

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
    if let Some(plan) = layouter.bib_layout.clone() {
        use super::bib_csl::plan::BibliographyPlan;
        let bibliography_chain = layouter.chain.clone();
        layouter.chain = layouter.chain.push_custom(
            "par.hanging-indent",
            crate::entities::value::Value::Length(
                crate::entities::layout_types::Length::ZERO,
            ),
        );
        match plan.as_ref() {
            BibliographyPlan::Columns(entries) => {
                layout_columns(layouter, entries);
            }
            BibliographyPlan::Hanging(entries) => layout_hanging(layouter, entries),
            BibliographyPlan::Plain(entries) => {
                for entry in entries {
                    layouter.layout_content(&entry_block(entry.body.clone()));
                }
            }
        }
        layouter.chain = bibliography_chain;
        return;
    }
    if !body.is_empty() {
        layouter.layout_content(&entry_block(body));
    }
}

fn layout_hanging<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    entries: &[super::bib_csl::plan::Entry],
) {
    use crate::entities::{dir::Dir, layout_types::Length, sides::Sides, value::Value};
    let amount = match layouter.chain.custom("bibliography.hanging-override") {
        Some(Value::Length(length)) => *length,
        _ => Length::em(1.5),
    }
    .resolve_pt(layouter.style.size.0);
    let dir = super::text::effective_style(layouter).dir.unwrap_or(Dir::LTR);
    let inset = Sides {
        top: Length::ZERO,
        bottom: Length::ZERO,
        left: Length::pt(if dir == Dir::LTR { amount } else { 0.0 }),
        right: Length::pt(if dir == Dir::RTL { amount } else { 0.0 }),
    };
    for entry in entries {
        let previous = layouter
            .pending_bib_escape
            .replace(super::paragraph_indent::Spec::escape(amount, dir));
        layouter.layout_content(&Content::block(
            entry.body.clone(),
            None,
            None,
            inset,
            true,
        ));
        layouter.pending_bib_escape = previous;
    }
}

fn entry_block(body: Content) -> Content {
    Content::block(
        body,
        None,
        None,
        crate::entities::sides::Sides::uniform(
            crate::entities::layout_types::Length::ZERO,
        ),
        true,
    )
}

fn layout_columns<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    entries: &[super::bib_csl::plan::Entry],
) {
    use crate::entities::{dir::Dir, layout_types::Length, sides::Sides};
    let prefix_width = entries
        .iter()
        .filter_map(|entry| entry.prefix.as_ref())
        .map(|prefix| {
            super::measurement::measure_content_real(
                prefix,
                &layouter.chain,
                &layouter.metrics,
            )
            .0
        })
        .fold(0.0_f64, f64::max);
    let indent = prefix_width + 0.65 * layouter.style.size.0;
    let rtl = super::text::effective_style(layouter).dir == Some(Dir::RTL);
    let inset = Sides {
        top: Length::ZERO,
        bottom: Length::ZERO,
        left: Length::pt(if rtl { 0.0 } else { indent }),
        right: Length::pt(if rtl { indent } else { 0.0 }),
    };
    for entry in entries {
        if entry.body.is_empty() {
            let previous = layouter.pending_bib_prefix.take();
            layouter.layout_content(&entry_block(
                entry.prefix.clone().unwrap_or(Content::Empty),
            ));
            layouter.pending_bib_prefix = previous;
            continue;
        }
        let pending = entry.prefix.as_ref().map(|prefix| PendingPrefix {
            body: prefix.clone(),
            chain: layouter.chain.clone(),
            style: layouter.style.clone(),
            indent,
            rtl,
        });
        let previous = std::mem::replace(&mut layouter.pending_bib_prefix, pending);
        layouter.layout_content(&Content::block(
            entry.body.clone(),
            None,
            None,
            inset,
            true,
        ));
        layouter.pending_bib_prefix = previous;
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

#[cfg(test)]
mod a2_structure_tests {
    use super::*;
    use crate::entities::{
        bib_entry::BibEntry,
        layout_types::{FrameItem, Length, Pt, TextStyle},
        style::{Style, Styles},
        value::Value,
    };
    use std::sync::Arc;

    fn render(font: f64, spacing: f64) -> Vec<(String, f64, f64)> {
        let b = Content::Bibliography(Arc::new(BibliographyElem {
            entries: vec![
                BibEntry::new("first", "Alpha, Ann", "FIRSTWORD", 2020),
                BibEntry::new("second", "Beta, Bob", "SECONDWORD", 2021),
            ],
            path: None,
            title: None,
            style: Some("ieee".into()),
            locale: None,
        }));
        let styles = Styles::from_iter([Style::Size(Length::pt(font))])
            .push_custom("par.spacing", Value::Length(Length::pt(spacing)));
        let content = Content::styled(b, styles);
        super::super::layout(&content)
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

    fn at<'a>(rows: &'a [(String, f64, f64)], word: &str) -> &'a (String, f64, f64) {
        rows.iter().find(|(text, _, _)| text == word).unwrap()
    }

    #[test]
    fn a2_structure_prefix_column_uses_relative_gutter() {
        use super::super::{FixedMetrics, FontMetrics};
        for font in [12.0, 18.0] {
            let rows = render(font, 0.0);
            let label = at(&rows, "[1]");
            let body = at(&rows, "A.");
            let prefix =
                FixedMetrics.advance("[1]", Pt(font), &TextStyle::regular(Pt(font))).0;
            assert!((body.1 - label.1 - prefix - 0.65 * font).abs() < 1e-8, "{rows:?}");
            assert!((body.2 - label.2).abs() < 1e-8, "{rows:?}");
            assert_eq!(body.1, at(&rows, "B.").1);
        }
    }

    #[test]
    fn a2_structure_entry_gap_uses_paragraph_spacing() {
        let zero = render(12.0, 0.0);
        let spaced = render(12.0, 24.0);
        let gap = |rows: &[(String, f64, f64)]| at(rows, "B.").2 - at(rows, "A.").2;
        assert!((gap(&spaced) - gap(&zero) - 24.0).abs() < 1e-8, "{zero:?}\n{spaced:?}");
        assert_eq!(at(&spaced, "A.").2, at(&zero, "A.").2);
    }
    #[test]
    fn a2_structure_checkpoint_restores_same_shared_plan() {
        use super::super::{bib_csl::plan::BibliographyPlan, FixedMetrics};
        use crate::entities::{
            image_sizer::NullImageSizer,
            introspector::{Introspector, TagIntrospector},
        };
        use comemo::Track;
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            FixedMetrics,
            NullImageSizer,
            12.0,
            (&intr as &dyn Introspector).track(),
        );
        let plan = Arc::new(BibliographyPlan::Hanging(vec![]));
        l.bib_layout = Some(plan.clone());
        let checkpoint = l.capture_flow_suffix_checkpoint();
        l.bib_layout = Some(Arc::new(BibliographyPlan::Columns(vec![])));
        l.restore_flow_suffix_checkpoint(checkpoint);
        assert!(Arc::ptr_eq(l.bib_layout.as_ref().unwrap(), &plan));
        assert!(matches!(l.bib_layout.as_deref(), Some(BibliographyPlan::Hanging(_))));
    }
    #[test]
    fn a2_structure_prefix_follows_admitted_body() {
        use crate::entities::layout_types::{PageDimension, PageMarginSpec};
        let bibliography = Content::Bibliography(Arc::new(BibliographyElem {
            entries: vec![BibEntry::new(
                "entry",
                "Alpha, Ann",
                "FIRSTWORD Long Reference Title",
                2020,
            )],
            path: None,
            title: None,
            style: Some("ieee".into()),
            locale: None,
        }));
        let page = Content::page_run(
            Some(PageDimension::Length(200.0)),
            Some(PageDimension::Length(70.0)),
            Some(PageMarginSpec::uniform(8.0)),
            None,
            None,
            Content::sequence(vec![
                Content::text("CITE"),
                bibliography,
                Content::Parbreak,
                Content::text("OUTSIDE"),
            ]),
        );
        let document = super::super::layout(&page);
        let first_page = |needle: &str| {
            document.pages.iter().position(|page| crate::compiler::layout::tests::frame_items_recursive(&page.items).iter().any(|item| matches!(item, FrameItem::Text{text,..} if text.as_str()==needle))).unwrap()
        };
        assert_eq!(
            first_page("[1]"),
            first_page("A."),
            "{:?}",
            document.pages.iter().map(|p| p.plain_text()).collect::<Vec<_>>()
        );
        assert!(document.plain_text().contains("OUTSIDE"));
    }
    #[test]
    fn a2_structure_prefix_only_entries_do_not_overlap() {
        use super::super::{bib_csl::plan::Entry, FixedMetrics};
        use crate::entities::{
            image_sizer::NullImageSizer,
            introspector::{Introspector, TagIntrospector},
        };
        use comemo::Track;
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            FixedMetrics,
            NullImageSizer,
            12.0,
            (&intr as &dyn Introspector).track(),
        );
        let entries = vec![
            Entry {
                prefix: Some(Content::text("[1]")),
                body: Content::Empty,
            },
            Entry {
                prefix: Some(Content::text("[2]")),
                body: Content::Empty,
            },
        ];
        layout_columns(&mut l, &entries);
        let document = l.finish();
        let rows: Vec<_> = document
            .pages
            .iter()
            .flat_map(|page| {
                crate::compiler::layout::tests::frame_items_recursive(&page.items)
            })
            .filter_map(|item| match item {
                FrameItem::Text { text, pos, .. } if text.as_str().starts_with('[') => {
                    Some((text.to_string(), pos.x.0, pos.y.0))
                }
                _ => None,
            })
            .collect();
        assert_eq!(
            rows.iter().map(|row| row.0.as_str()).collect::<Vec<_>>(),
            ["[1]", "[2]"]
        );
        assert_eq!(rows.len(), 2, "{rows:?}");
        assert!(rows[1].2 > rows[0].2, "{rows:?}");
    }
}
