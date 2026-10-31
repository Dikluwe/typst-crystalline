//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/h_space.md
//! @prompt-hash 98907595
//! @layer L1
//! @updated 2026-06-19
//!
//! Atomização (ADR-0109, P380): o layout de `HSpace` movido do monólito
//! `layout_content` para o arquivo da feature (forma B). Content-preserving.

use crate::entities::elements::h_space::HSpaceElem;
use crate::entities::layout_types::Pt;
use crate::entities::style_chain::StyleChain;

use super::{FontMetrics, ImageSizer, Layouter};

/// Original lexical anchor plus its monotonically narrowing common ancestor.
/// Equal style values in distinct siblings do not identify a shared wrapper.
#[derive(Clone)]
pub(super) struct Paragraph {
    anchor: StyleChain,
    shared_depth: usize,
}

impl Paragraph {
    fn include(&mut self, chain: &StyleChain) {
        let depth = self
            .anchor
            .raw_deltas()
            .skip(self.shared_depth)
            .position(|a| chain.raw_deltas().any(|b| std::ptr::eq(a, b)));
        self.shared_depth = depth.map_or_else(
            || self.anchor.raw_deltas().count(),
            |offset| self.shared_depth + offset,
        );
    }

    fn spacing(&self) -> f64 {
        if self.shared_depth == 0 {
            return super::paragraph::spacing(&self.anchor);
        }
        let suffix: Vec<_> =
            self.anchor.raw_deltas().skip(self.shared_depth).cloned().collect();
        let chain = suffix
            .into_iter()
            .rev()
            .fold(StyleChain::empty(), |chain, delta| chain.push(delta));
        super::paragraph::spacing(&chain)
    }
}

/// Layout de `h(...)` (espaço horizontal).
///
/// `Absolute`: avança o cursor_x imediatamente. `Fractional` (**P842,
/// achado #38**): regista a fração em `pending_fr` da região — a expansão
/// acontece no `flush_line`/`finish`, quando o espaço restante da linha é
/// conhecido (paridade vanilla `HElem` com `Spacing::Fractional`,
/// `layout/spacing.rs`). `weak` diferido.
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e: &HSpaceElem,
) {
    // Explicit spacing is language content even when its extent is zero.
    if !layouter.is_sub_frame {
        layouter.page_has_unpainted_material = true;
    }
    if !layouter.is_sub_frame
        && !super::html_frame::items_have_causal_line(
            &layouter.regions.current.current_line,
        )
    {
        if let Some(paragraph) = &mut layouter.pending_horizontal_paragraph {
            paragraph.include(&layouter.chain);
        } else {
            layouter.pending_horizontal_paragraph =
                Some(Paragraph { anchor: layouter.chain.clone(), shared_depth: 0 });
        }
    }
    match e.amount {
        crate::entities::elements::h_space::Spacing::Absolute(l) => {
            let pt = l.resolve_pt(layouter.style.size.val());
            layouter.regions.current.cursor_x += Pt(pt);
            layouter.record_flow_width_at(layouter.regions.current.cursor_x.0, 0.0);
        }
        crate::entities::elements::h_space::Spacing::Fractional(fr) => {
            if let Some(width) = super::sequence::inline_allocation::assigned(
                layouter.inline_allocation.take(),
            ) {
                layouter.regions.current.cursor_x += Pt(width);
                layouter.record_flow_width_at(layouter.regions.current.cursor_x.0, 0.0);
            } else {
                let insert_at = layouter.regions.current.current_line.len();
                layouter.regions.current.pending_fr.push((insert_at, fr));
                layouter.pending_fr_natural_widths.push(0.0);
            }
        }
    }
}

/// Close a real horizontal-only paragraph without manufacturing painted items.
/// A causal text/inline line absorbs the producer through ordinary closure.
pub(super) fn close_paragraph<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
) -> bool {
    let Some(paragraph) = layouter.pending_horizontal_paragraph.take() else {
        return false;
    };
    if layouter.is_sub_frame
        || super::html_frame::items_have_causal_line(
            &layouter.regions.current.current_line,
        )
    {
        return false;
    }
    let spacing = paragraph.spacing();
    let mut end = if layouter.initial_baseline_pending {
        layouter.regions.current.cursor_y.0
    } else if layouter.block_chain_active {
        let gap = if layouter.prev_margin_is_parbreak {
            layouter.prev_block_below_pending.max(spacing)
        } else {
            layouter.prev_block_below_pending
        };
        layouter.prev_line_baseline + layouter.prev_block_equation_descent + gap
    } else {
        layouter
            .flow_placement_y()
            .unwrap_or(layouter.regions.current.cursor_y.0)
    };
    // The zero-height frame is real material; its preceding gap must fit.
    // Test physical progress, including hidden blocks, rather than paint alone.
    if end > layouter.page_bottom_limit() && end > layouter.page_config.margin.top {
        if layouter.flow_suffix_transaction_active {
            layouter.flow_suffix_transaction_rejected = true;
            return true;
        }
        layouter.new_page();
        let top = if layouter.initial_baseline_pending {
            0.0
        } else {
            layouter.metrics.text_edges(layouter.style.size, &layouter.style).0 .0
        };
        end = layouter.regions.current.cursor_y.0 - top;
        layouter.last_block_descent_y = None;
        layouter.page_has_unpainted_material = true;
    }
    layouter.regions.current.cursor_y = Pt(end + spacing);
    layouter.regions.current.cursor_x = layouter.regions.current.line_start_x;
    layouter.prev_line_baseline = end;
    layouter.prev_block_equation_descent = 0.0;
    layouter.prev_block_below_pending = spacing;
    layouter.last_block_descent_y =
        Some(layouter.last_block_descent_y.map_or(end, |old| old.max(end)));
    layouter.initial_baseline_pending = false;
    layouter.prev_margin_is_parbreak = true;
    layouter.block_chain_active = true;
    layouter.last_flush_advance = 0.0;
    layouter.terminal_paragraph_line = false;
    layouter.record_flow_placement_end(end);
    true
}

#[cfg(test)]
mod tests {
    use super::Paragraph;
    use crate::entities::{
        layout_types::Length,
        style_chain::{StyleChain, StyleDelta},
        value::Value,
    };

    #[test]
    fn horizontal_scope_keeps_ancestor_custom_spacing_and_never_rejoins_siblings() {
        let root = StyleChain::default_chain();
        let parent = root.push_custom("par.spacing", Value::Length(Length::pt(9.0)));
        let child = || {
            parent.push(StyleDelta {
                size: Some(Length::pt(20.0)),
                ..StyleDelta::empty()
            })
        };
        let mut paragraph = Paragraph { anchor: child(), shared_depth: 0 };
        paragraph.include(&child());
        assert_eq!(paragraph.spacing(), 9.0);
        // A later outer producer permanently narrows the paragraph scope.
        paragraph.include(&root);
        paragraph.include(&child());
        assert!((paragraph.spacing() - 13.2).abs() < 1e-9);
        // Independent roots have no lexical custom paragraph wrapper in common.
        paragraph.include(&StyleChain::default_chain());
        assert!((paragraph.spacing() - 13.2).abs() < 1e-9);
    }
}
