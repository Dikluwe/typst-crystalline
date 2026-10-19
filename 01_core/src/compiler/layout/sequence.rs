//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/sequence.md
//! @prompt-hash 972f2662
//! @layer L1
//! @updated 2026-10-07
//!
//! Layout de `Content::Sequence` — iteração peekable com sticky
//! lookahead e spacing collapse. Extraído de `layout/mod.rs` no P425
//! (ADR-0109 forma B).

use crate::entities::elements::block::BlockElem;
use crate::entities::elements::block::BlockHeight;
use crate::entities::{content::Content, image_sizer::ImageSizer, layout_types::Pt};

use super::metrics::FontMetrics;
use super::{ItemGroup, Layouter};

mod fractional_preflight;
mod inline_prefix;

#[cfg(test)]
use self::fractional_preflight::fractional_region_remaining;
use self::fractional_preflight::{
    fractional_block, fractional_preflight, fractional_share, FractionalRegionBasis,
};
#[cfg(test)]
use self::inline_prefix::{
    p1380_inline_safe_styles, p1380_is_unwrapped, p1389_single_word,
};

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
    super::paragraph_indent::start_scope(layouter, parts);
    let needs_bidi = layouter.bidi_cursor.is_some()
        || parts.split(|p| matches!(p, Content::Parbreak)).any(|p| {
            super::paragraph::shared_direction(p, &layouter.chain)
                .and_then(|d| super::bidi_origin::prepare(layouter, p, d))
                .is_some()
        });
    if !needs_bidi
        && !layouter.paragraph_planning_suppressed
        && super::paragraph_plan::try_parts(layouter, parts)
    {
        return;
    }
    let previous = layouter.paragraph_planning_suppressed;
    layouter.paragraph_planning_suppressed = true;
    layout_parts(layouter, parts, FractionalRegionBasis::Inherited);
    layouter.paragraph_planning_suppressed = previous;
}

/// P1379 — entrada exclusiva para uma região irmã criada por `colbreak()`.
/// O próprio content é emprestado: Sequence usa a fatia existente, singleton
/// usa `from_ref` e vazio não aloca.
pub(super) fn layout_segmented_region<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<'_, M, S>,
    content: &Content,
) {
    let content = content.source_body();
    if let Content::Styled(styled) = content {
        let body = &styled.body;
        let styles = &styled.styles;
        let previous_chain = layouter.chain.clone();
        let previous_style = layouter.style.clone();
        layouter.chain = layouter.chain.push_styles(styles);
        layouter.style = crate::entities::layout_types::TextStyle::from(&layouter.chain);
        layout_segmented_unstyled(layouter, body);
        layouter.chain = previous_chain;
        layouter.style = previous_style;
        return;
    }
    layout_segmented_unstyled(layouter, content);
}

fn layout_segmented_unstyled<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<'_, M, S>,
    content: &Content,
) {
    match content.source_body() {
        Content::Sequence(parts) => {
            layout_parts(layouter, parts, FractionalRegionBasis::CursorRemaining)
        }
        Content::Empty => {
            layout_parts(layouter, &[], FractionalRegionBasis::CursorRemaining)
        }
        other => layout_parts(
            layouter,
            std::slice::from_ref(other),
            FractionalRegionBasis::CursorRemaining,
        ),
    }
}

fn layout_parts<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<'_, M, S>,
    parts: &[Content],
    basis: FractionalRegionBasis,
) {
    let saved_text_segment = layouter.text_segment_chain.clone();
    let saved_geometric_lines = layouter.geometric_text_lines;
    if !super::html_frame::items_have_causal_line(&layouter.regions.current.current_line)
    {
        layouter.text_segment_chain = Some(layouter.chain.clone());
    }
    let enclosing_dir = layouter.active_paragraph_dir;
    let enclosing_scope = layouter.bidi_scope_active;
    layouter.bidi_scope_active = true;
    let saved_below = layouter.prev_block_below_pending;
    let saved_chain = layouter.block_chain_active;
    let saved_descent = layouter.prev_block_equation_descent;
    let saved_fractional_height = layouter.fractional_block_height;
    let distribution = fractional_preflight(layouter, parts, basis);
    if let Some(prefix) =
        distribution.and_then(|(_, _, prefix)| prefix).filter(|p| p.multiline)
    {
        layouter.geometric_text_lines = true;
        layouter.text_segment_chain =
            super::paragraph::shared_chain(&parts[..prefix.block_index], &layouter.chain);
    }
    let terminal_flow_index = parts.iter().rposition(|part| {
        !matches!(part, Content::Space | Content::SpaceFromNewline | Content::Empty)
    });

    let mut iter = parts.iter().peekable();
    let mut part_index = 0usize;
    let mut text_segment_start = 0usize;
    let mut owns_text_segment = !super::html_frame::items_have_causal_line(
        &layouter.regions.current.current_line,
    );
    while let Some(part) = iter.next() {
        if enclosing_dir.is_none()
            && (part_index == 0
                || matches!(
                    parts[part_index - 1],
                    Content::Parbreak | Content::SetPage(_)
                ))
        {
            let end = parts[part_index..]
                .iter()
                .position(|p| matches!(p, Content::Parbreak | Content::SetPage(_)))
                .map_or(parts.len(), |n| part_index + n);
            layouter.active_paragraph_dir = super::paragraph::shared_direction(
                &parts[part_index..end],
                &layouter.chain,
            );
            if !enclosing_scope {
                layouter.bidi_cursor = layouter.active_paragraph_dir.and_then(|d| {
                    super::bidi_origin::prepare(layouter, &parts[part_index..end], d)
                });
            }
        }
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
        } else if !matches!(
            part,
            Content::Space
                | Content::SpaceFromNewline
                | Content::Empty
                | Content::Styled(..)
        ) {
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

        let inline_prefix = distribution.and_then(|(_, _, inline_prefix)| inline_prefix);
        if part_index == 0
            && inline_prefix.is_some_and(|prefix| prefix.first_fragment_overlong)
        {
            // P1391 — a palavra overlong fecha-se dentro do próprio layout de
            // texto, antes de Sequence alcançar `wrap_before`. Armar o próximo
            // flush preserva o ascent/leading exterior nesse fechamento único.
            layouter.arm_wrapped_inline_prefix_line();
        }
        if inline_prefix.is_some_and(|prefix| prefix.wrap_before == Some(part_index))
            && super::html_frame::items_have_material(
                &layouter.regions.current.current_line,
            )
        {
            layouter.flush_wrapped_inline_prefix_line();
        }
        if let Some(adjustment) = inline_prefix
            .filter(|prefix| prefix.wrap_before == Some(part_index))
            .map(|prefix| prefix.second_smaller_baseline_adjustment)
            .filter(|adjustment| *adjustment > 0.0)
        {
            layouter.regions.current.cursor_y += Pt(-adjustment);
        }
        if inline_prefix.is_some_and(|prefix| {
            prefix.wrap_before.is_some_and(|index| part_index > index)
                && part_index < prefix.block_index
                && matches!(
                    part,
                    Content::Space | Content::SpaceFromNewline | Content::Empty
                )
        }) && super::html_frame::items_have_material(
            &layouter.regions.current.current_line,
        ) {
            layouter.flush_wrapped_inline_prefix_line();
        }
        let geometric_prefix = inline_prefix
            .is_some_and(|prefix| prefix.geometric && prefix.block_index == part_index);
        let ordinary_boundary =
            matches!(part.source_body(), Content::Block(_) | Content::Align(_))
                && inline_prefix.is_none_or(|prefix| prefix.block_index != part_index);
        if geometric_prefix || ordinary_boundary {
            if owns_text_segment {
                super::paragraph::close_segment(
                    layouter,
                    &parts[text_segment_start..part_index],
                );
            } else if let Some(chain) = layouter.text_segment_chain.clone() {
                super::paragraph::close_in_chain(layouter, chain, true);
            }
            if inline_prefix.is_some_and(|p| p.multiline && p.block_index == part_index) {
                layouter.geometric_text_lines = saved_geometric_lines;
            }
        }
        if inline_prefix
            .is_some_and(|prefix| !prefix.geometric && prefix.block_index == part_index)
        {
            // P1388 — o whitespace terminal pode já ter fechado a linha.
            // A boundary pertence ao prefixo confirmado, não ao estado
            // incidental de `current_line`, e deve ser aplicada uma vez.
            if super::html_frame::items_have_material(
                &layouter.regions.current.current_line,
            ) {
                if inline_prefix.is_some_and(|prefix| prefix.wrap_before.is_some()) {
                    layouter.flush_wrapped_inline_prefix_line();
                } else {
                    layouter.flush_line();
                }
            }
            layouter.regions.current.cursor_y +=
                Pt(inline_prefix.map(|prefix| prefix.block_boundary).unwrap_or(0.0));
        }

        layouter.fractional_block_height = match (distribution, fractional_block(part)) {
            (Some((remaining, total, _)), Some(block)) => match block.height {
                Some(BlockHeight::Fractional(weight)) => {
                    fractional_share(remaining, weight, total)
                }
                _ => None,
            },
            (None, _) => saved_fractional_height,
            _ => None,
        };
        if distribution.is_some() && Some(part_index) == terminal_flow_index {
            layouter.fractional_flow_tail_pending = true;
        }
        if distribution.is_some() && part_index > 0 {
            if let Some(previous) = fractional_block(&parts[part_index - 1]) {
                if let Some(chain) =
                    super::paragraph::shared_chain(&parts[part_index..], &layouter.chain)
                {
                    layouter.fractional_flow_tail_pending = true;
                    if previous.below.is_none() && previous.spacing.is_none() {
                        layouter.prev_block_below_pending = layouter
                            .prev_block_below_pending
                            .max(super::paragraph::spacing(&chain));
                    }
                }
            }
        }
        let planning_guard = layouter.paragraph_planning_suppressed;
        let before_flow_break = parts[part_index + 1..]
            .iter()
            .find(|part| {
                !matches!(
                    part,
                    Content::Empty | Content::Space | Content::SpaceFromNewline
                )
            })
            .is_some_and(|part| {
                matches!(part, Content::Pagebreak(_) | Content::Parbreak)
            });
        if (terminal_flow_index == Some(part_index) || before_flow_break)
            && !super::html_frame::items_have_material(
                &layouter.regions.current.current_line,
            )
        {
            layouter.paragraph_planning_suppressed = false;
        }
        layouter.layout_content(part);
        layouter.paragraph_planning_suppressed = planning_guard;
        layouter.fractional_block_height = saved_fractional_height;

        // **P1107** — preservação da cadeia de colapso de margens:
        // Apenas elementos de bloco (Block, Shape, Parbreak, Heading, Equation)
        // e nós transparentes (Space, Empty quando fora de linha aberta)
        // mantêm o estado de colapso de margens.
        // **P1107/P1108** — Preservação da cadeia de colapso de margens:
        // A cadeia só é interrompida quando uma linha de texto inline aberta é populada.
        if super::html_frame::items_have_material(&layouter.regions.current.current_line)
        {
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
                | Content::SpaceFromNewline
                | Content::Space
                | Content::Parbreak
                | Content::Empty
                | Content::Styled(..)
        ) {
            layouter.enum_counter = None;
        }
        if super::paragraph::shared_chain(std::slice::from_ref(part), &layouter.chain)
            .is_none()
        {
            text_segment_start = part_index + 1;
            layouter.text_segment_chain = Some(layouter.chain.clone());
            owns_text_segment = !super::html_frame::items_have_causal_line(
                &layouter.regions.current.current_line,
            );
        }
        part_index += 1;
    }

    layouter.text_segment_chain = saved_text_segment;
    layouter.geometric_text_lines = saved_geometric_lines;

    // Preservar estado da cadeia anterior se a sequência foi composta exclusivamente por nós transparentes/estilos
    let is_transparent = parts.iter().all(|p| {
        matches!(
            p,
            Content::Empty
                | Content::Styled(..)
                | Content::Space
                | Content::SpaceFromNewline
                | Content::Parbreak
        )
    });
    if is_transparent && saved_chain {
        layouter.prev_block_below_pending = saved_below;
        layouter.block_chain_active = saved_chain;
        layouter.prev_block_equation_descent = saved_descent;
    }
    if distribution.is_some()
        && super::html_frame::items_have_material(&layouter.regions.current.current_line)
    {
        layouter.fractional_flow_tail_pending = true;
    }
    layouter.fractional_block_height = saved_fractional_height;
    if super::html_frame::items_have_material(&layouter.regions.current.current_line) {
        if let Some(dir) = layouter.active_paragraph_dir {
            layouter.pending_line_dir = Some(dir);
        }
    }
    layouter.active_paragraph_dir = enclosing_dir;
    if !enclosing_scope {
        layouter.bidi_cursor = None;
    }
    layouter.bidi_scope_active = enclosing_scope;
}

/// **P864** — devolve o grupo de item estrutural reconhecido, se aplicável.
fn item_group(content: &Content) -> Option<ItemGroup> {
    match content.source_body() {
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

#[cfg(test)]
mod p1374_tests {
    use super::*;

    #[test]
    fn p1374_fractional_share_is_proportional_and_zero_safe() {
        assert_eq!(fractional_share(186.0, 1.0, 3.0), Some(62.0));
        assert_eq!(fractional_share(186.0, 2.0, 3.0), Some(124.0));
        assert_eq!(fractional_share(186.0, 0.0, 0.0), Some(0.0));
        assert_eq!(fractional_share(186.0, 1.0, 0.0), None);
    }
}

#[cfg(test)]
mod p1375_tests {
    use std::sync::Arc;

    use crate::entities::{
        content::Content,
        corners::Corners,
        elements::block::{BlockElem, BlockHeight},
        layout_types::Length,
        sides::Sides,
        style::Styles,
    };

    use super::fractional_block;

    fn fractional(weight: f64) -> Content {
        Content::Block(Arc::new(BlockElem {
            body: Content::text("body"),
            width: None,
            height: Some(BlockHeight::Fractional(weight)),
            inset: Sides::uniform(Length::ZERO),
            breakable: true,
            outset: Sides::uniform(Length::ZERO),
            radius: Corners::uniform(Length::ZERO),
            clip: false,
            fill: None,
            stroke: None,
            spacing: None,
            above: None,
            below: None,
            sticky: false,
        }))
    }

    #[test]
    fn p1375_styled_singleton_exposes_fractional_block() {
        for space in [Content::Space, Content::SpaceFromNewline] {
            let wrapped = Content::styled(
                Box::new(Content::sequence(vec![space, fractional(1.0)])),
                Styles::new(),
            );
            assert!(matches!(
                fractional_block(&wrapped).and_then(|block| block.height),
                Some(BlockHeight::Fractional(1.0))
            ));
        }
    }

    #[test]
    fn p1375_styled_ambiguous_sequence_fails_closed() {
        let wrapped = Content::styled(
            Box::new(Content::sequence(vec![fractional(1.0), fractional(2.0)])),
            Styles::new(),
        );
        assert!(fractional_block(&wrapped).is_none());
    }
}

#[cfg(test)]
mod p1376_tests {
    use super::fractional_region_remaining;

    #[test]
    fn p1376_continuous_column_uses_cursor_relative_region() {
        assert!(
            (fractional_region_remaining(220.0, 230.0, 10.0, true) - 220.0).abs() < 1e-9
        );
        assert!(
            (fractional_region_remaining(220.0, 230.0, 30.438, true) - 199.562).abs()
                < 1e-9
        );
        assert_eq!(fractional_region_remaining(220.0, 230.0, 30.438, false), 220.0);
    }
}

#[cfg(test)]
mod p1380_tests {
    use crate::entities::style::{Style, Styles};

    use super::{p1380_inline_safe_styles, p1380_is_unwrapped, p1389_single_word};

    #[test]
    fn p1380_wrap_permanece_opaco_e_p1381_delega_size_ao_prefixo() {
        assert!(p1380_is_unwrapped(85.0, 85.0));
        assert!(!p1380_is_unwrapped(85.001, 85.0));
        assert!(!p1380_is_unwrapped(f64::INFINITY, 85.0));

        assert!(p1380_inline_safe_styles(&Styles::from_iter([Style::strong()])));
        assert!(p1380_inline_safe_styles(&Styles::from_iter([Style::Size(
            crate::entities::layout_types::Length::pt(8.0)
        )])));
    }

    #[test]
    fn p1389_word_unico_rejeita_whitespace_interno() {
        assert!(p1389_single_word("More"));
        assert!(!p1389_single_word("More text"));
        assert!(!p1389_single_word("More\ntext"));
        assert!(!p1389_single_word(""));
    }
}
