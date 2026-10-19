//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/paragraph_emit.md
//! @prompt-hash 26ccabae
//! @layer L1
//! @updated 2026-09-30

use super::{
    paragraph_prepare::{Gap, Prepared},
    paragraph_select::{before_gap, before_word, measure, Line},
    FontMetrics, ImageSizer, Layouter,
};
use crate::entities::layout_types::{FrameItem, Point, Pt};
fn emit_gap<M: FontMetrics, S: ImageSizer>(
    l: &mut Layouter<M, S>,
    gap: &Gap,
    width: f64,
) {
    let y = l.regions.current.cursor_y
        + Pt(gap.style.baseline_offset.resolve_pt(gap.style.size.val()));
    l.regions.current.current_line.push(FrameItem::TextSeparator {
        pos: Point { x: l.regions.current.cursor_x, y },
        advance: Pt(width),
        style: gap.style.clone(),
        origin: gap.origin,
        bidi: None,
    });
    l.regions.current.cursor_x += Pt(width);
    l.justify_opportunities.push(l.regions.current.cursor_x.0);
}
pub(super) fn emit<M: FontMetrics, S: ImageSizer>(
    l: &mut Layouter<M, S>,
    p: &Prepared,
    lines: &[Line],
    width: f64,
    justify: bool,
) {
    super::paragraph_indent::start(l);
    let saved = l.style.clone();
    let pagination = super::paragraph_pagination::certify_selected(l, p, lines);
    for (index, &line) in lines.iter().enumerate() {
        super::paragraph_pagination::admit(l, pagination, index, lines.len());
        emit_line(l, p, line, width, justify, index + 1 < lines.len());
    }
    l.style = saved;
}
fn emit_line<M: FontMetrics, S: ImageSizer>(
    l: &mut Layouter<M, S>,
    p: &Prepared,
    line: Line,
    width: f64,
    justify: bool,
    continued: bool,
) {
    let width = super::paragraph_indent::current_width(l, width);
    let m = measure(p, line);
    let delta = width + p.right_overhang - m.width;
    let (proportional, extra) = if delta < 0.0 && m.spaces > 0.0 {
        ((delta / m.spaces).max(-1.0 / 3.0), 0.0)
    } else if justify && continued && m.count > 0 {
        let ratio = (delta / m.spaces).min(0.5);
        (ratio, (delta - ratio * m.spaces).max(0.0) / (m.count as f64))
    } else {
        (0.0, 0.0)
    };
    for i in line.start..line.end {
        if i > line.start || line.start == 0 {
            for (gap_index, gap) in p.words[i].before.iter().enumerate() {
                l.regions.current.cursor_x += Pt(before_gap(p, line, i, gap_index));
                emit_gap(l, gap, gap.width * (1.0 + proportional) + extra);
            }
            l.regions.current.cursor_x += Pt(before_word(p, line, i));
        }
        for f in &p.words[i].fragments {
            if l.document_language.is_none() {
                l.document_language = f.style.lang;
            }
            super::word_layout::prepared(
                l,
                &f.text,
                Pt(f.width),
                &f.style,
                f.replacement.as_deref(),
            );
        }
    }
    let trailing =
        if line.end < p.words.len() { &p.words[line.end].before } else { &p.trailing };
    for gap in trailing {
        l.regions.current.cursor_x += Pt(p.tracking());
        emit_gap(l, gap, gap.width);
    }
    if continued {
        l.flush_line();
    }
}

pub(super) fn emit_prepared_lines<M: FontMetrics, S: ImageSizer>(
    l: &mut Layouter<M, S>,
    lines: &[Prepared],
    width: f64,
    justify: bool,
) {
    super::paragraph_indent::start(l);
    let saved = l.style.clone();
    let pagination = super::paragraph_pagination::certify_prepared(l, lines);
    for (i, p) in lines.iter().enumerate() {
        super::paragraph_pagination::admit(l, pagination, i, lines.len());
        emit_line(
            l,
            p,
            Line { start: 0, end: p.words.len() },
            width,
            justify,
            i + 1 < lines.len(),
        );
    }
    l.style = saved;
}

#[cfg(test)]
mod pagination_tests {
    use super::super::{FixedMetrics, Layouter};
    use crate::entities::{
        content::Content,
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
        layout_types::{PageMarginSpec, PageMargins, Pt, Ratio},
        value::Value,
    };
    use comemo::Track;
    fn pages(
        count: usize,
        height: f64,
        prefix: bool,
        widow: Option<f64>,
        orphan: Option<f64>,
        explicit: bool,
    ) -> Vec<String> {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            FixedMetrics,
            NullImageSizer,
            12.0,
            (&intr as &dyn Introspector).track(),
        );
        l.page_config.width = 100.0;
        l.page_config.height = height;
        l.page_config.margin = PageMargins::uniform(0.0);
        l.page_config.margin_spec = PageMarginSpec::uniform(0.0);
        l.regions.current.width = 100.0;
        l.regions.current.height = height;
        l.regions.current.cursor_x = Pt(0.0);
        l.regions.current.line_start_x = Pt(0.0);
        l.regions.current.cursor_y = Pt(0.0);
        if explicit {
            l.chain = l.chain.push_custom("par.linebreaks", Value::Str("simple".into()));
        }
        l.chain = l.chain.push_custom("text.hyphenate", Value::Bool(false));
        for (key, value) in [("text.costs.widow", widow), ("text.costs.orphan", orphan)] {
            if let Some(value) = value {
                l.chain = l.chain.push_custom(key, Value::Ratio(Ratio(value)));
            }
        }
        if prefix {
            l.layout_content(&Content::text("prefix"));
            l.layout_content(&Content::Parbreak);
        }
        let words = ["aaaaaaaaaaaa", "bbbbbbbbbbbb", "cccccccccccc", "dddddddddddd"];
        l.layout_content(&Content::text(words[..count].join(" ")));
        let doc = l.finish();
        doc.pages
            .iter()
            .map(|p| p.plain_text().split_whitespace().collect::<Vec<_>>().join(" "))
            .collect()
    }
    #[test]
    fn pagination_widow_keeps_two_final_lines_with_positive_cost() {
        let disabled = pages(4, 45.0, false, Some(0.0), Some(0.0), true);
        let enabled = pages(4, 45.0, false, Some(1.0), Some(0.0), true);
        assert_eq!(
            disabled,
            vec!["aaaaaaaaaaaa bbbbbbbbbbbb cccccccccccc", "dddddddddddd"]
        );
        assert_eq!(
            enabled,
            vec!["aaaaaaaaaaaa bbbbbbbbbbbb", "cccccccccccc dddddddddddd"]
        );
        assert_eq!(pages(4, 45.0, false, Some(0.0001), Some(0.0), true), enabled);
        assert_eq!(pages(4, 45.0, false, Some(100.0), Some(0.0), true), enabled);
        assert_eq!(pages(4, 45.0, false, Some(-1.0), Some(0.0), true), disabled);
    }
    #[test]
    fn pagination_three_lines_with_both_protections_move_together() {
        assert_eq!(
            pages(3, 45.0, true, None, None, true),
            vec!["prefix", "aaaaaaaaaaaa bbbbbbbbbbbb cccccccccccc"]
        );
    }
    #[test]
    fn pagination_two_lines_orphan_switch_is_independent_of_widow() {
        assert_eq!(
            pages(2, 40.0, true, Some(1.0), Some(0.0), true),
            vec!["prefix aaaaaaaaaaaa", "bbbbbbbbbbbb"]
        );
        assert_eq!(
            pages(2, 40.0, true, Some(1.0), Some(1.0), true),
            vec!["prefix", "aaaaaaaaaaaa bbbbbbbbbbbb"]
        );
    }
    #[test]
    fn pagination_default_without_algorithm_flag_is_protected() {
        assert_eq!(
            pages(4, 45.0, false, None, None, false),
            vec!["aaaaaaaaaaaa bbbbbbbbbbbb", "cccccccccccc dddddddddddd"]
        );
    }
    #[test]
    fn pagination_oversized_group_progresses_without_blank_pages() {
        let result = pages(3, 25.0, false, None, None, true);
        assert_eq!(result, vec!["aaaaaaaaaaaa bbbbbbbbbbbb", "cccccccccccc"]);
        assert_eq!(pages(1, 15.0, false, None, None, true), vec!["aaaaaaaaaaaa"]);
    }
}
