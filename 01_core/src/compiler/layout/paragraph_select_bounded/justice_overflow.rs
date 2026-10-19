//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/paragraph_select_bounded/justice_overflow.md
//! @prompt-hash 8885e2ff
//! @layer L1
//! @updated 2026-10-02

use super::Search;
use crate::compiler::layout::paragraph_select::{line_cost, ratio, Line};

pub(super) fn run(search: &mut Search<'_>) {
    for end in 1..=search.n {
        let old_active = search.active;
        for start in (old_active..end).rev() {
            let line = Line { start, end };
            let measure = search.measure(line);
            let r = ratio(measure, search.width, search.font);
            let line_price = search.price(line, measure);
            if search.totals[start] > search.upper_bound {
                search.pruned += 1;
            } else {
                search.evaluated += 1;
                let total = search.totals[start] + line_price;
                if total < search.totals[end] {
                    search.totals[end] = total;
                    search.predecessors[end] = start;
                }
            }
            let overflow_floor = if end == search.n && start + 1 == end {
                line_cost(
                    measure,
                    search.width,
                    search.font,
                    search.justify,
                    true,
                    false,
                    0.0,
                    search.costs,
                )
            } else {
                line_price
            };
            if search.monotone
                && r < search.min_ratio()
                && overflow_floor >= search.totals[end]
            {
                search.pruned += start - old_active;
                break;
            }
        }
        while search.active < end
            && ratio(
                search.measure(Line { start: search.active, end }),
                search.width,
                search.font,
            ) < search.min_ratio()
        {
            search.active += 1;
        }
    }
}
