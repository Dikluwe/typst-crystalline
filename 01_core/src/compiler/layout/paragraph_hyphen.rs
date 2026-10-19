//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/paragraph_hyphen.md
//! @prompt-hash 7740868f
//! @layer L1
//! @updated 2026-09-30

use super::{
    hyphenation,
    paragraph_prepare::{Fragment, Prepared, Word},
    paragraph_select::{line_cost, ratio, Measure, Strategy},
    text_measure, FontMetrics,
};
use crate::compiler::text_costs::LineCosts;
use crate::entities::lang::Lang;
// Pinned vanilla inline/linebreak.rs:29,663-666 and inline/line.rs:708-709.
// Edge and optical constants; effective penalties come from text_costs.
const HYPHEN_EDGE_DISTANCE: usize = 5;
const HYPHEN_EDGE_COST_STEP: f64 = 0.15;
const HYPHEN_OPTICAL_OVERHANG: f64 = 0.55;

#[derive(Clone, Debug)]
pub(super) struct Hyphenation {
    offsets: Vec<usize>,
    lengths: Vec<(usize, usize)>,
    widths: Vec<f64>,
    dash_width: f64,
}
/// Fail closed for mixed policies/styles inside a word, not for missing patterns.
pub(super) fn prepare<M: FontMetrics>(
    metrics: &M,
    word: &Word,
) -> Option<Option<Hyphenation>> {
    if !word.fragments.iter().any(|f| f.hyphenate) {
        return Some(None);
    }
    if word.fragments.len() != 1 {
        return None;
    }
    let f = &word.fragments[0];
    let text = &f.text;
    let alphabetic_end = text
        .char_indices()
        .find(|(_, c)| !c.is_alphabetic())
        .map_or(text.len(), |(i, _)| i);
    if alphabetic_end == 0 && text.chars().all(|c| c.is_ascii_digit()) {
        return Some(None);
    }
    if alphabetic_end == 0
        || !text[alphabetic_end..]
            .chars()
            .all(|c| matches!(c, '.' | ',' | '!' | '?'))
    {
        return None;
    }
    let alpha = &text[..alphabetic_end];
    let len = alpha.chars().count();
    let points = hyphenation::hyphenate(alpha, &f.style.lang.unwrap_or(Lang::ENGLISH));
    if points.is_empty() {
        return Some(None);
    }
    let chars: Vec<usize> = alpha
        .char_indices()
        .map(|(i, _)| i)
        .chain(std::iter::once(alpha.len()))
        .collect();
    let mut offsets = vec![0];
    let mut lengths = vec![(0, 0)];
    for point in points {
        if point > 0 && point < len {
            offsets.push(chars[point]);
            lengths.push((point, len - point));
        }
    }
    offsets.push(text.len());
    lengths.push((0, 0));
    let n = offsets.len();
    let mut widths = vec![0.0; n * n];
    for start in 0..n - 1 {
        for end in start + 1..n {
            let width = text_measure::word(
                metrics,
                &text[offsets[start]..offsets[end]],
                &f.style,
            )
            .0;
            if !width.is_finite() || width < 0.0 {
                return None;
            }
            widths[start * n + end] = width;
        }
    }
    let dash_width = text_measure::word(metrics, "-", &f.style).0;
    if !dash_width.is_finite() || dash_width < 0.0 {
        return None;
    }
    Some(Some(Hyphenation { offsets, lengths, widths, dash_width }))
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Point {
    word: usize,
    cut: usize,
}
fn last_cut(word: &Word) -> usize {
    word.hyphenation.as_ref().map_or(1, |h| h.offsets.len() - 1)
}
fn range_width(word: &Word, start: usize, end: usize) -> f64 {
    word.hyphenation
        .as_ref()
        .map_or(word.width, |h| h.widths[start * h.offsets.len() + end])
}
fn points(p: &Prepared) -> Vec<Point> {
    let mut points = vec![Point { word: 0, cut: 0 }];
    for (word, w) in p.words.iter().enumerate() {
        for cut in 1..last_cut(w) {
            points.push(Point { word, cut });
        }
        points.push(Point { word: word + 1, cut: 0 });
    }
    points
}
struct Measurements {
    prefix: Vec<Measure>,
    before: Vec<Measure>,
}
impl Measurements {
    fn new(p: &Prepared) -> Self {
        let before: Vec<Measure> = p
            .words
            .iter()
            .map(|w| Measure {
                width: w.before.iter().map(|g| g.width).sum(),
                spaces: w.before.iter().map(|g| g.width).sum(),
                count: w.before.len(),
            })
            .collect();
        let mut prefix = vec![Measure::default(); p.words.len() + 1];
        for i in 0..p.words.len() {
            prefix[i + 1] = Measure {
                width: prefix[i].width + p.words[i].width + before[i].width,
                spaces: prefix[i].spaces + before[i].spaces,
                count: prefix[i].count + before[i].count,
            };
        }
        Self { prefix, before }
    }
    fn measure(&self, p: &Prepared, a: Point, b: Point) -> Measure {
        let last = if b.cut > 0 { b.word } else { b.word - 1 };
        let endcut = if b.cut > 0 { b.cut } else { last_cut(&p.words[last]) };
        let trim = if a.word > 0 || a.cut > 0 {
            self.before[a.word]
        } else {
            Measure::default()
        };
        let mut m = Measure {
            width: self.prefix[last + 1].width - self.prefix[a.word].width - trim.width,
            spaces: self.prefix[last + 1].spaces
                - self.prefix[a.word].spaces
                - trim.spaces,
            count: self.prefix[last + 1].count - self.prefix[a.word].count - trim.count,
        };
        if a.word == last {
            m.width += range_width(&p.words[last], a.cut, endcut) - p.words[last].width;
        } else {
            m.width += range_width(&p.words[a.word], a.cut, last_cut(&p.words[a.word]))
                - p.words[a.word].width;
            m.width += range_width(&p.words[last], 0, endcut) - p.words[last].width;
        }
        if b.cut > 0 {
            m.width += p.words[last].hyphenation.as_ref().unwrap().dash_width;
        }
        m
    }
}
fn penalty(p: &Prepared, b: Point, previous: Point, costs: LineCosts) -> f64 {
    if b.cut == 0 {
        return 0.0;
    }
    let h = p.words[b.word].hyphenation.as_ref().unwrap();
    let (l, r) = h.lengths[b.cut];
    costs.hyphenation
        * (1.0
            + HYPHEN_EDGE_COST_STEP
                * (HYPHEN_EDGE_DISTANCE.saturating_sub(l)
                    + HYPHEN_EDGE_DISTANCE.saturating_sub(r)) as f64)
        + if previous.cut > 0 { costs.hyphenation } else { 0.0 }
}
fn materialize(p: &Prepared, a: Point, b: Point) -> Prepared {
    let last = if b.cut > 0 { b.word } else { b.word - 1 };
    let mut words = Vec::with_capacity(last - a.word + 1);
    for i in a.word..=last {
        let w = &p.words[i];
        let start = if i == a.word { a.cut } else { 0 };
        let end = if i == last && b.cut > 0 { b.cut } else { last_cut(w) };
        let mut fragments = if start == 0 && end == last_cut(w) {
            w.fragments.clone()
        } else {
            let h = w.hyphenation.as_ref().unwrap();
            let f = &w.fragments[0];
            vec![Fragment {
                replacement: None,
                text: f.text[h.offsets[start]..h.offsets[end]].into(),
                style: f.style.clone(),
                width: range_width(w, start, end),
                hyphenate: false,
                overhang: f.overhang,
            }]
        };
        let mut width = range_width(w, start, end);
        if i == last && b.cut > 0 {
            let h = w.hyphenation.as_ref().unwrap();
            let style = w.fragments[0].style.clone();
            fragments.push(Fragment {
                replacement: Some("\u{ad}".into()),
                text: "-".into(),
                style,
                width: h.dash_width,
                hyphenate: false,
                overhang: w.fragments[0].overhang,
            });
            width += h.dash_width;
        }
        let before = if i == a.word && (a.word > 0 || a.cut > 0) {
            Vec::new()
        } else {
            w.before.clone()
        };
        words.push(Word { fragments, before, width, hyphenation: None });
    }
    let trailing = if b.cut > 0 {
        Vec::new()
    } else if b.word < p.words.len() {
        p.words[b.word].before.clone()
    } else {
        p.trailing.clone()
    };
    let right_overhang = if b.cut > 0 && p.words[b.word].fragments[0].overhang {
        p.words[b.word].hyphenation.as_ref().unwrap().dash_width * HYPHEN_OPTICAL_OVERHANG
    } else {
        0.0
    };
    Prepared { words, trailing, right_overhang }
}
pub(super) fn select(
    p: &Prepared,
    width: f64,
    font: f64,
    justify: bool,
    strategy: Strategy,
    costs: LineCosts,
) -> Vec<Prepared> {
    select_shaped(
        p,
        super::paragraph_indent::Shape::uniform(width),
        font,
        justify,
        strategy,
        costs,
    )
}

pub(super) fn select_shaped(
    p: &Prepared,
    shape: super::paragraph_indent::Shape,
    font: f64,
    justify: bool,
    strategy: Strategy,
    costs: LineCosts,
) -> Vec<Prepared> {
    let points = points(p);
    let n = points.len();
    let measurements = Measurements::new(p);
    let mut selected = Vec::new();
    if strategy == Strategy::Simple {
        let mut start = 0;
        let mut last = None;
        for end in 1..n {
            let mut width = shape.width(start == 0);
            let mut m = measurements.measure(p, points[start], points[end]);
            if m.width > width + 0.001 {
                if let Some(fitting) = last.take() {
                    selected.push((start, fitting));
                    start = fitting;
                    width = shape.width(start == 0);
                    m = measurements.measure(p, points[start], points[end]);
                }
            }
            if end == n - 1 || m.width > width + 0.001 {
                selected.push((start, end));
                start = end;
                last = None;
            } else {
                last = Some(end);
            }
        }
        if let Some(end) = last {
            selected.push((start, end));
        }
    } else {
        let mut totals = vec![f64::INFINITY; n];
        totals[0] = 0.0;
        let mut pred = vec![0; n];
        let mut active = 0;
        for end in 1..n {
            // Keep this candidate range fixed while pruning future prefixes.
            let first_active = active;
            for start in first_active..end {
                let width = shape.width(start == 0);
                let m = measurements.measure(p, points[start], points[end]);
                if start == active
                    && ratio(m, width, font) < if justify { -1.0 } else { 0.0 }
                {
                    active += 1;
                }
                let cost = line_cost(
                    m,
                    width,
                    font,
                    justify,
                    end == n - 1,
                    end == start + 1,
                    penalty(p, points[end], points[start], costs),
                    costs,
                );
                let total = totals[start] + cost;
                if total <= totals[end] {
                    totals[end] = total;
                    pred[end] = start;
                }
            }
        }
        let mut end = n - 1;
        while end > 0 {
            let start = pred[end];
            selected.push((start, end));
            end = start;
        }
        selected.reverse();
    }
    selected
        .into_iter()
        .map(|(a, b)| materialize(p, points[a], points[b]))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::super::FixedMetrics;
    use super::*;
    use crate::entities::layout_types::{Pt, TextStyle};
    fn word(text: &str, enabled: bool) -> Word {
        let mut style = TextStyle::regular(Pt(12.0));
        style.lang = Some("pt".parse().unwrap());
        let width = text_measure::word(&FixedMetrics, text, &style).0;
        let mut word = Word {
            fragments: vec![Fragment {
                replacement: None,
                text: text.into(),
                style,
                width,
                hyphenate: enabled,
                overhang: true,
            }],
            before: Vec::new(),
            width,
            hyphenation: None,
        };
        word.hyphenation = prepare(&FixedMetrics, &word).unwrap();
        word
    }
    #[test]
    fn hyphenation_ranges_are_unicode_safe_and_reassemble_the_word() {
        let w = word("representação", true);
        let h = w.hyphenation.as_ref().unwrap();
        assert!(h.offsets.iter().all(|&i| w.fragments[0].text.is_char_boundary(i)));
        let p = Prepared {
            words: vec![w],
            trailing: Vec::new(),
            right_overhang: 0.0,
        };
        for strategy in [Strategy::Simple, Strategy::Optimized] {
            let lines = select(&p, 60.0, 12.0, true, strategy, LineCosts::default());
            assert!(lines.len() > 1);
            assert_eq!(
                lines
                    .iter()
                    .flat_map(|l| &l.words)
                    .flat_map(|w| &w.fragments)
                    .map(|f| f.text.replace('-', ""))
                    .collect::<String>(),
                "representação"
            );
            assert!(lines.iter().flat_map(|l| &l.words).all(|w| w.before.is_empty()));
        }
    }
    #[test]
    fn hyphenation_disabled_has_no_candidates_and_mixed_policy_is_declined() {
        assert!(word("representação", false).hyphenation.is_none());
        let mut w = word("representação", true);
        let mut other = w.fragments[0].clone();
        other.hyphenate = false;
        w.fragments.push(other);
        assert!(prepare(&FixedMetrics, &w).is_none());
    }
    #[test]
    fn hyphenation_penalty_counts_character_edges_and_consecutive_lines() {
        let w = word("representação", true);
        let h = w.hyphenation.as_ref().unwrap();
        let (left, right) = h.lengths[1];
        let p = Prepared {
            words: vec![w.clone()],
            trailing: Vec::new(),
            right_overhang: 0.0,
        };
        let end = Point { word: 0, cut: 1 };
        let first = penalty(&p, end, Point { word: 0, cut: 0 }, LineCosts::default());
        assert_eq!(
            first,
            135.0
                * (1.0
                    + 0.15
                        * (5usize.saturating_sub(left) + 5usize.saturating_sub(right))
                            as f64)
        );
        assert_eq!(penalty(&p, end, end, LineCosts::default()) - first, 135.0);
        assert_eq!(
            penalty(
                &p,
                end,
                end,
                LineCosts { hyphenation: 270.0, ..LineCosts::default() }
            ),
            2.0 * penalty(&p, end, end, LineCosts::default())
        );
        assert_eq!(
            penalty(&p, Point { word: 1, cut: 0 }, end, LineCosts::default()),
            0.0
        );
    }
    #[test]
    fn hyphenation_terminal_punctuation_does_not_change_word_penalty_lengths() {
        let a = word("representação", true);
        let b = word("representação,", true);
        assert_eq!(a.hyphenation.unwrap().lengths, b.hyphenation.unwrap().lengths);
    }
    struct NonAdditive;
    impl FontMetrics for NonAdditive {
        fn advance(&self, text: &str, _: Pt, _: &TextStyle) -> Pt {
            let count = text.chars().count() as f64;
            Pt(count * count)
        }
        fn vertical_metrics(&self, _: Pt, _: &TextStyle) -> (Pt, Pt) {
            panic!("preparation cannot request vertical metrics")
        }
        fn cap_height(&self, _: Pt, _: &TextStyle) -> Pt {
            panic!("preparation cannot request cap height")
        }
        fn text_edges(&self, _: Pt, _: &TextStyle) -> (Pt, Pt) {
            panic!("preparation cannot request edges")
        }
    }
    #[test]
    fn hyphenation_measures_slices_instead_of_summing_syllables() {
        let w = word("representação", true);
        let h = prepare(&NonAdditive, &w).unwrap().unwrap();
        let n = h.offsets.len();
        assert_eq!(h.widths[n - 1], 169.0);
        assert!(h.widths[1] + h.widths[n + n - 1] < h.widths[n - 1]);
        assert_eq!(h.dash_width, 1.0);
    }
}
