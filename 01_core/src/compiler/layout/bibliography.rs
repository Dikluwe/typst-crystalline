//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/bibliography.md
//! @prompt-hash e5e93ea0
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
    if let Some(t) = &b.title {
        layouter.layout_content(t);
        layouter.flush_line();
    }

    if let Some(bib_content) = layouter
        .bib_render_cache
        .as_ref()
        .and_then(|c| c.bibliography.clone())
    {
        layouter.layout_content(&bib_content);
        return;
    }

    // Fallback P418/P468: reordena entries por ordem de primeira citação.
    // Entries não citadas ficam no final (na ordem original).
    let citation_order: Vec<String> = layouter.introspector.citation_order().to_vec();
    let mut ordered: Vec<_> = b.entries.iter().collect();
    ordered.sort_by_key(|e| {
        citation_order.iter().position(|k| k == &e.key).unwrap_or(usize::MAX)
    });
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
        layouter.layout_content(&Content::text(line));
        layouter.flush_line();
    }
}
