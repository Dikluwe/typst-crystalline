//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/paragraph_select_bounded.md
//! @prompt-hash e66dad7b
//! @layer L1
//! @updated 2026-09-30

use super::paragraph_prepare::Prepared;
use super::paragraph_select::{self, line_cost, ratio, Line, Measure, Strategy};
use crate::compiler::text_costs::LineCosts;

#[derive(Debug)]
struct Trial {
    lines: Vec<Line>,
    evaluated: usize,
    pruned: usize,
    upper_bound: f64,
}

/// Experimental upper-bound pruning. This module is compiled only by tests.
fn select_bounded(
    p: &Prepared,
    width: f64,
    font: f64,
    justify: bool,
    costs: LineCosts,
) -> Trial {
    let n = p.words.len();
    if n == 0 {
        return Trial {
            lines: Vec::new(),
            evaluated: 0,
            pruned: 0,
            upper_bound: 0.0,
        };
    }
    let mut prefix = vec![Measure::default(); n + 1];
    let before: Vec<Measure> = p
        .words
        .iter()
        .enumerate()
        .map(|(index, word)| {
            let base = word.before.iter().map(|gap| gap.width).sum::<f64>();
            let count = word.before.len();
            let intervals = if index == 0 {
                count
            } else if count > 0 {
                count + 1
            } else {
                0
            };
            Measure {
                width: base + p.tracking() * intervals as f64,
                spaces: base,
                count,
            }
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
        let terminal = if line.end < n && !p.words[line.end].before.is_empty() {
            p.tracking()
        } else {
            0.0
        };
        Measure {
            width: prefix[line.end].width - prefix[line.start].width - trim.width
                + terminal,
            spaces: prefix[line.end].spaces - prefix[line.start].spaces - trim.spaces,
            count: prefix[line.end].count - prefix[line.start].count - trim.count,
        }
    };
    let score = |line: Line| {
        line_cost(
            measured(line),
            width,
            font,
            justify,
            line.end == n,
            line.end == line.start + 1,
            0.0,
            costs,
        )
    };
    let greedy =
        paragraph_select::select(p, width, font, justify, Strategy::Simple, costs);
    let upper_bound: f64 = greedy.into_iter().map(score).sum();
    if !upper_bound.is_finite() {
        return Trial {
            lines: paragraph_select::select(
                p,
                width,
                font,
                justify,
                Strategy::Optimized,
                costs,
            ),
            evaluated: 0,
            pruned: 0,
            upper_bound,
        };
    }
    let mut active = 0;
    let mut totals = vec![f64::INFINITY; n + 1];
    let mut predecessors = vec![0; n + 1];
    let (mut evaluated, mut pruned) = (0, 0);
    totals[0] = 0.0;
    for end in 1..=n {
        for start in active..end {
            // Keep the baseline's overfull retirement before considering the bound.
            let line = Line { start, end };
            let m = measured(line);
            if start == active && ratio(m, width, font) < if justify { -1.0 } else { 0.0 }
            {
                active += 1;
            }
            if totals[start] > upper_bound {
                pruned += 1;
                continue;
            }
            evaluated += 1;
            let total = totals[start]
                + line_cost(
                    m,
                    width,
                    font,
                    justify,
                    end == n,
                    end == start + 1,
                    0.0,
                    costs,
                );
            if total <= totals[end] {
                totals[end] = total;
                predecessors[end] = start;
            }
        }
    }
    if !totals[n].is_finite() {
        return Trial {
            lines: paragraph_select::select(
                p,
                width,
                font,
                justify,
                Strategy::Optimized,
                costs,
            ),
            evaluated,
            pruned,
            upper_bound,
        };
    }
    let mut lines = Vec::new();
    let mut end = n;
    while end > 0 {
        let start = predecessors[end];
        lines.push(Line { start, end });
        end = start;
    }
    lines.reverse();
    Trial { lines, evaluated, pruned, upper_bound }
}

#[cfg(test)]
mod tests {
    use super::super::paragraph_prepare::{Fragment, Gap, Word};
    use super::*;
    use crate::entities::layout_types::{Length, Pt, TextSeparatorOrigin, TextStyle};

    fn prepared(widths: &[f64], gap: f64, tracking: f64) -> Prepared {
        let mut style = TextStyle::regular(Pt(12.0));
        style.tracking = Some(Length::pt(tracking));
        Prepared {
            words: widths
                .iter()
                .enumerate()
                .map(|(index, &width)| Word {
                    fragments: vec![Fragment {
                        text: "x".into(),
                        replacement: None,
                        style: style.clone(),
                        width,
                        hyphenate: false,
                        overhang: false,
                    }],
                    before: if index == 0 {
                        Vec::new()
                    } else {
                        vec![Gap {
                            width: gap,
                            style: style.clone(),
                            origin: TextSeparatorOrigin::Markup,
                        }]
                    },
                    width,
                    hyphenation: None,
                })
                .collect(),
            trailing: Vec::new(),
            right_overhang: 0.0,
        }
    }

    #[test]
    fn prototype_matches_exact_across_word_sequences() {
        let mut pruned = 0;
        let costs =
            [LineCosts::default(), LineCosts { runt: -100.0, ..LineCosts::default() }];
        for size in 0..=24 {
            let widths: Vec<f64> =
                (0..size).map(|i| ((i * 37 + size * 11) % 43 + 3) as f64).collect();
            for gap in [0.0, 2.5, 6.0] {
                for tracking in [0.0, 0.2] {
                    let p = prepared(&widths, gap, tracking);
                    for width in [8.0, 36.0, 70.0, 120.0] {
                        for justify in [false, true] {
                            for cost in costs {
                                let exact = paragraph_select::select(
                                    &p,
                                    width,
                                    12.0,
                                    justify,
                                    Strategy::Optimized,
                                    cost,
                                );
                                let trial =
                                    select_bounded(&p, width, 12.0, justify, cost);
                                assert_eq!(trial.lines, exact, "size={size} gap={gap} tracking={tracking} width={width} justify={justify} runt={}", cost.runt);
                                assert!(trial.upper_bound.is_finite());
                                pruned += trial.pruned;
                            }
                        }
                    }
                }
            }
        }
        assert!(pruned > 0, "the test matrix must exercise upper-bound pruning");
    }

    #[test]
    #[ignore = "microbenchmark de release; executar explicitamente com --ignored --nocapture"]
    fn microbench_exact_vs_bounded() {
        use criterion::Criterion;
        use std::hint::black_box;
        use std::time::Duration;

        let mut criterion = Criterion::default()
            .sample_size(30)
            .warm_up_time(Duration::from_secs(1))
            .measurement_time(Duration::from_secs(2));
        for (name, count, width, justify) in [
            ("short_narrow", 40, 76.0, false),
            ("medium", 160, 240.0, false),
            ("wide_justified", 160, 1200.0, true),
            ("large_narrow", 1000, 76.0, false),
            ("large_medium", 1000, 240.0, false),
            ("large_wide_justified", 1000, 1200.0, true),
        ] {
            let widths: Vec<f64> =
                (0..count).map(|i| ((i * 17) % 19 + 9) as f64).collect();
            let p = prepared(&widths, 3.0, 0.0);
            let costs = LineCosts::default();
            let exact = paragraph_select::select(
                &p,
                width,
                12.0,
                justify,
                Strategy::Optimized,
                costs,
            );
            let trial = select_bounded(&p, width, 12.0, justify, costs);
            assert_eq!(trial.lines, exact, "case={name}");
            eprintln!(
                "case={name} evaluated={} pruned={} bound={}",
                trial.evaluated, trial.pruned, trial.upper_bound
            );
            let mut group = criterion.benchmark_group(name);
            group.bench_function("exact", |b| {
                b.iter(|| {
                    black_box(paragraph_select::select(
                        black_box(&p),
                        black_box(width),
                        12.0,
                        justify,
                        Strategy::Optimized,
                        costs,
                    ))
                })
            });
            group.bench_function("bounded", |b| {
                b.iter(|| {
                    black_box(select_bounded(
                        black_box(&p),
                        black_box(width),
                        12.0,
                        justify,
                        costs,
                    ))
                })
            });
            group.finish();
        }
        criterion.final_summary();
    }

    #[test]
    fn prototype_reports_work_without_changing_default_selector() {
        let widths: Vec<f64> = (0..40).map(|i| ((i * 17) % 19 + 9) as f64).collect();
        let p = prepared(&widths, 3.0, 0.0);
        let trial = select_bounded(&p, 76.0, 12.0, false, LineCosts::default());
        assert_eq!(
            trial.lines,
            paragraph_select::select(
                &p,
                76.0,
                12.0,
                false,
                Strategy::Optimized,
                LineCosts::default(),
            )
        );
        assert!(trial.evaluated > 0);
        assert!(trial.pruned > 0);
    }
}
