from pathlib import Path
f=Path('01_core/src/compiler/layout/block.rs');s=f.read_text()
old='''        let gap = if layouter.prev_margin_is_parbreak && has_custom_above {
            above_pt
        } else {
            layouter.prev_block_below_pending.max(above_pt)
        };'''
assert s.count(old)==1
s=s.replace(old,'''        let gap = collapse_margin(
            layouter.prev_block_below_pending,
            layouter.prev_margin_is_parbreak,
            above_pt,
            !has_custom_above,
        );''')
pos=s.index('/// Layout do container')
s=s[:pos]+'''/// Resolve causal margin priority before comparing their magnitudes.
pub(super) fn collapse_margin(
    previous: f64,
    previous_is_auto: bool,
    incoming: f64,
    incoming_is_auto: bool,
) -> f64 {
    if previous_is_auto == incoming_is_auto {
        previous.max(incoming)
    } else if previous_is_auto {
        incoming
    } else {
        previous
    }
}

'''+s[pos:];f.write_text(s)
f=Path('01_core/src/compiler/layout/heading.rs');s=f.read_text();old='let gap = layouter.prev_block_below_pending.max(above_pt);';assert s.count(old)==1
s=s.replace(old,'''let gap = super::block::collapse_margin(
            layouter.prev_block_below_pending,
            layouter.prev_margin_is_parbreak,
            above_pt,
            false,
        );''');f.write_text(s)
