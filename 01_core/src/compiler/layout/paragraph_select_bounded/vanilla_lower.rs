//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/paragraph_select_vanilla_lower.md
//! @prompt-hash 9165511b
//! @layer L1
//! @updated 2026-10-02

use super::Search;
use crate::compiler::layout::paragraph_select::{ratio, Line};

pub(super) fn run(search: &mut Search<'_>) {
    for end in 1..=search.n {
        let old_active = search.active;
        let mut line_lower_bound = None;
        for start in old_active..end {
            let line = Line { start, end };
            let measure = search.measure(line);
            let r = ratio(measure, search.width, search.font);
            if start == search.active && r < search.min_ratio() {
                search.active += 1;
            }
            if search.totals[start] > search.upper_bound
                || (search.monotone
                    && line_lower_bound.is_some_and(|lower| {
                        search.totals[start] + lower > search.upper_bound
                    }))
            {
                search.pruned += 1;
                continue;
            }
            search.evaluated += 1;
            let current_cost = search.price(line, measure);
            let total = search.totals[start] + current_cost;
            if search.monotone && end < search.n && r > 0.0 && line_lower_bound.is_none()
            {
                line_lower_bound = Some(current_cost);
            }
            if total <= search.totals[end] {
                search.totals[end] = total;
                search.predecessors[end] = start;
            }
        }
    }
}
