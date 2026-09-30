//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/paragraph_select.md
//! @prompt-hash da32b838
//! @layer L1
//! @updated 2026-09-30

use super::paragraph_prepare::Prepared;
use crate::compiler::text_costs::LineCosts;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Line {
    pub start: usize,
    pub end: usize,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Strategy {
    Simple,
    Optimized,
}
#[derive(Default, Clone, Copy)]
pub(super) struct Measure {
    pub width: f64,
    pub spaces: f64,
    pub count: usize,
}
/// Um intervalo precede cada gap salvo o primeiro literal do parágrafo.
pub(super) fn before_gap(p: &Prepared, line: Line, word: usize, gap: usize) -> f64 {
    if line.start == 0 && word == 0 && gap == 0 { 0.0 } else { p.tracking() }
}

/// A palavra após gaps na mesma linha recebe o intervalo terminal desses gaps.
pub(super) fn before_word(p: &Prepared, line: Line, word: usize) -> f64 {
    if (word > line.start || line.start == 0) && !p.words[word].before.is_empty() {
        p.tracking()
    } else { 0.0 }
}

/// O espaço da quebra é moldado, mesmo que seu avanço seja zerado.
fn terminal_tracking(p: &Prepared, line: Line) -> f64 {
    if line.end < p.words.len() && !p.words[line.end].before.is_empty() {
        p.tracking()
    } else {
        0.0
    }
}

pub(super) fn measure(p: &Prepared, line: Line) -> Measure {
    let mut m = Measure::default();
    for i in line.start..line.end {
        m.width += p.words[i].width;
        if i > line.start || line.start == 0 {
            for (gap_index, gap) in p.words[i].before.iter().enumerate() {
                m.width += gap.width + before_gap(p, line, i, gap_index);
                m.spaces += gap.width;
                m.count += 1;
            }
            m.width += before_word(p, line, i);
        }
    }
    m.width += terminal_tracking(p, line);
    m
}
pub(super) fn ratio(m: Measure, width: f64, font: f64) -> f64 {
    let mut delta = width - m.width;
    if delta.abs() < 0.001 {
        delta = 0.0;
    }
    let adjustment = m.spaces * if delta >= 0.0 { 0.5 } else { 1.0 / 3.0 };
    let mut r = delta / adjustment.max(0.0);
    if r.is_nan() {
        r = 0.0;
    }
    if r > 1.0 {
        r = 1.0 + (delta - adjustment) / (m.count.max(1) as f64) / (font / 2.0);
    }
    r.clamp(-2.0, 10.0)
}
fn cost(m: Measure, line: Line, width: f64, font: f64, justify: bool, last: bool, costs: LineCosts) -> f64 {
    line_cost(m, width, font, justify, last, line.end == line.start + 1, 0.0, costs)
}
pub(super) fn line_cost(
    m: Measure,
    width: f64,
    font: f64,
    justify: bool,
    last: bool,
    unbreakable: bool,
    penalty: f64,
    costs: LineCosts,
) -> f64 {
    let r = ratio(m, width, font);
    let badness = if r < if justify { -1.0 } else { 0.0 } {
        1_000_000.0
    } else if !last || r < 0.0 {
        100.0 * r.abs().powi(3)
    } else {
        0.0
    };
    let runt = if last && unbreakable { costs.runt } else { 0.0 };
    (1.0 + badness + runt + penalty).powi(2)
}
pub(super) fn select(
    p: &Prepared,
    width: f64,
    font: f64,
    justify: bool,
    strategy: Strategy,
    costs: LineCosts,
) -> Vec<Line> {
    let n = p.words.len();
    if strategy == Strategy::Simple {
        let mut lines = Vec::new();
        let mut start = 0;
        while start < n {
            let mut end = start + 1;
            while end < n && measure(p, Line { start, end: end + 1 }).width <= width {
                end += 1;
            }
            lines.push(Line { start, end });
            start = end;
        }
        return lines;
    }
    let mut prefix = vec![Measure::default(); n + 1];
    let before: Vec<Measure> = p
        .words
        .iter()
        .enumerate()
        .map(|(index, w)| {
            let base = w.before.iter().map(|g| g.width).sum::<f64>();
            let count = w.before.len();
            let intervals = if index == 0 { count } else if count > 0 { count + 1 } else { 0 };
            Measure { width: base + p.tracking() * intervals as f64, spaces: base, count }
        })
        .collect();
    for i in 0..n {
        prefix[i + 1] = Measure {
            width: prefix[i].width + p.words[i].width + before[i].width,
            spaces: prefix[i].spaces + before[i].spaces,
            count: prefix[i].count + before[i].count,
        };
    }
    let measured = |line: Line| {
        let trim = if line.start > 0 { before[line.start] } else { Measure::default() };
        Measure {
            width: prefix[line.end].width - prefix[line.start].width - trim.width
                + terminal_tracking(p, line),
            spaces: prefix[line.end].spaces - prefix[line.start].spaces - trim.spaces,
            count: prefix[line.end].count - prefix[line.start].count - trim.count,
        }
    };
    let mut active = 0;
    let mut totals = vec![f64::INFINITY; n + 1];
    let mut predecessors = vec![0; n + 1];
    totals[0] = 0.0;
    // Exhaustive exact DP: the observable cost is shared with the pinned source,
    // its approximate bounds and buffer mechanics are not architecture contracts.
    for end in 1..=n {
        for start in active..end {
            let line = Line { start, end };
            let m = measured(line);
            if start == active && ratio(m, width, font) < if justify { -1.0 } else { 0.0 }
            {
                active += 1;
            }
            let total = totals[start] + cost(m, line, width, font, justify, end == n, costs);
            if total <= totals[end] {
                totals[end] = total;
                predecessors[end] = start;
            }
        }
    }
    let mut lines = Vec::new();
    let mut end = n;
    while end > 0 {
        let start = predecessors[end];
        lines.push(Line { start, end });
        end = start;
    }
    lines.reverse();
    lines
}

#[cfg(test)]
mod tests {
    use super::super::paragraph_prepare::{Gap, Word};
    use super::*;
    use crate::entities::layout_types::{Pt, TextSeparatorOrigin, TextStyle};
    fn prepared(widths: &[f64], gap: f64) -> Prepared {
        Prepared {
            words: widths
                .iter()
                .enumerate()
                .map(|(i, &width)| Word {
                    width,
                    hyphenation: None,
                    fragments: Vec::new(),
                    before: if i == 0 {
                        Vec::new()
                    } else {
                        vec![Gap {
                            width: gap,
                            style: TextStyle::regular(Pt(12.0)),
                            origin: TextSeparatorOrigin::Markup,
                        }]
                    },
                })
                .collect(),
            trailing: Vec::new(),
            right_overhang: 0.0,
        }
    }
    #[test]
    fn simple_break_includes_tracking_to_trimmed_space() {
        use super::super::paragraph_prepare::Fragment;
        use crate::entities::layout_types::Length;
        let mut p = prepared(&[60.0, 63.536, 5.0], 0.0);
        let mut style = TextStyle::regular(Pt(12.0));
        style.tracking = Some(Length::pt(0.2));
        for (word, &width) in p.words.iter_mut().zip([60.0, 63.536, 5.0].iter()) {
            word.fragments.push(Fragment {
                text: "x".into(), replacement: None, style: style.clone(), width,
                hyphenate: false, overhang: false,
            });
            for gap in &mut word.before { gap.style = style.clone(); }
        }
        let candidate = Line { start: 0, end: 2 };
        assert!((measure(&p, candidate).width - 124.136).abs() < 1e-9);
        assert_eq!(select(&p, 124.0, 12.0, false, Strategy::Simple, LineCosts::default())[0],
            Line { start: 0, end: 1 });
    }
    #[test]
    fn selector_balances_complete_paragraph_and_can_shrink() {
        let p = prepared(&[30.0, 30.0, 30.0, 30.0, 30.0, 30.0, 10.0, 10.0], 6.0);
        let simple = select(&p, 100.0, 12.0, true, Strategy::Simple, LineCosts::default());
        let opt = select(&p, 100.0, 12.0, true, Strategy::Optimized, LineCosts::default());
        assert_eq!(simple[0], Line { start: 0, end: 2 });
        assert!(opt.iter().any(|&line| measure(&p, line).width > 100.0));
        assert!(opt.iter().all(|&line| {
            let m = measure(&p, line);
            m.width - m.spaces / 3.0 <= 100.0
        }));
        assert!(
            cost(measure(&p, opt[0]), opt[0], 100.0, 12.0, true, false, LineCosts::default())
                < cost(measure(&p, simple[0]), simple[0], 100.0, 12.0, true, false, LineCosts::default())
        );
    }
    #[test]
    fn selector_unbreakable_overlong_progresses_without_empty_lines() {
        let p = prepared(&[300.0, 20.0, 20.0], 3.0);
        for strategy in [Strategy::Simple, Strategy::Optimized] {
            let lines = select(&p, 50.0, 12.0, false, strategy, LineCosts::default());
            assert_eq!(lines[0], Line { start: 0, end: 1 });
            assert_eq!(lines.last().unwrap().end, 3);
            assert!(lines.iter().all(|line| line.start < line.end));
            assert!(lines.windows(2).all(|p| p[0].end == p[1].start));
        }
    }
    #[test]
    fn selector_final_natural_line_has_no_underfull_badness() {
        let line = Line { start: 0, end: 2 };
        let m = Measure { width: 30.0, spaces: 3.0, count: 1 };
        assert_eq!(cost(m, line, 100.0, 12.0, false, true, LineCosts::default()), 1.0);
        assert!(cost(m, line, 100.0, 12.0, false, false, LineCosts::default()) > 1.0);
        assert_eq!(ratio(Measure::default(), 0.0, 12.0), 0.0);
    }
}

#[cfg(test)]
mod final_line_cost_regression {
    use super::*;
    #[test]
    fn hardcode_final_line_preserves_natural_cost() {
        let m = Measure { width: 20.0, spaces: 10.0, count: 1 };
        assert_eq!(line_cost(m, 25.0, 12.0, false, true, false, 0.0, LineCosts::default()), 1.0);
        assert_eq!(line_cost(m, 25.0, 12.0, true, true, false, 0.0, LineCosts::default()), 1.0);
        assert_eq!(line_cost(m, 25.0, 12.0, true, false, false, 0.0, LineCosts::default()), 10201.0);
    }
}

#[cfg(test)]
mod configurable_runt_cost {
    use super::*;
    #[test]
    fn custom_costs_runt_multiplier_changes_only_penalty() {
        let m=Measure{width:20.0,spaces:0.0,count:0};
        assert_eq!(line_cost(m,25.0,12.0,false,true,true,0.0,LineCosts::default()),10201.0);
        assert_eq!(line_cost(m,25.0,12.0,false,true,true,0.0,LineCosts{runt:0.0,..LineCosts::default()}),1.0);
        assert_eq!(line_cost(m,25.0,12.0,false,true,true,0.0,LineCosts{runt:200.0,..LineCosts::default()}),40401.0);
    }
}
