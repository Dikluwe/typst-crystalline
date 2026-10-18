//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/paragraph_pagination.md
//! @prompt-hash 79361f71
//! @layer L1
//! @updated 2026-09-28

use super::{
    paragraph_prepare::Prepared, paragraph_select::Line, FontMetrics, ImageSizer,
    Layouter,
};
use crate::entities::layout_types::{PageMargins, TextSeparatorOrigin, TextStyle};

#[derive(Clone, Copy)]
enum Lines<'a> {
    Selected(&'a Prepared, &'a [Line]),
    Prepared(&'a [Prepared]),
}
impl<'a> Lines<'a> {
    fn len(self) -> usize {
        match self {
            Self::Selected(_, lines) => lines.len(),
            Self::Prepared(lines) => lines.len(),
        }
    }
    fn get(self, index: usize) -> (&'a Prepared, Line) {
        match self {
            Self::Selected(part, lines) => (part, lines[index]),
            Self::Prepared(lines) => {
                let part = &lines[index];
                (part, Line { start: 0, end: part.words.len() })
            }
        }
    }
}

#[derive(Clone, Copy)]
pub(super) struct Profile<'a> {
    lines: Lines<'a>,
    leading: f64,
    widow: bool,
    orphan: bool,
}

fn visit_styles(part: &Prepared, line: Line, visit: &mut impl FnMut(&TextStyle)) {
    for i in line.start..line.end {
        if i > line.start || line.start == 0 {
            for gap in &part.words[i].before {
                if gap.origin == TextSeparatorOrigin::Literal {
                    visit(&gap.style);
                }
            }
        }
        for fragment in &part.words[i].fragments {
            visit(&fragment.style);
        }
    }
    let trailing = if line.end < part.words.len() {
        &part.words[line.end].before
    } else {
        &part.trailing
    };
    for gap in trailing {
        if gap.origin == TextSeparatorOrigin::Literal {
            visit(&gap.style);
        }
    }
}

fn geometry<M: FontMetrics, S: ImageSizer>(
    l: &Layouter<M, S>,
    part: &Prepared,
    line: Line,
) -> (f64, f64) {
    let (mut top, mut descent) = (0.0_f64, 0.0_f64);
    visit_styles(part, line, &mut |style| {
        let edges = l.metrics.text_edges(style.size, style);
        top = top.max(edges.0 .0);
        descent = descent.max(-edges.1 .0);
    });
    (top, descent)
}

fn certify<'a, M: FontMetrics, S: ImageSizer>(
    l: &Layouter<M, S>,
    lines: Lines<'a>,
) -> Option<Profile<'a>> {
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
    let mut leading = None;
    for index in 0..lines.len() {
        let (part, line) = lines.get(index);
        if line.start >= line.end || line.end > part.words.len() {
            return None;
        }
        let mut valid = true;
        let mut any = false;
        visit_styles(part, line, &mut |style| {
            if !valid {
                return;
            }
            any = true;
            if style.size != l.style.size
                || style.baseline_offset.resolve_pt(style.size.0) != 0.0
            {
                valid = false;
                return;
            }
            let (top, bottom) = l.metrics.text_edges(style.size, style);
            let gap = style
                .leading
                .map(|v| v.resolve_pt(style.size.0))
                .unwrap_or(style.size.0 * super::vanilla_defaults::PAR_LEADING);
            if [top.0, -bottom.0, gap].iter().any(|v| !v.is_finite() || *v < 0.0)
                || leading.is_some_and(|before| before != gap)
            {
                valid = false;
                return;
            }
            leading = Some(gap);
        });
        if !valid || !any {
            return None;
        }
    }
    let (widow, orphan) = crate::compiler::text_costs::prevention(&l.chain);
    Some(Profile { lines, leading: leading?, widow, orphan })
}

pub(super) fn certify_selected<'a, M: FontMetrics, S: ImageSizer>(
    l: &Layouter<M, S>,
    part: &'a Prepared,
    lines: &'a [Line],
) -> Option<Profile<'a>> {
    certify(l, Lines::Selected(part, lines))
}
pub(super) fn certify_prepared<'a, M: FontMetrics, S: ImageSizer>(
    l: &Layouter<M, S>,
    lines: &'a [Prepared],
) -> Option<Profile<'a>> {
    certify(l, Lines::Prepared(lines))
}

fn group_size(index: usize, len: usize, widow: bool, orphan: bool) -> usize {
    if index == 0 && len == 3 && widow && orphan {
        3
    } else if (index == 0 && len >= 2 && orphan)
        || (index >= 2 && index + 2 == len && widow)
    {
        2
    } else {
        1
    }
}

pub(super) fn admit<M: FontMetrics, S: ImageSizer>(
    l: &mut Layouter<M, S>,
    profile: Option<Profile<'_>>,
    index: usize,
    len: usize,
) {
    let Some(profile) = profile else {
        return;
    };
    let (part, line) = profile.lines.get(index);
    let (top, descent) = geometry(l, part, line);
    l.ensure_line_baseline(top);
    let baseline =
        l.regions.current.cursor_y.0 + top - l.line_assumed_ascent.unwrap_or(top);
    let group = group_size(index, len, profile.widow, profile.orphan);
    let mut tail = descent;
    for offset in 1..group {
        let (part, line) = profile.lines.get(index + offset);
        let (next_top, next_descent) = geometry(l, part, line);
        tail += profile.leading + next_top + next_descent;
    }
    let next_page = std::num::NonZeroUsize::new(l.pages.len() + 2).unwrap();
    let margin = PageMargins::resolve_for_page(
        l.page_config.margin_spec,
        l.page_config.auto_margin(),
        l.page_config.binding.resolve(l.page_config.page_dir),
        next_page,
    );
    let next_bottom = l.page_config.height - margin.bottom;
    let next_baseline = margin.top + top;
    if !l.current_page_is_empty()
        && baseline + tail > l.page_bottom_limit()
        && next_baseline + tail <= next_bottom
    {
        l.new_page();
    }
}

#[cfg(test)]
mod face_pagination_regressions {
    use crate::compiler::layout::{FixedMetrics, FontMetrics};
    use crate::entities::{
        content::Content,
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
        layout_types::{Length, PageMarginSpec, PageMargins, Pt, TextStyle},
        style::{Style, Styles},
        value::Value,
    };
    use comemo::Track;
    struct Faces;
    impl FontMetrics for Faces {
        fn advance(&self, text: &str, size: Pt, style: &TextStyle) -> Pt {
            FixedMetrics.advance(text, size, style)
        }
        fn vertical_metrics(&self, size: Pt, _: &TextStyle) -> (Pt, Pt) {
            (size * 1.2, size * 1.5)
        }
        fn cap_height(&self, size: Pt, style: &TextStyle) -> Pt {
            size * if style.bold {
                1.0
            } else if style.italic {
                0.6
            } else {
                0.7
            }
        }
        fn text_edges(&self, size: Pt, style: &TextStyle) -> (Pt, Pt) {
            let (top, bottom) = FixedMetrics.text_edges(size, style);
            (
                if style.top_edge.is_some() { top } else { self.cap_height(size, style) },
                bottom,
            )
        }
    }
    fn configured(
        intr: &dyn Introspector,
        baseline: f64,
        assumed: f64,
    ) -> super::super::Layouter<'_, Faces, NullImageSizer> {
        let mut l =
            super::super::Layouter::new(Faces, NullImageSizer, 10.0, intr.track());
        l.chain = l
            .chain
            .push_styles(&Styles::from_iter([Style::Size(Length::pt(10.0))]));
        l.style = TextStyle::from(&l.chain);
        l.page_config.width = 100.0;
        l.page_config.height = f64::INFINITY;
        l.page_config.margin = PageMargins::uniform(0.0);
        l.page_config.margin_spec = PageMarginSpec::uniform(0.0);
        l.regions.current.width = 100.0;
        l.regions.current.height = f64::INFINITY;
        l.regions.current.line_start_x = Pt(0.0);
        l.regions.current.cursor_x = Pt(20.0);
        l.initial_baseline_pending = false;
        l.regions.current.cursor_y = Pt(baseline);
        l.line_assumed_ascent = Some(assumed);
        l
    }
    #[test]
    fn face_edges_prepared_line_after_block_uses_certified_top() {
        use crate::entities::layout_types::{FrameItem, Point};
        let intr = TagIntrospector::empty();
        let mut l = configured(&intr, 7.0, 7.0);
        l.regions.current.cursor_x = Pt(0.0);
        l.page_config.height = 20.0;
        l.regions.current.height = 20.0;
        l.regions.current.current_items.push(FrameItem::Text {
            pos: Point { x: Pt(0.0), y: Pt(7.0) },
            text: "PREV".into(),
            style: TextStyle::regular(Pt(10.0)),
            bidi: None,
        });
        l.block_chain_active = true;
        l.prev_line_baseline = 7.0;
        l.prev_block_below_pending = 6.5;
        l.chain = l.chain.push_custom("par.linebreaks", Value::Str("simple".into()));
        for key in ["text.costs.widow", "text.costs.orphan"] {
            l.chain = l.chain.push_custom(
                key,
                Value::Ratio(crate::entities::layout_types::Ratio(0.0)),
            );
        }
        let body = Content::sequence(vec![
            Content::emph(Content::text("aaaa bbbb cccc")),
            Content::Space,
            Content::text("dddd"),
        ]);
        l.layout_content(&body);
        let doc = l.finish();
        let first = doc
            .pages
            .iter()
            .enumerate()
            .find_map(|(page, p)| {
                super::super::tests::frame_items_recursive(&p.items)
                    .into_iter()
                    .find_map(|i| match i {
                        FrameItem::Text { pos, text, .. } if text.as_str() == "aaaa" => {
                            Some((page, pos.y.0))
                        }
                        _ => None,
                    })
            })
            .unwrap();
        assert_eq!(first, (0, 19.5));
    }
    #[test]
    fn face_edges_mixed_faces_keep_the_last_two_lines_together() {
        let intr = TagIntrospector::empty();
        for enabled in [true, false] {
            let mut l = configured(&intr, 7.0, 7.0);
            l.regions.current.cursor_x = Pt(0.0);
            l.page_config.height = 40.0;
            l.regions.current.height = 40.0;
            l.chain = l.chain.push_custom("par.linebreaks", Value::Str("simple".into()));
            l.chain = l.chain.push_custom("text.hyphenate", Value::Bool(false));
            if !enabled {
                l.chain = l.chain.push_custom(
                    "text.costs.widow",
                    Value::Ratio(crate::entities::layout_types::Ratio(0.0)),
                );
                l.chain = l.chain.push_custom(
                    "text.costs.orphan",
                    Value::Ratio(crate::entities::layout_types::Ratio(0.0)),
                );
            }
            let body = Content::sequence(vec![
                Content::text("aaaa bbbb cccc "),
                Content::emph(Content::text("dddd eeee ffff gggg hhhh iiii jjjj")),
                Content::text(" kkkk"),
            ]);
            l.layout_content(&body);
            let doc = l.finish();
            let counts: Vec<_> = doc
                .pages
                .iter()
                .map(|p| p.plain_text().split_whitespace().count())
                .collect();
            assert_eq!(counts, if enabled { vec![6, 5] } else { vec![9, 2] });
        }
    }
}
