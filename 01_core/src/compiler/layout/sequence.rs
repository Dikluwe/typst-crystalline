//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout.md
//! @prompt-hash 0450974a
//! @layer L1
//! @updated 2026-08-20
//!
//! Layout de `Content::Sequence` — iteração peekable com sticky
//! lookahead e spacing collapse. Extraído de `layout/mod.rs` no P425
//! (ADR-0109 forma B).

use crate::entities::{content::Content, image_sizer::ImageSizer, layout_types::Pt};

use super::metrics::FontMetrics;
use super::{ItemGroup, Layouter};

/// Layout de uma sequência de conteúdos.
///
/// P250 — refactor Sequence consumer para peekable + neighbour context.
/// Permite:
/// - Sticky lookahead (Block.sticky=true detecta next; se combined_h >
///   remaining + cabe em página inteira → new_page() antes do block).
/// - Spacing collapse via Layouter fields `prev_block_below_pending` +
///   `block_chain_active`.
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<'_, M, S>,
    parts: &[Content],
) {
    let saved_below = layouter.prev_block_below_pending;
    let saved_chain = layouter.block_chain_active;
    let saved_descent = layouter.prev_block_equation_descent;

    let mut iter = parts.iter().peekable();
    while let Some(part) = iter.next() {
        // **P864** — detetar `Content::Parbreak` entre itens do mesmo grupo.
        // Se o item anterior (mesmo tipo) foi separado por um Parbreak,
        // aplica o espaçamento de parágrafo entre grupos e reinicia o estado
        // de agrupamento de *espaçamento* (`last_was_loose_item`).
        //
        // **P1016** — o `enum_counter` NÃO é reiniciado aqui. Medido no
        // vanilla: `+ a\n+ b\n\n+ c` numera `1. 2. 3.` — a numeração
        // atravessa a linha em branco. Só conteúdo real termina a lista.
        if let Some(group) = item_group(part) {
            if layouter.parbreak_since_last_item
                && layouter.last_seen_item_group == Some(group)
            {
                layouter.regions.current.cursor_y += paragraph_advance(layouter);
                layouter.last_was_loose_item = false;
            }
            layouter.last_seen_item_group = Some(group);
            layouter.parbreak_since_last_item = false;
        } else if matches!(part, Content::Parbreak) {
            // Marca que um Parbreak ocorreu desde o último item, desde que
            // ainda estejamos num contexto de agrupamento válido.
            if layouter.last_seen_item_group.is_some() {
                layouter.parbreak_since_last_item = true;
            }
        } else if !matches!(part, Content::Space | Content::Empty | Content::Styled(..)) {
            // Qualquer outro conteúdo real quebra a consecutividade.
            layouter.last_seen_item_group = None;
            layouter.parbreak_since_last_item = false;
        }

        // P250 — sticky pre-layout lookahead 1-block.
        if matches!(part, Content::Block(e) if e.sticky) {
            if let Some(next) = iter.peek() {
                let avail_w = layouter.available_width();
                let (_, part_h) = layouter.measure_content_constrained(part, avail_w);
                let (_, next_h) = layouter.measure_content_constrained(next, avail_w);
                let combined = part_h + next_h;
                let remaining =
                    layouter.page_bottom_limit() - layouter.regions.current.cursor_y.0;
                let page_usable = layouter.available_height();
                if combined > remaining && combined <= page_usable {
                    layouter.new_page();
                }
                // else: cabem ambos OU overlong → emit normal.
            }
        }

        layouter.layout_content(part);

        // **P1107** — preservação da cadeia de colapso de margens:
        // Apenas elementos de bloco (Block, Shape, Parbreak, Heading, Equation)
        // e nós transparentes (Space, Empty quando fora de linha aberta)
        // mantêm o estado de colapso de margens.
        // **P1107/P1108** — Preservação da cadeia de colapso de margens:
        // A cadeia só é interrompida quando uma linha de texto inline aberta é populada.
        if !layouter.regions.current.current_line.is_empty() {
            layouter.block_chain_active = false;
            layouter.prev_block_below_pending = 0.0;
        }

        // P505 — apenas ListItem/EnumItem consecutivos com tight=false
        // partilham o espaçamento de parágrafo; qualquer outro conteúdo
        // reseta o estado.
        if !matches!(part, Content::ListItem(_) | Content::EnumItem(_)) {
            layouter.last_was_loose_item = false;
        }
        if !matches!(
            part,
            Content::EnumItem(_)
                | Content::Space
                | Content::Parbreak
                | Content::Empty
                | Content::Styled(..)
        ) {
            layouter.enum_counter = None;
        }
    }

    // Preservar estado da cadeia anterior se a sequência foi composta exclusivamente por nós transparentes/estilos
    let is_transparent = parts.iter().all(|p| {
        matches!(
            p,
            Content::Empty | Content::Styled(..) | Content::Space | Content::Parbreak
        )
    });
    if is_transparent && saved_chain {
        layouter.prev_block_below_pending = saved_below;
        layouter.block_chain_active = saved_chain;
        layouter.prev_block_equation_descent = saved_descent;
    }
}

/// **P864** — devolve o grupo de item estrutural reconhecido, se aplicável.
fn item_group(content: &Content) -> Option<ItemGroup> {
    match content {
        Content::ListItem(_) => Some(ItemGroup::List),
        Content::EnumItem(_) => Some(ItemGroup::Enum),
        Content::TermItem(_) => Some(ItemGroup::Terms),
        _ => None, // neutro: N16[β] — Content atómico sem sub-sequência retorna None (projeção de container)
    }
}

/// **P864** — avanço vertical de parágrafo (top-edge + |bottom-edge| +
/// leading default), usado como espaçamento entre grupos separados por
/// `Content::Parbreak`.
fn paragraph_advance<M: FontMetrics, S: ImageSizer>(layouter: &Layouter<M, S>) -> Pt {
    let font_size = layouter.style.size;
    let (top, bottom) = layouter.metrics.text_edges(font_size, &layouter.style);
    let leading = layouter
        .style
        .leading
        .map(|l| l.resolve_pt(font_size.val()))
        // PAR_LEADING, ver vanilla_defaults.rs
        .unwrap_or_else(|| font_size.val() * super::vanilla_defaults::PAR_LEADING);
    top + Pt(-bottom.0) + Pt(leading)
}
