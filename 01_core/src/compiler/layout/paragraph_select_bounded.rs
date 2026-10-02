//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/paragraph_select_bounded.md
//! @prompt-hash 1f461420
//! @layer L1
//! @updated 2026-10-02

mod justice_overflow;
mod vanilla_lower;

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

struct Search<'a> {
    p: &'a Prepared,
    width: f64,
    font: f64,
    justify: bool,
    costs: LineCosts,
    n: usize,
    prefix: Vec<Measure>,
    before: Vec<Measure>,
    upper_bound: f64,
    monotone: bool,
    active: usize,
    totals: Vec<f64>,
    predecessors: Vec<usize>,
    evaluated: usize,
    pruned: usize,
}

impl<'a> Search<'a> {
    fn new(
        p: &'a Prepared,
        width: f64,
        font: f64,
        justify: bool,
        costs: LineCosts,
        seed: bool,
    ) -> Self {
        let n = p.words.len();
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
        let mut prefix = vec![Measure::default(); n + 1];
        for i in 0..n {
            prefix[i + 1] = Measure {
                width: prefix[i].width + p.words[i].width + before[i].width,
                spaces: prefix[i].spaces + before[i].spaces,
                count: prefix[i].count + before[i].count,
            };
        }
        let monotone = width.is_finite()
            && font.is_finite()
            && font > 0.0
            && p.tracking().is_finite()
            && p.tracking() >= 0.0
            && p.words.iter().all(|word| {
                word.width.is_finite()
                    && word.width >= 0.0
                    && word
                        .before
                        .iter()
                        .all(|gap| gap.width.is_finite() && gap.width >= 0.0)
            });
        let mut search = Self {
            p,
            width,
            font,
            justify,
            costs,
            n,
            prefix,
            before,
            upper_bound: if seed { 0.0 } else { f64::INFINITY },
            monotone,
            active: 0,
            totals: vec![f64::INFINITY; n + 1],
            predecessors: vec![0; n + 1],
            evaluated: 0,
            pruned: 0,
        };
        search.totals[0] = 0.0;
        if seed && n > 0 {
            search.upper_bound = paragraph_select::select(
                p,
                width,
                font,
                justify,
                Strategy::Simple,
                costs,
            )
            .into_iter()
            .map(|line| search.price(line, search.measure(line)))
            .sum();
        }
        search
    }

    fn measure(&self, line: Line) -> Measure {
        let trim =
            if line.start > 0 { self.before[line.start] } else { Measure::default() };
        let terminal = if line.end < self.n && !self.p.words[line.end].before.is_empty() {
            self.p.tracking()
        } else {
            0.0
        };
        Measure {
            width: self.prefix[line.end].width
                - self.prefix[line.start].width
                - trim.width
                + terminal,
            spaces: self.prefix[line.end].spaces
                - self.prefix[line.start].spaces
                - trim.spaces,
            count: self.prefix[line.end].count
                - self.prefix[line.start].count
                - trim.count,
        }
    }

    fn price(&self, line: Line, m: Measure) -> f64 {
        line_cost(
            m,
            self.width,
            self.font,
            self.justify,
            line.end == self.n,
            line.end == line.start + 1,
            0.0,
            self.costs,
        )
    }

    fn min_ratio(&self) -> f64 {
        if self.justify {
            -1.0
        } else {
            0.0
        }
    }

    fn run_upper_only(&mut self) {
        for end in 1..=self.n {
            let old_active = self.active;
            for start in old_active..end {
                let line = Line { start, end };
                let m = self.measure(line);
                if start == self.active
                    && ratio(m, self.width, self.font) < self.min_ratio()
                {
                    self.active += 1;
                }
                if self.totals[start] > self.upper_bound {
                    self.pruned += 1;
                    continue;
                }
                self.evaluated += 1;
                let total = self.totals[start] + self.price(line, m);
                if total <= self.totals[end] {
                    self.totals[end] = total;
                    self.predecessors[end] = start;
                }
            }
        }
    }

    fn finish(self) -> Trial {
        if self.n == 0 {
            return Trial {
                lines: Vec::new(),
                evaluated: 0,
                pruned: 0,
                upper_bound: 0.0,
            };
        }
        if !self.totals[self.n].is_finite() {
            return Trial {
                lines: paragraph_select::select(
                    self.p,
                    self.width,
                    self.font,
                    self.justify,
                    Strategy::Optimized,
                    self.costs,
                ),
                evaluated: self.evaluated,
                pruned: self.pruned,
                upper_bound: self.upper_bound,
            };
        }
        let mut lines = Vec::new();
        let mut end = self.n;
        while end > 0 {
            let start = self.predecessors[end];
            lines.push(Line { start, end });
            end = start;
        }
        lines.reverse();
        Trial {
            lines,
            evaluated: self.evaluated,
            pruned: self.pruned,
            upper_bound: self.upper_bound,
        }
    }
}

fn select_bounded(
    p: &Prepared,
    width: f64,
    font: f64,
    justify: bool,
    costs: LineCosts,
) -> Trial {
    let mut search = Search::new(p, width, font, justify, costs, true);
    if search.upper_bound.is_finite() {
        search.run_upper_only();
    }
    search.finish()
}

fn select_bounded_vanilla_lower(
    p: &Prepared,
    width: f64,
    font: f64,
    justify: bool,
    costs: LineCosts,
) -> Trial {
    let mut search = Search::new(p, width, font, justify, costs, true);
    if search.upper_bound.is_finite() {
        vanilla_lower::run(&mut search);
    }
    search.finish()
}

fn select_bounded_justice(
    p: &Prepared,
    width: f64,
    font: f64,
    justify: bool,
    costs: LineCosts,
) -> Trial {
    let mut search = Search::new(p, width, font, justify, costs, true);
    if search.upper_bound.is_finite() {
        justice_overflow::run(&mut search);
    }
    search.finish()
}

fn select_justice_without_seed(
    p: &Prepared,
    width: f64,
    font: f64,
    justify: bool,
    costs: LineCosts,
) -> Trial {
    let mut search = Search::new(p, width, font, justify, costs, false);
    justice_overflow::run(&mut search);
    search.finish()
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
            ("mid_medium", 500, 240.0, false),
            ("very_large_medium", 2000, 240.0, false),
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
            let vanilla = select_bounded_vanilla_lower(&p, width, 12.0, justify, costs);
            let justice = select_bounded_justice(&p, width, 12.0, justify, costs);
            assert_eq!(vanilla.lines, exact, "case={name} vanilla");
            assert_eq!(justice.lines, exact, "case={name} justice");
            eprintln!(
                "case={name} upper_evaluated={} upper_pruned={} vanilla_evaluated={} vanilla_pruned={} justice_evaluated={} justice_pruned={} bound={}",
                trial.evaluated, trial.pruned,
                vanilla.evaluated, vanilla.pruned,
                justice.evaluated, justice.pruned, trial.upper_bound
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
            group.bench_function("vanilla_lower", |b| {
                b.iter(|| {
                    black_box(select_bounded_vanilla_lower(
                        black_box(&p),
                        black_box(width),
                        12.0,
                        justify,
                        costs,
                    ))
                })
            });
            group.bench_function("justice_overflow", |b| {
                b.iter(|| {
                    black_box(select_bounded_justice(
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

    #[test]
    fn lower_bound_variants_match_exact() {
        let mut vanilla_pruned = 0;
        let mut justice_pruned = 0;
        for size in 0..=28 {
            let widths: Vec<f64> =
                (0..size).map(|i| ((i * 37 + size * 11) % 43 + 3) as f64).collect();
            for gap in [0.0, 2.5, 6.0, -2.0] {
                for tracking in [0.0, 0.2, -0.1] {
                    let p = prepared(&widths, gap, tracking);
                    for width in [8.0, 36.0, 70.0, 120.0] {
                        for justify in [false, true] {
                            for costs in [
                                LineCosts::default(),
                                LineCosts { runt: -100.0, ..LineCosts::default() },
                                LineCosts { runt: 250.0, ..LineCosts::default() },
                            ] {
                                let expected = paragraph_select::select(
                                    &p,
                                    width,
                                    12.0,
                                    justify,
                                    Strategy::Optimized,
                                    costs,
                                );
                                let vanilla = select_bounded_vanilla_lower(
                                    &p, width, 12.0, justify, costs,
                                );
                                let justice = select_bounded_justice(
                                    &p, width, 12.0, justify, costs,
                                );
                                let justice_no_seed = select_justice_without_seed(
                                    &p, width, 12.0, justify, costs,
                                );
                                assert_eq!(vanilla.lines, expected, "vanilla size={size} gap={gap} tracking={tracking} width={width} justify={justify}");
                                assert_eq!(justice.lines, expected, "justice size={size} gap={gap} tracking={tracking} width={width} justify={justify}");
                                assert_eq!(justice_no_seed.lines, expected, "justice_no_seed size={size} gap={gap} tracking={tracking} width={width} justify={justify}");
                                vanilla_pruned += vanilla.pruned;
                                justice_pruned += justice.pruned;
                            }
                        }
                    }
                }
            }
        }
        assert!(vanilla_pruned > 0);
        assert!(justice_pruned > 0);
    }

    #[test]
    #[ignore = "microbenchmark de release; executar explicitamente com --ignored --nocapture"]
    fn microbench_justice_without_seed() {
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
            ("mid_medium", 500, 240.0, false),
            ("large_medium", 1000, 240.0, false),
            ("very_large_medium", 2000, 240.0, false),
            ("large_wide", 1000, 1200.0, true),
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
            let trial = select_justice_without_seed(&p, width, 12.0, justify, costs);
            assert_eq!(trial.lines, exact, "case={name}");
            eprintln!(
                "case={name} evaluated={} pruned={}",
                trial.evaluated, trial.pruned
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
            group.bench_function("justice_no_seed", |b| {
                b.iter(|| {
                    black_box(select_justice_without_seed(
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
