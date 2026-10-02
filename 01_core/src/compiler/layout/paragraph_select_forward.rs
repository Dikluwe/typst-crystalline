//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/paragraph_select_forward.md
//! @prompt-hash 829d6e0a
//! @layer L1
//! @updated 2026-10-02

use super::paragraph_prepare::Prepared;
use super::paragraph_select::{self, line_cost, ratio, Line, Measure, Strategy};
use crate::compiler::text_costs::LineCosts;

#[derive(Debug)]
struct Trial {
    lines: Vec<Line>,
    evaluated: usize,
    skipped_starts: usize,
    upper_bound: f64,
    bound_source: &'static str,
}

/// Alternative test-only DP: start is fixed while the line end moves.
fn select_forward(
    p: &Prepared,
    width: f64,
    font: f64,
    justify: bool,
    costs: LineCosts,
) -> Trial {
    select_forward_variant::<true, false>(p, width, font, justify, costs)
}

/// Ablation without the packed and balanced initial layouts.
fn select_forward_simple_bound(
    p: &Prepared,
    width: f64,
    font: f64,
    justify: bool,
    costs: LineCosts,
) -> Trial {
    select_forward_variant::<false, false>(p, width, font, justify, costs)
}

/// Ablation reusing prefix measures to build the greedy upper bound.
fn select_forward_prefix_bound(
    p: &Prepared,
    width: f64,
    font: f64,
    justify: bool,
    costs: LineCosts,
) -> Trial {
    select_forward_variant::<false, true>(p, width, font, justify, costs)
}

fn select_forward_variant<const EXTRA_SEEDS: bool, const PREFIX_GREEDY: bool>(
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
            skipped_starts: 0,
            upper_bound: 0.0,
            bound_source: "empty",
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
    let total_score = |lines: &[Line]| lines.iter().copied().map(score).sum::<f64>();
    let greedy = if PREFIX_GREEDY {
        let mut lines = Vec::new();
        let mut start = 0;
        while start < n {
            let mut end = start + 1;
            while end < n && measured(Line { start, end: end + 1 }).width <= width {
                end += 1;
            }
            lines.push(Line { start, end });
            start = end;
        }
        lines
    } else {
        paragraph_select::select(p, width, font, justify, Strategy::Simple, costs)
    };
    let mut upper_bound = total_score(&greedy);
    let mut bound_source = "simple";
    let min_ratio = if justify { -1.0 } else { 0.0 };
    if EXTRA_SEEDS {
        let mut word_widths = vec![0.0; n + 1];
        for i in 0..n {
            word_widths[i + 1] = word_widths[i] + p.words[i].width;
        }
        // Two seed layouts: the fullest legal line and the one nearest the
        // average remaining word width per estimated remaining line.
        for balanced in [false, true] {
            let mut lines = Vec::new();
            let mut start = 0;
            while start < n {
                let remaining = word_widths[n] - word_widths[start];
                let estimated_lines = (remaining / width).ceil().max(1.0) as usize;
                let estimated_lines = estimated_lines.clamp(1, n - start);
                let target = remaining / estimated_lines as f64;
                let mut chosen = start + 1;
                let mut distance =
                    (measured(Line { start, end: chosen }).width - target).abs();
                for end in start + 2..=n {
                    let m = measured(Line { start, end });
                    if ratio(m, width, font) < min_ratio {
                        break;
                    }
                    if balanced {
                        let candidate_distance = (m.width - target).abs();
                        if candidate_distance < distance {
                            chosen = end;
                            distance = candidate_distance;
                        }
                    } else {
                        chosen = end;
                    }
                }
                lines.push(Line { start, end: chosen });
                start = chosen;
            }
            let candidate = total_score(&lines);
            if candidate.is_finite()
                && (!upper_bound.is_finite() || candidate < upper_bound)
            {
                upper_bound = candidate;
                bound_source = if balanced { "balanced" } else { "packed" };
            }
        }
    }
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
            skipped_starts: 0,
            upper_bound,
            bound_source: "fallback",
        };
    }
    // Exactly reproduce the baseline's first active start at each end.
    // Each failed earliest candidate is retired after its current attempt.
    let mut active_before = vec![0; n + 1];
    let mut active = 0;
    for end in 1..=n {
        active_before[end] = active;
        while active < end
            && ratio(measured(Line { start: active, end }), width, font) < min_ratio
        {
            active += 1;
        }
    }
    let mut totals = vec![f64::INFINITY; n + 1];
    let mut predecessors = vec![0; n + 1];
    let (mut evaluated, mut skipped_starts) = (0, 0);
    totals[0] = 0.0;
    for start in 0..n {
        if totals[start] > upper_bound {
            skipped_starts += 1;
            continue;
        }
        for end in start + 1..=n {
            if start < active_before[end] {
                break;
            }
            evaluated += 1;
            let total = totals[start] + score(Line { start, end });
            if total <= upper_bound && total <= totals[end] {
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
            skipped_starts,
            upper_bound,
            bound_source: "fallback",
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
    Trial {
        lines,
        evaluated,
        skipped_starts,
        upper_bound,
        bound_source,
    }
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
    fn forward_matches_exact() {
        let costs =
            [LineCosts::default(), LineCosts { runt: -100.0, ..LineCosts::default() }];
        let mut skipped = 0;
        for size in 0..=28 {
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
                                    select_forward(&p, width, 12.0, justify, cost);
                                assert_eq!(trial.lines, exact,
                                    "size={size} gap={gap} tracking={tracking} width={width} justify={justify} runt={}", cost.runt);
                                let minimal = select_forward_simple_bound(
                                    &p, width, 12.0, justify, cost,
                                );
                                assert_eq!(
                                    minimal.lines, exact,
                                    "simple-only size={size} width={width}"
                                );
                                let prefix_seed = select_forward_prefix_bound(
                                    &p, width, 12.0, justify, cost,
                                );
                                assert_eq!(
                                    prefix_seed.lines, exact,
                                    "prefix-seed size={size} width={width}"
                                );
                                skipped += trial.skipped_starts;
                            }
                        }
                    }
                }
            }
        }
        assert!(skipped > 0, "the matrix must exercise whole-start pruning");
    }

    #[test]
    #[ignore = "microbenchmark de release; executar explicitamente com --ignored --nocapture"]
    fn microbench_forward_vs_exact() {
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
            let trial = select_forward(&p, width, 12.0, justify, costs);
            assert_eq!(trial.lines, exact, "case={name}");
            let minimal = select_forward_simple_bound(&p, width, 12.0, justify, costs);
            assert_eq!(minimal.lines, exact, "simple-only case={name}");
            let prefix_seed =
                select_forward_prefix_bound(&p, width, 12.0, justify, costs);
            assert_eq!(prefix_seed.lines, exact, "prefix-seed case={name}");
            eprintln!(
                "case={name} evaluated={} skipped_starts={} bound={} source={}",
                trial.evaluated,
                trial.skipped_starts,
                trial.upper_bound,
                trial.bound_source
            );
            let mut group = criterion.benchmark_group(format!("forward_{name}"));
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
            group.bench_function("forward", |b| {
                b.iter(|| {
                    black_box(select_forward(
                        black_box(&p),
                        black_box(width),
                        12.0,
                        justify,
                        costs,
                    ))
                })
            });
            group.bench_function("forward_simple_only", |b| {
                b.iter(|| {
                    black_box(select_forward_simple_bound(
                        black_box(&p),
                        black_box(width),
                        12.0,
                        justify,
                        costs,
                    ))
                })
            });
            group.bench_function("forward_prefix_seed", |b| {
                b.iter(|| {
                    black_box(select_forward_prefix_bound(
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
}
