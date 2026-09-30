//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/paragraph_pagination.md
//! @prompt-hash 76f1fc67
//! @layer L1
//! @updated 2026-09-28

use super::{paragraph_prepare::Prepared, FontMetrics, ImageSizer, Layouter};
use crate::entities::layout_types::PageMargins;

#[derive(Clone, Copy)]
pub(super) struct Profile {
    top: f64,
    descent: f64,
    leading: f64,
    widow: bool,
    orphan: bool,
}

/// Certify all vertical geometry before emitting the first line.
pub(super) fn certify<M: FontMetrics, S: ImageSizer>(
    l: &Layouter<M, S>,
    parts: &[&Prepared],
) -> Option<Profile> {
    if !l.page_config.height.is_finite()
        || l.column_mode
        || l.page_columns.is_some()
        || l.regions.cell.is_some()
        || l.is_sub_frame
        || !l.floats_pending.is_empty()
        || !l.pending_footnote_bodies.is_empty()
        || l.flow_suffix_transaction_active
        || l.regions.current.height != l.page_config.height
    {
        return None;
    }
    let (top, bottom) = l.metrics.text_edges(l.style.size, &l.style);
    let descent = -bottom.0;
    let mut leading = None;
    for part in parts {
        if part.words.is_empty() {
            return None;
        }
        for f in part.words.iter().flat_map(|w| &w.fragments) {
            let edges = l.metrics.text_edges(f.style.size, &f.style);
            let gap = f
                .style
                .leading
                .map(|v| v.resolve_pt(f.style.size.0))
                .unwrap_or(f.style.size.0 * super::vanilla_defaults::PAR_LEADING);
            if f.style.size != l.style.size
                || edges != (top, bottom)
                || f.style.baseline_offset.resolve_pt(f.style.size.0) != 0.0
                || leading.is_some_and(|before| before != gap)
            {
                return None;
            }
            leading = Some(gap);
        }
    }
    let leading = leading?;
    if [top.0, descent, leading].iter().any(|v| !v.is_finite() || *v < 0.0) {
        return None;
    }
    let (widow, orphan) = crate::compiler::text_costs::prevention(&l.chain);
    Some(Profile { top: top.0, descent, leading, widow, orphan })
}

fn group_size(index: usize, len: usize, widow: bool, orphan: bool) -> usize {
    if index == 0 && len == 3 && widow && orphan {
        3
    } else if index == 0 && len >= 2 && orphan {
        2
    } else if index >= 2 && index + 2 == len && widow {
        2
    } else {
        1
    }
}

/// Admit a selected line and its protected following lines, never hypothetical text.
pub(super) fn admit<M: FontMetrics, S: ImageSizer>(
    l: &mut Layouter<M, S>,
    profile: Option<Profile>,
    index: usize,
    len: usize,
) {
    let Some(profile) = profile else { return };
    l.ensure_initial_baseline();
    let group = group_size(index, len, profile.widow, profile.orphan);
    let tail = (group - 1) as f64 * (profile.top + profile.descent + profile.leading)
        + profile.descent;
    let next_page = std::num::NonZeroUsize::new(l.pages.len() + 2).unwrap();
    let margin = PageMargins::resolve_for_page(
        l.page_config.margin_spec,
        l.page_config.auto_margin(),
        l.page_config.binding.resolve(l.page_config.page_dir),
        next_page,
    );
    let next_bottom = l.page_config.height - margin.bottom;
    let next_baseline = margin.top + profile.top;
    if !l.current_page_is_empty()
        && l.regions.current.cursor_y.0 + tail > l.page_bottom_limit()
        && next_baseline + tail <= next_bottom
    {
        l.new_page();
    }
}
